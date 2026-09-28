---
title: Commands
description: Run your own scripts on the clipboard with a hotkey.
---

Commands bind a hotkey to a script. The script gets the current clipboard text, and its output can be copied back or pasted straight into the window you are working in. Use them to transform text, call a CLI tool, or control another app.

![Commands settings](/features/commands.webp)

## Creating a command

Open `Settings > Commands` and click **Add command**. Each command has:

| Field       | What it does                                                                |
| ----------- | --------------------------------------------------------------------------- |
| Name        | Shown in error notifications                                                |
| Toggle      | Turns the command on or off                                                 |
| Hotkey      | Ctrl, Alt, Shift, Super plus a key. Works while Clippy is hidden            |
| Output      | What happens with the script's output (see below)                           |
| Interpreter | Program that runs the script: `sh`, `bash`, `python3`, `node`, `powershell` |
| Script      | The script itself, typed inline                                             |

Changes are saved when you leave a field. **Test** runs the script right away with the current clipboard and shows its output or error.

## One script per operating system

Every command has a Linux, Windows and macOS script, because commands [sync](/features/cloud-sync) between your devices. A hotkey runs the script for the system it is pressed on, and does nothing where that script is empty. A dot on the tab marks the systems that have a script.

Leave the interpreter empty to use the default: `sh` on Linux and macOS, `powershell` on Windows. Any program on your `PATH` works; arguments are allowed, for example `python3 -u`.

## Input

The clipboard text is passed two ways:

- on **stdin**, always complete
- in the **`CLIPPY_TEXT`** environment variable (`$env:CLIPPY_TEXT` in PowerShell), empty for text over 32,000 characters because of Windows limits

## Output

| Mode          | Result                                                           |
| ------------- | ---------------------------------------------------------------- |
| Ignore output | Nothing, for commands that only have side effects                |
| Copy output   | Stdout replaces the clipboard and shows up in your history       |
| Paste output  | Stdout is copied, then pasted into the window you are working in |

One trailing newline is removed, so `echo` output pastes cleanly. Empty output leaves the clipboard alone.

If the script exits with an error or runs longer than 30 seconds, Clippy shows a notification with its error output and does not touch the clipboard.

## Examples

### Uppercase the clipboard

Output: **Copy output**

```sh
# Linux, macOS (sh)
tr '[:lower:]' '[:upper:]'
```

```powershell
# Windows (powershell)
[Console]::In.ReadToEnd().ToUpper()
```

### Pretty print JSON

Output: **Paste output**

```python
# Linux, macOS (python3)
import json, sys
print(json.dumps(json.load(sys.stdin), indent=2))
```

```powershell
# Windows (powershell)
[Console]::In.ReadToEnd() | ConvertFrom-Json | ConvertTo-Json -Depth 20
```

### Control Spotify with Trashbin+

[Trashbin+](https://github.com/0-don/trashbin-plus) ships scripts that trash the current song, skip, like and more through Spotify's debugging port. Paste [`trashbin.sh`](https://github.com/0-don/trashbin-plus/blob/main/scripts/trashbin.sh) as the Linux and macOS script with interpreter `bash`, and [`trashbin.ps1`](https://github.com/0-don/trashbin-plus/blob/main/scripts/trashbin.ps1) as the Windows script with interpreter `powershell`. Output: **Ignore output**. The Trashbin+ README lists the Spotify launch flags and the other actions.

## Tips

- Pick hotkeys that nothing else uses. Settings warns when a combination is already taken by Clippy.
- On Linux, `Ctrl + Alt + F1` to `F12` switch virtual terminals and never reach Clippy.
- `sh` is not `bash` on every distribution. Set the interpreter to `bash` if your script uses bash features.
