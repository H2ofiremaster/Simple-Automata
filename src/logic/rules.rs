use std::{collections::HashMap, fmt::Debug, ops::RangeInclusive, str::FromStr};

use anyhow::anyhow;

use crate::logic::{
    cell::{self, Cell, CellType, State},
    grid::{Direction, Grid},
};

#[derive(Debug)]
pub struct Ruleset {
    cells: Vec<CellType>,
    rules: Vec<Rule>,
}
impl Ruleset {
    pub fn new(cells: Vec<CellType>, rules: Vec<Rule>) -> Self {
        Self { cells, rules }
    }
    pub fn default_cell(&self) -> Cell {
        let cell_type = self.cells[0].clone();
        let cell_state = cell_type.default_states();
        Cell::new(cell_type, cell_state)
    }
    pub fn get_cell(&self, name: &str) -> Option<&CellType> {
        self.cells.iter().find(|cell| cell.name.as_str() == name)
    }
    pub fn iter_cells(&self) -> impl Iterator<Item = &CellType> {
        self.cells.iter()
    }
    pub fn iter_rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
    }
}

pub struct Rule {
    input: Pattern,
    output: Pattern,
    conditions: Vec<Condition>,
}
impl Rule {
    pub fn new(input: Pattern, output: Pattern, conditions: Vec<Condition>) -> Self {
        Self {
            input,
            output,
            conditions,
        }
    }

    pub fn apply(
        &self,
        cell: Cell,
        index: usize,
        grid: &Grid,
        rules: &Ruleset,
    ) -> Option<(Cell, Specificity)> {
        if !self.input.matches(&cell) {
            return None;
        }

        let conditions_pass = self
            .conditions
            .iter()
            .map(|c| c.matches(index, grid))
            .all(|p| p);

        if !conditions_pass {
            return None;
        }

        Some((
            self.output.modify(cell, rules),
            self.input.get_specificity(),
        ))
    }
}
impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rule: {{{:?} -> {:?}: {:?}}}",
            self.input, self.output, self.conditions
        )
    }
}

pub struct Pattern {
    cell_type: Option<String>,
    states: HashMap<String, String>,
    inverted: bool,
}
impl Pattern {
    fn new_all(cell_type: String, states: HashMap<String, String>, inverted: bool) -> Self {
        Self {
            cell_type: Some(cell_type),
            states,
            inverted,
        }
    }
    fn new_type(cell_type: String, inverted: bool) -> Self {
        Self {
            cell_type: Some(cell_type),
            states: HashMap::new(),
            inverted,
        }
    }
    fn new_states(states: HashMap<String, String>, inverted: bool) -> Self {
        Self {
            cell_type: None,
            states,
            inverted,
        }
    }
    fn new_empty() -> Self {
        Self {
            cell_type: None,
            states: HashMap::new(),
            inverted: false,
        }
    }

    pub fn matches(&self, cell: &Cell) -> bool {
        let matches = match (&self.cell_type, &self.states) {
            (Some(cell_type), states) => cell.is_type(cell_type) && cell.has_states(states),
            (None, states) => cell.has_states(states),
        };

        self.inverted != matches
    }
    pub fn modify(&self, input: Cell, rules: &Ruleset) -> Cell {
        let new_type = self
            .cell_type
            .as_ref()
            .and_then(|type_| rules.get_cell(type_))
            .unwrap_or(&input.cell_type)
            .clone();

        let reflected_states = cell::merge_states(&new_type.default_states(), &input.state);
        let new_state = cell::merge_states(&reflected_states, &self.states);

        Cell::new(new_type, new_state)
    }
    pub fn get_specificity(&self) -> Specificity {
        use Specificity as S;
        let has_type = self.cell_type.is_some();
        let has_states = !self.states.is_empty();
        match (has_type, has_states) {
            (true, true) => S::Both,
            (true, false) => S::Type,
            (false, true) => S::State,
            (false, false) => S::None,
        }
    }
}
impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inverted = s.starts_with('!');
        let s = s.trim_start_matches('!');
        let parts = s.split_once('[');
        match parts {
            Some((type_, states)) => {
                let states = states
                    .trim_end_matches(']')
                    .split(',')
                    .map(|state| {
                        state
                            .split_once(':')
                            .map(|(n, s)| (n.to_string(), s.to_string()))
                            .ok_or(anyhow!("state '{state}' didn't contain namespace."))
                    })
                    .collect::<anyhow::Result<State>>()?;
                if type_ == "*" {
                    Ok(Pattern::new_states(states, inverted))
                } else {
                    Ok(Pattern::new_all(type_.to_string(), states, inverted))
                }
            }
            None => {
                if s == "*" {
                    Ok(Pattern::new_empty())
                } else {
                    Ok(Pattern::new_type(s.to_string(), inverted))
                }
            }
        }
    }
}
impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_ = self.cell_type.as_deref().unwrap_or("*");
        let inverted = if self.inverted { "!" } else { "" };
        write!(f, "Pattern{{{inverted}{type_}{:?}}}", self.states)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Specificity {
    None,
    State,
    Type,
    Both,
}

#[derive(Debug)]
pub enum Condition {
    Directional(Vec<Direction>, Pattern),
    CountExact(u8, Pattern),
    CountArray(Vec<u8>, Pattern),
    CountRange(RangeInclusive<u8>, Pattern),
}
impl Condition {
    pub fn matches(&self, index: usize, grid: &Grid) -> bool {
        match self {
            Condition::Directional(directions, pattern) => directions
                .iter()
                .map(|dir| {
                    grid.get_neighbor(*dir, index)
                        .is_some_and(|cell| pattern.matches(cell))
                })
                .any(|p| p),
            Condition::CountExact(count, pattern) => {
                grid.get_matching_neighbors(index, pattern) == *count
            }
            Condition::CountArray(counts, pattern) => {
                counts.contains(&grid.get_matching_neighbors(index, pattern))
            }
            Condition::CountRange(counts, pattern) => {
                counts.contains(&grid.get_matching_neighbors(index, pattern))
            }
        }
    }
}
