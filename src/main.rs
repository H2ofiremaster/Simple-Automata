use crate::input::{handle_inputs, State};
use display::styles::Styles;
use logic::{grid::Grid, parser::parse_ruleset};
use macroquad::prelude::*;

pub mod display;
pub mod input;
pub mod logic;

#[macroquad::main("Automata")]
async fn main() {
    let ruleset =
        parse_ruleset("./test_files/conway.toml").expect("Test ruleset should parse correctly");

    let grid = Grid::new(40, 40, &ruleset);
    // grid.randomize(&ruleset);

    let styles = Styles::new().expect("Static styles should have parsed correctly.");

    let mut state: State = State::new(ruleset, grid, styles);
    loop {
        clear_background(BLACK);
        state = handle_inputs(state);
        // if is_key_pressed(KeyCode::Space) {
        //     stategrid = grid.get_next_generation(&ruleset);
        //     delay = 1.
        // } else if is_key_down(KeyCode::Space) {
        //     delay -= get_frame_time();
        //     if delay < 0. {
        //         grid = grid.get_next_generation(&ruleset);
        //         delay = 0.1
        //     }
        // }
        // if is_key_pressed(KeyCode::F3) {
        //     debug = !debug;
        // }
        // if debug {
        //     display_grid(&grid, 0.7);
        //     display_debug_screen(&grid, &ruleset, 0.7);
        // } else {
        //     display_grid(&grid, 1.)
        // }
        next_frame().await
    }
}

pub fn screen_center() -> Vec2 {
    vec2(screen_width() / 2., screen_height() / 2.)
}
