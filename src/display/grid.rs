use vizia::{
    binding::{Data, Lens, ResGet},
    context::{Context, EmitContext},
    layout::BoundingBox,
    vg,
    view::{Handle, View},
    window::WindowEvent,
};

use crate::{display, events::UpdateEvent, material::MaterialColor};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VisualGridState {
    size: usize,
    cells: Vec<MaterialColor>,
}
impl VisualGridState {
    pub const fn new(size: usize, cells: Vec<MaterialColor>) -> Self {
        Self { size, cells }
    }
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
