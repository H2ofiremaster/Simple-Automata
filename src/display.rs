use vizia::prelude::*;

use crate::{grid::Grid, AppData, AppEvent};

pub fn left_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {}).class(style::SIDE_PANEL);
}

pub fn center_panel(cx: &mut Context) {
    let grid = AppData::grid;
    Binding::new(cx, AppData::grid, |cx, grid| {
        ZStack::new(cx, |cx| {
            grid.get(cx).display(cx);
        })
        .size(AppData::window_size.map(|bounds| Pixels(margined_square_size(bounds))))
        .class(style::CENTER_PANEL);
    });
}

pub fn right_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {}).class("side-panel");
}

fn margined_square_size(bounds: &BoundingBox) -> f32 {
    bounds
        .height()
        .min(bounds.width() * style::CENTER_MARGIN_FACTOR)
        - (style::BACKGROUND_PADDING * 2.0)
}

fn display_cell(grid: &Grid, cx: &mut Context, x: usize, y: usize) {
    let Some(cell) = grid.cell_at(x, y) else {
        println!("Cell at '{x}, {y}' doesn't exist; skipping...");
        return;
    };
    cell.display(cx)
        .on_hover(move |cx| cx.emit(AppEvent::CellHovered(x, y)))
        .on_mouse_down(move |cx, button| cx.emit(AppEvent::CellClicked(x, y, button)));
}

pub mod style {
    pub const SIDE_PANEL: &str = "side-panel";
    pub const CENTER_PANEL: &str = "center-panel";
    pub const CELL: &str = "cell";
    pub const BACKGROUND: &str = "background";

    /// The maximum percentage of the screen the center square can take up.
    pub const CENTER_MARGIN_FACTOR: f32 = 0.6;
    /// This should always be equal to the 'child-space' property under '.background' in 'style.css'
    pub const BACKGROUND_PADDING: f32 = 10.0;
}
