use std::{collections::BTreeMap, fmt::Debug, ops::RangeInclusive, str::FromStr};

use anyhow::anyhow;

use crate::logic::{
    cell::{Cell, Material, State},
    grid::{Direction, Grid},
};

use super::cell;

#[derive(Debug)]
pub struct Ruleset {
    materials: Vec<Material>,
    rules: Vec<Rule>,
}
impl Ruleset {
    pub fn new(materials: Vec<Material>, rules: Vec<Rule>) -> Self {
        Self { materials, rules }
    }
    pub fn default_cell(&self) -> Cell {
        let material = self.materials[0].clone();
        let state = material.default_states();
        Cell::new(material, state)
    }
    pub fn default_material(&self) -> &Material {
        &self.materials[0]
    }
    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials
            .iter()
            .find(|cell| cell.name.as_str() == name)
    }
    pub fn iter_materials(&self) -> impl Iterator<Item = &Material> {
        self.materials.iter()
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
    material: Option<String>,
    states: State,
    inverted: bool,
}
impl Pattern {
    fn new_all(material: String, states: State, inverted: bool) -> Self {
        Self {
            material: Some(material),
            states,
            inverted,
        }
    }
    fn new_material(material: String, inverted: bool) -> Self {
        Self {
            material: Some(material),
            states: BTreeMap::new(),
            inverted,
        }
    }
    fn new_states(states: State, inverted: bool) -> Self {
        Self {
            material: None,
            states,
            inverted,
        }
    }
    fn new_empty() -> Self {
        Self {
            material: None,
            states: BTreeMap::new(),
            inverted: false,
        }
    }

    pub fn matches(&self, cell: &Cell) -> bool {
        let matches = match (&self.material, &self.states) {
            (Some(material), states) => cell.is_material(material) && cell.has_state(states),
            (None, states) => cell.has_state(states),
        };

        self.inverted != matches
    }
    pub fn modify(&self, input: Cell, rules: &Ruleset) -> Cell {
        let new_material = self
            .material
            .as_ref()
            .and_then(|material| rules.get_material(material))
            .unwrap_or(&input.material)
            .clone();

        let reflected_states = cell::merge_states(&new_material.default_states(), &input.state);
        let new_state = cell::merge_states(&reflected_states, &self.states);

        Cell::new(new_material, new_state)
    }
    pub fn get_specificity(&self) -> Specificity {
        use Specificity as S;
        let has_type = self.material.is_some();
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
            Some((material, states)) => {
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
                if material == "*" {
                    Ok(Pattern::new_states(states, inverted))
                } else {
                    Ok(Pattern::new_all(material.to_string(), states, inverted))
                }
            }
            None => {
                if s == "*" {
                    Ok(Pattern::new_empty())
                } else {
                    Ok(Pattern::new_material(s.to_string(), inverted))
                }
            }
        }
    }
}
impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let material = self.material.as_deref().unwrap_or("*");
        let inverted = if self.inverted { "!" } else { "" };
        write!(f, "Pattern{{{inverted}{material}{:?}}}", self.states)
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
pub struct Condition {
    directions: Vec<Direction>,
    count: Count,
    pattern: Pattern,
}
impl Condition {
    pub(super) fn new(directions: Vec<Direction>, count: Count, pattern: Pattern) -> Self {
        Self {
            directions,
            count,
            pattern,
        }
    }

    pub fn matches(&self, index: usize, grid: &Grid) -> bool {
        let directions: &[Direction] = &self.directions;

        let neighbors = grid.matching_neighbors(index, &self.pattern, directions);
        match &self.count {
            Count::Exact(count) => neighbors == *count,
            Count::Array(array) => array.contains(&neighbors),
            Count::Range(range) => range.contains(&neighbors),
        }
    }
}

#[derive(Debug)]
pub(super) enum Count {
    Exact(u8),
    Array(Vec<u8>),
    Range(RangeInclusive<u8>),
}
