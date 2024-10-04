use cell::Material;
use display::style;
use grid::Grid;
use ruleset::Ruleset;
use vizia::prelude::*;

mod cell;
mod display;
mod grid;
mod ruleset;

const INITIAL_WINDOW_SIZE: (u32, u32) = (1920 / 2, 1080 / 2);

#[derive(Debug, Lens)]
pub struct AppData {
    window_size: BoundingBox,
    selected_material: Material,
    selected_ruleset: Ruleset,
    grid: Grid,
    tooltip: String,
    hovered_index: Option<usize>,
}
impl Default for AppData {
    fn default() -> Self {
        let ruleset = Ruleset::blank();
        let grid = Grid::generate(ruleset.clone(), 5);
        Self {
            window_size: BoundingBox {
                x: 0.,
                y: 0.,
                w: INITIAL_WINDOW_SIZE.0 as f32,
                h: INITIAL_WINDOW_SIZE.1 as f32,
            },
            selected_material: ruleset.default_material().clone(),
            selected_ruleset: ruleset,
            grid,
            tooltip: String::new(),
            hovered_index: None,
        }
    }
}

enum AppEvent {
    UpdateWindowSize,
    CellHovered(usize, usize),
    CellUnhovered,
    CellClicked(usize, usize, MouseButton),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            AppEvent::UpdateWindowSize => {
                self.window_size = cx.bounds();
            }
            AppEvent::CellHovered(x, y) => {
                self.hovered_index = Some(self.grid.cell_index(*x, *y));
                println!("{:?}", self.hovered_index)
            }
            AppEvent::CellUnhovered => {
                self.hovered_index = None;
            }
            AppEvent::CellClicked(_, _, _) => {}
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("resources/style.css"))
            .expect("failed to add stylesheet.");

        AppData::default().build(cx);
        HStack::new(cx, |cx| {
            display::left_panel(cx);
            display::center_panel(cx);
            display::right_panel(cx);
        })
        .on_geo_changed(|cx, changes| {
            if changes.contains(GeoChanged::WIDTH_CHANGED)
                || changes.contains(GeoChanged::HEIGHT_CHANGED)
            {
                cx.emit(AppEvent::UpdateWindowSize);
            }
        })
        .class(style::BACKGROUND);
    })
    .inner_size(INITIAL_WINDOW_SIZE)
    .run()
}
