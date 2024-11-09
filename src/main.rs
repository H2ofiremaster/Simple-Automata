#![allow(clippy::expl_impl_clone_on_copy)]

use display::{InputName, Screen};
use grid::{Cell, Grid};
use id::Identifiable;
use material::{Material, MaterialColor, MaterialGroup, MaterialId};
use rand::seq::index;
use ruleset::Ruleset;
use vizia::prelude::*;

mod display;
mod grid;
mod id;
mod material;
mod pattern;
mod ruleset;

const INITIAL_WINDOW_SIZE: (u32, u32) = (1920 / 2, 1080 / 2);

#[derive(Debug, Lens)]
pub struct AppData {
    window_size: BoundingBox,

    rulesets: Vec<Ruleset>,
    screen: Screen,
    selected_ruleset: usize,
    selected_material: MaterialId,
    running: bool,
    speed: f32,
    grid_size: usize,

    tooltip: String,
    hovered_index: Option<usize>,
    new_object_name: String,
    displayed_input: display::InputName,
    selected_tab: display::EditorTab,
    group_material_index: usize,

    editor_enabled: bool,
}
#[allow(clippy::cast_precision_loss)]
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

        let material = ruleset.materials.default().id();
        let grid = Grid::new(ruleset.clone(), 5);
        Self {
            window_size: BoundingBox {
                x: 0.,
                y: 0.,
                w: INITIAL_WINDOW_SIZE.0 as f32,
                h: INITIAL_WINDOW_SIZE.1 as f32,
            },

            rulesets: Ruleset::load_all().unwrap_or_else(|err| {
                println!("Failed to load rulesets; falling back: {err}");
                vec![ruleset]
            }),
            selected_ruleset: 0,
            screen: Screen::Grid(grid),
            selected_material: material,
            running: false,
            speed: 1.0,
            grid_size: 5,

            tooltip: String::new(),
            hovered_index: None,
            new_object_name: String::from("TESTS"),
            displayed_input: display::InputName::None,
            selected_tab: display::EditorTab::Materials,
            group_material_index: 0,

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

    SelectRuleset(usize),
    SaveRuleset,
    StartNewRuleset,
    NewRuleset(String),
    ReloadRulesets,

    NewMaterial,
    MaterialName(usize, String),
    MaterialColor(usize, String),
    DeleteMaterial(MaterialId),

    NewGroup,
    EditGroup(usize, usize, usize),
    DeleteFromGroup(usize, usize),
    AddToGroup(usize),

    ToggleRunning,
    SetSpeed(f32),
    Step,

    ToggleEditor(bool),
    SwitchTab(display::EditorTab),
}

impl Model for AppData {
    #[allow(clippy::too_many_lines)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            AppEvent::UpdateWindowSize => self.window_size = cx.bounds(),

            AppEvent::CellHovered(x, y) => {
                if let Screen::Grid(ref grid) = self.screen {
                    self.hovered_index = Some(grid.cell_index(*x, *y));
                }
            }
            AppEvent::CellUnhovered => self.hovered_index = None,
            AppEvent::CellClicked(x, y, button) => {
                let Screen::Grid(ref mut grid) = self.screen else {
                    return;
                };
                let new_material: MaterialId = match button {
                    MouseButton::Left => self.selected_material,
                    MouseButton::Right => grid.ruleset.materials.default().id(),
                    _ => return,
                };

                let cell = Cell::new(new_material);
                grid.set_cell(*x, *y, cell);
            }
            AppEvent::MaterialSelected(material_id) => self.selected_material = *material_id,

            AppEvent::SelectRuleset(index) => {
                self.selected_ruleset = *index;
                let ruleset = self.rulesets[*index].clone();
                match self.screen {
                    Screen::Grid(_) => {
                        self.screen = Screen::Grid(Grid::new(ruleset, self.grid_size));
                    }
                    Screen::Editor(_) => self.screen = Screen::Editor(ruleset),
                }
            }
            AppEvent::SaveRuleset => {
                if let Err(err) = self.screen.ruleset().save() {
                    println!("{err}");
                }
            }
            AppEvent::StartNewRuleset => {
                self.displayed_input = display::InputName::Ruleset;
                self.new_object_name = String::new();
            }
            AppEvent::NewRuleset(name) => {
                let new_ruleset = Ruleset::new(name.clone());
                self.rulesets.push(new_ruleset);
                self.new_object_name.clone_from(name);

                self.displayed_input = display::InputName::None;
                cx.emit(AppEvent::SelectRuleset(self.rulesets.len() - 1));
            }
            AppEvent::ReloadRulesets => {
                self.rulesets = Ruleset::load_all().unwrap_or_else(|err| {
                    println!("Failed to load rulesets; falling back: {err}");
                    vec![Ruleset::blank()]
                });
            }

            AppEvent::NewMaterial => {
                let material = Material::new(self.screen.ruleset());
                self.screen.ruleset_mut().materials.push(material);
            }
            AppEvent::MaterialName(index, text) => {
                if let Some(material) = self.screen.ruleset_mut().materials.get_mut_at(*index) {
                    material.name.clone_from(text);
                };
            }
            AppEvent::MaterialColor(index, text) => {
                if let Some(material) = self.screen.ruleset_mut().materials.get_mut_at(*index) {
                    if let Ok(color) = text.parse() {
                        material.color = color;
                    }
                }
            }
            AppEvent::DeleteMaterial(material_id) => {
                self.screen.ruleset_mut().materials.remove(*material_id);
            }

            AppEvent::NewGroup => {
                let ruleset = self.screen.ruleset_mut();
                ruleset.groups.push(MaterialGroup::new(ruleset));
                println!("Test");
            }
            AppEvent::EditGroup(group_index, entry_index, new_material_index) => {
                let ruleset = self.screen.ruleset_mut();
                if let Some(group) = ruleset.groups.get_mut(*group_index) {
                    if let Some(material_id) = ruleset
                        .materials
                        .get_at(*new_material_index)
                        .map(Material::id)
                    {
                        if let Some(old_material) = group.get_mut(*entry_index) {
                            let _ = std::mem::replace(old_material, material_id);
                        }
                    };
                };
            }
            AppEvent::DeleteFromGroup(group_index, entry_index) => {
                let ruleset = self.screen.ruleset_mut();
                if let Some(group) = ruleset.groups.get_mut(*group_index) {
                    group.remove_at(*entry_index);
                }
            }
            AppEvent::AddToGroup(group_index) => {
                let ruleset = self.screen.ruleset_mut();
                if let Some(group) = ruleset.groups.get_mut(*group_index) {
                    let material = ruleset.materials.default();
                    group.push(material.id());
                    self.group_material_index = 0;
                    self.displayed_input = InputName::None;
                };
            }

            AppEvent::ToggleRunning => self.running = !self.running,
            AppEvent::SetSpeed(speed) => self.speed = (*speed * 100.0).round() / 100.0,
            AppEvent::Step => {
                if let Screen::Grid(ref mut grid) = self.screen {
                    grid.next_generation();
                }
            }

            AppEvent::ToggleEditor(toggle_on) => {
                self.editor_enabled = *toggle_on;
                let ruleset = self.screen.ruleset().clone();
                match toggle_on {
                    true => self.screen = Screen::Editor(ruleset),
                    false => self.screen = Screen::Grid(Grid::new(ruleset, self.grid_size)),
                }
            }
            AppEvent::SwitchTab(tab) => self.selected_tab = *tab,
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
