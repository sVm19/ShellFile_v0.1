!define APP_NAME "ShellFile"
!define INSTALL_DIR "$PROGRAMFILES\ShellFile"
!define MUI_ICON "assets\icon.ico"

Name "${APP_NAME}"
OutFile "ShellFile_Installer.exe"
InstallDir "${INSTALL_DIR}"
RequestExecutionLevel admin

Section "Install"
  SetOutPath "${INSTALL_DIR}"
  File "..\target\release\shellfile.exe"
  
  SetOutPath "${INSTALL_DIR}\templates"
  File /r "..\templates\*.*"

  ; Register context menu
  ExecWait '"${INSTALL_DIR}\shellfile.exe" --register'

  ; Create AppData dir
  CreateDirectory "$LOCALAPPDATA\ShellFile"
  CreateDirectory "$LOCALAPPDATA\ShellFile\templates"
SectionEnd

Section "Uninstall"
  ExecWait '"${INSTALL_DIR}\shellfile.exe" --unregister'
  RMDir /r "${INSTALL_DIR}"
  RMDir /r "$LOCALAPPDATA\ShellFile"
SectionEnd
