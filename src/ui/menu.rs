use crate::core::{CssModule, ExportMode};
use crate::ui::dialogs::DialogKind;
use crate::ui::helpers::{
    backup_overwritten_modules, clear_import_file, ensure_import_file_exists, inspect_import_file,
    open_path, save_bundle_to_file, summarize_module_names, write_modules_to_dir, BackupReport,
    BACKUP_DIR, BLOGGER_EXPORT_FILE, IMPORT_FILE, PROJECT_ROOT, STANDARD_EXPORT_FILE, TARGET_DIR,
};
use crate::ui::icons::{ICON_COPY, ICON_FILE, ICON_FOLDER, ICON_REFACTOR, ICON_SAVE};
use crate::ui::tooltips::status_tip_fn;
use copypasta::{ClipboardContext, ClipboardProvider};
use floem::peniko::Color;
use floem::prelude::*;

const ZOOM_MIN: f64 = 0.80;
const ZOOM_MAX: f64 = 1.25;
const ZOOM_STEP: f64 = 0.10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MenuKind {
    File,
    Edit,
    View,
    Tools,
    Help,
}

#[derive(Clone, Copy)]
pub struct MenuContext {
    pub css_content: RwSignal<String>,
    pub export_mode: RwSignal<ExportMode>,
    pub file_tick: RwSignal<u32>,
    pub status_message: RwSignal<String>,
    pub pending_modules: RwSignal<Vec<CssModule>>,
    pub active_dialog: RwSignal<Option<DialogKind>>,
    pub workbench_zoom: RwSignal<f64>,
}

pub fn app_menu_bar(
    css_content: RwSignal<String>,
    export_mode: RwSignal<ExportMode>,
    file_tick: RwSignal<u32>,
    status_message: RwSignal<String>,
    pending_modules: RwSignal<Vec<CssModule>>,
    active_dialog: RwSignal<Option<DialogKind>>,
    workbench_zoom: RwSignal<f64>,
) -> impl IntoView {
    let active_menu = RwSignal::new(MenuKind::File);

    let ctx = MenuContext {
        css_content,
        export_mode,
        file_tick,
        status_message,
        pending_modules,
        active_dialog,
        workbench_zoom,
    };

    v_stack((
        h_stack((
            menu_heading("File", MenuKind::File, active_menu),
            menu_heading("Edit", MenuKind::Edit, active_menu),
            menu_heading("View", MenuKind::View, active_menu),
            menu_heading("Tools", MenuKind::Tools, active_menu),
            menu_heading("Help", MenuKind::Help, active_menu),
        ))
        .style(|s| s.items_center().margin_bottom(6.0)),
        h_stack((
            active_menu_label(active_menu),
            command_button(active_menu, 0, ctx),
            command_button(active_menu, 1, ctx),
            command_button(active_menu, 2, ctx),
            command_button(active_menu, 3, ctx),
            command_button(active_menu, 4, ctx),
            command_button(active_menu, 5, ctx),
            command_button(active_menu, 6, ctx),
            command_button(active_menu, 7, ctx),
        ))
        .style(|s| s.items_center()),
    ))
    .style(|s| {
        s.width_full()
            .padding(8.0)
            .margin_bottom(10.0)
            .border(1.0)
            .border_color(Color::rgb8(45, 45, 45))
            .background(Color::rgb8(8, 8, 8))
    })
}

fn active_menu_label(active_menu: RwSignal<MenuKind>) -> impl IntoView {
    label(move || format!("{}:", menu_name(active_menu.get()))).style(|s| {
        s.padding_horiz(8.0)
            .padding_vert(5.0)
            .margin_right(6.0)
            .font_size(12.0)
            .color(Color::rgb8(150, 150, 150))
    })
}

fn menu_heading(
    text: &'static str,
    kind: MenuKind,
    active_menu: RwSignal<MenuKind>,
) -> impl IntoView {
    label(move || text.to_string())
        .on_click_stop(move |_| active_menu.set(kind))
        .style(move |s| {
            let active = active_menu.get() == kind;

            s.padding_horiz(12.0)
                .padding_vert(5.0)
                .margin_right(6.0)
                .border(1.0)
                .border_radius(3.0)
                .font_size(13.0)
                .apply_if(active, |s| {
                    s.border_color(Color::rgb8(120, 120, 120))
                        .background(Color::rgb8(25, 25, 25))
                        .color(Color::rgb8(235, 235, 235))
                })
                .apply_if(!active, |s| {
                    s.border_color(Color::rgb8(40, 40, 40))
                        .background(Color::rgb8(12, 12, 12))
                        .color(Color::rgb8(150, 150, 150))
                })
        })
}

