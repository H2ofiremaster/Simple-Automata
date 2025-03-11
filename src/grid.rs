use vizia::{
    binding::{Data, Lens, ResGet},
    context::{Context, EmitContext},
    layout::BoundingBox,
    modifiers::{ActionModifiers, StyleModifiers},
    style::RGBA,
    vg,
    view::{Handle, View},
    views::{Button, Element},
    window::WindowEvent,
};

use crate::{
    condition::Direction,
    display::{self, style},
    events::UpdateEvent,
    id::Identifiable,
    material::{MaterialColor, MaterialId},
    pattern::Pattern,
    ruleset::Ruleset,
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
            self.get_neighbor(index, 1, 1),
        ];
        CellNeighbors::new(array)
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

    pub fn visual_state(&self) -> VisualGridState {
        VisualGridState {
            size: self.size,
            cells: self.cells.iter().map(|&c| c.color(&self.ruleset)).collect(),
        }
    }
    pub fn functional_state(&self) -> FunctionalGridState {
        FunctionalGridState {
            size: self.size,
            cells: self.cells.clone(),
        }
    }

    pub fn load_state(&mut self, state: FunctionalGridState) {
        self.size = state.size;
        self.cells = state.cells;
    }
}
impl Data for Grid {
    fn same(&self, other: &Self) -> bool {
        self.size == other.size && self.cells == other.cells && self.ruleset == other.ruleset
    }
}

