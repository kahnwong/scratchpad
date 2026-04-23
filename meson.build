project('scratchpad',
  version: '0.1.0',
  meson_version: '>= 0.62.0',
)

i18n = import('i18n')
cargo = find_program('cargo', required: true)

subdir('data')
subdir('po')

if get_option('buildtype') == 'release'
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_script = find_program('build-aux/cargo.sh')

cargo_build = custom_target(
  'scratchpad',
  build_by_default: true,
  build_always_stale: true,
  output: 'scratchpad',
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.project_source_root(),
    rust_target,
    '@OUTPUT@',
  ],
)
