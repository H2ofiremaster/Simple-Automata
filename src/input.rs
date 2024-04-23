use std::fmt::Debug;

use macroquad::prelude::*;

use crate::{
    display::{
        cell_selector,
        debug::display_debug_screen,
        grid::{display_grid, get_hovered_cell_pos},
        styles::Styles,
    },
    logic::{
        cell::{Cell, Material},
        grid::Grid,
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
    pub step_timer: f32,
    pub selected_cell_state: usize,
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
            step_timer: 0.,
            selected_cell_state: 0,
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
const STEP_HOLD_DELAY: f32 = 0.1;
pub fn handle_inputs(state: State) -> State {
    let mut state: State = state;

    for key in get_keys_pressed() {
        match key {
            OPEN_CELL_SELECTOR => {
                state.displayed_menu = toggle_menu(&state.displayed_menu, Menu::CellSelector)
            }
            OPEN_DEBUG_MENU => {
                state.displayed_menu = toggle_menu(&state.displayed_menu, Menu::Debug)
            }
            OPEN_OPTIONS => todo!(),
            _ => {}
        }
    }

    if state.step_timer > 0. {
        state.step_timer -= get_frame_time();
    } else if is_key_down(STEP) {
        state.grid = state.grid.get_next_generation(&state.ruleset);
        state.step_timer = STEP_HOLD_DELAY;
    }

    let size_multiplier = if state.displayed_menu == Menu::Debug {
        DEBUG_MULTIPLIER
    } else {
        1.0
    };

    if let (Some(selected_material), Menu::None) = (&state.selected_material, state.displayed_menu)
    {
        handle_clicks(
            selected_material,
            state.ruleset.default_material(),
            &mut state.grid,
            size_multiplier,
        )
    }

    display_grid(
        &state.grid,
        size_multiplier,
        &state.styles.font,
        state.selected_cell_state,
    );
    state.display_menu();
    if state.menu_should_close && !is_mouse_button_down(MouseButton::Left) {
        state.displayed_menu = Menu::None;
        state.menu_should_close = false;
    }

    state
}

fn toggle_menu(current_menu: &Menu, desired_menu: Menu) -> Menu {
    if current_menu == &desired_menu {
        Menu::None
    } else {
        desired_menu
    }
}

fn handle_clicks(
    material: &Material,
    defualt_material: &Material,
    grid: &mut Grid,
    size_multiplier: f32,
) {
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
    } else if is_mouse_button_down(MouseButton::Right) && !cell.is_material(&defualt_material.name)
    {
        grid.set_cell(
            cell_pos.0,
            cell_pos.1,
            Cell::new_default(defualt_material.clone()),
        )
    }
}
