use egui::{
    Color32,
    Context,
    FontFamily::Proportional,
    FontId,
    TextStyle,
    vec2
};

use std::collections::BTreeMap;

const BUTTON_BG_COLOR: Color32 =  Color32::from_rgb(60, 80, 130);

pub fn get_button_fill() -> Color32 {
    BUTTON_BG_COLOR
}

pub fn set_styles(ctx: &Context) {
    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Body, FontId::new(20.0, Proportional)),
        (TextStyle::Button, FontId::new(20.0, Proportional)),
        (TextStyle::Heading, FontId::new(20.0, Proportional)),
        (TextStyle::Monospace, FontId::new(20.0, Proportional)),
        (TextStyle::Small, FontId::new(20.0, Proportional))
    ].into();

    ctx.all_styles_mut(move |style| {
        style.text_styles = text_styles.clone();

        style.spacing.button_padding = vec2(6.0, 0.0);
        style.spacing.item_spacing = vec2(4.0, 4.0);
        style.visuals.override_text_color = Some(Color32::from_rgb(255, 255, 255));
        style.visuals.widgets.hovered.weak_bg_fill = BUTTON_BG_COLOR;
        style.visuals.widgets.inactive.weak_bg_fill = BUTTON_BG_COLOR;
    });
}