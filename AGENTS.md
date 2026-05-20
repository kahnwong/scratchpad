# AGENTS.md

## Project Overview

Scratchpad is a Rust GTK/libadwaita app using GtkSourceView for editing snippets with syntax highlighting, JSON/YAML formatting, search/replace, Flatpak packaging, and Debian packaging.

## Common Commands

- Check Rust formatting: `cargo fmt --check`
- Format Rust code: `cargo fmt`
- Check compilation: `cargo check`
- Run tests: `cargo test --all-features`
- Run locally: `cargo run`
- Install Flatpak locally: `make install`
- Run installed Flatpak: `make run-flatpak`
- Build Flatpak bundle: `make bundle`
- Build Debian package: `make deb`

## Packaging

- Flatpak manifest: `io.github.kahnwong.Scratchpad.yaml`
- Meson build file: `meson.build`
- Debian build script: `scripts/build-deb.sh`
- GitHub release packaging workflow: `.github/workflows/release.yaml`
- Flatpak builds should use Meson `--buildtype=release`.
- Debian builds should go through `scripts/build-deb.sh`, not duplicated Makefile logic.

## Code Guidance

- Keep UI changes small and consistent with the existing single-file GTK setup in `src/main.rs`.
- Prefer existing helpers such as `make_button` and `connect_lang_button` when adding language buttons.
- For new syntax highlighting support, use GtkSourceView language IDs through `LanguageManager`.
- Avoid adding dependencies unless the current GTK, libadwaita, GtkSourceView, or Rust standard APIs are insufficient.

## Verification

After Rust source changes, run:

```bash
cargo fmt --check
cargo check
```

After packaging changes, at minimum run the relevant dry-run or syntax checks, such as:

```bash
make -n bundle
make -n deb
bash -n scripts/build-deb.sh
```
