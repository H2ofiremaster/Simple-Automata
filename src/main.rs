#![allow(clippy::expl_impl_clone_on_copy)]

use condition::{Condition, ConditionVariant, Operator};
use display::Screen;
use events::{
    ConditionEvent, EditorEvent, GridEvent, GroupEvent, MaterialEvent, RuleEvent, RulesetEvent,
    UpdateEvent,
};
use grid::{Cell, FunctionalGridState, Grid};
use id::Identifiable;
use material::{Material, MaterialGroup, MaterialId};
use pattern::Pattern;
use ruleset::{Rule, Ruleset};
use vizia::prelude::*;

mod condition;
mod display;
mod events;
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
    timer: Timer,
    grid_size: usize,
    saved_state: Option<FunctionalGridState>,
    grid_lines_enabled: bool,

    tooltip: (String, Color),
    hovered_index: Option<usize>,
    selected_tab: display::EditorTab,
    group_material_index: usize,

    editor_enabled: bool,
}
#[allow(clippy::cast_precision_loss)]
impl AppData {
    fn new(timer: Timer) -> Self {
        let ruleset = Ruleset::blank();

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
            timer,
            grid_size: 5,
            saved_state: None,
            grid_lines_enabled: true,

            tooltip: (String::new(), Color::white()),
            hovered_index: None,
            selected_tab: display::EditorTab::Materials,
            group_material_index: 0,

            editor_enabled: false,
        }
    }
}

