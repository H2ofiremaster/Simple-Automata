use std::fmt::Debug;

use macroquad::prelude::*;

use crate::{
    display::{cell_selector, debug::display_debug_screen, grid::display_grid, styles::Styles},
    logic::{
        cell::{Cell, Material},
        grid::{get_hovered_cell_pos, Grid},
        rules::Ruleset,
    },
};

#[derive(Debug)]
pub struct State {
    displayed_menu: Menu,
    menu_should_close: bool,
    selected_material: Option<Material>,
    pub ruleset: Ruleset,
    pub grid: Grid,
    styles: Styles,
}
impl State {
    pub fn new(ruleset: Ruleset, grid: Grid, styles: Styles) -> Self {
        Self {
            displayed_menu: Menu::None,
            menu_should_close: false,
            ruleset,
            grid,
            styles,
            selected_material: None,
        }
    }

    fn display_menu(&mut self) {
        match self.displayed_menu {
            Menu::CellSelector => {
                //
                let should_close = cell_selector::display(
                    &self.ruleset,
                    &self.styles.cell_selector,
                    &mut self.selected_material,
                );
                self.menu_should_close |= should_close;
            }
            Menu::Debug => display_debug_screen(&self.grid, &self.ruleset, DEBUG_MULTIPLIER),
            Menu::Options => todo!(),
            Menu::None => self.menu_should_close = false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Menu {
    None,
    CellSelector,
    Debug,
    Options,
}

const OPEN_CELL_SELECTOR: KeyCode = KeyCode::C;
const OPEN_DEBUG_MENU: KeyCode = KeyCode::F3;
const OPEN_OPTIONS: KeyCode = KeyCode::Escape;
const STEP: KeyCode = KeyCode::Space;
const DEBUG_MULTIPLIER: f32 = 0.7;

pub fn handle_inputs(state: State) -> State {
    let mut state: State = state;

    for key in get_keys_pressed() {
        match key {
            OPEN_CELL_SELECTOR => {
                state.displayed_menu = match state.displayed_menu {
                    Menu::CellSelector => Menu::None,
                    _ => Menu::CellSelector,
                };
            }
            OPEN_DEBUG_MENU => todo!(),
            STEP => {
                state.grid = state.grid.get_next_generation(&state.ruleset);
            }
            OPEN_OPTIONS => todo!(),
            _ => {}
        }
    }
    let size_multiplier = if state.displayed_menu == Menu::Debug {
        DEBUG_MULTIPLIER
    } else {
        1.0
    };
    if let (Some(selected_material), Menu::None) = (&state.selected_material, state.displayed_menu)
    {
        handle_clicks(selected_material, &mut state.grid, size_multiplier)
    }

    display_grid(&state.grid, size_multiplier);
    state.display_menu();
    if state.menu_should_close && !is_mouse_button_down(MouseButton::Left) {
        state.displayed_menu = Menu::None;
        state.menu_should_close = false;
    }

    state
}

fn handle_clicks(material: &Material, grid: &mut Grid, size_multiplier: f32) {
    let cell_pos = get_hovered_cell_pos(grid, size_multiplier);
    let Some(cell_pos) = cell_pos else {
        return;
    };
    let cell = grid.get_cell(cell_pos.0, cell_pos.1);
    let Some(cell) = cell else {
        return;
    };

    if is_mouse_button_down(MouseButton::Left) && !cell.is_material(&material.name) {
        grid.set_cell(cell_pos.0, cell_pos.1, Cell::new_default(material.clone()))
    }
}
