//! The offer to recover unsaved work an earlier run left in the autosave folder.

use crate::SoundApp;
use crate::i18n;
use crate::theme::{Tokens, bold};
use egui::{Align2, vec2};

/// Offer the newest leftover autosave, one at a time.
pub fn show(app: &mut SoundApp, ctx: &egui::Context) {
    let Some(left) = app.leftovers.first().cloned() else { return };
    let mut open = true;
    let mut recover = None;
    egui::Window::new(egui::RichText::new(i18n::tr("Recover Unsaved Work")).font(bold(13.0)))
        .id(egui::Id::unique("Recover Unsaved Work"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, vec2(0.0, -80.0))
        .show(ctx, |ui| {
            let age = left.saved.elapsed().map_or_else(|_| i18n::tr("recently").into(), |d| ago(d.as_secs()));
            let message = if left.recording {
                i18n::trf("“{}” has unsaved changes and an unfinished recording from an earlier run, autosaved {}.", &[&left.name, &age])
            } else {
                i18n::trf("“{}” has unsaved changes from an earlier run, autosaved {}.", &[&left.name, &age])
            };
            ui.label(message);
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button(i18n::tr("Discard")).clicked() {
                    recover = Some(false);
                }
                let recover_button = egui::Button::new(egui::RichText::new(i18n::tr("Recover")).strong()).fill(Tokens::current().accent);
                if ui.add_enabled(!app.engine.transport.recording, recover_button).on_disabled_hover_text(i18n::tr("Stop recording first")).clicked()
                {
                    recover = Some(true);
                }
            });
        });
    if !open || recover.is_some() {
        app.leftovers.retain(|l| *l != left);
    }
    match recover {
        Some(true) => app.recover(&left),
        Some(false) => soundcraft_engine::autosave::discard(&left),
        None => {}
    }
}

fn ago(secs: u64) -> String {
    let num = |n: u64| n.to_string();
    match secs {
        0..60 => i18n::tr("less than a minute ago").into(),
        60..120 => i18n::tr("1 minute ago").into(),
        120..3_600 => i18n::trf("{} minutes ago", &[&num(secs / 60)]),
        3_600..7_200 => i18n::tr("1 hour ago").into(),
        7_200..86_400 => i18n::trf("{} hours ago", &[&num(secs / 3_600)]),
        86_400..172_800 => i18n::tr("1 day ago").into(),
        _ => i18n::trf("{} days ago", &[&num(secs / 86_400)]),
    }
}
