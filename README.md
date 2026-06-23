<div align="center">
  <h1>MdViewer</h1>
  <p><strong>A beautiful, portable Markdown viewer for Windows</strong></p>
  <p>
    <img src="https://img.shields.io/badge/Rust-1.96+-orange?logo=rust" alt="Rust">
    <img src="https://img.shields.io/badge/platform-Windows-blue?logo=windows" alt="Windows">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="MIT">
    <img src="https://img.shields.io/badge/size-6.6%20MB-important" alt="Size">
  </p>
  <p>
    <a href="#features">Features</a> •
    <a href="#download">Download</a> •
    <a href="#usage">Usage</a> •
    <a href="#build-from-source">Build</a> •
    <a href="#license">License</a>
  </p>
  <br>
  <p>
    <img src="https://placehold.co/800x500/1a1a2e/ffffff?text=MdViewer+Screenshot" alt="MdViewer Screenshot" width="80%">
  </p>
</div>

---

## Features

- **Portable** — Single 6.6 MB EXE, zero dependencies, no installation required
- **Live Preview** — Renders CommonMark with headings, lists, tables, code blocks, images
- **Edit Mode** — Switch to a built-in code editor to modify files and save directly
- **Ctrl+Click Links** — Links highlight on hover; open in browser only with Ctrl+Click
- **Dark & Light Themes** — Toggle between elegant dark and light modes
- **Font Controls** — Increase/decrease font size (60%–200%), switch between Proportional and Monospace
- **Drag & Drop** — Drop any `.md` file onto the window
- **Keyboard Shortcuts** — `Ctrl+O` to open, `Ctrl+S` to save

## Download

| File | Size | Description |
|------|------|-------------|
| [mdviewer.exe](https://github.com/peter14l/mdviewer/releases/latest/download/mdviewer.exe) | 6.6 MB | Portable executable for Windows (x86_64) |

Grab the latest from the [Releases](https://github.com/peter14l/mdviewer/releases) page.

## Usage

**Open a file:**
```cmd
mdviewer.exe README.md
```
Or launch the app and use `Ctrl+O` / drag-and-drop.

**Read vs Edit:**
- **Read mode** — Rendered markdown preview (default)
- **Edit mode** — Click the ✏️ `Edit` button in the toolbar to edit the raw markdown; `Ctrl+S` or `Save` to write changes back

**Links:**
- Hover over a link to see the URL tooltip and underline highlight
- **Ctrl+Left Click** to open in your default browser

## Build from Source

```bash
# Prerequisites: Rust 1.96+
git clone https://github.com/peter14l/mdviewer.git
cd mdviewer

# Build the portable EXE
cargo build --release

# The binary is at:
# ./target/release/mdviewer.exe
```

The EXE is self-contained — copy it anywhere and run.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| GUI | [egui](https://github.com/emilk/egui) (immediate-mode, native) |
| Framework | [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) |
| Markdown | [egui_commonmark](https://github.com/lampsitter/egui_commonmark) + [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) |
| File Dialog | [rfd](https://github.com/PolyMeilex/rfd) (native Windows) |
| Rendering | OpenGL via glow backend |

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">
  <a href="https://ko-fi.com/A0A8HZQZN">
    <img src="https://ko-fi.com/img/githubbutton_sm.svg" alt="ko-fi" width="200">
  </a>
  <br>
  <sub>If you find this tool useful, consider buying me a coffee ❤️</sub>
</div>
