APP_ID := io.github.kahnwong.Scratchpad

install:
	flatpak-builder --user --install --force-clean build-dir $(APP_ID).yaml

bundle:
	flatpak-builder --install-deps-from=flathub --force-clean --repo=repo build-dir $(APP_ID).yaml
	flatpak build-bundle repo scratchpad.flatpak $(APP_ID)

deb:
	scripts/build-deb.sh

run-flatpak:
	flatpak run $(APP_ID)
