use crate::settings::AppSettings;
use crate::ui::buttons::panel_style;
use crate::ui::panels::panel_title;
use floem::peniko::Color;
use floem::prelude::*;

pub fn bundle_preview_panel(
    css_content: RwSignal<String>,
    app_settings: RwSignal<AppSettings>,
) -> impl IntoView {
    v_stack((
        panel_title("Bundle Preview"),
        label(|| "Generated output preview".to_string()).style(|s| {
            s.font_size(13.0)
                .margin_bottom(8.0)
                .color(Color::rgb8(145, 145, 145))
        }),
        scroll(
            label(move || css_content.get()).style(move |s| {
                let font_size = app_settings.get().preview_font_size;

                s.font_family("monospace".to_string())
                    .font_size(font_size)
                    .line_height(1.5)
                    .color(Color::rgb8(185, 185, 185))
            }),
        )
        .style(|s| {
            s.size_full()
                .padding(15.0)
                .border(1.0)
                .border_radius(4.0)
                .border_color(Color::rgb8(45, 45, 45))
                .background(Color::rgb8(3, 3, 3))
        }),
    ))
    .style(|s| panel_style(s).width(780.0).height(620.0))
}
