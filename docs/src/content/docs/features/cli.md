---
title: Command line
description: Read, copy and add clipboards, and run commands, from a terminal.
---

While Clippy is running, the `clippy` executable also works as a command line tool. It talks to the running app, so it never opens a second window and it sees the same history, including encrypted clipboards once Clippy is unlocked.

```sh
clippy list                 # the 10 newest clipboards
clippy get                  # print the newest clipboard
clippy get 3                # print the third newest
clippy copy 3               # put it back on the system clipboard
clippy add "some text"      # copy text
echo "piped" | clippy add   # copy whatever comes in on stdin
clippy edit 2               # change a clipboard in your $EDITOR
clippy edit 2 "new text"    # or replace it directly
clippy star 1               # add to favorites
clippy search invoice -n 20 # search the history
clippy run "Trash song"     # run a command as if its hotkey was pressed
```

## Commands

| Command                      | What it does                                                         |
| ---------------------------- | -------------------------------------------------------------------- |
| `list [-n COUNT] [--json]`   | Shows recent clipboards, newest first. Default 10, at most 1000      |
| `search QUERY [-n COUNT]`    | Searches the history, same matching as the search tab                |
| `get [INDEX\|ID]`            | Prints the full text of a clipboard. Index 1 is the newest           |
| `copy [INDEX\|ID]`           | Puts a clipboard back on the system clipboard in its original format |
| `add [TEXT]`                 | Copies text, read from stdin when TEXT is missing                    |
| `edit INDEX\|ID [TEXT]`      | Replaces a text clipboard. Without TEXT it reads stdin, or opens `$VISUAL`, `$EDITOR` (Notepad on Windows, `vi` otherwise) with the current text |
| `rename INDEX\|ID [NAME]`    | Names a clipboard, without NAME the name is removed                  |
| `star` / `unstar INDEX\|ID`  | Adds to or removes from favorites                                    |
| `delete INDEX\|ID`           | Deletes a clipboard, here and in sync                                |
| `run NAME`                   | Runs an enabled command by name, including its "Run on" setting      |
| `help`                       | Prints usage                                                         |

`--json` on `list` and `search` prints id, types, star flag, creation time and a preview for each clipboard, which is handy for scripts. An ID from that output can be passed to `get` and `copy` instead of an index.

## Exit codes

| Code | Meaning                                       |
| ---- | --------------------------------------------- |
| 0    | Success                                       |
| 1    | The request failed, the reason is on stderr   |
| 2    | Clippy is not running                         |

When encryption is on and Clippy has not been unlocked yet, every command except `add` and `run` fails with exit code 1.

Commands that change a clipboard (`edit`, `rename`, `star`, `unstar`, `delete`) need an explicit index or ID.

## How it connects

On start, Clippy listens on a random port on `127.0.0.1` and writes the port and a random token to `cli.json` in its data folder, readable only by your user. The command line tool reads that file and sends the token with every request, so other users on the same machine cannot use it.
