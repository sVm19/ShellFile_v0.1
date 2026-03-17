; ShellFile NSIS installer
; Build with: makensis installer\shellfile.nsi

!include "MUI2.nsh"

!define PROJECT_ROOT ".."
!define APP_NAME "ShellFile"
!define APP_VERSION "1.0.0"
!define APP_PUBLISHER "ShellFile Developer"
!define INSTALL_DIR "$PROGRAMFILES\ShellFile"
!define UNINSTALL_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\ShellFile"

Name "${APP_NAME}"
OutFile "${PROJECT_ROOT}\ShellFile-Setup.exe"
InstallDir "${INSTALL_DIR}"
InstallDirRegKey HKLM "${UNINSTALL_KEY}" "InstallLocation"
RequestExecutionLevel admin

;Icon "${PROJECT_ROOT}\assets\icon.ico"
;UninstallIcon "${PROJECT_ROOT}\assets\icon.ico"
SetCompressor /SOLID lzma

Var StartMenuFolder
Var RegisterExitCode
Var UnregisterExitCode

!define MUI_ABORTWARNING
;!define MUI_ICON "..\assets\icon.ico"
;!define MUI_UNICON "${PROJECT_ROOT}\assets\icon.ico"
!define MUI_FINISHPAGE_RUN
!define MUI_FINISHPAGE_RUN_TEXT "Open installation folder"
!define MUI_FINISHPAGE_RUN_FUNCTION OpenInstallDir

!insertmacro MUI_PAGE_WELCOME
!if /FileExists "${PROJECT_ROOT}\LICENSE.txt"
  !insertmacro MUI_PAGE_LICENSE "${PROJECT_ROOT}\LICENSE.txt"
!else
  !warning "LICENSE.txt not found. License page is skipped until LICENSE.txt is added."
!endif
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "Install"
  SetShellVarContext all

  SetOutPath "$INSTDIR"
  File "${PROJECT_ROOT}\target\release\shellfile.exe"

  SetOutPath "$INSTDIR\templates"
  File /r "${PROJECT_ROOT}\templates\*.*"

  CreateDirectory "$LOCALAPPDATA\ShellFile"

  WriteUninstaller "$INSTDIR\Uninstall.exe"

  StrCpy $StartMenuFolder "$SMPROGRAMS\ShellFile"
  CreateDirectory "$StartMenuFolder"
  CreateShortcut "$StartMenuFolder\ShellFile.lnk" "$INSTDIR\shellfile.exe"
  CreateShortcut "$StartMenuFolder\Uninstall ShellFile.lnk" "$INSTDIR\Uninstall.exe"

  WriteRegStr HKLM "${UNINSTALL_KEY}" "DisplayName" "${APP_NAME}"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "DisplayVersion" "${APP_VERSION}"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "Publisher" "${APP_PUBLISHER}"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
  WriteRegStr HKLM "${UNINSTALL_KEY}" "InstallLocation" "$INSTDIR"

  ExecWait '"$INSTDIR\shellfile.exe" --register' $RegisterExitCode
  DetailPrint "shellfile --register exit code: $RegisterExitCode"
SectionEnd

Section "Uninstall"
  SetShellVarContext all

  MessageBox MB_ICONQUESTION|MB_YESNO "Are you sure you want to uninstall ${APP_NAME}?" IDYES +2
  Abort

  IfFileExists "$INSTDIR\shellfile.exe" 0 +3
  ExecWait '"$INSTDIR\shellfile.exe" --unregister' $UnregisterExitCode
  DetailPrint "shellfile --unregister exit code: $UnregisterExitCode"

  Delete "$SMPROGRAMS\ShellFile\ShellFile.lnk"
  Delete "$SMPROGRAMS\ShellFile\Uninstall ShellFile.lnk"
  RMDir "$SMPROGRAMS\ShellFile"

  RMDir /r "$LOCALAPPDATA\ShellFile"
  RMDir /r "$INSTDIR"

  DeleteRegKey HKLM "${UNINSTALL_KEY}"
SectionEnd

Function OpenInstallDir
  ExecShell "open" "$INSTDIR"
FunctionEnd
