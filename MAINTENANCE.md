# Maintenance

## What we support

- Current VCC on Windows, identified by `WebApp/Dist/buildInfo.json` (and the Dist `index-*.js` name).
- Official installs (registry `InstallPath`) and portable copies (`CreatorCompanion.exe` or `CreatorCompanionBeta.exe` next to `WebApp/Dist`).
- Older VCC: known English keys still translate. Unknown keys stay English and are listed as missing.
- Newer VCC: re-fingerprint, re-extract, merge new keys into `catalogs/zh-CN.json`. Do not ship a whole replacement `index-*.js` per version.
- 2.5.0-beta.2 is a supported fingerprint. Keep 2.4 keys.

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

## After a VCC update

1. Run `vcc-i18n.exe --dry-run` (or `cargo run -- --dry-run`) against the new Dist.
2. If Dist is `clean` because VCC replaced `index.html`, confirm install again.
3. If the table lists missing keys, add translations to `catalogs/zh-CN.json` from those English strings (taken from this VCC, not from another project).
4. Keep old keys. Bump `Cargo.toml` + `CHANGELOG.md`, then cut a release.

## Console

Launcher output is English. Print detection (path, VCC version, OS, Windows language list, proposed catalog) before `[Y/n]`.

## Release

Version in `Cargo.toml` must match the git tag `vMAJOR.MINOR.PATCH`.

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. Bump `Cargo.toml` / `Cargo.lock`, add a `CHANGELOG.md` section, commit with Conventional Commits.
5. `git tag -a vX.Y.Z -m "vX.Y.Z"`
6. `git push origin main --follow-tags`

Pushing the tag runs `.github/workflows/release.yml`: it builds `vcc-i18n.exe` on `windows-latest` and attaches it to the GitHub Release.

Local package (same layout as CI):

```
powershell -File scripts/package-release.ps1
```

That writes `dist/vcc-i18n.exe` and `dist/SHA256SUMS.txt`. Do not commit `dist/`.
