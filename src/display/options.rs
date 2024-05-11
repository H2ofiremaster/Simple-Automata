use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{Button, Group, Label, Window},
        Layout, Skin,
    },
};

use crate::screen_center;

const TITLE_HEIGHT: f32 = 32.;
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
        ui.same_line(0.);
        Group::new(
            hash!(),
            vec2(window_size.x / 3., window_size.y - TITLE_HEIGHT),
        )
        .layout(Layout::Vertical)
        .ui(ui, |ui| {
            Label::new("Select Ruleset").ui(ui);
            let offset = TITLE_HEIGHT * 2. + 10.;
            Group::new(hash!(), vec2(window_size.x / 3., window_size.y - offset))
                .layout(Layout::Vertical)
                .ui(ui, |ui| {});
            //
        });
    });
}
