#!/bin/bash
set -euo pipefail

version="${VERSION:-0.1.0}"
arch="$(dpkg --print-architecture)"
deb_file="scratchpad_${version}_${arch}.deb"
root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$root_dir"

rm -rf deb-build deb-root "$deb_file"
meson setup deb-build --buildtype=release --prefix=/usr
meson compile -C deb-build
DESTDIR="$root_dir/deb-root" meson install -C deb-build

cat >deb-root/usr/bin/scratchpad-launcher.sh <<'EOF'
#!/bin/sh
if [ ! -e /dev/dri ] || [ -z "$(ls /dev/dri/ 2>/dev/null)" ]; then
	export GSK_RENDERER=cairo
fi
exec /usr/bin/scratchpad "$@"
EOF
chmod 755 deb-root/usr/bin/scratchpad-launcher.sh

mkdir -p deb-root/DEBIAN
cat >deb-root/DEBIAN/control <<EOF
Package: scratchpad
Version: $version
Section: utils
Priority: optional
Architecture: $arch
Maintainer: kahnwong
Depends: libgtk-4-1, libadwaita-1-0, libgtksourceview-5-0
Description: Scratchpad for editing snippets
EOF

dpkg-deb --build deb-root "$deb_file"
