//! The menu bar in Japanese: the top-level menus and the items inside them take their Japanese
//! text. This is its own test binary, and it holds one test: the display language is process-wide,
//! so the Japanese and English cases run one after the other.

use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use soundcraft_ui_egui::i18n::Language;
use soundcraft_ui_egui::{Services, SoundApp};

fn harness(lang: Language) -> Harness<'static, SoundApp> {
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.language = lang;
    let mut h = Harness::builder().with_size(vec2(1400.0, 1000.0)).build_ui_state(
        |ui, app: &mut SoundApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

#[test]
fn the_menu_bar_follows_the_language_setting() {
    let mut h = harness(Language::Japanese);
    assert!(h.query_by_label("ファイル").is_some(), "the File menu is not in Japanese");
    assert!(h.query_by_label("File").is_none(), "an English menu title is still shown");
    h.get_by_label("ファイル").click();
    h.run_steps(8);
    assert!(h.query_all_by_label_contains("新規").next().is_some(), "the File menu's items are not in Japanese");
    assert!(h.query_all_by_label_contains("New...").next().is_none(), "an English menu item is still shown");

    // An engine error shows in the status line in Japanese: its frame and its stock reason.
    let mut app = SoundApp::new(soundcraft_engine::Engine::default(), None, Services::default());
    soundcraft_ui_egui::i18n::set(Language::Japanese);
    assert!(app.run("edit.undo", serde_json::json!({})).is_err());
    assert!(app.ui.status.contains("取り消せる操作がありません"), "status: {}", app.ui.status);
    assert!(app.ui.status.contains("使えません"), "status: {}", app.ui.status);

    let h = harness(Language::English);
    assert!(h.query_by_label("File").is_some(), "the File menu is not in English");
    assert!(h.query_by_label("ファイル").is_none(), "a Japanese menu title is shown in English");
}
