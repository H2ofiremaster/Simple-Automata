use std::fmt::Debug;

use anyhow::anyhow;
use macroquad::prelude::*;

use crate::{
    display::{
        cell_selector,
        debug::display_debug_screen,
        grid::{display_grid, get_hovered_cell_pos},
        options,
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
            Menu::Options => options::display(&self.styles.options),
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
            OPEN_OPTIONS => {
                state.displayed_menu = toggle_menu(&state.displayed_menu, Menu::Options)
            }
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

    if state.displayed_menu == Menu::None {
        if let Some(selected_material) = &state.selected_material {
            handle_clicks(
                selected_material,
                state.ruleset.default_material(),
                &mut state.grid,
                size_multiplier,
            )
        }

        if let Err(err) = cycle_cell_state(
            &mut state.grid,
            size_multiplier,
            &mut state.selected_cell_state,
        ) {
            println!("Cell state cycler threw an error: {err}")
        };
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

fn cycle_cell_state(
    grid: &mut Grid,
    size_multiplier: f32,
    selected_state: &mut usize,
) -> anyhow::Result<()> {
    let cell_pos = get_hovered_cell_pos(grid, size_multiplier);
    let Some(cell_pos) = cell_pos else {
        return Ok(());
    };
    let cell = grid.get_cell(cell_pos.0, cell_pos.1);
    let Some(cell) = cell else {
        return Ok(());
    };
    if cell.state.is_empty() {
        return Ok(());
    }
    if is_mouse_button_pressed(MouseButton::Middle) {
        *selected_state += 1;
        if *selected_state >= cell.material.states.len() {
            *selected_state = 0;
        }
    }
    let scroll_amount = mouse_wheel().1;
    if scroll_amount == 0. {
        return Ok(());
    }
    let state = cell
        .material
        .states
        .iter()
        .nth(*selected_state)
        .ok_or(anyhow!(
            "Index {selected_state} was out of bounds for {:?}.",
            cell.material.states
        ))?;
    let mut new_cell = cell.clone();
    let selected_substate = new_cell
        .state
        .get_mut(state.0)
        .ok_or(anyhow!("Cell's state did not contain '{:?}'.", state.0))?;
    let (index, _) = state
        .1
        .iter()
        .enumerate()
        .find(|(_, value)| *value == selected_substate)
        .ok_or(anyhow!(
            "Vec {:?} did not contain {:?}.",
            state.1,
            selected_substate
        ))?;

    if scroll_amount > 0. {
        *selected_substate = state.1.get(index + 1).unwrap_or(&state.1[0]).clone();
    } else if scroll_amount < 0. {
        if index < 1 {
            *selected_substate = state.1[state.1.len() - 1].clone();
        } else {
            *selected_substate = state.1[index - 1].clone();
        }
    }
    grid.set_cell(cell_pos.0, cell_pos.1, new_cell);
    Ok(())
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
