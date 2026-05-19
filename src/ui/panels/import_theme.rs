use crate::core::CssModule;
use crate::ui::buttons::action_button_secondary;
use crate::ui::helpers::{
    backup_overwritten_modules, clear_import_file, copy_theme_file_to_import_css, find_overwrites,
    inspect_import_file, open_path, summarize_module_names, write_modules_to_dir, BackupReport,
    BACKUP_DIR, IMPORT_FILE, SPLIT_MARKER_EXAMPLE, TARGET_DIR,
};
use crate::ui::panels::{drop_zone, format_inline_names, format_name_lines, panel_title};
use crate::ui::tooltips::status_tip;
use floem::event::{Event, EventListener};
use floem::peniko::Color;
use floem::prelude::*;
use std::fs;
use std::path::Path;

pub fn import_theme_panel(
    file_tick: RwSignal<u32>,
    status_message: RwSignal<String>,
    pending_modules: RwSignal<Vec<CssModule>>,
) -> impl IntoView {
    v_stack((
        panel_title("Import Theme"),
        label(|| "Drop or open a monolithic theme for inspection.".to_string()).style(|s| {
            s.font_size(13.0)
                .line_height(1.4)
                .margin_bottom(8.0)
                .color(Color::rgb8(145, 145, 145))
        }),
        status_tip(
            label(|| format!("Split marker format:\n{SPLIT_MARKER_EXAMPLE}")).style(|s| {
                s.font_family("monospace".to_string())
                    .font_size(12.0)
                    .line_height(1.35)
                    .margin_bottom(10.0)
                    .padding(10.0)
                    .border(1.0)
                    .border_radius(4.0)
                    .border_color(Color::rgb8(45, 45, 45))
                    .background(Color::rgb8(8, 8, 8))
                    .color(Color::rgb8(165, 165, 165))
            }),
            status_message,
            "Place this marker before each section in import.css. The filename becomes the output sheet name.",
        ),
        status_tip(
            theme_drop_zone(status_message, pending_modules),
            status_message,
            "Drop a .css, .xml, or .txt theme here. It is copied to import.css and inspected for split markers.",
        ),
        status_tip(
            label(|| "Open import.css".to_string())
                .on_click_stop(move |_| {
                    if !Path::new(IMPORT_FILE).exists() {
                        let starter = "/* --- 01_base.css --- */\nbody {\n  margin: 0;\n}\n";

                        if let Err(err) = fs::write(IMPORT_FILE, starter) {
                            status_message.set(format!("Could not create import.css: {err}"));
                            return;
                        }
                    }

                    match open_path(IMPORT_FILE) {
                        Ok(_) => status_message.set("Opened import.css.".to_string()),
                        Err(err) => status_message.set(format!("Could not open import.css: {err}")),
                    }
                })
                .style(action_button_secondary),
            status_message,
            "Open the import inbox file. Use it for monolithic themes that should be split into sheets.",
        ),
        status_tip(
            label(|| "Inspect Import".to_string())
                .on_click_stop(move |_| {
                    inspect_import_into_pending(status_message, pending_modules);
                })
                .style(action_button_secondary),
            status_message,
            "Scan import.css for split markers and preview the sheets to be written or overwritten.",
        ),
        status_tip(
            label(move || {
                let modules = pending_modules.get();

                if modules.is_empty() {
                    "Refactor into Sheets".to_string()
                } else {
                    let overwrites = find_overwrites(&modules, TARGET_DIR);

                    if overwrites.is_empty() {
                        "Refactor into Sheets".to_string()
                    } else {
                        "Overwrite Existing Sheets".to_string()
                    }
                }
            })
            .on_click_stop(move |_| {
                refactor_pending_or_import(file_tick, status_message, pending_modules);
            })
            .style(action_button_secondary),
            status_message,
            "Write detected sections into src_css/. Existing sheets are backed up before overwrite.",
        ),
        status_tip(
            label(|| "Cancel Pending Import".to_string())
                .on_click_stop(move |_| {
                    pending_modules.set(Vec::new());
                    status_message.set("Canceled pending import preview.".to_string());
                })
                .style(action_button_secondary),
            status_message,
            "Clear the current import preview without changing import.css or src_css/.",
        ),
        status_tip(
            label(|| "Open Refactored Folder".to_string())
                .on_click_stop(move |_| {
                    let _ = fs::create_dir_all(TARGET_DIR);

                    match open_path(TARGET_DIR) {
                        Ok(_) => {
                            status_message.set("Opened refactored sheets folder: src_css.".to_string())
                        }
                        Err(err) => {
                            status_message.set(format!("Could not open refactored folder: {err}"))
                        }
                    }
                })
                .style(action_button_secondary),
            status_message,
            "Open src_css/, where refactored sheets are written.",
        ),
        status_tip(
            label(|| "Clear import.css".to_string())
                .on_click_stop(move |_| match clear_import_file(IMPORT_FILE) {
                    Ok(_) => {
                        pending_modules.set(Vec::new());
                        status_message
                            .set("Cleared import.css and canceled pending import preview.".to_string());
                    }
                    Err(err) => status_message.set(format!("Could not clear import.css: {err}")),
                })
                .style(action_button_secondary),
            status_message,
            "Empty import.css and cancel any pending import preview.",
        ),
        label(move || pending_import_preview_text(pending_modules.get())).style(|s| {
            s.font_family("monospace".to_string())
                .font_size(13.0)
                .line_height(1.45)
                .margin_top(8.0)
                .margin_bottom(18.0)
                .color(Color::rgb8(165, 165, 165))
        }),
    ))
    .style(|s| s.width_full())
}

