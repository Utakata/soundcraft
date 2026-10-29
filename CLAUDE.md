Read `AGENTS.md`: it is the entry point for agents in this repo.

New workspace crates must include `[lints] workspace = true` in `Cargo.toml` to inherit
`[workspace.lints]`. Production crate roots must declare the required crate-level lint attributes
before modules or other crate items; crate-level `//!` documentation may precede them.

Display text in `soundcraft-ui-egui` goes through `i18n::tr("English text")` (or `i18n::trf` with
`{}` placeholders) where it is drawn; the English text stays the key, so ids, menu paths and
comparisons stay English. A new wrapped string needs a row in `crates/ui-egui/i18n/ja/*.tsv`
(`English<TAB>日本語`): `cargo test -p soundcraft-ui-egui --test japanese_coverage` lists the
drawn ones that lack it. Do not wrap user data (session, track, clip or plugin names) or numbers.
