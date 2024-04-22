use std::str::FromStr;

use anyhow::anyhow;
use macroquad::{
    input::mouse_position,
    math::Vec2,
    window::{screen_height, screen_width},
};
use rand::RngCore;

use crate::logic::{
    cell::Cell,
    rules::{Pattern, Ruleset},
};

use super::cell::Material;

#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
}
impl Grid {
    pub fn new(width: usize, height: usize, ruleset: &Ruleset) -> Self {
        let default_cell = ruleset.default_cell();
        Self {
            width,
            height,
            cells: vec![default_cell; width * height],
        }
    }
    pub fn get_next_generation(self, ruleset: &Ruleset) -> Self {
        let new_cells: Vec<Cell> = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                ruleset
                    .iter_rules()
                    .filter_map(|rule| rule.apply(cell.clone(), index, &self, ruleset))
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(cell, _)| cell)
                    .unwrap_or(cell.clone())
            })
            .collect();
        Grid {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }

    #[rustfmt::skip]
    pub fn get_neighbor(&self, direction: Direction, index: usize) -> Option<&Cell> {
        self.cells.get(index)?;
        let (x, y) = self.get_coords(index);
        match direction {
            Direction::North => {
                if y < 1 { return None }
                self.get_cell(x, y - 1)
            }
            Direction::South => self.get_cell(x, y + 1),
            Direction::East => self.get_cell(x + 1, y),
            Direction::West => {
                if x < 1 { return None }
                self.get_cell(x - 1, y)
            }
            Direction::Northeast => {
                if y < 1 { return None }
                self.get_cell(x + 1, y - 1)
            }
            Direction::Southeast => self.get_cell(x + 1, y + 1),
            Direction::Northwest => {
                if y < 1 || x < 1 { return None }
                self.get_cell(x - 1, y - 1)
            },
            Direction::Southwest => {
                if x < 1 { return None }
                self.get_cell(x - 1, y + 1)
            },
        }
    }

    pub fn get_matching_neighbors(&self, index: usize, pattern: &Pattern) -> u8 {
        Direction::all()
            .into_iter()
            .filter_map(|dir| self.get_neighbor(dir, index))
            .filter(|cell| pattern.matches(cell))
            .count() as u8
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(self.get_index(x, y))
    }
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let index = self.get_index(x, y);
        if index > self.cells.len() {
            return;
        }
        let _ = std::mem::replace(&mut self.cells[index], cell);
    }

    pub fn randomize(&mut self, ruleset: &Ruleset) {
        let mut rng = rand::thread_rng();
        let cells = ruleset.iter_materials().collect::<Vec<_>>();
        for index in 0..self.cells.len() {
            let random_number = rng.next_u32() as usize % cells.len();
            let random_type: &Material = cells[random_number];
            let _ = std::mem::replace(
                &mut self.cells[index],
                Cell::new_default(random_type.clone()),
            );
        }
    }
    pub const fn get_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }
    pub const fn get_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Southeast,
    Northwest,
    Southwest,
}
impl Direction {
    fn all() -> [Direction; 8] {
        [
            Self::North,
            Self::South,
            Self::East,
            Self::West,
            Self::Northeast,
            Self::Southeast,
            Self::Northwest,
            Self::Southwest,
        ]
    }
}
impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "n" => Ok(Self::North),
            "s" => Ok(Self::South),
            "e" => Ok(Self::East),
            "w" => Ok(Self::West),
            "ne" => Ok(Self::Northeast),
            "se" => Ok(Self::Southeast),
            "nw" => Ok(Self::Northwest),
            "sw" => Ok(Self::Southwest),
            _ => Err(anyhow!("'{s}' is not a valid direction.")),
        }
    }
}

pub fn get_grid_size(size_multiplier: f32) -> f32 {
    screen_width().min(screen_height()) * size_multiplier
}
pub fn get_cell_size(grid: &Grid, size_multiplier: f32) -> f32 {
    get_grid_size(size_multiplier) / grid.width as f32
}
pub fn get_grid_offset(size_multiplier: f32) -> Vec2 {
    crate::screen_center() - get_grid_size(size_multiplier) / 2.
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
