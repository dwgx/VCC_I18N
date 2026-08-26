# Contributing to VCC_I18N

## Identity

VCC_I18N is an independent project. Contributions must not:

- Copy catalogs, injectors, installers, README blocks, or screenshots from other VCC tools
- Use names such as `vcc-auto-translate`, ALCOM, or CreatorCompanionPatcher as our artifacts
- Claim affiliation with VRChat or those tools

English UI keys must come from an installed VCC (or a fixture taken from VCC). Translating the same English sentence as someone else is fine. Pasting their JSON is not.

## Git

- Default branch: `main`
- Feature work: `feat/<short-topic>`
- Fixes: `fix/<short-topic>`
- Commits: [Conventional Commits](https://www.conventionalcommits.org/)
  - `feat:`, `fix:`, `docs:`, `chore:`, `ci:`, `test:`, `refactor:`
- Do not add AI attribution to commits or pull requests
- Pull requests target `main`

## CI

Pushes and pull requests to `main` run format, Clippy, and tests on Windows.

A tag `vMAJOR.MINOR.PATCH` that matches `Cargo.toml` publishes `vcc-i18n.exe` on GitHub Releases. See [MAINTENANCE.md](./MAINTENANCE.md).

## Versioning

Semantic Versioning. Releases are git tags `vMAJOR.MINOR.PATCH` plus a `CHANGELOG.md` entry.

## Pull requests

1. Fork `dwgx/VCC_I18N` (or a branch on this repo).
2. Keep the diff on one topic.
3. If you add UI strings to the launcher, they stay English.
4. If you add catalog keys, include the VCC version fingerprint they were taken from.
5. Open the pull request and wait for review.

## License

By contributing you agree the change is MIT, same as [LICENSE](./LICENSE).
