use macroquad::{
    math::{vec2, Vec2},
    texture::{Image, Texture2D},
    ui::{
        hash, root_ui,
        widgets::{Button, Label, Window},
        Skin,
    },
    window::{screen_height, screen_width},
};

use crate::{logic::rules::Ruleset, screen_center};

const CELL_SIZE: Vec2 = vec2(150., 150.);
const CELL_PADDING_MULTIPLIER: f32 = 1.5;
const WINDOW_SIZE: Vec2 = vec2(0.75, 0.75);
pub fn display(ruleset: &Ruleset, skin: &Skin) {
    let window_size = vec2(screen_width(), screen_height()) * WINDOW_SIZE;
    //
    root_ui().push_skin(skin);
    Window::new(
        hash!(screen_center().to_string()),
        screen_center() - window_size / 2.,
        window_size,
    )
    .label("Material Selector")
    .ui(&mut root_ui(), |ui| {
        for (index, material) in ruleset.iter_cells().enumerate() {
            let button_image =
                Image::gen_image_color(CELL_SIZE.x as u16, CELL_SIZE.y as u16, material.color);
            let button_texture = Texture2D::from_image(&button_image);
            let cell_label = Label::new(&material.name);
            let label_size = ui.calc_size(&material.name);
            ui.group(
                hash!("Material", index),
                vec2(
                    CELL_SIZE.x,
                    CELL_SIZE.y + label_size.y * CELL_PADDING_MULTIPLIER,
                ),
                |ui| {
                    cell_label.size(vec2(CELL_SIZE.x, label_size.y)).ui(ui);
                    Button::new(button_texture).ui(ui);
                },
            );
            ui.same_line(0.);
        }
    });
    root_ui().pop_skin();
}
