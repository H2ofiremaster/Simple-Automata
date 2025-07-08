use vizia::binding::Data;

use crate::{
    condition::Direction,
    display::grid::VisualGridState,
    id::Identifiable,
    material::{MaterialColor, MaterialId},
    pattern::Pattern,
    ruleset::Ruleset,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    pub ruleset: Ruleset,
    cells: Vec<Cell>,
    pub size: usize,
}
impl Grid {
    pub fn new(ruleset: Ruleset, size: usize) -> Self {
        let material = ruleset.materials.default();
        let cell = Cell::new(material.id());
        let cells = vec![cell; size * size];
        Self {
            ruleset,
            cells,
            size,
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, new: Cell) {
        let index = self.cell_index(x, y);
        if self.cells.get(index).is_none() {
            println!("Tried setting value of non-existent cell. Aborting.");
            return;
        }
        let _ = std::mem::replace(&mut self.cells[index], new);
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(self.cell_index(x, y)).copied()
    }

    pub const fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }
    pub const fn cell_coordinates(&self, index: usize) -> (usize, usize) {
        (index % self.size, index / self.size)
    }

    pub fn neighbors(&self, index: usize) -> CellNeighbors {
        let array = [
            self.get_neighbor(index, -1, -1),
            self.get_neighbor(index, 0, -1),
            self.get_neighbor(index, 1, -1),
            self.get_neighbor(index, -1, 0),
            self.get_neighbor(index, 1, 0),
            self.get_neighbor(index, -1, 1),
            self.get_neighbor(index, 0, 1),
            self.get_neighbor(index, 1, 1),
        ];
        CellNeighbors::new(array)
    }

    pub fn get_neighbor(&self, index: usize, x_offset: isize, y_offset: isize) -> Option<Cell> {
        let (x, y) = self.cell_coordinates(index);
        let x = x.wrapping_add_signed(x_offset);
        let y = y.wrapping_add_signed(y_offset);
        if x >= self.size || y >= self.size {
            None
        } else {
            self.cell_at(x, y)
        }
    }

    pub fn next_generation(&mut self) {
        let new_cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                self.ruleset
                    .rules
                    .iter()
                    .find_map(|rule| rule.transformed(self, *cell, index))
                    .unwrap_or(*cell)
            })
            .collect();
        self.cells = new_cells;
    }

    pub fn visual_state(&self) -> VisualGridState {
        VisualGridState::new(
            self.size,
            self.cells.iter().map(|&c| c.color(&self.ruleset)).collect(),
        )
    }
    pub fn functional_state(&self) -> FunctionalGridState {
        FunctionalGridState {
            size: self.size,
            cells: self.cells.clone(),
        }
    }

    pub fn load_state(&mut self, state: FunctionalGridState) {
        self.size = state.size;
        self.cells = state.cells;
    }
}
impl Data for Grid {
    fn same(&self, other: &Self) -> bool {
        self.size == other.size && self.cells == other.cells && self.ruleset == other.ruleset
    }
}

#[derive(Debug, Clone)]
pub struct FunctionalGridState {
    size: usize,
    cells: Vec<Cell>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub material_id: MaterialId,
}
impl Cell {
    pub const fn new(material_id: MaterialId) -> Self {
        Self { material_id }
    }

    pub fn color(self, ruleset: &Ruleset) -> MaterialColor {
        ruleset
            .materials
            .get(self.material_id)
            .unwrap_or_else(|| {
                println!("'Cell::color' called on foreign cell.");
                ruleset.materials.default()
            })
            .color
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellNeighbors(pub [Option<Cell>; 8]);
impl CellNeighbors {
    pub const fn new(array: [Option<Cell>; 8]) -> Self {
        Self(array)
    }

    pub fn count_matching(&self, ruleset: &Ruleset, pattern: Pattern) -> u8 {
        let mut count: u8 = 0;
        for cell in &self.0 {
            if cell.is_some_and(|cell| pattern.matches(ruleset, cell)) {
                count += 1;
            }
        }
        count
    }
    pub const fn in_direction(&self, direction: Direction) -> Option<Cell> {
        match direction {
            Direction::Northwest => self.0[0],
            Direction::North => self.0[1],
            Direction::Northeast => self.0[2],
            Direction::West => self.0[3],
            Direction::East => self.0[4],
            Direction::Southwest => self.0[5],
            Direction::South => self.0[6],
            Direction::Southeast => self.0[7],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        condition::{Condition, ConditionVariant},
        material::Material,
        ruleset::Rule,
    };

    use super::*;

    fn test_ruleset() -> Ruleset {
        let mut ruleset = Ruleset::new();

        let mut material = Material::new(&ruleset);
        material.color = MaterialColor::new(255, 255, 255);
        material.name = String::from("Filled");
        ruleset.materials.push(material);

        let blank_id = ruleset.materials.default().id();
        let filled_id = ruleset.materials.get_mut_at(1).expect("").id();

        let mut rule = Rule::new(&ruleset);
        rule.input = Pattern::Material(blank_id);
        rule.output = filled_id;

        let mut condition = Condition::new(&ruleset);
        condition.variant = ConditionVariant::Directional(vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]);
        condition.pattern = Pattern::Material(blank_id);
        rule.conditions.push(condition);

        ruleset.rules.push(rule);

        let mut rule = Rule::new(&ruleset);
        rule.input = Pattern::Material(filled_id);
        rule.output = blank_id;

        let mut condition = Condition::new(&ruleset);
        condition.variant = ConditionVariant::Count(crate::condition::Operator::Greater(4));
        condition.pattern = Pattern::Material(filled_id);
        rule.conditions.push(condition);

        ruleset.rules.push(rule);

        ruleset
    }

    #[test]
    fn perf_test() {
        let ruleset = test_ruleset();
        let mut grid = Grid::new(ruleset, 500);

        for _ in 0..10000 {
            grid.next_generation();
        }
    }
}
