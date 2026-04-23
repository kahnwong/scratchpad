install:
	flatpak-builder --user --install --force-clean build-dir io.github.kahnwong.Scratchpad.yaml
run-flatpak: install
	flatpak run io.github.kahnwong.Scratchpad
