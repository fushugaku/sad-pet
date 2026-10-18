; Definitions are supplied by package_windows.py after validating both bundles.
[Setup]
AppId=audio.sadpet.plugins
AppName=SAD PET Plug-ins
AppVersion={#PluginVersion}
AppPublisher=SAD PET
AppPublisherURL=https://github.com/fushugaku/sad-pet
AppSupportURL=https://github.com/fushugaku/sad-pet/issues
DefaultDirName={autopf}\SAD PET\Plugins
DisableDirPage=yes
DisableProgramGroupPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
PrivilegesRequired=admin
OutputDir={#OutputDir}
OutputBaseFilename=SAD-PET-Plugins-Windows-x64-Setup
Compression=lzma2/normal
SolidCompression=yes
WizardStyle=modern
CloseApplications=no
RestartApplications=no
SetupMutex=SADPETPluginInstaller
UninstallDisplayName=SAD PET Plug-ins
VersionInfoVersion={#PluginVersion}

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"

[Messages]
english.WelcomeLabel2=Install the SAD PET instrument and SAD PET FX in VST3 format.%n%nClose your DAW before continuing. If WebView2 is missing, Setup downloads it from Microsoft; an internet connection is needed only for this step.
english.FinishedLabel=Reopen your DAW and rescan VST3 plug-ins. SAD PET and SAD PET FX are ready to use.
russian.WelcomeLabel2=Установка инструмента SAD PET и эффекта SAD PET FX в формате VST3.%n%nПеред продолжением закройте DAW. Если WebView2 отсутствует, установщик загрузит его с сайта Microsoft; для этого потребуется интернет.
russian.FinishedLabel=Откройте DAW и обновите список VST3-плагинов. SAD PET и SAD PET FX готовы к работе.

[Files]
Source: "{#SourceDir}\VST3\SAD PET.vst3\*"; DestDir: "{commoncf64}\VST3\SAD PET.vst3"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#SourceDir}\VST3\SAD PET FX.vst3\*"; DestDir: "{commoncf64}\VST3\SAD PET FX.vst3"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#SourceDir}\README.md"; DestDir: "{app}\Documentation"; Flags: ignoreversion
Source: "{#SourceDir}\Licenses\*"; DestDir: "{app}\Documentation\Licenses"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#PrerequisitesDir}\MicrosoftEdgeWebview2Setup.exe"; Flags: dontcopy
Source: "{#PrerequisitesDir}\vc_redist.x64.exe"; Flags: dontcopy

[InstallDelete]
Type: filesandordirs; Name: "{commoncf64}\VST3\SAD PET.vst3"
Type: filesandordirs; Name: "{commoncf64}\VST3\SAD PET FX.vst3"

[UninstallDelete]
Type: filesandordirs; Name: "{commoncf64}\VST3\SAD PET.vst3"
Type: filesandordirs; Name: "{commoncf64}\VST3\SAD PET FX.vst3"

[Code]
function WebViewAt(Root: Integer): Boolean;
var
  Version: String;
  Key: String;
begin
  Key := 'Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}';
  Result := RegQueryStringValue(Root, Key, 'pv', Version) and
            (Version <> '') and (Version <> '0.0.0.0');
end;

function WebViewInstalled: Boolean;
begin
  Result := WebViewAt(HKLM32) or WebViewAt(HKCU32) or WebViewAt(HKCU64);
end;

function VCRuntimeAt(Root: Integer): Boolean;
var
  Text: String;
  Installed, Required: Int64;
begin
  Result := False;
  if RegQueryStringValue(Root, 'Software\Microsoft\VisualStudio\14.0\VC\Runtimes\x64', 'Version', Text) then begin
    if Copy(Text, 1, 1) = 'v' then Delete(Text, 1, 1);
    if StrToVersion(Text, Installed) and StrToVersion('{#VCVersion}', Required) then
      Result := ComparePackedVersion(Installed, Required) >= 0;
  end;
end;

function VCRuntimeInstalled: Boolean;
begin
  Result := VCRuntimeAt(HKLM32) or VCRuntimeAt(HKLM64);
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  Code: Integer;
begin
  Result := '';
  if not VCRuntimeInstalled then begin
    ExtractTemporaryFile('vc_redist.x64.exe');
    if not Exec(ExpandConstant('{tmp}\vc_redist.x64.exe'), '/install /quiet /norestart', '', SW_HIDE, ewWaitUntilTerminated, Code) then
      Code := -1;
    if (Code <> 0) and (Code <> 3010) then begin
      Result := 'Microsoft Visual C++ Runtime could not be installed. Error: ' + IntToStr(Code);
      Exit;
    end;
    NeedsRestart := NeedsRestart or (Code = 3010);
  end;
  if not WebViewInstalled then begin
    ExtractTemporaryFile('MicrosoftEdgeWebview2Setup.exe');
    if not Exec(ExpandConstant('{tmp}\MicrosoftEdgeWebview2Setup.exe'), '/silent /install', '', SW_HIDE, ewWaitUntilTerminated, Code) then
      Code := -1;
    if (Code <> 0) or not WebViewInstalled then
      Result := 'Microsoft WebView2 could not be installed. Check your internet connection and retry. Error: ' + IntToStr(Code);
  end;
end;
