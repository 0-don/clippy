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
<!-- npx npm-check-updates -u -->
<!-- sea-orm-cli migrate fresh -v -d migration && sea-orm-cli generate entity -l -o ./entity/src --expanded-format --with-serde both -->

successor of [the electron clippy](https://github.com/0-don/clippy-ts)

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
