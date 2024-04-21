use macroquad::prelude::*;

use crate::logic::{
    cell::Cell,
    grid::{Direction, Grid},
    rules::{Rule, Ruleset},
};

use super::grid::{get_cell_size, get_grid_offset};

const FONT_SIZE: f32 = 25.;
pub fn display_debug_screen(grid: &Grid, ruleset: &Ruleset) {
    let Some((cell, index)) = get_hovered_cell(grid) else {
        return;
    };
    let current_cell = format!("{cell:?}; ({}, {})", index % grid.width, index / grid.width);
    let text_size: TextDimensions = measure_text(&current_cell, None, FONT_SIZE as u16, 1.);
    let mut text_offset = 1.0;
    draw_text(
        &current_cell,
        0.,
        text_size.offset_y * text_offset,
        FONT_SIZE,
        GREEN,
    );
    let neighbours = format!(
        "Neighbors: ({:?}), ({:?}), ({:?}), ({:?}),",
        grid.get_neighbor(Direction::North, index),
        grid.get_neighbor(Direction::South, index),
        grid.get_neighbor(Direction::East, index),
        grid.get_neighbor(Direction::West, index),
    );
    let neighbours_2: String = format!(
        "   ({:?}), ({:?}), ({:?}), ({:?})",
        grid.get_neighbor(Direction::Northeast, index),
        grid.get_neighbor(Direction::Southeast, index),
        grid.get_neighbor(Direction::Northwest, index),
        grid.get_neighbor(Direction::Southwest, index),
    );
    text_offset += 1.1;

    draw_text(
        &neighbours,
        0.,
        text_size.offset_y * 2.1 * text_offset,
        FONT_SIZE,
        GREEN,
    );
    text_offset += 1.1;

    draw_text(
        &neighbours_2,
        0.,
        text_size.offset_y * text_offset,
        FONT_SIZE,
        GREEN,
    );
    text_offset += 1.1;

    let mut applied_rules: Vec<&Rule> = Vec::new();
    let next = ruleset
        .iter_rules()
        .filter_map(|rule| {
            let result = rule.apply(cell.clone(), index, grid, ruleset);
            if result.is_some() {
                applied_rules.push(rule);
            }
            result
        })
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(cell, _)| cell)
        .unwrap_or(cell.clone());

    draw_text(
        &format!("Next: {next:?} Applied:"),
        0.,
        text_size.offset_y * text_offset,
        FONT_SIZE,
        GREEN,
    );
    text_offset += 1.1;

    display_applied(applied_rules, &text_size, &mut text_offset);
}

fn display_applied(applied_rules: Vec<&Rule>, text_size: &TextDimensions, text_offset: &mut f32) {
    for rule in applied_rules {
        draw_text(
            &format!("{:?}", rule),
            0.,
            text_size.offset_y * (*text_offset),
            FONT_SIZE,
            GREEN,
        );
        *text_offset += 1.1;
    }
}

fn get_hovered_cell(grid: &Grid) -> Option<(&Cell, usize)> {
    let offset: Vec2 = get_grid_offset();
    let cell_size: f32 = get_cell_size(grid);

    let mouse_pos: Vec2 = mouse_position().into();
    let cell_position: Vec2 = (mouse_pos - offset) / cell_size;
    grid.get_cell(cell_position.x as usize, cell_position.y as usize)
        .map(|cell| {
            (
                cell,
                grid.get_index(cell_position.x as usize, cell_position.y as usize),
            )
        })
}
