# Changelog

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
