# VCC_I18N

Standalone Windows launcher that localizes VRChat Creator Companion (VCC).

VCC ships an English WebView UI. This tool detects the machine and the installed VCC, prints that in a console, then applies a translation catalog you confirm.

## Independent project

This repository is not VRChat, not ALCOM, and not any other VCC translator or patcher.

If screens, string lists, or hook ideas look familiar, that is because VCC's English UI is the same for everyone. That overlap is expected. This tree does not reuse another project's name, installer, injected script, screenshots, or translation files. Catalog keys are extracted from VCC, not imported from another repo.

Binary: `vcc-i18n.exe`. Injected marker: `vcc-i18n`.

## Status

v0.1.0 launcher: detect, confirm, install, uninstall. Catalog is Simplified Chinese for VCC 2.4.x UI strings.

## Use

```
cargo run --release -- --dry-run
cargo run --release -- --yes --no-launch
cargo run --release -- --uninstall --yes --no-launch
```

Or run `target/release/vcc-i18n.exe`.

- `--dry-run` prints the detect table only
- `--yes` accepts default prompts
- `--uninstall` restores `index.html`
- `--no-launch` does not start VCC
- `--vcc-dir PATH` overrides the registry install path

Re-run the exe after a successful install to be asked about uninstall.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Maintenance rules: [MAINTENANCE.md](./MAINTENANCE.md).

## License

[MIT](./LICENSE)
