# QR Forge GUI

A clean, fast desktop GUI for generating QR codes.  
Built with [egui](https://github.com/emilk/egui) and powered by the [qrforge](https://github.com/AhmaDREzA2022/qrforge) library.

![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)

## Features

- **Live preview** — QR code updates as you type
- **Error correction levels** — L / M / Q / H
- **Custom colors** — foreground & background color pickers
- **Invert mode** — quick dark-on-light / light-on-dark
- **Scale & quiet zone** — full control over size and border
- **Save as PNG** or **SVG**
- **Copy to clipboard**
- Dark theme by default
- Pure Rust — no Electron, no browser required

## Screenshots

<!-- Add a screenshot later -->
<!-- ![QR Forge GUI](docs/screenshot.png) -->

## Requirements

- Rust 1.70 or newer
- A working C compiler (for some native dependencies)

## Installation

```bash
git clone https://github.com/AhmaDREzA2022/qrforge-gui.git
cd qrforge-gui
cargo run --release
