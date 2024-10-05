use vizia::{
    binding::{Data, LensExt},
    context::{Context, EmitContext},
    layout::Units::{Pixels, Stretch},
    modifiers::{ActionModifiers, LayoutModifiers, StyleModifiers},
    style::{LengthOrPercentage, RGBA},
    views::{HStack, VStack},
};

use crate::{cell::Cell, display::style, ruleset::Ruleset, AppData, AppEvent};

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    ruleset: Ruleset,
    cells: Vec<Cell>,
    pub size: usize,
}
impl Grid {
    pub fn display<'a>(&'a self, cx: &'a mut Context) {
        VStack::new(cx, |cx| {
            (0..self.size).for_each(|y| self.display_row(cx, y))
        });
    }
    fn display_row(&self, cx: &mut Context, y: usize) {
        HStack::new(cx, |cx| {
            (0..self.size).for_each(|x| self.display_cell(cx, x, y));
        });
    }
    fn display_cell(&self, cx: &mut Context, x: usize, y: usize) {
        let Some(cell) = self.cell_at(x, y) else {
            println!("Cell at '{x}, {y}' doesn't exist; skipping...");
            return;
        };
        let cell_index = self.cell_index(x, y);
        let mut cell_display = cell
            .display(cx)
            .border_color(AppData::hovered_index.map(move |index| {
                if index.is_some_and(|index| cell_index == index) {
                    "black"
                } else {
                    "transparent"
                }
            }))
            .border_width(LengthOrPercentage::Percentage(5.0))
            .on_hover(move |cx| cx.emit(AppEvent::CellHovered(x, y)))
            .on_mouse_down(move |cx, button| cx.emit(AppEvent::CellClicked(x, y, button)));
        // if x == 0 {
        //     cell_display = cell_display.left(Stretch(style::CELL_SPACE / 2.0));
        // } else if x == self.size - 1 {
        //     cell_display = cell_display.right(Stretch(style::CELL_SPACE / 2.0));
        // }

        // if y == 0 {
        //     cell_display.top(Stretch(style::CELL_SPACE / 2.0));
        // } else if y == self.size - 1 {
        //     cell_display.bottom(Stretch(style::CELL_SPACE / 2.0));
        // }
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.size + x)
    }

    pub const fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    pub fn generate(ruleset: Ruleset, size: usize) -> Self {
        let material = ruleset.default_material();
        let cell = Cell::new(material.clone());
        let cells = vec![cell; size * size];
        Self {
            ruleset,
            cells,
            size,
        }
    }
}

impl Data for Grid {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}
