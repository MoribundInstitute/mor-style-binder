use crate::core::ExportMode;
use crate::ui::buttons::{action_button_primary, action_button_secondary};
use crate::ui::helpers::{
    open_existing_path, open_path, BLOGGER_EXPORT_FILE, PROJECT_ROOT, STANDARD_EXPORT_FILE,
    TARGET_DIR,
};
use crate::ui::panels::panel_title;
use crate::ui::tooltips::status_tip;
use copypasta::{ClipboardContext, ClipboardProvider};
use floem::peniko::Color;
use floem::prelude::*;
use std::fs;

pub fn export_panel(
    css_content: RwSignal<String>,
    export_mode: RwSignal<ExportMode>,
    status_message: RwSignal<String>,
) -> impl IntoView {
    v_stack((
        panel_title("Export"),
        status_tip(
            label(|| "Copy Bundle CSS".to_string())
                .on_click_stop(move |_| {
                    let text_to_copy = css_content.get();

                    match ClipboardContext::new() {
                        Ok(mut ctx) => {
                            if ctx.set_contents(text_to_copy).is_ok() {
                                status_message.set("Copied bundle CSS.".to_string());
                            } else {
                                status_message.set("Could not copy bundle CSS.".to_string());
                            }
                        }
                        Err(_) => {
                            status_message.set("Could not access clipboard.".to_string());
                        }
                    }
                })
                .style(action_button_primary),
            status_message,
            "Copy the generated bundle preview to the clipboard.",
        ),
        status_tip(
            label(|| "Save bundle.css".to_string())
                .on_click_stop(move |_| {
                    let bundle =
                        crate::fs_engine::build_bundle_from_dir(TARGET_DIR, ExportMode::Standard);

                    match fs::write(STANDARD_EXPORT_FILE, bundle) {
                        Ok(_) => status_message.set(format!(
                            "Exported plain CSS bundle to {STANDARD_EXPORT_FILE}."
                        )),
                        Err(err) => status_message.set(format!("Could not export bundle.css: {err}")),
                    }
                })
                .style(action_button_secondary),
            status_message,
            "Save the plain CSS bundle to ./bundle.css.",
        ),
        status_tip(
            label(|| "Open bundle.css".to_string())
                .on_click_stop(move |_| match open_existing_path(STANDARD_EXPORT_FILE) {
                    Ok(_) => status_message.set(format!("Opened {STANDARD_EXPORT_FILE}.")),
                    Err(err) => status_message.set(format!(
                        "Could not open {STANDARD_EXPORT_FILE}. Export it first. {err}"
                    )),
                })
                .style(action_button_secondary),
            status_message,
            "Open ./bundle.css with the operating system's default app.",
        ),
        status_tip(
            label(|| "Export Blogger Skin".to_string())
                .on_click_stop(move |_| {
                    let bundle =
                        crate::fs_engine::build_bundle_from_dir(TARGET_DIR, ExportMode::BloggerXml);

                    match fs::write(BLOGGER_EXPORT_FILE, bundle) {
                        Ok(_) => status_message.set(format!(
                            "Exported Blogger skin wrapper to {BLOGGER_EXPORT_FILE}."
                        )),
                        Err(err) => {
                            status_message.set(format!("Could not export blogger-theme.xml: {err}"))
                        }
                    }
                })
                .style(action_button_secondary),
            status_message,
            "Save a Blogger-compatible <b:skin> XML wrapper to ./blogger-theme.xml.",
        ),
        status_tip(
            label(|| "Open blogger-theme.xml".to_string())
                .on_click_stop(move |_| match open_existing_path(BLOGGER_EXPORT_FILE) {
                    Ok(_) => status_message.set(format!("Opened {BLOGGER_EXPORT_FILE}.")),
                    Err(err) => status_message.set(format!(
                        "Could not open {BLOGGER_EXPORT_FILE}. Export it first. {err}"
                    )),
                })
                .style(action_button_secondary),
            status_message,
            "Open ./blogger-theme.xml. If your OS has odd XML associations, use Open Export Folder instead.",
        ),
        status_tip(
            label(|| "Open Export Folder".to_string())
                .on_click_stop(move |_| match open_path(PROJECT_ROOT) {
                    Ok(_) => status_message.set("Opened export folder.".to_string()),
                    Err(err) => status_message.set(format!("Could not open export folder: {err}")),
                })
                .style(action_button_secondary),
            status_message,
            "Open the project folder containing bundle.css and blogger-theme.xml.",
        ),
        label(move || {
            let mode = match export_mode.get() {
                ExportMode::Standard => "Current mode: plain CSS bundle",
                ExportMode::BloggerXml => "Current mode: Blogger <b:skin> wrapper",
            };

            mode.to_string()
        })
        .style(|s| {
            s.font_size(13.0)
                .line_height(1.4)
                .margin_top(10.0)
                .color(Color::rgb8(145, 145, 145))
        }),
    ))
    .style(|s| s.width_full())
}
