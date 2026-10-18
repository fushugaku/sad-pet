import hashlib
from pathlib import Path
import plistlib
import tempfile
import unittest
import zipfile

from publish_release import PACKAGES, stage_assets, validate_tag


class ReleaseAssets(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.artifacts = self.root / "artifacts"
        self.artifacts.mkdir()
        self.output = self.root / "output"
        self.version = validate_tag("plugins-v0.3.1")
        (self.artifacts / PACKAGES[0]).write_bytes(b"xar!package")
        (self.artifacts / PACKAGES[2]).write_bytes(b"MZinstaller")
        self.mac_archive()
        self.windows_archive(self.version)
        self.checksums()

    def mac_archive(self):
        with zipfile.ZipFile(self.artifacts / PACKAGES[1], "w") as archive:
            for name in ("SAD PET", "SAD PET FX"):
                archive.writestr(f"SAD-PET-Plugins-macOS-universal/AU/{name}.component/Contents/Info.plist",
                                 plistlib.dumps({"CFBundleShortVersionString": self.version}))

    def windows_archive(self, version):
        with zipfile.ZipFile(self.artifacts / PACKAGES[3], "w") as archive:
            for name in ("SAD PET", "SAD PET FX"):
                archive.writestr(f"SAD-PET-Plugins-Windows-x64/VST3/{name}.vst3/Contents/Resources/moduleinfo.json",
                                 '{"Version":"' + version + '",}')

    def checksums(self):
        for name in PACKAGES:
            checksum = hashlib.sha256((self.artifacts / name).read_bytes()).hexdigest()
            (self.artifacts / (name + ".sha256")).write_text(f"{checksum}  {name}\n")

    def test_both_systems_and_all_checksums_are_staged_together(self):
        files = stage_assets(self.artifacts, self.output, self.version)
        self.assertEqual(len(files), 9)
        self.assertEqual(len((self.output / "SHA256SUMS.txt").read_text().splitlines()), 4)

    def test_bad_installer_checksum_blocks_every_asset(self):
        (self.artifacts / PACKAGES[2]).write_bytes(b"MZcorrupted")
        with self.assertRaisesRegex(ValueError, "Checksum mismatch"):
            stage_assets(self.artifacts, self.output, self.version)
        self.assertFalse(self.output.exists())

    def test_mixed_plugin_versions_block_every_asset(self):
        self.windows_archive("0.3.0")
        self.checksums()
        with self.assertRaisesRegex(ValueError, "Expected 0.3.1"):
            stage_assets(self.artifacts, self.output, self.version)
        self.assertFalse(self.output.exists())

    def test_missing_system_and_wrong_release_tag_are_rejected(self):
        (self.artifacts / PACKAGES[0]).unlink()
        with self.assertRaisesRegex(ValueError, "Expected one"):
            stage_assets(self.artifacts, self.output, self.version)
        with self.assertRaisesRegex(ValueError, "Expected release tag"):
            validate_tag("plugins-v99.0.0")


if __name__ == "__main__":
    unittest.main()
