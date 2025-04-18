# -*- mode: python ; coding: utf-8 -*-

a = Analysis(
    ['./main.py'],
    pathex=[],
    binaries=[],
    datas=[
    ('./resources', './resources'),
    ('./modules', './modules'),
    ],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
    noconfirm=True,
    optimize=0,
)

pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name='glang',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch='universal2',
    codesign_identity=None,
    entitlements_file=None,
)

coll = COLLECT(
    exe,
    a.binaries,
    a.datas,
    strip=False,
    upx=True,
    upx_exclude=[],
    name='glang',
)

app = BUNDLE(
    coll,
    name='glang.app',
    icon='resources/icons/george_language_icon.icns',
    bundle_identifier=None,
)
