use crate::core::ExportMode;
use crate::settings::{
    save_settings,
    AppSettings,
    DEFAULT_CONFIG_FILE,
};
use floem::peniko::Color;
use floem::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogKind {
    About,
    SplitMarkerHelp,
    UsageGuide,
    Settings,
}

pub fn dialog_layer(
    active_dialog: RwSignal<Option<DialogKind>>,
    app_settings: RwSignal<AppSettings>,
    export_mode: RwSignal<ExportMode>,
    status_message: RwSignal<String>,
) -> impl IntoView {
    v_stack((
        h_stack((
            label(move || dialog_title(active_dialog.get()).to_string()).style(|s| {
                s.font_size(16.0)
                    .color(Color::rgb8(230, 230, 230))
                    .margin_right(12.0)
            }),

            label(|| "Close".to_string())
                .on_click_stop(move |_| active_dialog.set(None))
                .style(move |s| {
                    let hidden = active_dialog.get().is_none();

                    s.padding_horiz(10.0)
                        .padding_vert(5.0)
                        .border(1.0)
                        .border_radius(3.0)
                        .apply_if(hidden, |s| {
                            s.width(0.0)
                                .height(0.0)
                                .padding(0.0)
                                .border(0.0)
                                .color(Color::rgb8(15, 15, 15))
                        })
                        .apply_if(!hidden, |s| {
                            s.border_color(Color::rgb8(100, 100, 100))
                                .background(Color::rgb8(25, 25, 25))
                                .color(Color::rgb8(220, 220, 220))
                        })
                }),
        ))
        .style(|s| s.items_center().margin_bottom(8.0)),

        label(move || dialog_body(active_dialog.get()).to_string()).style(move |s| {
            let hidden = active_dialog.get().is_none() || active_dialog.get() == Some(DialogKind::Settings);

            s.font_size(13.0)
                .line_height(1.45)
                .apply_if(hidden, |s| {
                    s.height(0.0)
                        .margin_bottom(0.0)
                        .color(Color::rgb8(15, 15, 15))
                })
                .apply_if(!hidden, |s| s.color(Color::rgb8(180, 180, 180)))
        }),

        settings_dialog(active_dialog, app_settings, export_mode, status_message),
    ))
    .style(move |s| {
        let hidden = active_dialog.get().is_none();

        s.width_full()
            .apply_if(hidden, |s| {
                s.height(0.0)
                    .padding(0.0)
                    .margin_bottom(0.0)
                    .border(0.0)
                    .background(Color::rgb8(15, 15, 15))
            })
            .apply_if(!hidden, |s| {
                s.padding(12.0)
                    .margin_bottom(10.0)
                    .border(1.0)
                    .border_radius(4.0)
                    .border_color(Color::rgb8(70, 70, 70))
                    .background(Color::rgb8(10, 10, 10))
            })
    })
}

fn settings_dialog(
    active_dialog: RwSignal<Option<DialogKind>>,
    app_settings: RwSignal<AppSettings>,
    export_mode: RwSignal<ExportMode>,
    status_message: RwSignal<String>,
) -> impl IntoView {
    v_stack((
        label(|| "These settings are saved to config.toml.".to_string()).style(|s| {
            s.font_size(13.0)
                .margin_bottom(10.0)
                .color(Color::rgb8(170, 170, 170))
        }),

        bool_setting_row(
            "Confirm before overwriting sheets",
            app_settings,
            |settings| settings.confirm_before_overwriting_sheets,
            |settings, value| settings.confirm_before_overwriting_sheets = value,
        ),

        export_mode_setting_row(app_settings, export_mode),

        font_size_setting_row(app_settings),

        bool_setting_row(
            "Remember window size",
            app_settings,
            |settings| settings.remember_window_size,
            |settings, value| settings.remember_window_size = value,
        ),

        bool_setting_row(
            "Auto-inspect import.css after drop",
            app_settings,
            |settings| settings.auto_inspect_import_after_drop,
            |settings, value| settings.auto_inspect_import_after_drop = value,
        ),

        bool_setting_row(
            "Show verbose status messages",
            app_settings,
            |settings| settings.show_verbose_status_messages,
            |settings, value| settings.show_verbose_status_messages = value,
        ),

        h_stack((
            label(|| "Save Settings".to_string())
                .on_click_stop(move |_| {
                    let settings = app_settings.get();

                    match save_settings(DEFAULT_CONFIG_FILE, &settings) {
                        Ok(_) => {
                            export_mode.set(settings.default_export_mode.into());
                            status_message.set("Saved settings to config.toml.".to_string());
                        }
                        Err(err) => {
                            status_message.set(format!("Could not save config.toml: {err}"));
                        }
                    }
                })
                .style(action_button_style),

            label(|| "Close".to_string())
                .on_click_stop(move |_| active_dialog.set(None))
                .style(action_button_style),
        ))
        .style(|s| s.items_center().margin_top(10.0)),
    ))
    .style(move |s| {
        let hidden = active_dialog.get() != Some(DialogKind::Settings);

        s.apply_if(hidden, |s| {
            s.height(0.0)
                .padding(0.0)
                .border(0.0)
                .color(Color::rgb8(15, 15, 15))
        })
        .apply_if(!hidden, |s| {
            s.padding(4.0)
                .color(Color::rgb8(180, 180, 180))
        })
    })
}

