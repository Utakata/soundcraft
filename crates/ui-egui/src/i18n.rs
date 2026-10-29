//! The display language. Only what is drawn changes: command ids, menu paths and any label that
//! code compares stay English, and a string is translated where it is shown, with [`tr`]. The
//! Japanese text lives in `i18n/ja/*.tsv` (one `English<TAB>日本語` table per area), so a
//! translator can work on a table without touching Rust.

use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};

/// The interface language (`ui.language`, Setup › Appearance). English is the source language.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Language {
    #[default]
    #[serde(rename = "en")]
    English = 0,
    #[serde(rename = "ja")]
    Japanese = 1,
}

impl Language {
    pub const ALL: [Self; 2] = [Self::English, Self::Japanese];

    /// The name shown in the Language menu, in that language.
    pub const fn label(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Japanese => "日本語",
        }
    }

    /// The id used in `ui.language {"lang": …}` and in the saved UI prefs.
    pub const fn id(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Japanese => "ja",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|l| l.id().eq_ignore_ascii_case(s.trim()))
    }
}

/// The language being drawn, set from `UiState::language` at the start of each frame.
static CURRENT: AtomicU8 = AtomicU8::new(Language::English as u8);

pub fn set(lang: Language) {
    CURRENT.store(lang as u8, Ordering::Relaxed);
}

pub fn current() -> Language {
    if CURRENT.load(Ordering::Relaxed) == Language::Japanese as u8 { Language::Japanese } else { Language::English }
}

/// The Japanese tables, merged. Each file covers one area, so translators can work in parallel.
const JA_TABLES: &[&str] = &[
    include_str!("../i18n/ja/menus-file-edit-clip.tsv"),
    include_str!("../i18n/ja/menus-view-window.tsv"),
    include_str!("../i18n/ja/menus-track-event-suite-options-setup.tsv"),
    include_str!("../i18n/ja/ui-chrome.tsv"),
    include_str!("../i18n/ja/ui-windows.tsv"),
    include_str!("../i18n/ja/ui-labels.tsv"),
    include_str!("../i18n/ja/ui-errors.tsv"),
];

/// The `English<TAB>translation` pairs in one table. Blank lines and `#` comments (a `#` line with
/// no tab, so a label like `# OF AUDIO FILES:` is still an entry) are skipped, and so is anything
/// malformed: a bad line leaves its label in English rather than failing.
fn entries(text: &str) -> impl Iterator<Item = (&str, &str)> {
    text.lines()
        .map(|line| line.trim_end_matches('\r'))
        .filter(|line| !line.is_empty() && !(line.starts_with('#') && !line.contains('\t')))
        .filter_map(|line| line.split_once('\t'))
        .filter(|(en, ja)| !en.is_empty() && !ja.trim().is_empty())
        .map(|(en, ja)| (en, ja.trim()))
}

fn japanese() -> &'static HashMap<&'static str, &'static str> {
    static JA: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    JA.get_or_init(|| {
        let mut map = HashMap::new();
        for (en, ja) in JA_TABLES.iter().copied().flat_map(entries) {
            map.entry(en).or_insert(ja);
        }
        map
    })
}

/// `en` in `lang`: its Japanese text when there is one, otherwise `en` itself.
pub fn translate(lang: Language, en: &str) -> &str {
    match lang {
        Language::English => en,
        Language::Japanese => japanese().get(en).copied().unwrap_or_else(|| {
            note_missing(en);
            en
        }),
    }
}

/// Whether strings with no translation are remembered; see [`missing`]. Off unless a test or tool
/// asks, so drawing stays free of locking.
static REPORT: AtomicBool = AtomicBool::new(false);
static MISSED: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

/// Start (or stop) remembering the strings that were looked up in Japanese and not found.
pub fn report_missing(on: bool) {
    REPORT.store(on, Ordering::Relaxed);
}

/// The strings remembered since [`report_missing`] was switched on, sorted.
pub fn missing() -> Vec<String> {
    MISSED.lock().unwrap_or_else(PoisonError::into_inner).iter().cloned().collect()
}

fn note_missing(en: &str) {
    if REPORT.load(Ordering::Relaxed) && en.chars().any(|c| c.is_alphabetic()) {
        MISSED.lock().unwrap_or_else(PoisonError::into_inner).insert(en.to_owned());
    }
}

