# Changelog

## 0.3.0 - 2026-11-04

- Replace a stale `index.html.vcc-i18n-orig` when VCC has replaced `index.html`, so re-injecting after an update keeps the new bundle instead of reverting to the previous one.
- Stop translating the `jsx`/`jsxs` key argument: `proxy(fn, translateChildren)` only walks extra arguments for `createElement`.
- Translate `header`, `description`, `alt`, and `aria-label` props; extraction and injection now cover the same key list.
- Inject a `white-space: nowrap` style for `.fui-Title*` / `.fui-Subtitle*` so CJK page titles stop wrapping one glyph per line.
- Add `--report-keys [--out PATH]`: read-only catalog coverage (missing and unused keys) as console output and JSON.
- Add `--version`.

## 0.2.0 - 2026-08-26

- Detect portable VCC installs (`CreatorCompanionBeta.exe` plus Dist) when the registry path is missing.
- Print pre-release versions such as `2.5.0-beta.2` and the Dist `index-*.js` fingerprint.
- Merge VCC 2.5 UI strings into the living zh-CN catalog (launch parameters, external links, changelog, fragments).
- List catalog keys still missing from the current Dist bundle.
- Windows CI on `main`. Pushing a `vMAJOR.MINOR.PATCH` tag publishes `vcc-i18n.exe`.

## 0.1.0 - 2026-08-26

- Detect Windows language list, VCC path, version, and Dist state.
- Print an English console table, then confirm install or uninstall.
- Transactional Dist inject/restore using `index.html.vcc-i18n-orig`.
- Runtime jsx/jsxs/createElement proxy with a zh-CN catalog taken from VCC 2.4.5 UI strings.
