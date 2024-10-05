use vizia::{
    binding::{Data, LensExt},
    context::{Context, EmitContext},
    modifiers::{ActionModifiers, StyleModifiers},
    style::RGBA,
    view::Handle,
    views::{Button, Element, HStack, VStack},
};

use crate::{
    display::style,
    material::{Material, MaterialId},
    ruleset::Ruleset,
    AppData, AppEvent,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    pub ruleset: Ruleset,
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
        cell.display(cx, &self.ruleset)
            .border_color(AppData::hovered_index.map(move |index| {
                if index.is_some_and(|index| cell_index == index) {
                    "black"
                } else {
                    "transparent"
                }
            }))
            .on_hover(move |cx| cx.emit(AppEvent::CellHovered(x, y)))
            .on_mouse_down(move |cx, button| cx.emit(AppEvent::CellClicked(x, y, button)));
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.size + x)
    }

    pub const fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    pub fn new(ruleset: Ruleset, size: usize) -> Self {
        let material = ruleset.materials.default();
        let cell = Cell::new(material);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub material_id: MaterialId,
}
impl Cell {
    pub fn new(material: &Material) -> Self {
        Self {
            material_id: material.id,
        }
    }

    pub fn color(&self, ruleset: &Ruleset) -> RGBA {
        ruleset
            .materials
            .get(self.material_id)
            .expect("cell should point to a valid material id for this ruleset.")
            .color
            .to_rgba()
    }

    pub fn display<'c>(&self, cx: &'c mut Context, ruleset: &Ruleset) -> Handle<'c, Button> {
        Button::new(cx, |cx| Element::new(cx))
            .class(style::CELL)
            .background_gradient(self.gradient(ruleset).as_str())
            .on_hover_out(|cx| cx.emit(AppEvent::CellUnhovered))
    }
    #[rustfmt::skip]
    fn gradient(&self, ruleset: &Ruleset) -> String {
        let color = self.color(ruleset);
        let darken_value = style::CELL_GRADIENT_DARKEN;
        let dark_color = RGBA::rgb(
            color.r().saturating_sub(darken_value),
            color.g().saturating_sub(darken_value),
            color.b().saturating_sub(darken_value)
        );
        format!(
            "radial-gradient(rgba({}, {}, {}), rgba({}, {}, {}))",
            color.r(),      color.g(),      color.b(),
            dark_color.r(), dark_color.g(), dark_color.b()
        )
    }
}
