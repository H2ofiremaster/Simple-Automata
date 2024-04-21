use display::{debug::display_debug_screen, grid::display_grid};
use logic::{grid::Grid, parser::parse_ruleset};
use macroquad::prelude::*;

pub mod display;
pub mod logic;
pub mod text;
pub mod ui;

#[macroquad::main("Automata")]
async fn main() {
    let mut debug: bool = false;
    let ruleset =
        parse_ruleset("./test_files/conway.toml").expect("Test ruleset should parse correctly");
    let mut grid = Grid::new(20, 20, &ruleset);
    let mut delay: f32 = 0.;
    grid.randomize(&ruleset);
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Space) {
            grid = grid.get_next_generation(&ruleset);
            delay = 1.
        } else if is_key_down(KeyCode::Space) {
            delay -= get_frame_time();
            if delay < 0. {
                grid = grid.get_next_generation(&ruleset);
                delay = 0.1
            }
        }
        if is_key_pressed(KeyCode::F3) {
            debug = !debug;
        }
        if debug {
            display_grid(&grid, 0.7);
            display_debug_screen(&grid, &ruleset);
        } else {
            display_grid(&grid, 1.)
        }
        next_frame().await
    }
}

pub fn get_screen_center() -> Vec2 {
    vec2(screen_width() / 2., screen_height() / 2.)
}
