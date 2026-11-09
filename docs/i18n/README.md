# Documentation localization · 文件多語化

**Currently maintained / 目前維護：**
- [English](../../README.md) — canonical source
- [繁體中文 zh-TW](../../README.zh-TW.md) — full translation

**Future languages / 未來語言：** Japanese `ja-JP`, Korean `ko-KR`,
Simplified Chinese `zh-CN`, Spanish `es-ES` and others as contributors join.
These are **planned**, not existing files. Do not add dead navigation links.

## File naming

- Root README: `README.md` (en), `README.<language-tag>.md` (translations).
- Changelog: `CHANGELOG.md`, `CHANGELOG.<language-tag>.md`.
- Contributions: `CONTRIBUTING.md`, `CONTRIBUTING.<language-tag>.md`.
- Release notes: `docs/releases/vX.Y.Z-alpha.N.md` + `...<language-tag>.md`.
- Standalone guides can be translated under `docs/<language-tag>/` as needed.

## Contribution workflow

1. Create an Issue describing the new language and your translation scope.
2. Copy the English document to the appropriate locale filename.
3. Translate all essential sections **without changing feature-status meaning**.
4. Keep links to technical documentation working; maintain safety/consent warnings.
5. Add the new language to existing README language navigation **only after the file exists**.
6. Submit a PR; tag future release-note changes to remain bilingual where practical.

Feature status, OS requirements, license warnings and privacy language must
stay in sync between English and translated versions. Avoid automatic
translation of code samples, paths, model names or API field names.