fn theme_drop_zone(
    status_message: RwSignal<String>,
    pending_modules: RwSignal<Vec<CssModule>>,
) -> impl IntoView {
    let is_hovered = RwSignal::new(false);

    drop_zone(
        "Drop Theme File Here",
        "Drop a .css, .xml, or .txt monolithic theme here.\nIt will be copied to import.css and inspected.",
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
        handle_theme_drop(&event, status_message, pending_modules);
    })
}

pub(super) fn handle_theme_drop(
    event: &Event,
    status_message: RwSignal<String>,
    pending_modules: RwSignal<Vec<CssModule>>,
) {
    let Event::DroppedFile(dropped) = event else {
        return;
    };

    match copy_theme_file_to_import_css(&dropped.path, IMPORT_FILE, TARGET_DIR) {
        Ok(report) => {
            let count = report.detected_modules.len();

            if count == 0 {
                pending_modules.set(Vec::new());
                status_message
                    .set("Dropped theme file, but no split markers were found.".to_string());
                return;
            }

            let names = summarize_module_names(&report.detected_modules);
            let overwrites = report.overwritten.clone();
            pending_modules.set(report.detected_modules);

            if overwrites.is_empty() {
                status_message.set(format!(
                    "Dropped import.css and detected {count} sheet(s): {names}"
                ));
            } else {
                status_message.set(format!(
                    "Dropped import.css and detected {count} sheet(s). Will overwrite: {}. Review the preview, then choose Overwrite Existing Sheets.",
                    format_inline_names(&overwrites)
                ));
            }
        }
        Err(err) => {
            status_message.set(format!("Unsupported theme file type or import error: {err}"));
        }
    }
}

pub(super) fn inspect_import_into_pending(
    status_message: RwSignal<String>,
    pending_modules: RwSignal<Vec<CssModule>>,
) {
    match inspect_import_file(IMPORT_FILE, TARGET_DIR) {
        Ok(report) => {
            let count = report.detected_modules.len();

            if count == 0 {
                pending_modules.set(Vec::new());
                status_message.set(
                    "No split markers found in import.css. Use /* --- filename.css --- */."
                        .to_string(),
                );
                return;
            }

            let names = summarize_module_names(&report.detected_modules);
            let overwrites = report.overwritten.clone();
            pending_modules.set(report.detected_modules);

            if overwrites.is_empty() {
                status_message.set(format!("Detected {count} sheet(s): {names}"));
            } else {
                status_message.set(format!(
                    "Detected {count} sheet(s): {names}. Will overwrite: {}. Review the preview, then choose Overwrite Existing Sheets.",
                    format_inline_names(&overwrites)
                ));
            }
        }
        Err(err) => {
            pending_modules.set(Vec::new());
            status_message.set(format!("Could not inspect import.css: {err}"));
        }
    }
}

