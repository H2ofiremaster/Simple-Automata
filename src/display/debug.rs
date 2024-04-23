use macroquad::prelude::*;

use crate::logic::{
    grid::{Direction, Grid},
    rules::{Rule, Ruleset},
};

use super::grid::get_hovered_cell_pos;

const FONT_SIZE: f32 = 25.;
pub fn display_debug_screen(grid: &Grid, ruleset: &Ruleset, size_multiplier: f32) {
    let Some((cell_x, cell_y)) = get_hovered_cell_pos(grid, size_multiplier) else {
        return;
    };
    let Some(cell) = grid.get_cell(cell_x, cell_y) else {
        return;
    };
    let index = grid.get_index(cell_x, cell_y);

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
    text_offset += 1.1;

    let neighbours = format!(
        "Neighbors: N: ({:?}), S: ({:?}), E: ({:?}), W: ({:?}),",
        grid.get_neighbor(Direction::North, index),
        grid.get_neighbor(Direction::South, index),
        grid.get_neighbor(Direction::East, index),
        grid.get_neighbor(Direction::West, index),
    );
    let neighbours_2: String = format!(
        "   NE: ({:?}), SE: ({:?}), NW: ({:?}), SW: ({:?})",
        grid.get_neighbor(Direction::Northeast, index),
        grid.get_neighbor(Direction::Southeast, index),
        grid.get_neighbor(Direction::Northwest, index),
        grid.get_neighbor(Direction::Southwest, index),
    );

    draw_text(
        &neighbours,
        0.,
        text_size.offset_y * text_offset,
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
