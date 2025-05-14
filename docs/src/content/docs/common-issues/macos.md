---
title: MacOS
description: Common issues and solutions for MacOS users.
---

##### "clippy" is damaged and can't be opened. You should move it to the Trash.

![Mac Quarantine](/common-issues/mac_quarantine_black.png)

This is a common issue with MacOS. It happens when the app is downloaded from the internet and not from the App Store. To fix this, you need to remove the quarantine attribute from the app.

```bash
xattr -r -d com.apple.quarantine /Applications/clippy.app
```
