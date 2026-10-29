//! Every label the app draws in the Edit and Mix windows, with all the panels and windows open,
//! has a Japanese translation. Session names, track names and other user data are not looked up.
//! This is its own test binary: the display language is process-wide.

use egui::vec2;
use egui_kittest::Harness;
use soundcraft_ui_egui::i18n::{self, Language};
use soundcraft_ui_egui::{MainWindow, Services, SoundApp, UiState};

fn app(window: MainWindow) -> SoundApp {
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    let mut ui = serde_json::to_value(UiState::default()).unwrap_or_default();
    if let Some(flags) = ui.as_object_mut() {
        for (key, value) in flags.iter_mut() {
            if key.starts_with("show_") {
                *value = serde_json::Value::Bool(true);
            }
        }
    }
    app.ui = serde_json::from_value(ui).unwrap_or_default();
    app.ui.window = window;
    app.ui.language = Language::Japanese;
    app.extra.show_renderer = true;
    app.extra.show_ui_customization = true;
    app
}

#[test]
fn every_drawn_label_has_a_japanese_translation() {
    i18n::report_missing(true);
    for window in [MainWindow::Edit, MainWindow::Mix] {
        let mut h = Harness::builder().with_size(vec2(1800.0, 1100.0)).build_ui_state(
            |ui, app: &mut SoundApp| {
                app.logic(&ui.ctx().clone());
                app.ui(ui);
            },
            app(window),
        );
        h.run_steps(8);
    }
    draw_dialogs(None);
    let missing = i18n::missing();
    assert!(missing.is_empty(), "labels drawn in Japanese with no translation:\n{}", missing.join("\n"));
}

/// The dialogs a menu command opens, drawn one at a time (called from the test above).
const DIALOG_COMMANDS: &[&str] = &[
    "track.new",
    "file.bounce_mix",
    "session.new",
    "file.import_audio",
    "file.import_midi",
    "session.open",
    "session.save",
    "session.save_as",
    "session.save_copy",
    "track.group",
    "edit.repeat",
    "edit.shift",
    "edit.fades_create",
    "setup.session",
    "event.quantize",
    "event.transpose",
    "edit.strip_silence",
    "clip.rename",
    "clip.clear",
    "clip.auto_rename",
    "clip.loop",
    "file.score_setup",
];

/// Draws each dialog in `DIALOG_COMMANDS`, adding its painted text to `out` when given. Clip
/// commands get the first clip of the first track that has one as their selection, the others
/// the first two tracks.
fn draw_dialogs(mut out: Option<&mut std::collections::BTreeSet<String>>) {
    for command in DIALOG_COMMANDS {
        let mut app = app(MainWindow::Edit);
        let s = app.engine.session();
        let selection = if command.starts_with("clip.") {
            let clip = s.tracks.iter().find_map(|t| t.clips().first().map(|c| c.id.0));
            serde_json::json!({"clips": clip.into_iter().collect::<Vec<_>>()})
        } else {
            serde_json::json!({"tracks": s.tracks.iter().take(2).map(|t| t.id.0).collect::<Vec<_>>()})
        };
        let _ = app.engine.execute("edit.select", &selection);
        soundcraft_ui_egui::menus::invoke_menu(&mut app, command, "");
        let mut h = Harness::builder().with_size(vec2(1800.0, 1100.0)).build_ui_state(
            |ui, app: &mut SoundApp| {
                app.logic(&ui.ctx().clone());
                app.ui(ui);
            },
            app,
        );
        h.run_steps(6);
        if let Some(out) = out.as_deref_mut() {
            painted(&h.output().shapes, out);
        }
    }
}

/// Every text shape in a frame, including the ones nested in `Shape::Vec`.
fn painted(shapes: &[egui::epaint::ClippedShape], out: &mut std::collections::BTreeSet<String>) {
    fn walk(shape: &egui::Shape, out: &mut std::collections::BTreeSet<String>) {
        match shape {
            egui::Shape::Text(t) => {
                out.insert(t.galley.text().to_string());
            }
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    shapes.iter().for_each(|s| walk(&s.shape, out));
}

/// Lists every English-looking string painted in the Edit and Mix windows with all panels open, and
/// in the dialogs above. It is an
/// audit, not a gate: session, track, clip and plugin names are user data and stay as they are.
/// Run it with `cargo test -p soundcraft-ui-egui --test japanese_coverage painted_english -- --ignored --nocapture`.
#[test]
#[ignore]
fn painted_english() {
    let mut all = std::collections::BTreeSet::new();
    for window in [MainWindow::Edit, MainWindow::Mix] {
        let mut h = Harness::builder().with_size(vec2(1800.0, 1100.0)).build_ui_state(
            |ui, app: &mut SoundApp| {
                app.logic(&ui.ctx().clone());
                app.ui(ui);
            },
            app(window),
        );
        h.run_steps(10);
        painted(&h.output().shapes, &mut all);
    }
    draw_dialogs(Some(&mut all));
    for s in all {
        let has_cjk = s.chars().any(|c| ('\u{3040}'..='\u{30ff}').contains(&c) || ('\u{4e00}'..='\u{9fff}').contains(&c));
        let has_word = s.as_bytes().windows(3).any(|w| w.iter().all(u8::is_ascii_alphabetic));
        if has_word && !has_cjk {
            println!("EN: {s}");
        }
    }
}
