# Maintenance

## What we support

- Current VCC on Windows, identified by `WebApp/Dist/buildInfo.json` (and the Dist asset hash).
- Older VCC: known English keys still translate. Unknown keys stay English and are listed as missing.
- Newer VCC: re-fingerprint, re-extract, merge new keys into `catalogs/zh-CN.json`. Do not ship a whole replacement `index-*.js` per version.

## Dist writes

The launcher owns Dist mutation:

1. Record original `index.html` bytes and hash.
2. Inject only if the hash is still the original or our last known inject.
3. Uninstall restores the recorded original.
4. If VCC overwrites Dist, treat it as a new fingerprint and ask before injecting again.

Never leave a foreign tool's script tag in Dist.

## Catalogs

- Living file: `catalogs/zh-CN.json` (English key to Simplified Chinese).
- `zh-TW` is a skeleton until a maintainer owns it.
- Extractor compares the current VCC bundle to the catalog. CI should fail or warn on missing keys for the pinned fixture version.
- Do not import another project's translation JSON.

## Console

Launcher output is English. Print detection (path, VCC version, OS, Windows language list, proposed catalog) before `[Y/n]`.

## Release

1. `cargo fmt`, `cargo clippy`, `cargo test` when those gates exist.
2. Bump version, `CHANGELOG.md`, tag `vX.Y.Z`.
3. Attach `vcc-i18n.exe`.
