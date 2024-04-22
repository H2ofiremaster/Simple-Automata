use macroquad::prelude::*;

use crate::logic::{
    cell::Cell,
    grid::{get_cell_size, get_grid_offset, get_grid_size, Grid},
};

pub fn display_grid(grid: &Grid, size_multiplier: f32) {
    let grid_size: f32 = get_grid_size(size_multiplier);
    let cell_size: f32 = get_cell_size(grid, size_multiplier);
    let grid_offset: Vec2 = get_grid_offset(size_multiplier);
    for row in 0..grid.height {
        (0..grid.height).for_each(|col| {
            let cell = grid
                .get_cell(col, row)
                .expect("Cell index should not be pout of bounds");
            display_cell(col, row, cell_size, grid_offset, cell)
        });
    }
    display_grid_lines(grid, cell_size, grid_size, grid_offset);
}

fn display_cell(col: usize, row: usize, cell_size: f32, offset: Vec2, cell: &Cell) {
    let col: f32 = col as f32;
    let row: f32 = row as f32;
    let color = cell.get_color();
    draw_rectangle(
        col * cell_size + offset.x,
        row * cell_size + offset.y,
        cell_size,
        cell_size,
        color,
    )
}

const GRID_COLOR: Color = GRAY;
const GRID_WIDTH: f32 = 10.;
fn display_grid_lines(grid: &Grid, cell_size: f32, grid_size: f32, offset: Vec2) {
    draw_rectangle_lines(
        offset.x,
        offset.y,
        grid_size,
        grid_size,
        grid.width as f32 / GRID_WIDTH,
        GRID_COLOR,
    );
    for row in 0..grid.height {
        draw_line(
            offset.x,
            row as f32 * cell_size + offset.y,
            offset.x + grid_size,
            row as f32 * cell_size + offset.y,
            grid.width as f32 / GRID_WIDTH,
            GRID_COLOR,
        )
    }
    for col in 0..grid.width {
        draw_line(
            col as f32 * cell_size + offset.x,
            offset.y,
            col as f32 * cell_size + offset.x,
            offset.y + grid_size,
            grid.width as f32 / GRID_WIDTH,
            GRID_COLOR,
        )
    }
}
