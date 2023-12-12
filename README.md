<p align="center">
  <a href="https://github.com/don-cryptus/clippy">
    <img src="public/clippy2.png" alt="Logo" width=400 />
  </a>
  <p align="center">
    <br />
    Clipboard Manager made with Tauri, Solid & Sea-Orm.
    <br />
    <a href="https://github.com/Don-Cryptus/clippy/releases/latest">Try it out</a>
    ·
    <a href="https://github.com/Don-Cryptus/clippy/issues">Report Bug</a>
    ·
    <a href="https://github.com/Don-Cryptus/clippy/issues">Request Feature</a>
    <br />
  </p>
  <p align="center">
    <img src="public/clippy-showcase.webp" alt="Logo" >
  </p>
</p>
<!-- npx npm-check-updates -u -->
<!-- sea-orm-cli migrate fresh -v -d migration && sea-orm-cli generate entity -l -o ./entity/src --expanded-format --with-serde both -->

successor of [the electron clippy](https://github.com/Don-Cryptus/clippy-ts)

### Features

- display/hide **ctrl+y** or **ctrl+d** (change in settings)
- type out clipboard **ctrl+b** (where pasting isn't allowed)
- images & text support
- keybinds for everything
- add favorite clipboards
- smart search for links, colors, images, hex
- instant search
- change database location
- change keyboard bindings
- dark mode / white mode

### Prerequisites Fedora Linux

```bash
sudo dnf install javascriptcoregtk4.0 webkit2gtk4.0 libxdo libappindicator-gtk3 xdotool
```

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

#### note

Tested on Linux(x11), Windows
on Mac still lots of issues with hotkeys you probably need to change the hotkey