use vizia::{
    binding::{Data, LensExt},
    context::{Context, EmitContext},
    modifiers::{ActionModifiers, StyleModifiers},
    style::RGBA,
    view::Handle,
    views::{Button, Element, HStack, VStack},
};

use crate::{
    condition::Direction, display::style, events::UpdateEvent, id::Identifiable,
    material::MaterialId, ruleset::Ruleset, AppData,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    pub ruleset: Ruleset,
    cells: Vec<Cell>,
    pub size: usize,
}
impl Grid {
    pub fn new(ruleset: Ruleset, size: usize) -> Self {
        let material = ruleset.materials.default();
        let cell = Cell::new(material.id());
        let cells = vec![cell; size * size];
        Self {
            ruleset,
            cells,
            size,
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, new: Cell) {
        let index = self.cell_index(x, y);
        if self.cells.get(index).is_none() {
            println!("Tried setting value of non-existent cell. Aborting.");
            return;
        }
        let _ = std::mem::replace(&mut self.cells[index], new);
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(self.cell_index(x, y)).copied()
    }

    pub const fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }
    pub const fn cell_coordinates(&self, index: usize) -> (usize, usize) {
        (index % self.size, index / self.size)
    }

    pub fn neighbors(&self, index: usize) -> CellNeighbors {
        let array = [
            self.get_neighbor(index, -1, -1),
            self.get_neighbor(index, 0, -1),
            self.get_neighbor(index, 1, -1),
            self.get_neighbor(index, -1, 0),
            self.get_neighbor(index, 1, 0),
            self.get_neighbor(index, -1, 1),
            self.get_neighbor(index, 0, 1),
            self.get_neighbor(index, -1, 1),
        ];
        CellNeighbors(array)
    }
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn get_neighbor(&self, index: usize, x_offset: i8, y_offset: i8) -> Option<Cell> {
        let (x, y) = self.cell_coordinates(index);
        let x = x as isize + x_offset as isize;
        let y = y as isize + y_offset as isize;
        if x < 0 || x >= self.size as isize || y < 0 || y >= self.size as isize {
            None
        } else {
            self.cell_at(x as usize, y as usize)
        }
    }

    pub fn next_generation(&mut self) {
        let new_cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                self.ruleset
                    .rules
                    .iter()
                    .find_map(|rule| rule.transformed(self, *cell, index))
                    .unwrap_or(*cell)
            })
            .collect();
        self.cells = new_cells;
    }

    // Display
    pub fn display<'a>(&'a self, cx: &'a mut Context) {
        VStack::new(cx, |cx| {
            (0..self.size).for_each(|y| self.display_row(cx, y));
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
            .on_hover(move |cx| cx.emit(UpdateEvent::CellHovered { x, y }))
            .on_mouse_down(move |cx, button| cx.emit(UpdateEvent::CellClicked { x, y, button }));
    }
}
impl Data for Grid {
    fn same(&self, other: &Self) -> bool {
        self.size == other.size && self.cells == other.cells && self.ruleset == other.ruleset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub material_id: MaterialId,
}
impl Cell {
    pub const fn new(material_id: MaterialId) -> Self {
        Self { material_id }
    }

    pub fn color(self, ruleset: &Ruleset) -> RGBA {
        ruleset
            .materials
            .get(self.material_id)
            .expect("cell should point to a valid material id for this ruleset.")
            .color
            .to_rgba()
    }

    pub fn display<'c>(self, cx: &'c mut Context, ruleset: &Ruleset) -> Handle<'c, Button> {
        Button::new(cx, Element::new)
            .class(style::CELL)
            .background_gradient(self.gradient(ruleset).as_str())
            .on_hover_out(|cx| cx.emit(UpdateEvent::CellUnhovered))
    }
    #[rustfmt::skip]
    fn gradient(self, ruleset: &Ruleset) -> String {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellNeighbors([Option<Cell>; 8]);
impl CellNeighbors {
    pub const fn new(array: [Option<Cell>; 8]) -> Self {
        Self(array)
    }

    pub fn count(&self) -> u8 {
        self.0
            .iter()
            .filter(|cell| cell.is_some())
            .count()
            .try_into()
            .expect("CellNeighbors count should not exceed 8.")
    }
    pub const fn in_direction(&self, direction: Direction) -> Option<Cell> {
        match direction {
            Direction::Northwest => self.0[0],
            Direction::North => self.0[1],
            Direction::Northeast => self.0[2],
            Direction::West => self.0[3],
            Direction::East => self.0[4],
            Direction::Southwest => self.0[5],
            Direction::South => self.0[6],
            Direction::Southeast => self.0[7],
        }
    }
}
