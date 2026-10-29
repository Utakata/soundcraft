# Localization parity

> **Last reviewed:** 2026-10-23 · **Last updated:** 2026-10-23 · **Change:** major (Japanese display language) · **Target:** Avid Pro Tools Ultimate 2026.4.1

SoundCraft ships English and a partial Japanese (Setup › Appearance › Language, or the `ui.language`
command). Strings stay English literals in Rust and are translated where they are drawn, with
`i18n::tr` (`crates/ui-egui/src/i18n.rs`), from `English<TAB>translation` tables in
`crates/ui-egui/i18n/ja/*.tsv`. Command ids, menu paths and engine messages stay English. Japanese
today: all 514 menu labels, the application menu, Undo/Redo, the Appearance window and about 560 strings in
windows, dialogs and panels (a test fails when a label drawn in the Edit or Mix window, a panel or a main dialog has no translation). Not yet: the engine's session-info text report and engine error messages,
and a native-speaker review. Fonts: Noto Sans JP (SIL OFL, `assets/fonts/`) is bundled, so Japanese draws the same on every platform, the web build included; only Windows rendering has been checked on a real machine. Pro Tools ships its UI in
**8 languages** (bundle `.lproj` folders: English, German, Spanish, French, Japanese, Korean,
Simplified Chinese, Traditional Chinese).

**Localization: ~20 %** (estimate: English is complete, Japanese is partial, the target has 8 UI
languages). Remaining ≈ 60–100 h: wrapping the remaining literals (≈ 10–15 h), a font fallback for
Devanagari/Arabic and the web (PR #141 open), RTL layout and complex-script shaping (egui has neither:
≈ 25–40 h, or ship Arabic/Hindi menus-only), then ≈ 3–5 h of agent translation per language, plus
native-speaker review (human).

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
| Japanese | ja | menus 514 labels (100 %), app menu, Appearance, about 560 window, dialog and panel strings | 🟡 partial | 🟡 Noto Sans JP bundled (Windows checked; macOS, Linux, web not), no IME testing | no | partial | 4–8 | ✅ |
| German | de | 0 (0 %) | ❌ | Latin ✅ | no | none | 3–5 | ✅ |
| Korean | ko | 0 (0 %) | ❌ | ❌ no Hangul font fallback | no | none | 6–10 | ✅ |
| Vietnamese | vi | 0 (0 %) | ❌ | 🟡 Latin with stacked diacritics, untested | no | none | 3–5 | ❌ |

Other languages shipped: **0**. (Pro Tools also ships Traditional Chinese, zh-Hant.)

Per-language hours exclude the shared catalog and script work above.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-10 | major | Created: English only, no catalog; the twelve key languages at 0 %; Pro Tools' 8 languages listed from its bundle |
| 2026-10-23 | major | Japanese added: `i18n` module and TSV tables, Language setting and `ui.language`, menus, macOS native menu, Appearance and window strings, CJK font fallback |
