---
title: MacOS
description: Common issues and solutions for MacOS users.
---

##### "clippy" is damaged and can't be opened. You should move it to the Trash.

![Mac Quarantine](/common-issues/mac_quarantine_black.png)

```bash
xattr -r -d com.apple.quarantine /Applications/clippy.app
```
