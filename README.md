<p align="center">
  <a href="https://github.com/0-don/clippy">
    <img src="public/clippy2.png" alt="Logo" width=400 />
  </a>
  <p align="center">
    <br />
    Clipboard Manager made with Tauri, Solid & Sea-Orm
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

successor of [the electron clippy](https://github.com/0-don/clippy-ts)

### Features

- display/hide **ctrl+y** or **ctrl+d** (change in settings)
- type out clipboard **ctrl+b** (where pasting isn't allowed)
- text, html, rtf, image, file support
- keybinds for everything & custom keybinds
- add favorite clipboards
- smart search, for links, colors, images, hex, etc.
- change/sync database location
- dark mode / white mode
- multilanguage support
- display scale

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
<!-- sudo dpkg -i ./code_1.96.2-1734607745_amd64.deb -->
<!-- sudo apt install pkg-config libglib2.0-dev libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev -->