fn command_button(
    active_menu: RwSignal<MenuKind>,
    index: usize,
    ctx: MenuContext,
) -> impl IntoView {
    status_tip_fn(
        h_stack((
            // Dynamic SVG placement
            create_icon_view(active_menu, index),
            label(move || command_label(active_menu.get(), index).to_string())
                .style(|s| s.font_size(12.0)),
        ))
        .on_click_stop(move |_| {
            if command_label(active_menu.get(), index).is_empty() {
                return;
            }
            run_command(active_menu.get(), index, ctx);
        })
        .style(move |s| {
            let hidden = command_label(active_menu.get(), index).is_empty();

            s.padding_horiz(10.0)
                .padding_vert(5.0)
                .margin_right(5.0)
                .border(1.0)
                .border_radius(3.0)
                .items_center()
                .apply_if(hidden, |s| {
                    s.width(0.0)
                        .height(0.0)
                        .padding(0.0)
                        .margin_right(0.0)
                        .border(0.0)
                        .color(Color::rgb8(8, 8, 8))
                        .background(Color::rgb8(8, 8, 8))
                })
                .apply_if(!hidden, |s| {
                    s.border_color(Color::rgb8(60, 60, 60))
                        .background(Color::rgb8(18, 18, 18))
                        .color(Color::rgb8(190, 190, 190))
                        .active(|s| {
                            s.background(Color::rgb8(42, 42, 42))
                                .border_color(Color::rgb8(110, 110, 110))
                        })
                })
        }),
        ctx.status_message,
        move || command_tooltip(active_menu.get(), index),
    )
}

fn create_icon_view(active_menu: RwSignal<MenuKind>, index: usize) -> impl IntoView {
    dyn_container(
        move || command_icon(active_menu.get(), index),
        |icon_opt| {
            if let Some(icon_str) = icon_opt {
                svg(icon_str.to_string())
                    .style(|s| {
                        s.width(14.0)
                            .height(14.0)
                            .color(Color::rgb8(160, 160, 160))
                            .margin_right(6.0)
                    })
                    .into_any()
            } else {
                empty().into_any()
            }
        },
    )
}

fn menu_name(menu: MenuKind) -> &'static str {
    match menu {
        MenuKind::File => "File",
        MenuKind::Edit => "Edit",
        MenuKind::View => "View",
        MenuKind::Tools => "Tools",
        MenuKind::Help => "Help",
    }
}

fn command_icon(menu: MenuKind, index: usize) -> Option<&'static str> {
    match (menu, index) {
        (MenuKind::File, 0) => Some(ICON_FOLDER),
        (MenuKind::File, 1) => Some(ICON_FILE),
        (MenuKind::File, 2) => Some(ICON_SAVE),
        (MenuKind::File, 3) => Some(ICON_SAVE),
        (MenuKind::File, 4) => Some(ICON_FOLDER),

        (MenuKind::Edit, 0) => Some(ICON_COPY),

        (MenuKind::Tools, 0) => Some(ICON_FILE),
        (MenuKind::Tools, 1) => Some(ICON_REFACTOR),
        (MenuKind::Tools, 3) => Some(ICON_FOLDER),
        (MenuKind::Tools, 4) => Some(ICON_FILE),
        _ => None,
    }
}

fn command_label(menu: MenuKind, index: usize) -> &'static str {
    match (menu, index) {
        (MenuKind::File, 0) => "Open Modular Sheets Folder",
        (MenuKind::File, 1) => "Open import.css",
        (MenuKind::File, 2) => "Save bundle.css",
        (MenuKind::File, 3) => "Export Blogger Skin",
        (MenuKind::File, 4) => "Open Export Folder",
        (MenuKind::File, 5) => "Quit",

        (MenuKind::Edit, 0) => "Copy Bundle CSS",

        (MenuKind::View, 0) => "Plain CSS Bundle",
        (MenuKind::View, 1) => "Blogger Skin Wrapper",
        (MenuKind::View, 2) => "Compact View",
        (MenuKind::View, 3) => "Normal View",
        (MenuKind::View, 4) => "Comfortable View",
        (MenuKind::View, 5) => "Zoom Out",
        (MenuKind::View, 6) => "Reset Zoom",
        (MenuKind::View, 7) => "Zoom In",

        (MenuKind::Tools, 0) => "Inspect Import",
        (MenuKind::Tools, 1) => "Refactor into Sheets",
        (MenuKind::Tools, 2) => "Clear Pending Import",
        (MenuKind::Tools, 3) => "Open Refactored Folder",
        (MenuKind::Tools, 4) => "Clear import.css",
        (MenuKind::Tools, 5) => "Settings",

        (MenuKind::Help, 0) => "Split Marker Help",
        (MenuKind::Help, 1) => "About",
        (MenuKind::Help, 2) => "Usage Guide",

        _ => "",
    }
}