#[derive(Debug, Clone)]
pub struct FunctionalGridState {
    size: usize,
    cells: Vec<Cell>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VisualGridState {
    size: usize,
    cells: Vec<MaterialColor>,
}
impl Data for VisualGridState {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct GridDisplay<L1, L2, L3>
where
    L1: Lens<Target = VisualGridState>,
    L2: Lens<Target = Option<usize>>,
    L3: Lens<Target = bool>,
{
    grid: L1,
    hovered: L2,
    lines_enabled: L3,
}
impl<L1, L2, L3> GridDisplay<L1, L2, L3>
where
    L1: Lens<Target = VisualGridState>,
    L2: Lens<Target = Option<usize>>,
    L3: Lens<Target = bool>,
{
    const PADDING_MARGIN: f32 = 0.1;
    pub fn new(cx: &mut Context, grid: L1, hovered: L2, lines_enabled: L3) -> Handle<Self> {
        Self {
            grid,
            hovered,
            lines_enabled,
        }
        .build(cx, move |_| {})
        .bind(grid, |mut cx, _| cx.needs_redraw())
        .bind(hovered, |mut cx, _| cx.needs_redraw())
        .bind(lines_enabled, |mut cx, _| cx.needs_redraw())
    }

    #[allow(clippy::cast_precision_loss)]
    fn cell_size(grid_size: usize, bounds: BoundingBox, lines_enabled: bool) -> (f32, f32) {
        let original_cell_size = bounds.width() / grid_size as f32;
        let padding =
            1.0_f32.max(Self::PADDING_MARGIN * original_cell_size) * f32::from(lines_enabled);
        let cell_size = original_cell_size - padding;
        (cell_size, padding)
    }
}
impl<L1, L2, L3> View for GridDisplay<L1, L2, L3>
where
    L1: Lens<Target = VisualGridState>,
    L2: Lens<Target = Option<usize>>,
    L3: Lens<Target = bool>,
{
    #[allow(clippy::cast_precision_loss)]
    fn draw(&self, cx: &mut vizia::context::DrawContext, canvas: &vizia::vg::Canvas) {
        let mut main_paint = vg::Paint::default();
        main_paint.set_color(cx.background_color());
        let mut border_paint = vg::Paint::default();

        let grid_size = self.grid.get(cx).size;
        let hovered = self.hovered.get(cx);
        let lines_enabled = self.lines_enabled.get(cx);
        let cells: &[MaterialColor] = &self.grid.get(cx).cells;

        let full_bounds = cx.bounds();
        let bounds = display::rect_bounds(&full_bounds);
        let (cell_size, padding) = Self::cell_size(grid_size, bounds, lines_enabled);
        for y in 0..grid_size {
            for x in 0..grid_size {
                // Equivelent to: ((x as f32) * (padding + cell_size) + bounds.left) + padding / 2.0;
                let cell_x = (x as f32).mul_add(padding + cell_size, bounds.left()) + padding / 2.0;
                let cell_y = (y as f32).mul_add(padding + cell_size, bounds.top()) + padding / 2.0;
                let rect = vg::Rect::from_xywh(cell_x, cell_y, cell_size, cell_size);

                let color: MaterialColor = *cells
                    .get((y * grid_size) + x)
                    .unwrap_or(&MaterialColor::DEFAULT);
                main_paint.set_color(color);
                border_paint.set_color(color.invert_grayscale());

                if hovered.is_some_and(|s| s == (y * grid_size) + x) {
                    let border = rect.with_outset((cell_size * 0.05, cell_size * 0.05));
                    canvas.draw_rect(border, &border_paint);
                }
                canvas.draw_rect(rect, &main_paint);
            }
        }
    }

    fn event(&mut self, cx: &mut vizia::context::EventContext, event: &mut vizia::events::Event) {
        event.map(|event: &WindowEvent, meta| match event {
            WindowEvent::MouseMove(x, y) => {
                if meta.target != cx.current() {
                    return;
                }
                let full_bounds = cx.bounds();
                if !full_bounds.contains_point(*x, *y) {
                    return;
                }
                let bounds = display::rect_bounds(&full_bounds);
                if !bounds.contains_point(*x, *y) {
                    return;
                }
                let grid_size = self.grid.get(cx).size;
                let (cell_size, padding) =
                    Self::cell_size(grid_size, bounds, self.lines_enabled.get(cx));
                let x = x - bounds.left() - (padding / 2.0);
                let y = y - bounds.top() - (padding / 2.0);
                let normalized_x = x / (cell_size + padding);
                let normalized_y = y / (cell_size + padding);
                let in_cell = normalized_x - normalized_x.floor() < 1.0 - Self::PADDING_MARGIN
                    && normalized_y - normalized_y.floor() < 1.0 - Self::PADDING_MARGIN;
                if in_cell {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    cx.emit(UpdateEvent::CellHovered {
                        x: normalized_x as usize,
                        y: normalized_y as usize,
                    });
                } else {
                    cx.emit(UpdateEvent::CellUnhovered);
                }
            }
            WindowEvent::MouseDown(button) => {
                cx.emit(UpdateEvent::CellClicked(*button));
            }
            _ => {}
        });
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

    pub fn color(self, ruleset: &Ruleset) -> MaterialColor {
        ruleset
            .materials
            .get(self.material_id)
            .unwrap_or_else(|| {
                println!("'Cell::color' called on foreign cell.");
                ruleset.materials.default()
            })
            .color
    }

    pub fn display<'c>(self, cx: &'c mut Context, ruleset: &Ruleset) -> Handle<'c, Button> {
        Button::new(cx, Element::new)
            .class(style::CELL)
            .background_gradient(self.gradient(ruleset).as_str())
            .on_hover_out(|cx| cx.emit(UpdateEvent::CellUnhovered))
    }
    #[rustfmt::skip]
    fn gradient(self, ruleset: &Ruleset) -> String {
        let color = self.color(ruleset).to_rgba();
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
pub struct CellNeighbors(pub [Option<Cell>; 8]);
impl CellNeighbors {
    pub const fn new(array: [Option<Cell>; 8]) -> Self {
        Self(array)
    }

    pub fn count_matching(&self, ruleset: &Ruleset, pattern: Pattern) -> u8 {
        // println!("Matching: ---");
        self.0
            .iter()
            .filter(|cell| cell.is_some_and(|cell| pattern.matches(ruleset, cell)))
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
