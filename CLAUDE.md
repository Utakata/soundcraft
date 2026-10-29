# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Read [`AGENTS.md`](AGENTS.md) first: it is the entry point for agents in this repo** (session start, the asset and clean-room rules, never-crash rules, layering, "everything is a command", quality gates, release, credits). This file adds only a command cheat sheet and a map of how the pieces connect. [`docs/architecture.md`](docs/architecture.md) covers the session document and the audio path.

## Commands

- All gates, before every commit: `cargo xtask ci` = `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, then `cargo xtask assets`, `cargo xtask layers`, `cargo xtask wasm` (needs `rustup target add wasm32-unknown-unknown`). Each of those can be run on its own.
- Packages are `soundcraft-<dir>` (`crates/engine` → `soundcraft-engine`); apps are `soundcraft`, `soundcraft-cli`, `soundcraft-web`.
- One crate: `cargo test -p soundcraft-engine`. One integration-test file (`crates/<crate>/tests/<name>.rs`): `cargo test -p soundcraft-ui-egui --test appearance`. One test: `cargo test -p soundcraft-engine <name-substring>` (append `-- --exact` for an exact path).
- `cargo xtask parity` regenerates `docs/parity-checklist.md` (never hand-edit it); `cargo xtask version [set X.Y.Z]`.
- Web: `cd apps/soundcraft-web && trunk serve --release`. The UI crates need Rust ≥ 1.95 (core crates declare 1.90). Linux needs the ALSA headers.
- `SOUNDCRAFT_NO_PREFS=1`: scripted/test runs neither read nor write `ui.json` and write no log file. `SOUNDCRAFT_NO_SYSTEM_FONTS=1`: only egui's bundled fonts. `RUST_LOG` sets log levels.
- Style: rustfmt `max_width = 150`, so long one-line expressions are normal; `clippy.toml` allows unwrap/expect/panic/indexing in tests only.

## How the pieces connect

- **Dispatch.** Menus, shortcuts, the control channel and MCP all end in `SoundApp::run(id, params)` (`crates/ui-egui/src/lib.rs`). UI-only ids (declared in `UI_COMMANDS`, e.g. `ui.theme`) are handled by `menus::run_ui_command` (`crates/ui-egui/src/menus.rs`); everything else goes to `Engine::execute` (`crates/engine/src/lib.rs`), which checks `enabled` (skipped when the params name explicit targets such as `track`/`clips`/`start`), runs the command inside `catch_unwind`, restores the document on `Err` or panic, pushes an undo step named after the spec's `label`, and appends to `journal`. Continuous gestures use `execute_merged(id, params, key)` so a drag is one undo step.
- **Defining commands.** The `cmd!` macro in `crates/engine/src/cmd/mod.rs`: plain = undoable and journaled; `noundo` = journaled only; `query` = neither; `view` = may only touch `edit.zoom`. Each module's `specs()` is listed in `command_specs()`.
- **Menus and parity.** `menus::tree()` builds the menu bar from `crates/engine/catalog/menus.txt` plus a few UI-added paths. A leaf is live when a `CommandSpec` has the same menu path and label, or when `catalog::ALIASES` (engine) or `menus::ui_aliases()` (UI) maps the path to an id; the same match drives `engine.parity` and the `parity_floor` test. English labels and paths are identifiers: code compares them (`root.label == "Clip"`) and kittest tests find widgets by them.
- **UI state.** `UiState` (`crates/ui-egui/src/lib.rs`; `ui.set` patches it) is saved as `ui.json` in the settings directory by the desktop app (`apps/soundcraft/src/main.rs`, `config.rs`). It is `#[serde(default)]`, so a new field only needs a value in `impl Default for UiState` for older files to load.
- **UI tests.** `crates/ui-egui/tests/*.rs` drive the real app headlessly with `egui_kittest`: `SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default())` in a `Harness` whose frame calls `app.logic(&ctx); app.ui(ui);`, then query by accesskit role or label. To look at a change, render it with the `ui_shot` example; its script lines are control-channel requests.
- **Display strings.** Text the user reads is wrapped at the point it is drawn: `i18n::tr("English")`, or `i18n::trf("Undo {}", &[…])` for `{}` placeholders (`crates/ui-egui/src/i18n.rs`). The key is the English text, so ids, menu paths and comparisons stay English. Japanese is in `crates/ui-egui/i18n/ja/*.tsv` (`English<TAB>日本語`). A key in two tables must have one translation, and every menu label must have one; both are unit tests in `i18n.rs`. `tests/japanese_coverage.rs` fails when a label drawn in the Edit or Mix window, a panel or a main dialog has no translation (its `painted_english` audit, run with `--ignored`, lists English text on screen). The language is `UiState::language`, set with `ui.language {"lang": "en" | "ja"}` or Setup › Appearance.
- **Fonts.** Only Noto Sans JP is bundled (listed in ATTRIBUTION.md): `crates/ui-egui/src/fonts.rs` loads the OS UI fonts at runtime (via `fc-match` on Linux/BSD) and appends the bundled Noto Sans JP (`assets/fonts/`) and a Hangul fallback to every family.
- **Build-time data.** `crates/ui-egui/build.rs` bakes `contributors/*.json|toml` into the About window; the engine `include_str!`s the menu catalog.
