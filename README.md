# VCC_I18N

Standalone Windows launcher that localizes VRChat Creator Companion (VCC).

VCC ships an English WebView UI. This tool detects the machine and the installed VCC, prints that in a console, then applies a translation catalog you confirm.

## Independent project

This repository is not VRChat, not ALCOM, and not any other VCC translator or patcher.

If screens, string lists, or hook ideas look familiar, that is because VCC's English UI is the same for everyone. That overlap is expected. This tree does not reuse another project's name, installer, injected script, screenshots, or translation files. Catalog keys are extracted from VCC, not imported from another repo.

Binary: `vcc-i18n.exe`. Injected marker: `vcc-i18n`.

## Status

v0.3.0 launcher: detect, confirm, install, uninstall, key-coverage report.

Catalog is Simplified Chinese for VCC 2.4.x and 2.5.x UI strings, including portable `2.5.0-beta.2`.

## Download

Get `vcc-i18n.exe` from [GitHub Releases](https://github.com/dwgx/VCC_I18N/releases/latest).

No extra runtime. After a VCC update, download the latest release and run it again.

## Use

```
vcc-i18n.exe
vcc-i18n.exe --dry-run
vcc-i18n.exe --report-keys
vcc-i18n.exe --report-keys --out coverage.json
vcc-i18n.exe --version
vcc-i18n.exe --yes --no-launch
vcc-i18n.exe --uninstall --yes --no-launch
```

From a clone:

```
cargo run --release -- --dry-run
```

- `--dry-run` prints the detect table only
- `--report-keys` prints catalog coverage for the current Dist bundle (missing and unused keys) and never writes into Dist
- `--out PATH` with `--report-keys` writes the coverage report as JSON
- `--version` prints `vcc-i18n <version>`
- `--yes` accepts default prompts
- `--uninstall` restores `index.html`
- `--no-launch` does not start VCC
- `--vcc-dir PATH` overrides auto-detect (registry, then portable folders under `D:\Software` and Program Files)

Re-run the exe after a successful install to be asked about uninstall.

## 使用

1. 从 [Releases](https://github.com/dwgx/VCC_I18N/releases/latest) 下载 `vcc-i18n.exe`。
2. 运行。窗口先打印 Windows 语言和 VCC 检测表，再问 `[Y/n]`。
3. 确认后写入 VCC 的 `WebApp/Dist`（会备份 `index.html.vcc-i18n-orig`）。
4. 卸载：再运行 exe，或 `vcc-i18n.exe --uninstall`。
5. VCC 升级后 Dist 会被覆盖：再跑一次最新 exe 即可。

支持本机安装版和便携版（`CreatorCompanion.exe` / `CreatorCompanionBeta.exe`）。

## Supported VCC

- Official Windows install (registry `InstallPath`)
- Portable copies next to `WebApp/Dist`
- Fingerprint: `WebApp/Dist/buildInfo.json` and the Dist `index-*.js` name
- Known English keys stay translated on older VCC. Unknown keys stay English and are listed as missing
- Newer VCC: re-run, merge missing keys into `catalogs/zh-CN.json`, then cut a new tag

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Maintenance and release: [MAINTENANCE.md](./MAINTENANCE.md). Changelog: [CHANGELOG.md](./CHANGELOG.md).

## License

[MIT](./LICENSE)