fn command_tooltip(menu: MenuKind, index: usize) -> &'static str {
    match (menu, index) {
        (MenuKind::File, 0) => "Open the src_css folder where modular CSS sheets are stored.",
        (MenuKind::File, 1) => "Open the import inbox file for monolithic themes.",
        (MenuKind::File, 2) => "Write the current plain CSS bundle to ./bundle.css.",
        (MenuKind::File, 3) => "Write the current bundle wrapped as Blogger <b:skin> XML.",
        (MenuKind::File, 4) => {
            "Open the project folder containing bundle.css and blogger-theme.xml."
        }
        (MenuKind::File, 5) => "Close the application and terminate the file watcher.",

        (MenuKind::Edit, 0) => "Copy the generated bundle preview to the clipboard.",

        (MenuKind::View, 0) => "Preview/export regular CSS without a Blogger wrapper.",
        (MenuKind::View, 1) => {
            "Preview/export CSS inside a Blogger <b:skin><![CDATA[ ... ]]></b:skin> wrapper."
        }
        (MenuKind::View, 2) => "Use a narrower workbench width for smaller screens.",
        (MenuKind::View, 3) => "Return the workbench to the default width.",
        (MenuKind::View, 4) => "Use a wider workbench width for roomy desktop layouts.",
        (MenuKind::View, 5) => "Reduce the workbench width preset.",
        (MenuKind::View, 6) => "Reset the workbench width preset to 100%.",
        (MenuKind::View, 7) => "Increase the workbench width preset.",

        (MenuKind::Tools, 0) => {
            "Scan import.css for split markers and preview the files to be written."
        }
        (MenuKind::Tools, 1) => {
            "Write inspected sections into src_css/. Overwrites are backed up first."
        }
        (MenuKind::Tools, 2) => "Clear the in-memory import preview without changing import.css.",
        (MenuKind::Tools, 3) => "Open src_css/, where refactored sheets are written.",
        (MenuKind::Tools, 4) => "Empty import.css and clear any pending import preview.",
        (MenuKind::Tools, 5) => "Open the settings dialog.",

        (MenuKind::Help, 0) => "Show the /* --- filename.css --- */ split marker format.",
        (MenuKind::Help, 1) => "Show app information.",
        (MenuKind::Help, 2) => "Show a short workflow guide.",

        _ => "",
    }
}

