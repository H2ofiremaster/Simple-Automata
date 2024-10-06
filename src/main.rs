use std::path::Path;

use app_data_derived_lenses::editor_enabled;
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

    rulesets: Vec<Ruleset>,
    grid: Grid,
    selected_ruleset: usize,
    selected_material: MaterialId,
    running: bool,
    speed: f32,

    tooltip: String,
    hovered_index: Option<usize>,

    editor_enabled: bool,
}
impl Default for AppData {
    fn default() -> Self {
        let mut ruleset = Ruleset::blank();
        let mut second_material = Material::new(&ruleset);
        second_material.color = MaterialColor::new(255, 0, 0);
        second_material.name = String::from("Red");
        ruleset.materials.push(second_material);

        let mut ruleset_2 = Ruleset::blank();
        ruleset_2.name = String::from("Second");
        let mut r2m2 = Material::new(&ruleset);
        r2m2.color = MaterialColor::new(0, 255, 0);
        r2m2.name = String::from("Green");
        ruleset_2.materials.push(r2m2);

        let material = ruleset.materials.default().id;
        let grid = Grid::new(ruleset.clone(), 5);
        Self {
            window_size: BoundingBox {
                x: 0.,
                y: 0.,
                w: INITIAL_WINDOW_SIZE.0 as f32,
                h: INITIAL_WINDOW_SIZE.1 as f32,
            },

            rulesets: vec![ruleset, ruleset_2],
            selected_ruleset: 0,
            grid,
            selected_material: material,
            running: false,
            speed: 1.0,

            tooltip: String::new(),
            hovered_index: None,

            editor_enabled: false,
        }
    }
}

enum AppEvent {
    UpdateWindowSize,

    CellHovered(usize, usize),
    CellUnhovered,
    CellClicked(usize, usize, MouseButton),
    MaterialSelected(MaterialId),

    SelectRulest(usize),
    SaveRuleset,

    ToggleRunning,
    SetSpeed(f32),
    Step,

    ToggleEditor(bool),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            AppEvent::UpdateWindowSize => self.window_size = cx.bounds(),

            AppEvent::CellHovered(x, y) => self.hovered_index = Some(self.grid.cell_index(*x, *y)),
            AppEvent::CellUnhovered => self.hovered_index = None,
            AppEvent::CellClicked(_, _, _) => {}
            AppEvent::MaterialSelected(material_id) => self.selected_material = *material_id,

            AppEvent::SelectRulest(index) => {
                self.selected_ruleset = *index;
                self.grid = Grid::new(self.rulesets[*index].clone(), self.grid.size);
            }
            AppEvent::SaveRuleset => {
                if let Err(err) = self.grid.ruleset.save() {
                    println!("{err}");
                }
            }

            AppEvent::ToggleRunning => self.running = !self.running,
            AppEvent::SetSpeed(speed) => self.speed = (*speed * 100.0).round() / 100.0,
            AppEvent::Step => {}

            AppEvent::ToggleEditor(toggle_on) => self.editor_enabled = *toggle_on,
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("resources/style.css"))
            .expect("failed to add stylesheet.");

        AppData::default().build(cx);
        ZStack::new(cx, |cx| {
            Binding::new(cx, AppData::editor_enabled, |cx, enabled| {
                if enabled.get(cx) {
                    display::ruleset_editor(cx);
                } else {
                    display::game_board(cx);
                }
            });
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
