use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{Button, Group, Label, Window},
        Skin,
    },
};

use crate::screen_center;

const WINDOW_SIZE: Vec2 = vec2(0.75, 0.75);
pub fn display(skin: &Skin) {
    let window_size = vec2(screen_width(), screen_height()) * WINDOW_SIZE;

    root_ui().push_skin(skin);
    Window::new(
        hash!(screen_center().to_string()),
        screen_center() - window_size / 2.,
        window_size,
    )
    .label("Options")
    .titlebar(true)
    .movable(false)
    .ui(&mut root_ui(), |ui| {
        Label::new("Select Ruleset").ui(ui);
        ui.same_line(0.);
        Button::new("Button").ui(ui);
        Group::new(hash!(), window_size / 3.)
            .layout(macroquad::ui::Layout::Vertical)
            .ui(ui, |ui| {

                //
            });
    });
}
