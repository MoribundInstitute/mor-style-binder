use crate::core::{CssModule, ExportMode};
use crate::ui::buttons::panel_style;
use floem::event::EventListener;
use floem::peniko::Color;
use floem::prelude::*;

pub mod bundle_preview;
pub mod export;
pub mod header;
pub mod import_theme;
pub mod modular_sheets;
pub mod status;

pub use bundle_preview::bundle_preview_panel;
pub use header::app_header;
pub use modular_sheets::module_tray_panel;
pub use status::status_strip;

/// Compatibility wrapper used by main.rs.
///
/// The old `panels.rs` exposed one right-side panel named `import_export_panel`.
/// Internally this is now split into `import_theme_panel` and `export_panel`,
/// while preserving the public function name and behavior.
pub fn import_export_panel(
    css_content: RwSignal<String>,
    export_mode: RwSignal<ExportMode>,
    file_tick: RwSignal<u32>,
    status_message: RwSignal<String>,
    pending_modules: RwSignal<Vec<CssModule>>,
) -> impl IntoView {
    v_stack((
        import_theme::import_theme_panel(file_tick, status_message, pending_modules),
        export::export_panel(css_content, export_mode, status_message),
    ))
    .style(panel_style)
    .on_event_stop(EventListener::DroppedFile, move |event| {
        import_theme::handle_theme_drop(&event, status_message, pending_modules);
    })
}

pub(super) fn panel_title(text: &'static str) -> impl IntoView {
    label(move || text.to_string()).style(|s| {
        s.font_size(16.0)
            .margin_bottom(10.0)
            .color(Color::rgb8(225, 225, 225))
    })
}

pub(super) fn drop_zone(
    title: &'static str,
    body: &'static str,
    is_hovered: RwSignal<bool>,
) -> impl IntoView {
    v_stack((
        label(move || title.to_string()).style(move |s| {
            s.font_size(14.0)
                .margin_bottom(6.0)
                .apply_if(is_hovered.get(), |s| s.color(Color::rgb8(245, 245, 245)))
                .apply_if(!is_hovered.get(), |s| s.color(Color::rgb8(220, 220, 220)))
        }),
        label(move || body.to_string()).style(move |s| {
            s.font_size(12.0)
                .line_height(1.4)
                .apply_if(is_hovered.get(), |s| s.color(Color::rgb8(185, 185, 185)))
                .apply_if(!is_hovered.get(), |s| s.color(Color::rgb8(145, 145, 145)))
        }),
    ))
    .style(move |s| {
        s.padding(14.0)
            .margin_bottom(12.0)
            .border(1.0)
            .border_radius(4.0)
            .apply_if(is_hovered.get(), |s| {
                s.border_color(Color::rgb8(190, 190, 190))
                    .background(Color::rgb8(28, 28, 28))
            })
            .apply_if(!is_hovered.get(), |s| {
                s.border_color(Color::rgb8(90, 90, 90))
                    .background(Color::rgb8(18, 18, 18))
            })
    })
}

pub(super) fn format_inline_names(names: &[String]) -> String {
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(", ")
    }
}

pub(super) fn format_name_lines(names: &[String]) -> String {
    if names.is_empty() {
        return "▰ none".to_string();
    }

    names
        .iter()
        .map(|name| format!("▰ {name}"))
        .collect::<Vec<_>>()
        .join("\n")
}
