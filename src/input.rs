use std::fmt::Debug;

use macroquad::prelude::*;

use crate::{
    display::{cell_selector, debug::display_debug_screen, grid::display_grid, styles::Styles},
    logic::{grid::Grid, rules::Ruleset},
};

#[derive(Debug)]
pub struct State {
    displayed_menu: Menu,
    pub ruleset: Ruleset,
    pub grid: Grid,
    styles: Styles,
}
impl State {
    pub fn new(ruleset: Ruleset, grid: Grid, styles: Styles) -> Self {
        Self {
            displayed_menu: Menu::None,
            ruleset,
            grid,
            styles,
        }
    }

    fn display_menu(&self) {
        match self.displayed_menu {
            Menu::CellSelector => cell_selector::display(&self.ruleset, &self.styles.cell_selector),
            Menu::Debug => display_debug_screen(&self.grid, &self.ruleset, DEBUG_MULTIPLIER),
            Menu::Options => todo!(),
            Menu::None => {}
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
            _ => {}
        }
    }
    let size_multiplier = if state.displayed_menu == Menu::Debug {
        DEBUG_MULTIPLIER
    } else {
        1.0
    };

    display_grid(&state.grid, size_multiplier);
    state.display_menu();

    state
}
