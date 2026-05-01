#define MyAppName "George Language"
#ifndef MyAppVersion
#define MyAppVersion "dev"
#endif

#define MyAppPublisher "The George Language Foundation"
#define MyAppExeName "glang.exe"
#define MyAppURL "https://github.com/george-language/glang"

#define MyAppAssocName MyAppName + " File"
#define MyAppAssocExt ".glang"
#define MyAppAssocKey StringChange(MyAppAssocName, " ", "") + MyAppAssocExt


[Setup]
; No admin required — user-local install only
PrivilegesRequired=lowest

AppId={{1AA7E52E-8F6B-475E-B3B4-6C4FC061B20D}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}

AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}

; Install to ~/.glang style location
DefaultDirName={%USERPROFILE%}\.glang

ChangesAssociations=yes
ChangesEnvironment=yes
DisableProgramGroupPage=yes

LicenseFile=LICENSE

OutputDir=dist
OutputBaseFilename=GeorgeLanguage

Compression=lzma
SolidCompression=yes
WizardStyle=modern


[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"


[Tasks]
Name: "addtopath"; \
    Description: "Add glang to PATH"; \
    Flags: unchecked

Name: "desktopicon"; \
    Description: "{cm:CreateDesktopIcon}"; \
    GroupDescription: "{cm:AdditionalIcons}"; \
    Flags: unchecked


[Files]
; Install executable into ~/.glang/bin
Source: "target\x86_64-pc-windows-msvc\release\{#MyAppExeName}"; \
    DestDir: "{app}\bin"; \
    Flags: ignoreversion


[Registry]

; -----------------------------
; File association (.glang)
; -----------------------------

Root: HKA; \
    Subkey: "Software\Classes\{#MyAppAssocExt}\OpenWithProgids"; \
    ValueType: string; \
    ValueName: "{#MyAppAssocKey}"; \
    ValueData: ""; \
    Flags: uninsdeletevalue

Root: HKA; \
    Subkey: "Software\Classes\{#MyAppAssocKey}"; \
    ValueType: string; \
    ValueName: ""; \
    ValueData: "{#MyAppAssocName}"; \
    Flags: uninsdeletekey

Root: HKA; \
    Subkey: "Software\Classes\{#MyAppAssocKey}\DefaultIcon"; \
    ValueType: string; \
    ValueName: ""; \
    ValueData: "{app}\bin\{#MyAppExeName},0"

Root: HKA; \
    Subkey: "Software\Classes\{#MyAppAssocKey}\shell\open\command"; \
    ValueType: string; \
    ValueName: ""; \
    ValueData: """{app}\bin\{#MyAppExeName}"" ""%1"""

Root: HKA; \
    Subkey: "Software\Classes\Applications\{#MyAppExeName}\SupportedTypes"; \
    ValueType: string; \
    ValueName: "{#MyAppAssocExt}"; \
    ValueData: ""


; -----------------------------
; Add ~/.glang/bin to USER PATH
; -----------------------------

Root: HKCU; \
    Subkey: "Environment"; \
    ValueType: expandsz; \
    ValueName: "Path"; \
    ValueData: "{olddata};{app}\bin"; \
    Flags: preservestringtype; \
    Tasks: addtopath


[Icons]

Name: "{autoprograms}\{#MyAppName}"; \
    Filename: "{app}\bin\{#MyAppExeName}"

Name: "{autodesktop}\{#MyAppName}"; \
    Filename: "{app}\bin\{#MyAppExeName}"; \
    Tasks: desktopicon


[Run]

Filename: "{app}\bin\{#MyAppExeName}"; \
    Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; \
    Flags: nowait postinstall runascurrentuser skipifsilent
