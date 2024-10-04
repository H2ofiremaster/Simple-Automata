use vizia::prelude::*;

use crate::{grid::Grid, ruleset::Ruleset, AppData, AppEvent};

pub fn left_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {});
}

pub fn center_panel(cx: &mut Context) {
    let grid = AppData::grid;
    Binding::new(cx, AppData::grid, |cx, grid| {
        ZStack::new(cx, |cx| {
            grid.get(cx).display(cx);
        })
        .size(AppData::window_size.map(|bounds| Pixels(margined_square_size(bounds))))
        .background_color("gray");
    });
}

pub fn right_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {});
}

fn margined_square_size(bounds: &BoundingBox) -> f32 {
    bounds.height().min(bounds.width() * 0.6)
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
