<p align="center">
  <a href="https://github.com/0-don/clippy">
    <img src="public/clippy2.png" alt="Logo" width=400 />
  </a>
  <p align="center">
    <br />
    clipboard manager with sync & encryption
    <br />
    <a href="https://github.com/0-don/clippy/releases/latest">Try it out</a>
    ·
    <a href="https://github.com/0-don/clippy/issues">Report Bug</a>
    ·
    <a href="https://github.com/0-don/clippy/issues">Request Feature</a>
    <br />
  </p>
  <p align="center">
    <img src="public/clippy-showcase.webp" alt="Logo" >
  </p>
</p>

<div align="center">

<a href="https://github.com/0-don/clippy/releases/download/v1.3.1/clippy_1.3.1_x64-setup.exe">
  <img src="./public/windows.png"> Windows (x64)
</a>
•
<a href="https://github.com/0-don/clippy/releases/download/v1.3.1/clippy_1.3.1_x64-setup.exe">
  Windows (arm64)
</a>
<br>
<a href="https://github.com/0-don/clippy/releases/download/v1.4.0/clippy_1.4.0_amd64.deb">
  <img src="./public/linux.png"> Linux (deb)
</a>
•
<a href="https://github.com/0PandaDEV/Qopy/releases/download/v0.3.1/Qopy-0.3.1.rpm">
  Linux (rpm)
</a>
•
<a href="https://github.com/0-don/clippy/releases/download/v1.4.0/clippy-bin-1.4.0-1-x86_64.pkg.tar.zst">
  Linux (zst)
</a>
•
<a href="https://github.com/0-don/clippy/releases/download/v1.4.0/clippy_1.4.0_amd64.AppImage">
  Linux (AppImage)
</a>
<br>
<a href="https://github.com/0-don/clippy/releases/download/v1.4.0/clippy_1.4.0_aarch64.dmg">
  <img src="./public/apple.png"> macOS (Silicon)
</a>
•
<a href="https://github.com/0-don/clippy/releases/download/v1.4.0/clippy_1.4.0_aarch64.dmg">
  macOS (Intel)
</a>
<br>
<br>

</div>

### Features

- **Multi-content support:**
  - Text, HTML, RTF support
  - Image support with thumbnails
  - File support with metadata
- **Smart clipboard features:**
  - Type out clipboard content (where pasting isn't allowed) **ctrl+b**
  - Smart search for links, colors, images, hex codes etc.
  - Add favorite clipboards
  - Clear history by type
- **Security & Privacy:**
  - End-to-end encryption support
  - Password protection
  - Configurable size limits for different content types
- **Cloud sync:**
  - Google Drive integration
  - Sync favorites and history
  - Configurable sync limits
- **Customization:**
  - Global hotkeys for all functions
  - Custom keybinds
  - Adjustable display scale
  - Dark/Light mode
  - Multiple languages support
  - Configurable window positions
  - Database location customization
- **System Integration:**
  - Autostart option
  - System tray support
  - Display toggle with **ctrl+y** or **ctrl+d**

### Prerequisites Development

Before you begin, ensure you have met the following requirements:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Installation and Running the Application

1. **Install Dependencies:**
   First, install the necessary Node.js dependencies:

   ```bash
   npm install
   ```

2. **Run the Application in Watch Mode:**
   To start the Tauri application in watch mode, use:
   ```bash
   npm run d
   ```

#### tested on

- Linux(x11) KDE Plasma (Disable Focus Stealing Prevention)
- Windows
- Mac (hotkeys needs to be changed in settings)

<!-- DEBIAN GNOME X11 -->
<!-- su - -->
<!-- usermod -aG sudo don -->
<!-- echo "disable wayland" && sudo nano /etc/gdm3/daemon.conf -->
<!-- sudo apt install ./clippy_1.3.0_amd64.deb -->
<!-- sudo apt install pkg-config libglib2.0-dev libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev -->

<!-- OPENSUSE KDE X11 -->
<!-- sudo zypper --non-interactive install -t pattern devel_basis && sudo zypper --non-interactive install glib2-devel webkit2gtk3-devel gtk3-devel libopenssl-devel xdotool-devel libayatana-appindicator3-1 libayatana-appindicator3-devel -->
<!-- sudo zypper install ./clippy-1.3.0-1.x86_64.rpm -->

<!-- ENDEVOUROS KDE X11 -->
<!-- sudo pacman -U clippy-bin-1.3.0-1-x86_64.pkg.tar.zst -->

<!-- git reset --hard origin/master -->
