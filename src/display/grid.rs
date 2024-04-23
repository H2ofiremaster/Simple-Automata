use macroquad::prelude::*;

use crate::logic::{cell::Cell, grid::Grid};

use super::styles::{SELECTED_TEXT_COLOR, TEXT_COLOR};

pub fn display_grid(grid: &Grid, size_multiplier: f32, font: &Font, selected_state: usize) {
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
    display_hover_text(
        grid,
        grid_size,
        grid_offset,
        font,
        size_multiplier,
        selected_state,
    );
}

fn display_cell(col: usize, row: usize, cell_size: f32, offset: Vec2, cell: &Cell) {
    let col: f32 = col as f32;
    let row: f32 = row as f32;
    let color = cell.color();
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

fn display_hover_text(
    grid: &Grid,
    grid_size: f32,
    grid_offset: Vec2,
    font: &Font,
    size_multiplier: f32,
    selected_state: usize,
) {
    let bottom_padding = screen_height() - screen_height() * BOTTOM_PADDING;
    let Some(cell) = get_hovered_cell_pos(grid, size_multiplier)
        .and_then(|pos: (usize, usize)| grid.get_cell(pos.0, pos.1))
    else {
        return;
    };
    let mut cell_strings: Vec<String> = cell
        .state
        .iter()
        .map(|(k, v)| format!("{k}:{v}, "))
        .collect();
    let mut has_states = false;
    let cell_name: String = if cell_strings.is_empty() {
        cell.name().to_string()
    } else {
        has_states = true;
        format!("{} [", cell.name())
    };
    cell_strings.splice(..0, vec![cell_name].drain(..));
    if has_states {
        cell_strings.push("]".to_string());
        let last_index = cell_strings.len() - 2;
        cell_strings[last_index] = cell_strings[last_index]
            .strip_suffix(", ")
            .unwrap_or("Error: String didn't have correct suffix.")
            .to_string();
    }

    let measure = |text: &_, font_size| measure_text(text, Some(font), font_size, 1.0);
    let font_size = (bottom_padding / 1.5) as u16;
    let (text_locations, font_size) =
        get_text_locations(grid_offset.x, grid_size, font_size, &cell_strings, measure);
    for (index, location) in text_locations.iter().enumerate() {
        let color = if index > 0 && (index - 1) == selected_state {
            SELECTED_TEXT_COLOR
        } else {
            TEXT_COLOR
        };
        draw_text_ex(
            &cell_strings[index],
            *location,
            screen_height() - bottom_padding / 4.,
            TextParams {
                font: Some(font),
                color,
                font_size,
                ..Default::default()
            },
        )
    }
    // draw_text_ex(
    //     &cell_name,
    //     grid_offset.x,
    //     screen_height() - bottom_padding / 4.,
    //
    // );
}

fn get_text_locations<F>(
    start_location: f32,
    grid_size: f32,
    font_size: u16,
    strings: &[String],
    measure_fn: F,
) -> (Vec<f32>, u16)
where
    F: Fn(&str, u16) -> TextDimensions,
{
    let sizes: Vec<TextDimensions> = strings.iter().map(|s| measure_fn(s, font_size)).collect();
    let full_width: f32 = sizes.iter().map(|d| d.width).sum();
    let (final_sizes, final_font_size): (Vec<_>, u16) = if full_width > grid_size {
        let quotient = full_width / grid_size;
        let new_font_size = (font_size as f32 / quotient) as u16;
        let new_sizes = strings
            .iter()
            .map(|s| measure_fn(s, new_font_size))
            .collect();
        (new_sizes, new_font_size)
    } else {
        (sizes, font_size)
    };
    let mut current_size: f32 = start_location;
    let mut locations: Vec<f32> = Vec::new();
    for size in final_sizes.iter() {
        locations.push(current_size);
        current_size += size.width;
    }

    (locations, final_font_size)
}

const BOTTOM_PADDING: f32 = 0.95;
pub fn get_grid_size(size_multiplier: f32) -> f32 {
    screen_width().min(screen_height() * BOTTOM_PADDING) * size_multiplier
}
pub fn get_cell_size(grid: &Grid, size_multiplier: f32) -> f32 {
    get_grid_size(size_multiplier) / grid.width as f32
}
pub fn get_grid_offset(size_multiplier: f32) -> Vec2 {
    let x = crate::screen_center().x - get_grid_size(size_multiplier) / 2.;
    vec2(x, 0.)
}

pub fn get_hovered_cell_pos(grid: &Grid, size_multiplier: f32) -> Option<(usize, usize)> {
    let offset: Vec2 = get_grid_offset(size_multiplier);
    let cell_size: f32 = get_cell_size(grid, size_multiplier);
    let grid_size: f32 = get_grid_size(size_multiplier);

    let mouse_pos: Vec2 = Vec2::from(mouse_position()) - offset;
    if mouse_pos.x > grid_size || mouse_pos.y > grid_size || mouse_pos.x < 0. || mouse_pos.y < 0. {
        return None;
    }
    let cell_pos = mouse_pos / cell_size;
    Some((cell_pos.x as usize, cell_pos.y as usize))
}