impl Model for AppData {
    #[allow(clippy::too_many_lines)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event: &UpdateEvent, _| match event {
            UpdateEvent::WindowSizeChanged => self.window_size = cx.bounds(),
            UpdateEvent::CellHovered { x, y } => {
                if let Screen::Grid(ref grid) = self.screen {
                    let index = grid.cell_index(*x, *y);
                    if self.hovered_index.is_some_and(|i| i == index) {
                        return;
                    }
                    self.hovered_index = Some(index);

                    if let Some(cell) = grid.cell_at(*x, *y) {
                        self.tooltip = (
                            grid.ruleset
                                .materials
                                .get(cell.material_id)
                                .map_or_else(String::new, |m| m.name.clone()),
                            Color::yellow(),
                        );
                    }

                    let mouse_state = cx.mouse();
                    let button = match (mouse_state.left.state, mouse_state.right.state) {
                        (MouseButtonState::Pressed, MouseButtonState::Released) => {
                            MouseButton::Left
                        }
                        (_, MouseButtonState::Pressed) => MouseButton::Right,
                        _ => return,
                    };
                    cx.emit(UpdateEvent::CellClicked(button));
                }
            }
            UpdateEvent::CellUnhovered => {
                self.hovered_index = None;
                self.tooltip = (
                    self.screen
                        .ruleset()
                        .materials
                        .get(self.selected_material)
                        .map_or_else(String::new, |m| m.name.clone()),
                    Color::white(),
                );
            }
            UpdateEvent::CellClicked(button) => {
                let Screen::Grid(ref mut grid) = self.screen else {
                    return;
                };
                let new_material: MaterialId = match button {
                    MouseButton::Left => self.selected_material,
                    MouseButton::Right => grid.ruleset.materials.default().id(),
                    _ => return,
                };
                self.tooltip = (
                    grid.ruleset
                        .materials
                        .get(new_material)
                        .map_or_else(String::new, |m| m.name.clone()),
                    Color::yellow(),
                );
                let cell = Cell::new(new_material);
                let Some(index) = self.hovered_index else {
                    return;
                };
                let x = index % grid.size;
                let y = index / grid.size;
                grid.set_cell(x, y, cell);
            }
            UpdateEvent::MaterialSelected(material_id) => self.selected_material = *material_id,
            UpdateEvent::MaterialHovered(material_id) => {
                self.tooltip = (
                    self.screen
                        .ruleset()
                        .materials
                        .get(*material_id)
                        .map_or_else(String::new, |m| m.name.clone()),
                    Color::aqua(),
                );
            }
            UpdateEvent::MaterialUnhovered => {
                self.tooltip = (
                    self.screen
                        .ruleset()
                        .materials
                        .get(self.selected_material)
                        .map_or_else(String::new, |m| m.name.clone()),
                    Color::white(),
                );
            }
        });
        event.map(|event: &RulesetEvent, _| match event {
            RulesetEvent::Selected(index) => {
                self.selected_ruleset = *index;
                let ruleset = self.rulesets[*index].clone();
                match self.screen {
                    Screen::Grid(_) => {
                        self.screen = Screen::Grid(Grid::new(ruleset, self.grid_size));
                    }
                    Screen::Editor(_) => self.screen = Screen::Editor(ruleset),
                }
            }
            RulesetEvent::Saved => {
                if let Err(err) = self.screen.ruleset().save() {
                    println!("{err}");
                }
            }
            RulesetEvent::Created => {
                let new_ruleset = Ruleset::new();
                self.rulesets.push(new_ruleset);

                cx.emit(RulesetEvent::Selected(self.rulesets.len() - 1));
            }
            RulesetEvent::Renamed(name) => {
                self.screen.ruleset_mut().name.clone_from(name);
                self.rulesets[self.selected_ruleset].name.clone_from(name);
            }
            RulesetEvent::Reloaded => {
                self.rulesets = Ruleset::load_all().unwrap_or_else(|err| {
                    println!("Failed to load rulesets; falling back: {err}");
                    vec![Ruleset::blank()]
                });
            }
        });
        event.map(|event: &MaterialEvent, _| match event {
            MaterialEvent::Created => {
                let material = Material::new(self.screen.ruleset());
                self.screen.ruleset_mut().materials.push(material);
            }
            MaterialEvent::Renamed(index, name) => {
                if let Some(material) = self.screen.ruleset_mut().materials.get_mut_at(*index) {
                    material.name.clone_from(name);
                };
            }
            MaterialEvent::Recolored(index, color) => {
                if let Some(material) = self.screen.ruleset_mut().materials.get_mut_at(*index) {
                    if let Ok(color) = color.parse() {
                        material.color = color;
                    }
                }
            }
            MaterialEvent::Deleted(material_id) => {
                self.screen.ruleset_mut().materials.remove(*material_id);
            }
        });
        event.map(|event: &GroupEvent, _| match event {
            GroupEvent::Created => {
                let ruleset = self.screen.ruleset_mut();
                ruleset.groups.push(MaterialGroup::new(ruleset));
            }
            GroupEvent::Deleted(group_index) => {
                self.screen.ruleset_mut().groups.remove(*group_index);
            }
            GroupEvent::Edited {
                group_index,
                entry_index,
                new_material_index,
            } => {
                let ruleset = self.screen.ruleset_mut();
                let Some(group) = ruleset.groups.get_mut(*group_index) else {
                    return;
                };
                let Some(material_id) = ruleset
                    .materials
                    .get_at(*new_material_index)
                    .map(Material::id)
                else {
                    return;
                };
                let Some(old_material) = group.get_mut(*entry_index) else {
                    return;
                };
                let _ = std::mem::replace(old_material, material_id);
            }
            GroupEvent::Renamed(group_index, name) => {
                let ruleset = self.screen.ruleset_mut();
                if let Some(group) = ruleset.groups.get_mut(*group_index) {
                    group.name.clone_from(name);
                }
            }
            GroupEvent::EntryDeleted {
                group_index,
                entry_index,
            } => {
                let ruleset = self.screen.ruleset_mut();
                if let Some(group) = ruleset.groups.get_mut(*group_index) {
                    group.remove_at(*entry_index);
                }
            }
            GroupEvent::EntryAdded(group_index) => {
                let ruleset = self.screen.ruleset_mut();
                if let Some(group) = ruleset.groups.get_mut(*group_index) {
                    let material = ruleset.materials.default();
                    group.push(material.id());
                    self.group_material_index = 0;
                };
            }
        });
        event.map(|event: &RuleEvent, _| match event {
            RuleEvent::Created => {
                let ruleset = self.screen.ruleset_mut();
                ruleset.rules.push(Rule::new(ruleset));
            }
            RuleEvent::Copied(index) => {
                let ruleset = self.screen.ruleset_mut();
                let rule = index.rule(ruleset);
                ruleset.rules.insert(index.value(), rule.clone());
            }
            RuleEvent::Deleted(index) => {
                self.screen.ruleset_mut().rules.remove(index.value());
            }
            RuleEvent::OutputSet(rule_index, material_index) => {
                let ruleset = self.screen.ruleset_mut();
                let Some(material) = ruleset.materials.get_at(*material_index) else {
                    return;
                };
                rule_index.rule_mut(ruleset).output = material.id();
            }
            RuleEvent::InputSet(rule_index, pattern_index) => {
                let ruleset = self.screen.ruleset_mut();
                let Some(pattern) = Pattern::from_index(ruleset, *pattern_index) else {
                    return;
                };
                rule_index.rule_mut(ruleset).input = pattern;
            }
        });
        event.map(|event: &ConditionEvent, _| match event {
            ConditionEvent::Created(index) => {
                let ruleset = self.screen.ruleset_mut();
                let new_condition = Condition::new(ruleset);
                index.rule_mut(ruleset).conditions.push(new_condition);
            }
            ConditionEvent::Copied(index) => {
                let ruleset = self.screen.ruleset_mut();
                let new_condition = index.condition(ruleset).clone();
                index
                    .rule_mut(ruleset)
                    .conditions
                    .insert(index.values().1, new_condition);
            }
            ConditionEvent::Deleted(index) => {
                let ruleset = self.screen.ruleset_mut();
                index.rule_mut(ruleset).conditions.remove(index.values().1);
            }
            ConditionEvent::PatternSet(condition_index, pattern_index) => {
                let ruleset = self.screen.ruleset_mut();
                let Some(pattern) = Pattern::from_index(ruleset, *pattern_index) else {
                    return;
                };
                let condition = condition_index.condition_mut(ruleset);
                condition.pattern = pattern;
            }
            ConditionEvent::DirectionToggled(index, direction) => {
                let ruleset = self.screen.ruleset_mut();
                let condition = index.condition_mut(ruleset);
                let Some(directions) = condition.variant.directions() else {
                    return;
                };
                let index = directions.iter().position(|dir| dir == direction);
                match index {
                    Some(index) => {
                        directions.remove(index);
                    }
                    None => directions.push(*direction),
                }
            }
            ConditionEvent::CountUpdated(index, count_string) => {
                let condition = index.condition_mut(self.screen.ruleset_mut());

                let ConditionVariant::Count(variant) = &condition.variant else {
                    return;
                };

                let mut elements: Vec<u8> = count_string
                    .chars()
                    .filter_map(|char| char.to_digit(10).and_then(|num| num.try_into().ok()))
                    .filter(|&n| n <= 8)
                    .collect();
                elements.sort_unstable();
                elements.dedup();
                condition.variant = ConditionVariant::Count(variant.with_elements(elements));
            }
            ConditionEvent::VariantChanged(index, variant) => {
                let ruleset = self.screen.ruleset_mut();
                index.condition_mut(ruleset).variant.clone_from(variant);
            }
            ConditionEvent::OperatorChanged(index) => {
                let ruleset = self.screen.ruleset_mut();
                let condition = index.condition_mut(ruleset);
                let ConditionVariant::Count(variant) = &condition.variant else {
                    return;
                };
                let new_variant = match variant {
                    Operator::List(vec) => Operator::Greater(vec.first().copied().unwrap_or(0)),
                    Operator::Greater(value) => Operator::Less(*value),
                    Operator::Less(value) => Operator::List(vec![*value]),
                };
                condition.variant = ConditionVariant::Count(new_variant);
            }
            ConditionEvent::Inverted(index) => {
                let ruleset = self.screen.ruleset_mut();
                let condition = index.condition_mut(ruleset);
                condition.inverted = !condition.inverted;
            }
        });
        event.map(|event: &GridEvent, _| match event {
            GridEvent::Stepped => {
                if let Screen::Grid(ref mut grid) = self.screen {
                    grid.next_generation();
                }
            }
            GridEvent::Toggled => {
                self.running = !self.running;
                if self.running {
                    cx.start_timer(self.timer);
                } else {
                    cx.stop_timer(self.timer);
                }
            }
            GridEvent::SpeedSet(speed) => {
                self.speed = (*speed * 100.0).round() / 100.0;
                cx.modify_timer(self.timer, |state| {
                    state.set_interval(Duration::from_secs_f32(self.speed));
                });
            }
            GridEvent::Resized(size) => {
                self.grid_size = *size;
                if let Screen::Grid(ref grid) = self.screen {
                    self.screen = Screen::Grid(Grid::new(grid.ruleset.clone(), *size));
                }
            }
            GridEvent::StateSaved => {
                if let Screen::Grid(ref grid) = self.screen {
                    self.saved_state = Some(grid.functional_state());
                };
            }
            GridEvent::StateLoaded => {
                if let Screen::Grid(ref mut grid) = self.screen {
                    if let Some(state) = &self.saved_state {
                        grid.load_state(state.clone());
                    }
                }
            }
            GridEvent::GridLinesToggled => self.grid_lines_enabled = !self.grid_lines_enabled,
        });
        event.map(|event: &EditorEvent, _| match event {
            EditorEvent::Enabled => {
                self.editor_enabled = true;
                let ruleset = self.screen.ruleset().clone();
                self.saved_state = None;
                self.screen = Screen::Editor(ruleset);
            }
            EditorEvent::Disabled => {
                self.editor_enabled = false;
                let ruleset = self.screen.ruleset().clone();
                self.screen = Screen::Grid(Grid::new(ruleset, self.grid_size));
            }
            EditorEvent::TabSwitched(tab) => self.selected_tab = *tab,
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("resources/style.css"))
            .expect("failed to add stylesheet.");

        let timer = cx.add_timer(Duration::from_secs_f32(1.0), None, |cx, event| {
            if let TimerAction::Tick(_) = event {
                cx.emit(GridEvent::Stepped);
            }
        });

        AppData::new(timer).build(cx);
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
                cx.emit(UpdateEvent::WindowSizeChanged);
            }
        });
    })
    .inner_size(INITIAL_WINDOW_SIZE)
    .run()
}