pub(super) fn refactor_pending_or_import(
    file_tick: RwSignal<u32>,
    status_message: RwSignal<String>,
    pending_modules: RwSignal<Vec<CssModule>>,
) {
    let modules = pending_modules.get();

    if modules.is_empty() {
        match inspect_import_file(IMPORT_FILE, TARGET_DIR) {
            Ok(report) => {
                let count = report.detected_modules.len();

                if count == 0 {
                    status_message
                        .set("No split markers found in import.css. Nothing to refactor.".to_string());
                    return;
                }

                if !report.overwritten.is_empty() {
                    let names = summarize_module_names(&report.detected_modules);
                    let overwrites = report.overwritten.clone();
                    pending_modules.set(report.detected_modules);
                    status_message.set(format!(
                        "Detected {count} sheet(s): {names}. Will overwrite: {}. Choose Overwrite Existing Sheets to confirm.",
                        format_inline_names(&overwrites)
                    ));
                    return;
                }

                match write_modules_to_dir(&report.detected_modules, TARGET_DIR) {
                    Ok(written) => {
                        file_tick.update(|tick| *tick += 1);
                        pending_modules.set(Vec::new());
                        status_message.set(format!(
                            "Refactored {written} sheet(s). Open Refactored Folder to view them."
                        ));
                    }
                    Err(err) => status_message.set(format!("Could not write sheets: {err}")),
                }
            }
            Err(err) => status_message.set(format!("Could not inspect import.css: {err}")),
        }

        return;
    }

    let overwrites = find_overwrites(&modules, TARGET_DIR);

    let backup_report = match backup_overwritten_modules(&modules, TARGET_DIR, BACKUP_DIR) {
        Ok(report) => report,
        Err(err) => {
            status_message.set(format!("Could not back up overwritten sheets: {err}"));
            return;
        }
    };

    match write_modules_to_dir(&modules, TARGET_DIR) {
        Ok(count) => {
            file_tick.update(|tick| *tick += 1);
            pending_modules.set(Vec::new());

            if overwrites.is_empty() {
                status_message
                    .set(format!("Refactored {count} sheet(s). Open Refactored Folder to view them."));
            } else {
                status_message.set(format!(
                    "Refactored {count} sheet(s); overwrote: {}; {} Open Refactored Folder to view them.",
                    format_inline_names(&overwrites),
                    format_backup_suffix(&backup_report)
                ));
            }
        }
        Err(err) => status_message.set(format!("Could not write sheets: {err}")),
    }
}

fn format_backup_suffix(report: &BackupReport) -> String {
    match &report.backup_dir {
        Some(path) if report.backed_up_count > 0 => {
            format!(
                "backed up {} sheet(s) to {}.",
                report.backed_up_count,
                path.display()
            )
        }
        _ => "no backup needed.".to_string(),
    }
}

fn pending_import_preview_text(modules: Vec<CssModule>) -> String {
    if modules.is_empty() {
        return "No inspected import waiting.".to_string();
    }

    let files_to_write = format_module_lines(&modules);
    let overwrites = find_overwrites(&modules, TARGET_DIR);

    if overwrites.is_empty() {
        format!("Preview Files to Be Written:\n{files_to_write}")
    } else {
        format!(
            "Preview Files to Be Written:\n{files_to_write}\n\nWill overwrite existing sheets:\n{}",
            format_name_lines(&overwrites)
        )
    }
}

fn format_module_lines(modules: &[CssModule]) -> String {
    let mut names = modules
        .iter()
        .map(|module| module.name.clone())
        .collect::<Vec<_>>();

    names.sort();
    format_name_lines(&names)
}
