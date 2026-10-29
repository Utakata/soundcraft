# Localization parity

> **Last reviewed:** 2026-10-29 · **Last updated:** 2026-10-29 · **Change:** minor (Japanese display language) · **Target:** Avid Pro Tools Ultimate 2026.4.1

SoundCraft ships English and Japanese. Display strings stay English literals in Rust
(`crates/engine/src/cmd/*`, `crates/ui-egui/src/*`) and are translated where the UI draws them:
`crates/ui-egui/src/i18n.rs` looks the English text up in per-language tables
(`crates/ui-egui/i18n/ja/*.tsv`, `English<TAB>translation`), so ids, menu paths and comparisons stay
English. The language is the Language setting in the Appearance window, or the `ui.language` command
(`{"lang": "en" | "ja"}`), saved in `ui.json`. Engine error text is translated by exact match and
by `{}` templates. Pro Tools ships its UI in **8 languages** (bundle `.lproj` folders: English,
German, Spanish, French, Japanese, Korean, Simplified Chinese, Traditional Chinese).

**Localization: ~23 %** (measured: 2 of the target's 8 UI languages; Japanese covers the menus,
the Edit and Mix windows, the panels, the dialogs and engine errors, but has had no native-speaker
review). Remaining ≈ 55–90 h: extending the lookup to the strings not yet routed through it
(command descriptions, some tooltips, plugin parameter names: ≈ 8–15 h), a font fallback for
Devanagari/Arabic (PR #141 open), RTL layout and complex-script shaping (egui has neither:
≈ 25–40 h, or ship Arabic/Hindi menus-only), then ≈ 3–5 h of agent translation per language, plus
native-speaker review (human). Adding a language is a new `Language` variant plus its tables.

Open work: PR #143 (French and Spanish, a Language setting), PR #141 (bundled multilingual font
fallbacks), issue #67 (Ukrainian).

| Language | Code | UI strings translated | Dialogs / tooltips / help | Script support | Native review | Status | Est. to full (h) | In Pro Tools |
|---|---|---|---|---|---|---|---:|---|
| English | en | all (source) | ✅ | Latin ✅ | — | **full** | 0 | ✅ |
| Simplified Chinese | zh-Hans | 0 (0 %) | ❌ | ❌ no CJK font fallback, no IME testing | no | none | 6–10 | ✅ |
| Spanish | es | 0 (0 %) — PR #143 pending | ❌ | Latin ✅ | no | none | 3–5 | ✅ |
| Hindi | hi | 0 (0 %) | ❌ | ❌ no Devanagari font or shaping | no | none | 8–15 | ❌ |
| Arabic | ar | 0 (0 %) | ❌ | ❌ no RTL layout, no shaping | no | none | 15–25 | ❌ |
| French | fr | 0 (0 %) — PR #143 pending | ❌ | Latin ✅ | no | none | 3–5 | ✅ |
| Portuguese | pt | 0 (0 %) | ❌ | Latin ✅ | no | none | 3–5 | ❌ |
| Indonesian | id | 0 (0 %) | ❌ | Latin ✅ | no | none | 3–5 | ❌ |
| Japanese | ja | ~1,390 rows: every menu leaf and every label drawn in the Edit/Mix windows, panels and dialogs (tests enforce both) | 🟡 dialogs ✅, engine errors ✅, some tooltips and command descriptions English | ✅ Noto Sans JP bundled (desktop and web); IME untested | no | **partial** | 2–4 | ✅ |
| German | de | 0 (0 %) | ❌ | Latin ✅ | no | none | 3–5 | ✅ |
| Korean | ko | 0 (0 %) | ❌ | ❌ no Hangul font fallback | no | none | 6–10 | ✅ |
| Vietnamese | vi | 0 (0 %) | ❌ | 🟡 Latin with stacked diacritics, untested | no | none | 3–5 | ❌ |

Other languages shipped: **1** (Japanese, partial). (Pro Tools also ships Traditional Chinese, zh-Hant.)

Per-language hours exclude the shared catalog and script work above.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-29 | minor | Japanese display language: lookup in `i18n.rs`, tables in `crates/ui-egui/i18n/ja`, `ui.language`, bundled Noto Sans JP; ~12 % → ~23 % |
| 2026-10-10 | major | Created: English only, no catalog; the twelve key languages at 0 %; Pro Tools' 8 languages listed from its bundle |