/// [`translate`] in the current language.
pub fn tr(en: &str) -> &str {
    translate(current(), en)
}

/// [`tr`] for text that mixes fixed labels with user data (names, paths): a string with no
/// translation is data, not a gap, so it is not remembered by [`report_missing`].
pub fn tr_data(en: &str) -> &str {
    match current() {
        Language::English => en,
        Language::Japanese => japanese().get(en).copied().unwrap_or(en),
    }
}

/// The templates among the Japanese entries: keys with `{}` placeholders, as the fixed pieces around
/// them, with their translation. The one with the most fixed text comes first, so the most specific
/// template wins.
fn templates() -> &'static [(Vec<&'static str>, &'static str)] {
    static TEMPLATES: OnceLock<Vec<(Vec<&'static str>, &'static str)>> = OnceLock::new();
    TEMPLATES.get_or_init(|| {
        let mut all: Vec<(Vec<&'static str>, &'static str)> = japanese()
            .iter()
            .filter(|(en, _)| en.contains("{}"))
            .map(|(en, ja)| (en.split("{}").collect::<Vec<_>>(), *ja))
            // Two placeholders in a row cannot be told apart, and a template with almost no fixed text
            // would match anything.
            .filter(|(pieces, _)| {
                let inner = pieces.get(1..pieces.len().saturating_sub(1)).unwrap_or_default();
                inner.iter().all(|piece| !piece.is_empty()) && pieces.iter().map(|piece| piece.chars().count()).sum::<usize>() >= 5
            })
            .collect();
        all.sort_by(|a, b| {
            let (la, lb) = (a.0.iter().map(|p| p.len()).sum::<usize>(), b.0.iter().map(|p| p.len()).sum::<usize>());
            lb.cmp(&la).then_with(|| a.0.cmp(&b.0))
        });
        all
    })
}

/// What a template's placeholders stand for in `text`, in order, or `None` when `text` is not an
/// instance of the template (`pieces` are the fixed texts around the placeholders).
fn capture<'a>(pieces: &[&str], text: &'a str) -> Option<Vec<&'a str>> {
    let (first, rest) = pieces.split_first()?;
    let (last, middle) = rest.split_last()?;
    let mut s = text.strip_prefix(first)?.strip_suffix(last)?;
    let mut captured = Vec::with_capacity(rest.len());
    for piece in middle {
        let at = s.find(piece)?;
        captured.push(s.get(..at)?);
        s = s.get(at + piece.len()..)?;
    }
    captured.push(s);
    Some(captured)
}

/// An engine message in `lang`: its exact translation, else the translation of the template it is an
/// instance of with the parts it carries filled in, else the message itself.
pub fn translate_message(lang: Language, en: &str) -> Cow<'_, str> {
    if lang == Language::English {
        return Cow::Borrowed(en);
    }
    if let Some(ja) = japanese().get(en) {
        return Cow::Borrowed(*ja);
    }
    for (pieces, ja) in templates() {
        if let Some(parts) = capture(pieces, en) {
            return Cow::Owned(fill(ja, &parts));
        }
    }
    Cow::Borrowed(en)
}

/// [`translate_message`] in the current language.
pub fn tr_message(en: &str) -> Cow<'_, str> {
    translate_message(current(), en)
}

/// [`tr`] for text with `{}` placeholders, filled in order from `args`. A translation keeps the
/// placeholders in the same order.
pub fn trf(en: &str, args: &[&str]) -> String {
    fill(tr(en), args)
}