fn bool_setting_row(
    label_text: &'static str,
    app_settings: RwSignal<AppSettings>,
    getter: fn(&AppSettings) -> bool,
    setter: fn(&mut AppSettings, bool),
) -> impl IntoView {
    h_stack((
        label(move || label_text.to_string()).style(|s| {
            s.width(260.0)
                .font_size(13.0)
                .color(Color::rgb8(190, 190, 190))
        }),

        label(move || {
            let value = getter(&app_settings.get());
            if value {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            }
        })
        .on_click_stop(move |_| {
            app_settings.update(|settings| {
                let next = !getter(settings);
                setter(settings, next);
            });
        })
        .style(toggle_button_style),
    ))
    .style(|s| s.items_center().margin_bottom(8.0))
}

fn export_mode_setting_row(
    app_settings: RwSignal<AppSettings>,
    export_mode: RwSignal<ExportMode>,
) -> impl IntoView {
    h_stack((
        label(|| "Default export mode".to_string()).style(|s| {
            s.width(260.0)
                .font_size(13.0)
                .color(Color::rgb8(190, 190, 190))
        }),

        label(move || app_settings.get().default_export_mode.label().to_string())
            .on_click_stop(move |_| {
                app_settings.update(|settings| {
                    settings.default_export_mode = settings.default_export_mode.toggled();
                    export_mode.set(settings.default_export_mode.into());
                });
            })
            .style(toggle_button_style),
    ))
    .style(|s| s.items_center().margin_bottom(8.0))
}

fn font_size_setting_row(app_settings: RwSignal<AppSettings>) -> impl IntoView {
    h_stack((
        label(|| "Preview font size".to_string()).style(|s| {
            s.width(260.0)
                .font_size(13.0)
                .color(Color::rgb8(190, 190, 190))
        }),

        label(|| "-".to_string())
            .on_click_stop(move |_| {
                app_settings.update(|settings| {
                    settings.preview_font_size = (settings.preview_font_size - 1.0).max(10.0);
                });
            })
            .style(action_button_style),

        label(move || format!("{:.0}px", app_settings.get().preview_font_size)).style(|s| {
            s.width(70.0)
                .font_size(13.0)
                .color(Color::rgb8(220, 220, 220))
        }),

        label(|| "+".to_string())
            .on_click_stop(move |_| {
                app_settings.update(|settings| {
                    settings.preview_font_size = (settings.preview_font_size + 1.0).min(24.0);
                });
            })
            .style(action_button_style),
    ))
    .style(|s| s.items_center().margin_bottom(8.0))
}

fn action_button_style(s: floem::style::Style) -> floem::style::Style {
    s.padding_horiz(10.0)
        .padding_vert(5.0)
        .margin_right(6.0)
        .border(1.0)
        .border_radius(3.0)
        .border_color(Color::rgb8(80, 80, 80))
        .background(Color::rgb8(22, 22, 22))
        .color(Color::rgb8(220, 220, 220))
        .active(|s| {
            s.background(Color::rgb8(42, 42, 42))
                .border_color(Color::rgb8(120, 120, 120))
        })
}

fn toggle_button_style(s: floem::style::Style) -> floem::style::Style {
    s.padding_horiz(10.0)
        .padding_vert(5.0)
        .border(1.0)
        .border_radius(3.0)
        .border_color(Color::rgb8(80, 80, 80))
        .background(Color::rgb8(22, 22, 22))
        .color(Color::rgb8(220, 220, 220))
        .active(|s| {
            s.background(Color::rgb8(42, 42, 42))
                .border_color(Color::rgb8(120, 120, 120))
        })
}

fn dialog_title(dialog: Option<DialogKind>) -> &'static str {
    match dialog {
        Some(DialogKind::About) => "About MorStyleBinder",
        Some(DialogKind::SplitMarkerHelp) => "Split Marker Help",
        Some(DialogKind::UsageGuide) => "Usage Guide",
        Some(DialogKind::Settings) => "Settings",
        None => "",
    }
}

fn dialog_body(dialog: Option<DialogKind>) -> &'static str {
    match dialog {
        Some(DialogKind::About) => {
            "MorStyleBinder is a lightweight desktop workbench for binding modular CSS sheets into one exportable theme, and refactoring monolithic Blogger/Tumblr/SpaceHey-style themes back into manageable files.\n\nThe app is built around a small CSS bundle/refactor engine: src_css/ is the modular source folder, import.css is the monolithic-theme inbox, and bundle.css or blogger-theme.xml are the export targets."
        }
        Some(DialogKind::SplitMarkerHelp) => {
            "To split a monolithic theme into separate sheets, place a marker before each section:\n\n/* --- filename.css --- */\n\nExample:\n\n/* --- 01_base.css --- */\nbody { margin: 0; }\n\n/* --- 02_links.css --- */\na { text-decoration: underline; }\n\nThen use Inspect Import to preview the detected files. Use Refactor into Sheets to write them into src_css/."
        }
        Some(DialogKind::UsageGuide) => {
            "Basic workflow:\n\n1. Drop modular .css sheets into Modular Sheets to bundle them.\n2. Use Plain CSS Bundle or Blogger Skin Wrapper to choose the output format.\n3. Drop a monolithic theme file into Import Theme to inspect split markers.\n4. Review Preview Files to Be Written before writing anything.\n5. Use Refactor into Sheets to write detected sections into src_css/.\n6. Use Export to copy or save bundle.css or blogger-theme.xml.\n\nSafety note:\nIf imported sections would overwrite existing files, MorStyleBinder previews the filenames first. Choose Overwrite Existing Sheets only after reviewing the list, or use Cancel Pending Import to back out."
        }
        Some(DialogKind::Settings) => "",
        None => "",
    }
}
