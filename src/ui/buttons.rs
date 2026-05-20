use floem::peniko::Color;

pub fn action_button_primary(s: floem::style::Style) -> floem::style::Style {
    s.width(260.0)
        .padding_horiz(14.0)
        .padding_vert(7.0)
        .margin_bottom(8.0)
        .border(1.0)
        .border_radius(3.0)
        .border_color(Color::rgb8(220, 220, 220))
        .color(Color::rgb8(15, 15, 15))
        .background(Color::rgb8(220, 220, 220))
        .active(|s| s.background(Color::rgb8(150, 150, 150)))
}

pub fn action_button_secondary(s: floem::style::Style) -> floem::style::Style {
    s.width(260.0)
        .padding_horiz(14.0)
        .padding_vert(7.0)
        .margin_bottom(8.0)
        .border(1.0)
        .border_radius(3.0)
        .border_color(Color::rgb8(120, 120, 120))
        .color(Color::rgb8(220, 220, 220))
        .background(Color::rgb8(30, 30, 30))
        .active(|s| s.background(Color::rgb8(60, 60, 60)))
}

pub fn panel_style(s: floem::style::Style) -> floem::style::Style {
    s.padding(14.0)
        .margin_right(12.0)
        .border(1.0)
        .border_radius(4.0)
        .border_color(Color::rgb8(48, 48, 48))
        .background(Color::rgb8(8, 8, 8))
}