fn fill(text: &str, args: &[&str]) -> String {
    let mut args = args.iter();
    let mut pieces = text.split("{}");
    let mut out = String::from(pieces.next().unwrap_or_default());
    for piece in pieces {
        out.push_str(args.next().copied().unwrap_or_default());
        out.push_str(piece);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::MenuNode;
    use crate::{Services, SoundApp, UiState};
    use serde_json::json;

    #[test]
    fn untranslated_text_and_english_come_back_unchanged() {
        assert_eq!(translate(Language::English, "Open"), "Open");
        assert_eq!(translate(Language::Japanese, "no such label"), "no such label");
    }

    #[test]
    fn language_ids_round_trip_through_prefs() {
        for lang in Language::ALL {
            assert_eq!(Language::parse(lang.id()), Some(lang));
            let back: Language = serde_json::from_value(serde_json::to_value(lang).unwrap()).unwrap();
            assert_eq!(back, lang);
        }
        assert_eq!(serde_json::to_value(Language::Japanese).unwrap(), json!("ja"));
        assert_eq!(Language::parse(" JA "), Some(Language::Japanese));
        assert_eq!(Language::parse("fr"), None);
        assert_eq!(UiState::default().language, Language::English);
        // Prefs saved before the setting existed load as English.
        let old: UiState = serde_json::from_value(json!({"theme": "light"})).unwrap();
        assert_eq!(old.language, Language::English);
    }

    #[test]
    fn tables_skip_comments_blank_and_malformed_lines() {
        let found: Vec<_> = entries("# header\n\nOpen\t開く\r\nNo tab here\nClose\t\nSave\t保存\n").collect();
        assert_eq!(found, [("Open", "開く"), ("Save", "保存")]);
    }

    #[test]
    fn a_template_matches_any_text_in_its_placeholders() {
        assert_eq!(capture(&["`", "` required"], "`track` required"), Some(vec!["track"]));
        assert_eq!(capture(&["no ", " in ", ""], "no clip in track 3"), Some(vec!["clip", "track 3"]));
        assert_eq!(capture(&["`", "` required"], "track required"), None);
        // The fixed texts must not overlap.
        assert_eq!(capture(&["ab", "ab"], "ab"), None);
    }

    #[test]
    fn an_engine_message_is_translated_by_its_template() {
        let ja = translate_message(Language::Japanese, "`edit.undo` is not available: nothing to undo");
        assert!(ja.starts_with("`edit.undo` は使えません"), "{ja}");
        let en = translate_message(Language::English, "`edit.undo` is not available: nothing to undo");
        assert_eq!(en, "`edit.undo` is not available: nothing to undo");
        // Text that matches nothing comes back unchanged.
        assert_eq!(translate_message(Language::Japanese, "no such sentence anywhere"), "no such sentence anywhere");
    }

    #[test]
    fn a_translation_keeps_the_placeholders_of_its_key() {
        for (en, ja) in JA_TABLES.iter().copied().flat_map(entries) {
            assert_eq!(en.matches("{}").count(), ja.matches("{}").count(), "{en}: the placeholders differ in {ja}");
        }
    }

    #[test]
    fn placeholders_fill_in_order() {
        assert_eq!(fill("Undo {}", &["Trim"]), "Undo Trim");
        assert_eq!(fill("{} of {}", &["2", "5"]), "2 of 5");
        assert_eq!(fill("{} of {}", &["2"]), "2 of ");
        assert_eq!(fill("No placeholders", &["x"]), "No placeholders");
    }

    #[test]
    fn every_menu_label_has_a_japanese_translation() {
        fn missing(nodes: &[MenuNode], out: &mut Vec<String>) {
            for n in nodes {
                if !japanese().contains_key(n.label.as_str()) {
                    out.push(n.path.clone());
                }
                missing(&n.children, out);
            }
        }
        let mut out = Vec::new();
        missing(&crate::menus::tree(), &mut out);
        assert!(out.is_empty(), "menu labels without a Japanese translation:\n{}", out.join("\n"));
    }

    #[test]
    fn a_label_in_two_tables_has_one_translation() {
        let mut seen = HashMap::new();
        for (en, ja) in JA_TABLES.iter().copied().flat_map(entries) {
            assert!(!ja.contains('\t'), "{en}: a translation must not contain a tab");
            if let Some(prev) = seen.insert(en, ja) {
                assert_eq!(prev, ja, "{en} has two Japanese texts");
            }
        }
    }

    #[test]
    fn language_command_sets_and_reports_the_language() {
        let mut app = SoundApp::new(soundcraft_engine::Engine::default(), None, Services::default());
        assert_eq!(app.run("ui.language", json!({})), Ok(json!({"lang": "en"})));
        assert_eq!(app.run("ui.language", json!({"lang": "ja"})), Ok(json!({"lang": "ja"})));
        assert_eq!(app.ui.language, Language::Japanese);
        assert!(app.run("ui.language", json!({"lang": "fr"})).is_err());
        assert_eq!(app.ui.language, Language::Japanese);
        assert!(crate::menus::UI_COMMANDS.iter().any(|c| c.0 == "ui.language"));
    }
}
