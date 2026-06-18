install_data(
  'me.karnwong.Scratchpad.desktop',
  install_dir: get_option('datadir') / 'applications',
)

install_data(
  'me.karnwong.Scratchpad.metainfo.xml',
  install_dir: get_option('datadir') / 'metainfo',
)

icon_sizes = ['16x16', '48x48', '128x128', '256x256']
foreach size : icon_sizes
  install_data(
    'icons' / 'hicolor' / size / 'apps' / 'me.karnwong.Scratchpad.png',
    install_dir: get_option('datadir') / 'icons' / 'hicolor' / size / 'apps'
  )
endforeach

# Keep the scalable SVG as the master source
# install_data(
#   'icons/hicolor/scalable/apps/me.karnwong.Scratchpad.svg',
#   install_dir: get_option('datadir') / 'icons' / 'hicolor' / 'scalable' / 'apps'
# )
