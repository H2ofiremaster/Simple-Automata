use anyhow::{anyhow, Result};
use macroquad::{
    color::*,
    math::RectOffset,
    text::{load_ttf_font_from_bytes, Font},
    texture::Image,
    ui::{root_ui, Skin},
};

#[derive(Debug)]
pub struct Styles {
    pub font: Font,
    pub cell_selector: Skin,
}
impl Styles {
    pub fn new() -> Result<Self> {
        Ok(Self {
            font: load_ttf_font_from_bytes(FONT)?,
            cell_selector: cell_selector()?,
        })
    }
}

const FONT: &[u8] = include_bytes!("../../resources/Roboto-Regular.ttf");
pub const FONT_SIZE: u16 = 16;
pub const TEXT_COLOR: Color = WHITE;
pub const SELECTED_TEXT_COLOR: Color = GREEN;
const TITLE_HEIGHT: f32 = 30.;
const WINDOW_COLOR: Color = GRAY;
const TEXT_MARGINS: f32 = 10.;
fn cell_selector() -> Result<Skin> {
    let window_background_image = Image::gen_image_color(100, 100, WINDOW_COLOR);

    let window_style = root_ui()
        .style_builder()
        .background(window_background_image)
        .color(TEXT_COLOR)
        .build();
    let window_titlebar_style = root_ui()
        .style_builder()
        .color(TEXT_COLOR)
        .font_size(FONT_SIZE)
        .font(FONT)
        .map_err(|err| anyhow!("Failed to load font: {err}"))?
        .margin(all_margins(TEXT_MARGINS))
        .build();

    let label_style = root_ui()
        .style_builder()
        .color(TEXT_COLOR)
        .font_size(FONT_SIZE)
        .font(FONT)
        .map_err(|err| anyhow!("Failed to load font: {err}"))?
        .margin(all_margins(TEXT_MARGINS))
        .build();

    Ok(Skin {
        window_style,
        window_titlebar_style,
        label_style,
        title_height: TITLE_HEIGHT,
        ..root_ui().default_skin()
    })
}

fn all_margins(margins: f32) -> RectOffset {
    RectOffset::new(margins, margins, margins, margins)
}
