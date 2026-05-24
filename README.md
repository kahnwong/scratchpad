# scratchpad

Scratchpad for when you need to edit/note snippets.

## Features

- Syntax highlighting (JSON, YAML, Bash, Go, JavaScript, Rust, Python)
- Convert between JSON and YAML

## Install

See [releases](https://github.com/kahnwong/scratchpad/releases).

## Development

### Pre-reqs

```bash
sudo apt install -y \
    flatpak-builder \
    meson \
    ninja-build \
    libgtk-4-dev \
    libadwaita-1-dev \
    libgtksourceview-5-dev

flatpak install flathub org.gnome.Platform//50 org.gnome.Sdk//50 -y
```

### Run Locally

```bash
cargo run
```

## Screenshots

![json](docs/json.webp)
![rust](docs/rust.webp)
