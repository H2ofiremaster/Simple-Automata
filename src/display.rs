use vizia::prelude::*;

use crate::{grid::Grid, AppData, AppEvent};

pub fn left_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Button::new(cx, |cx| {
                Label::new(
                    cx,
                    AppData::running.map(|runnning| if *runnning { "Stop" } else { "Start" }),
                )
            })
            .on_press(|cx| cx.emit(AppEvent::ToggleRunning))
            .class(style::BUTTON);
            Button::new(cx, |cx| Label::new(cx, "Step"))
                .on_press(|cx| cx.emit(AppEvent::Step))
                .class(style::BUTTON);
        })
        .class(style::MENU_ELEMENT);
        HStack::new(cx, |cx: &mut Context| {
            Slider::new(cx, AppData::speed.map(|speed| 0_f32.max(*speed).min(1.0)))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0))
                .space(Stretch(0.05))
                .range(0.01..1.0)
                .on_changing(|cx, progress| cx.emit(AppEvent::SetSpeed(progress)));
            Textbox::new(cx, AppData::speed.map(|speed| format!("{speed:.2}")))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0))
                .space(Stretch(0.05))
                .on_edit(|cx, text| {
                    if let Ok(speed) = text.parse() {
                        cx.emit(AppEvent::SetSpeed(speed));
                    }
                });
        })
        .class(style::MENU_ELEMENT);
        VStack::new(cx, |cx| {
            Label::new(cx, AppData::tooltip);
        })
        .size(Stretch(10.));
    })
    .class(style::SIDE_PANEL);
}

pub fn center_panel(cx: &mut Context) {
    Binding::new(cx, AppData::grid, |cx, grid| {
        ZStack::new(cx, |cx| {
            grid.get(cx).display(cx);
        })
        .size(AppData::window_size.map(|bounds| Pixels(margined_square_size(bounds))))
        .class(style::CENTER_PANEL);
    });
}

pub fn right_panel(cx: &mut Context) {
    VStack::new(cx, |_cx| {}).class("side-panel");
}

fn margined_square_size(bounds: &BoundingBox) -> f32 {
    let max_width =
        bounds.width() * (style::CENTER_MARGIN_FACTOR - style::BACKGROUND_PADDING * 2.0);
    let max_height = bounds.height() * (1.0 - style::BACKGROUND_PADDING * 2.0);

    max_width.min(max_height)
    // bounds
    //     .height()
    //     .min(bounds.width() * style::CENTER_MARGIN_FACTOR)
    //     - (bounds.width().max(bounds.height()) * style::BACKGROUND_PADDING)
}

fn display_cell(grid: &Grid, cx: &mut Context, x: usize, y: usize) {
    let Some(cell) = grid.cell_at(x, y) else {
        println!("Cell at '{x}, {y}' doesn't exist; skipping...");
        return;
    };
    cell.display(cx, &grid.ruleset)
        .on_hover(move |cx| cx.emit(AppEvent::CellHovered(x, y)))
        .on_mouse_down(move |cx, button| cx.emit(AppEvent::CellClicked(x, y, button)));
}

pub mod style {
    pub const SIDE_PANEL: &str = "side-panel";
    pub const CENTER_PANEL: &str = "center-panel";
    pub const CELL: &str = "cell";
    pub const BACKGROUND: &str = "background";
    pub const BUTTON: &str = "button";
    pub const MENU_ELEMENT: &str = "menu-element";

    /// The maximum percentage of the screen the center square can take up.
    pub const CENTER_MARGIN_FACTOR: f32 = 0.6;
    /// Mirrors '.backround/child-space' in 'style.css'.
    pub const BACKGROUND_PADDING: f32 = 0.01;
    /// Mirrors '.cell/space' in 'style.css'.
    pub const CELL_SPACE: f32 = 0.06;
    /// How much darker the corners of a cell should be compared to the center, as a munber from 0.-255.
    pub const CELL_GRADIENT_DARKEN: u8 = 92;
}