fn run_command(menu: MenuKind, index: usize, ctx: MenuContext) {
    let MenuContext {
        css_content,
        export_mode,
        file_tick,
        status_message,
        pending_modules,
        active_dialog,
        workbench_zoom,
    } = ctx;

    match (menu, index) {
        (MenuKind::File, 0) => {
            let _ = std::fs::create_dir_all(TARGET_DIR);

            match open_path(TARGET_DIR) {
                Ok(_) => status_message.set("Opened Modular Sheets folder: src_css.".to_string()),
                Err(err) => status_message.set(format!("Could not open src_css folder: {err}")),
            }
        }
        (MenuKind::File, 1) => match ensure_import_file_exists(IMPORT_FILE) {
            Ok(_) => match open_path(IMPORT_FILE) {
                Ok(_) => status_message.set("Opened import.css.".to_string()),
                Err(err) => status_message.set(format!("Could not open import.css: {err}")),
            },
            Err(err) => status_message.set(format!("Could not create import.css: {err}")),
        },
        (MenuKind::File, 2) => {
            match save_bundle_to_file(STANDARD_EXPORT_FILE, ExportMode::Standard) {
                Ok(_) => status_message.set(format!(
                    "Exported plain CSS bundle to {STANDARD_EXPORT_FILE}."
                )),
                Err(err) => status_message.set(format!("Could not export bundle.css: {err}")),
            }
        }
        (MenuKind::File, 3) => {
            match save_bundle_to_file(BLOGGER_EXPORT_FILE, ExportMode::BloggerXml) {
                Ok(_) => status_message.set(format!(
                    "Exported Blogger skin wrapper to {BLOGGER_EXPORT_FILE}."
                )),
                Err(err) => {
                    status_message.set(format!("Could not export blogger-theme.xml: {err}"))
                }
            }
        }
        (MenuKind::File, 4) => match open_path(PROJECT_ROOT) {
            Ok(_) => status_message.set("Opened export folder.".to_string()),
            Err(err) => status_message.set(format!("Could not open export folder: {err}")),
        },
        (MenuKind::File, 5) => {
            // Instantly terminates the main thread and the background file watcher
            std::process::exit(0);
        }

        (MenuKind::Edit, 0) => {
            let text_to_copy = css_content.get();

            match ClipboardContext::new() {
                Ok(mut ctx) => {
                    if ctx.set_contents(text_to_copy).is_ok() {
                        status_message.set("Copied bundle CSS.".to_string());
                    } else {
                        status_message.set("Could not copy bundle CSS.".to_string());
                    }
                }
                Err(_) => status_message.set("Could not access clipboard.".to_string()),
            }
        }

        (MenuKind::View, 0) => {
            export_mode.set(ExportMode::Standard);
            status_message.set("Switched to plain CSS bundle mode.".to_string());
        }
        (MenuKind::View, 1) => {
            export_mode.set(ExportMode::BloggerXml);
            status_message.set("Switched to Blogger Skin wrapper mode.".to_string());
        }
        (MenuKind::View, 2) => {
            set_workbench_zoom(workbench_zoom, status_message, 0.90, "Compact View")
        }
        (MenuKind::View, 3) => {
            set_workbench_zoom(workbench_zoom, status_message, 1.00, "Normal View")
        }
        (MenuKind::View, 4) => {
            set_workbench_zoom(workbench_zoom, status_message, 1.10, "Comfortable View")
        }
        (MenuKind::View, 5) => {
            let next = (workbench_zoom.get() - ZOOM_STEP).max(ZOOM_MIN);
            set_workbench_zoom(workbench_zoom, status_message, next, "Zoom Out");
        }
        (MenuKind::View, 6) => {
            set_workbench_zoom(workbench_zoom, status_message, 1.00, "Reset Zoom")
        }
        (MenuKind::View, 7) => {
            let next = (workbench_zoom.get() + ZOOM_STEP).min(ZOOM_MAX);
            set_workbench_zoom(workbench_zoom, status_message, next, "Zoom In");
        }

        (MenuKind::Tools, 0) => {
            inspect_import_into_pending(status_message, pending_modules);
        }
        (MenuKind::Tools, 1) => {
            refactor_pending_or_import(file_tick, status_message, pending_modules);
        }
        (MenuKind::Tools, 2) => {
            pending_modules.set(Vec::new());
            status_message.set("Cleared pending import.".to_string());
        }
        (MenuKind::Tools, 3) => {
            let _ = std::fs::create_dir_all(TARGET_DIR);

            match open_path(TARGET_DIR) {
                Ok(_) => {
                    status_message.set("Opened refactored sheets folder: src_css.".to_string())
                }
                Err(err) => status_message.set(format!("Could not open refactored folder: {err}")),
            }
        }
        (MenuKind::Tools, 4) => match clear_import_file(IMPORT_FILE) {
            Ok(_) => {
                pending_modules.set(Vec::new());
                status_message
                    .set("Cleared import.css and canceled pending import preview.".to_string());
            }
            Err(err) => status_message.set(format!("Could not clear import.css: {err}")),
        },
        (MenuKind::Tools, 5) => {
            active_dialog.set(Some(DialogKind::Settings));
            status_message.set("Opened settings dialog.".to_string());
        }

        (MenuKind::Help, 0) => {
            active_dialog.set(Some(DialogKind::SplitMarkerHelp));
            status_message.set("Opened split marker help.".to_string());
        }
        (MenuKind::Help, 1) => {
            active_dialog.set(Some(DialogKind::About));
            status_message.set("Opened About dialog.".to_string());
        }
        (MenuKind::Help, 2) => {
            active_dialog.set(Some(DialogKind::UsageGuide));
            status_message.set("Opened usage guide.".to_string());
        }

        _ => {}
    }
}

fn set_workbench_zoom(
    workbench_zoom: RwSignal<f64>,
    status_message: RwSignal<String>,
    zoom: f64,
    label: &'static str,
) {
    let zoom = zoom.clamp(ZOOM_MIN, ZOOM_MAX);
    workbench_zoom.set(zoom);
    status_message.set(format!(
        "{label}: workbench width preset set to {:.0}%.",
        zoom * 100.0
    ));
}

fn inspect_import_into_pending(
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
                    "Detected {count} sheet(s): {names}. Will overwrite: {}. Choose Refactor into Sheets again to confirm.",
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

fn refactor_pending_or_import(
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
                    status_message.set(
                        "No split markers found in import.css. Nothing to refactor.".to_string(),
                    );
                    return;
                }

                if !report.overwritten.is_empty() {
                    let names = summarize_module_names(&report.detected_modules);
                    let overwrites = report.overwritten.clone();
                    pending_modules.set(report.detected_modules);
                    status_message.set(format!(
                        "Detected {count} sheet(s): {names}. Will overwrite: {}. Choose Refactor into Sheets again to confirm.",
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

    let overwrites = modules
        .iter()
        .filter(|module| std::path::Path::new(TARGET_DIR).join(&module.name).exists())
        .map(|module| module.name.clone())
        .collect::<Vec<_>>();

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
                status_message.set(format!(
                    "Refactored {count} sheet(s). Open Refactored Folder to view them."
                ));
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

fn format_inline_names(names: &[String]) -> String {
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(", ")
    }
}