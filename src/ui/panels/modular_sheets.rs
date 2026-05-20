use crate::ui::buttons::{action_button_secondary, panel_style};
use crate::ui::helpers::{copy_css_files_to_target_dir, open_path, TARGET_DIR};
use crate::ui::panels::{drop_zone, format_inline_names, panel_title};
use crate::ui::tooltips::status_tip;
use floem::event::{Event, EventListener};
use floem::peniko::Color;
use floem::prelude::*;
use std::fs;

pub fn module_tray_panel(
    module_count: RwSignal<usize>,
    module_names: RwSignal<Vec<String>>,
    blogger_vars_detected: RwSignal<bool>,
    status_message: RwSignal<String>,
    file_tick: RwSignal<u32>,
) -> impl IntoView {
    let is_panel_hovered = RwSignal::new(false);

    v_stack((
        panel_title("Modular Sheets"),
        label(move || {
            let count = module_count.get();
            let vars = if blogger_vars_detected.get() {
                "blogger_vars.xml detected"
            } else {
                "No Blogger variables file"
            };

            format!("{count} modules bundled\n{vars}")
        })
        .style(|s| {
            s.font_size(13.0)
                .line_height(1.4)
                .margin_bottom(12.0)
                .color(Color::rgb8(165, 165, 165))
        }),
        status_tip(
            css_drop_zone(file_tick, status_message),
            status_message,
            "Drop .css files anywhere in the Modular Sheets panel to copy them into src_css/ and rebuild the bundle.",
        ),
        label(move || {
            let names = module_names.get();

            if names.is_empty() {
                "No modular sheets loaded yet.\nDrop .css files here or open src_css to begin."
                    .to_string()
            } else {
                names
                    .iter()
                    .map(|name| format!("▰ {name}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        })
        .style(|s| {
            s.font_family("monospace".to_string())
                .font_size(13.0)
                .line_height(1.55)
                .margin_top(12.0)
                .margin_bottom(14.0)
                .color(Color::rgb8(205, 205, 205))
        }),
        label(|| "Open Modular Sheets Folder".to_string())
            .on_click_stop(move |_| {
                let _ = fs::create_dir_all(TARGET_DIR);

                match open_path(TARGET_DIR) {
                    Ok(_) => status_message.set("Opened Modular Sheets folder: src_css.".to_string()),
                    Err(err) => status_message.set(format!("Could not open src_css folder: {err}")),
                }
            })
            .style(action_button_secondary),
    ))
    .style(move |s| {
        panel_style(s).apply_if(is_panel_hovered.get(), |s| {
            s.border_color(Color::rgb8(150, 150, 150))
                .background(Color::rgb8(14, 14, 14))
        })
    })
    .on_event_stop(EventListener::DragEnter, move |_| {
        is_panel_hovered.set(true);
    })
    .on_event_stop(EventListener::DragLeave, move |_| {
        is_panel_hovered.set(false);
    })
    .on_event_stop(EventListener::DroppedFile, move |event| {
        is_panel_hovered.set(false);
        handle_css_drop(event, file_tick, status_message);
    })
}

fn css_drop_zone(file_tick: RwSignal<u32>, status_message: RwSignal<String>) -> impl IntoView {
    let is_hovered = RwSignal::new(false);

    drop_zone(
        "Drop .css Sheets Here",
        "Drop modular CSS files here.\nThey will be copied into src_css and bundled automatically.",
        is_hovered,
    )
    .on_event_stop(EventListener::DragEnter, move |_| {
        is_hovered.set(true);
    })
    .on_event_stop(EventListener::DragLeave, move |_| {
        is_hovered.set(false);
    })
    .on_event_stop(EventListener::DroppedFile, move |event| {
        is_hovered.set(false);
        handle_css_drop(event, file_tick, status_message);
    })
}

fn handle_css_drop(event: &Event, file_tick: RwSignal<u32>, status_message: RwSignal<String>) {
    let Event::DroppedFile(dropped) = event else {
        return;
    };

    let paths = vec![dropped.path.clone()];

    match copy_css_files_to_target_dir(&paths, TARGET_DIR) {
        Ok(report) => {
            if report.imported_count == 0 {
                status_message.set(format!(
                    "⚠ No CSS sheets imported; ignored {} file(s). Drop only .css files into Modular Sheets.",
                    report.ignored_count
                ));
                return;
            }

            file_tick.update(|tick| *tick += 1);

            if !report.overwritten.is_empty() {
                status_message.set(format!(
                    "✓ Imported {} CSS sheet(s); overwrote: {}; bundle preview refreshed.",
                    report.imported_count,
                    format_inline_names(&report.overwritten)
                ));
            } else if report.ignored_count > 0 {
                status_message.set(format!(
                    "✓ Imported {} CSS sheet(s); ignored {} non-CSS file(s); bundle preview refreshed.",
                    report.imported_count,
                    report.ignored_count
                ));
            } else {
                status_message.set(format!(
                    "✓ Imported {} CSS sheet(s); bundle preview refreshed.",
                    report.imported_count
                ));
            }
        }
        Err(err) => {
            status_message.set(format!("✗ Could not import CSS sheet: {err}"));
        }
    }
}
