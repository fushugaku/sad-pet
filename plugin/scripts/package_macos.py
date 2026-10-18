#!/usr/bin/env python3
"""Create a macOS Installer package from the staged AU and VST3 bundles."""
import argparse
import hashlib
from pathlib import Path
import plistlib
import shutil
import subprocess
import tempfile
import xml.etree.ElementTree as ET


PACKAGE_ID = "audio.sadpet.pkg.plugins"


def run(*args):
    subprocess.run([str(arg) for arg in args], check=True)


def build_pkg(source: Path) -> Path:
    source = source.resolve()
    destination = source.parent / f"{source.name}.pkg"
    bundles = [
        (source / kind / (name + extension), Path("Library/Audio/Plug-Ins") / folder)
        for kind, extension, folder in (("AU", ".component", "Components"), ("VST3", ".vst3", "VST3"))
        for name in ("SAD PET", "SAD PET FX")
    ]
    versions = set()
    architectures = set()
    for bundle, _ in bundles:
        info = plistlib.loads((bundle / "Contents/Info.plist").read_bytes())
        versions.add(info["CFBundleShortVersionString"])
        executable = bundle / "Contents/MacOS" / info["CFBundleExecutable"]
        arches = subprocess.check_output(["lipo", "-archs", str(executable)], text=True)
        architectures.add(tuple(sorted(arches.split())))
    if len(versions) != 1:
        raise ValueError("All four plug-in bundles must have the same version")
    if len(architectures) != 1:
        raise ValueError("All four plug-in bundles must support the same architectures")
    version = versions.pop()
    host_architectures = ",".join(architectures.pop())

    with tempfile.TemporaryDirectory(prefix="sad-pet-installer-") as temporary:
        work = Path(temporary)
        scripts = work / "scripts"
        scripts.mkdir()
        postinstall = scripts / "postinstall"
        # CurrentUserHomeDirectory runs this as the installing user, never root.
        postinstall.write_text(
            "#!/bin/sh\n"
            "plugin_install_root=\"$2\"\n"
            "[ -d \"$plugin_install_root/Library/Audio/Plug-Ins/Components/SAD PET.component\" ] || plugin_install_root=\"$HOME\"\n"
            "# Installer preserves payload timestamps. Mark upgraded AUs as new\n"
            "# so a negative discovery cache from replacement is invalidated.\n"
            "for bundle in \"$plugin_install_root/Library/Audio/Plug-Ins/Components/SAD PET.component\" \"$plugin_install_root/Library/Audio/Plug-Ins/Components/SAD PET FX.component\" \"$plugin_install_root/Library/Audio/Plug-Ins/Components\"; do\n"
            "    [ ! -d \"$bundle\" ] || /usr/bin/touch \"$bundle\"\n"
            "done\n"
            "# Refresh AU discovery for this user; hosts are reopened separately.\n"
            "/usr/bin/killall AudioComponentRegistrar 2>/dev/null || true\n"
            "exit 0\n",
            encoding="utf-8",
        )
        postinstall.chmod(0o755)
        packages = []
        # AU and VST3 share bundle identifiers. Separate component packages keep
        # Installer's upgrade bookkeeping from conflating their two install paths.
        for kind in ("AU", "VST3"):
            payload = work / kind / "payload"
            components = []
            for bundle, folder in bundles:
                if bundle.parent.name != kind:
                    continue
                relative = folder / bundle.name
                shutil.copytree(bundle, payload / relative)
                components.append({
                    "RootRelativeBundlePath": relative.as_posix(),
                    "BundleIsRelocatable": False,
                    # Also replace the same version when the user reinstalls it.
                    "BundleIsVersionChecked": False,
                    "BundleHasStrictIdentifier": True,
                    "BundleOverwriteAction": "upgrade",
                })
            component_plist = work / kind / "components.plist"
            component_plist.write_bytes(plistlib.dumps(components))
            script_options = []
            if kind == "AU":
                documentation = payload / "Library/Application Support/SAD PET/Documentation"
                documentation.mkdir(parents=True)
                shutil.copy2(source / "README.md", documentation / "README.md")
                shutil.copytree(source / "Licenses", documentation / "Licenses")
                script_options = ["--scripts", scripts]
            component = work / f"{kind.lower()}.pkg"
            identifier = f"{PACKAGE_ID}.{kind.lower()}"
            run("pkgbuild", "--root", payload, "--component-plist", component_plist,
                "--identifier", identifier, "--version", version,
                "--install-location", "/", "--ownership", "recommended",
                *script_options, component)
            packages.append((identifier, component))

        resources = work / "resources"
        messages = {
            "en": (
                f"SAD PET {version}",
                "SAD PET instrument and SAD PET FX, in Audio Unit and VST3 formats.",
                "Close your DAW before continuing. The plug-ins will be installed for your user account.",
                "Installation complete",
                "Reopen your DAW and rescan plug-ins. In Logic, SAD PET FX is under AU MIDI-controlled Effects.",
            ),
            "ru": (
                f"SAD PET {version}",
                "Инструмент SAD PET и эффект SAD PET FX в форматах Audio Unit и VST3.",
                "Перед продолжением закройте DAW. Плагины будут установлены для вашей учётной записи.",
                "Установка завершена",
                "Откройте DAW и обновите список плагинов. В Logic эффект SAD PET FX находится в AU MIDI-controlled Effects.",
            ),
        }
        for language, (title, description, instructions, complete, rescan) in messages.items():
            localized = resources / f"{language}.lproj"
            localized.mkdir(parents=True)
            for filename, heading, paragraphs in (
                ("welcome.html", title, (description, instructions)),
                ("conclusion.html", complete, (rescan,)),
            ):
                html = (
                    '<!doctype html><html><head><meta charset="utf-8">'
                    '<style>body{font:13px -apple-system,sans-serif;color:#222;'
                    'margin:16px 20px;line-height:1.5}h1{font-size:22px;font-weight:600;'
                    'margin:0 0 18px}p{margin:0 0 14px}code{font-size:11px}</style>'
                    f"</head><body><h1>{heading}</h1>"
                    + "".join(f"<p>{paragraph}</p>" for paragraph in paragraphs)
                    + "<p><code>~/Library/Audio/Plug-Ins/Components<br>"
                    "~/Library/Audio/Plug-Ins/VST3</code></p></body></html>"
                )
                (localized / filename).write_text(html, encoding="utf-8")

        distribution = ET.Element("installer-gui-script", minSpecVersion="2")
        ET.SubElement(distribution, "title").text = "SAD PET"
        ET.SubElement(distribution, "welcome", file="welcome.html", **{"mime-type": "text/html"})
        ET.SubElement(distribution, "conclusion", file="conclusion.html", **{"mime-type": "text/html"})
        ET.SubElement(distribution, "options", customize="never", **{
            "require-scripts": "false", "hostArchitectures": host_architectures,
        })
        # Keep the existing Install.command destination and avoid shadow copies
        # between /Library and ~/Library. Installer does not need administrator access.
        ET.SubElement(distribution, "domains", enable_anywhere="false",
                      enable_currentUserHome="true", enable_localSystem="false")
        outline = ET.SubElement(distribution, "choices-outline")
        ET.SubElement(outline, "line", choice="plugins")
        choice = ET.SubElement(distribution, "choice", id="plugins", visible="false",
                               title="SAD PET + SAD PET FX", description="Audio Units and VST3")
        for identifier, component in packages:
            ET.SubElement(choice, "pkg-ref", id=identifier)
            ET.SubElement(distribution, "pkg-ref", id=identifier, version=version,
                          onConclusion="None").text = component.name
        distribution_path = work / "Distribution.xml"
        ET.indent(distribution)
        ET.ElementTree(distribution).write(distribution_path, encoding="utf-8", xml_declaration=True)
        run("productbuild", "--distribution", distribution_path, "--package-path", work,
            "--resources", resources, destination)

    checksum = hashlib.sha256(destination.read_bytes()).hexdigest()
    destination.with_suffix(".pkg.sha256").write_text(f"{checksum}  {destination.name}\n")
    print(f"Packaged {destination}")
    return destination


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path, help="Staged SAD-PET-Plugins-macOS-* directory")
    build_pkg(parser.parse_args().source)
