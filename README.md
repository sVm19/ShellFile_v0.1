# ShellFile

Create files instantly from the Windows right-click menu.

## Download

Get ShellFile on Gumroad:  
https://shubhamster657.gumroad.com/l/shellfile

## Demo Video

[Watch the demo video](assets/demo/shellfile-demo.mp4)

## What ShellFile Does

- Adds **ShellFile** actions to the Windows folder background context menu
- Creates common dev files in one click
- Supports templates for pre-filled file content
- Generates project scaffolds (`python`, `node`, `rust`)
- Runs fully offline

## Quick Start

1. Download the installer from Gumroad.
2. Run `ShellFile-Setup.exe` as Administrator.
3. Right-click inside any folder background.
4. Create files instantly from the **ShellFile** menu.

## Build From Source

```bash
cargo build --release
```

Binary output:

`target/release/shellfile.exe`

## Package Installer (NSIS)

```powershell
& "C:\Program Files (x86)\NSIS\makensis.exe" installer\shellfile.nsi
```

Installer output:

`ShellFile-Setup.exe`

## License

MIT License. See [LICENSE.txt](LICENSE.txt).
