use crate::core::ExportMode;
use crate::ui::tooltips::status_tip;
use floem::peniko::Color;
use floem::prelude::*;

pub fn mode_selector(
    export_mode: RwSignal<ExportMode>,
    status_message: RwSignal<String>,
) -> impl IntoView {
    h_stack((
        label(|| "Binding Mode: ".to_string())
            .style(|s| s.color(Color::rgb8(150, 150, 150)).margin_right(10.0)),
        status_tip(
            label(|| "Plain CSS Bundle".to_string())
                .on_click_stop(move |_| export_mode.set(ExportMode::Standard))
                .style(move |s| {
                    s.padding_horiz(12.0)
                        .padding_vert(6.0)
                        .margin_right(10.0)
                        .border(1.0)
                        .border_radius(3.0)
                        .apply_if(export_mode.get() == ExportMode::Standard, |s| {
                            s.border_color(Color::rgb8(220, 220, 220))
                                .color(Color::rgb8(235, 235, 235))
                        })
                        .apply_if(export_mode.get() != ExportMode::Standard, |s| {
                            s.border_color(Color::rgb8(50, 50, 50))
                                .color(Color::rgb8(110, 110, 110))
                        })
                }),
            status_message,
            "Preview and export a regular CSS bundle from src_css/.",
        ),
        status_tip(
            label(|| "Blogger Skin Wrapper".to_string())
                .on_click_stop(move |_| export_mode.set(ExportMode::BloggerXml))
                .style(move |s| {
                    s.padding_horiz(12.0)
                        .padding_vert(6.0)
                        .border(1.0)
                        .border_radius(3.0)
                        .apply_if(export_mode.get() == ExportMode::BloggerXml, |s| {
                            s.border_color(Color::rgb8(220, 220, 220))
                                .color(Color::rgb8(235, 235, 235))
                        })
                        .apply_if(export_mode.get() != ExportMode::BloggerXml, |s| {
                            s.border_color(Color::rgb8(50, 50, 50))
                                .color(Color::rgb8(110, 110, 110))
                        })
                }),
            status_message,
            "Preview and export a Blogger <b:skin><![CDATA[ ... ]]></b:skin> wrapper.",
        ),
    ))
    .style(|s| s.items_center())
}

pub fn app_header(
    export_mode: RwSignal<ExportMode>,
    status_message: RwSignal<String>,
) -> impl IntoView {
    v_stack((
        label(|| "MorStyleBinder".to_string()).style(|s| {
            s.font_size(26.0)
                .margin_bottom(8.0)
                .color(Color::rgb8(225, 225, 225))
        }),
        label(|| {
            "Bind modular CSS sheets into one theme, or refactor a monolithic theme into modular files."
                .to_string()
        })
        .style(|s| {
            s.font_size(13.0)
                .margin_bottom(14.0)
                .color(Color::rgb8(145, 145, 145))
        }),
        mode_selector(export_mode, status_message),
    ))
    .style(|s| s.margin_bottom(14.0))
}
