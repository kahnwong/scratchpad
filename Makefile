install:
	flatpak-builder --user --install --force-clean build-dir io.github.kahnwong.Scratchpad.yaml

bundle:
	flatpak-builder --force-clean --repo=repo build-dir io.github.kahnwong.Scratchpad.yaml
	flatpak build-bundle repo scratchpad.flatpak io.github.kahnwong.Scratchpad

run-flatpak:
	flatpak run io.github.kahnwong.Scratchpad
