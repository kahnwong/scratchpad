# scratchpad

Scratchpad for when you need to edit/note snippets.

## Pre-reqs

### apt

```bash
sudo apt install -y \
    flatpak-builder \
    meson \
    ninja-build \
    libgtk-4-dev \
    libadwaita-1-dev \
    libgtksourceview-5-dev
```

### Flatpak runtimes

```bash
flatpak install flathub org.gnome.Platform//48 org.gnome.Sdk//48 -y
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//24.08 -y
```

## Build & run

```bash
cargo run
```

## Local build (without Flatpak)

```bash
make run
```
