use std::path::Path;

use display::style;
use grid::Grid;
use material::{Material, MaterialColor, MaterialId};
use ruleset::Ruleset;
use vizia::prelude::*;

mod display;
mod grid;
mod material;
mod ruleset;

const INITIAL_WINDOW_SIZE: (u32, u32) = (1920 / 2, 1080 / 2);
const RULESET_PATH: &str = "./rulesets/";

#[derive(Debug, Lens)]
pub struct AppData {
    window_size: BoundingBox,

    grid: Grid,
    selected_material: MaterialId,
    running: bool,
    speed: f32,

    tooltip: String,
    hovered_index: Option<usize>,

    edit_screen_enabled: bool,
}
impl Default for AppData {
    fn default() -> Self {
        let mut ruleset = Ruleset::blank();
        let mut second_material = Material::new(&ruleset);
        second_material.color = MaterialColor::new(255, 0, 0);
        second_material.name = String::from("Red");
        ruleset.materials.push(second_material);
        let material = ruleset.materials.default().id;
        let grid = Grid::new(ruleset.clone(), 5);
        Self {
            window_size: BoundingBox {
                x: 0.,
                y: 0.,
                w: INITIAL_WINDOW_SIZE.0 as f32,
                h: INITIAL_WINDOW_SIZE.1 as f32,
            },

            grid,
            selected_material: material,
            running: false,
            speed: 1.0,

            tooltip: String::new(),
            hovered_index: None,

            edit_screen_enabled: false,
        }
    }
}

enum AppEvent {
    UpdateWindowSize,

    CellHovered(usize, usize),
    CellUnhovered,
    CellClicked(usize, usize, MouseButton),
    MaterialSelected(MaterialId),

    SaveRuleset,

    ToggleRunning,
    SetSpeed(f32),
    Step,

    ToggleEditScreen(bool),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            AppEvent::UpdateWindowSize => self.window_size = cx.bounds(),

            AppEvent::CellHovered(x, y) => self.hovered_index = Some(self.grid.cell_index(*x, *y)),
            AppEvent::CellUnhovered => self.hovered_index = None,
            AppEvent::CellClicked(_, _, _) => {}
            AppEvent::MaterialSelected(material_id) => self.selected_material = *material_id,

            AppEvent::SaveRuleset => {
                if let Err(err) = self.grid.ruleset.save() {
                    println!("{err}");
                }
            }

            AppEvent::ToggleRunning => self.running = !self.running,
            AppEvent::SetSpeed(speed) => self.speed = (*speed * 100.0).round() / 100.0,
            AppEvent::Step => {}

            AppEvent::ToggleEditScreen(toggle_on) => self.edit_screen_enabled = *toggle_on,
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("resources/style.css"))
            .expect("failed to add stylesheet.");

        AppData::default().build(cx);
        ZStack::new(cx, |cx| {
            display::game_board(cx);
        })
        .on_geo_changed(|cx, changes| {
            if changes.contains(GeoChanged::WIDTH_CHANGED)
                || changes.contains(GeoChanged::HEIGHT_CHANGED)
            {
                cx.emit(AppEvent::UpdateWindowSize);
            }
        });
    })
    .inner_size(INITIAL_WINDOW_SIZE)
    .run()
}
