use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    grid::{Cell, CellNeighbors, Grid},
    id::Identifiable,
    material::{GroupId, Material, MaterialGroup, MaterialId, MaterialMap},
    pattern::Pattern,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    pub rules: Vec<Rule>,
    pub materials: MaterialMap,
    pub groups: Vec<MaterialGroup>,
}
impl Ruleset {
    pub fn blank() -> Self {
        Self {
            name: String::from("Blank"),
            rules: Vec::new(),
            materials: MaterialMap::new(Material::blank()),
            groups: vec![],
        }
    }
    pub fn save(&self) -> Result<(), String> {
        let string = toml::to_string(self).map_err(|err| {
            format!("Could not save ruleset '{self:?}'; serialization failed: {err}")
        })?;
        let mut path = PathBuf::from(crate::RULESET_PATH);
        path.push(&self.name);
        path.set_extension("toml");
        fs::write(path, string)
            .map_err(|err| format!("Could not save ruleset '{self:?}'; file IO failed: {err}"))?;
        Ok(())
    }

    pub fn group(&self, id: GroupId) -> Option<&MaterialGroup> {
        self.groups.iter().find(|group| group.id() == id)
    }

    pub fn pattern_values(&self) -> Vec<String> {
        let material_names = self.materials.iter().map(|m| m.name.clone());
        let group_names = self.groups.iter().map(|g| format!("#{}", g.name.clone()));
        material_names.chain(group_names).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    input: Pattern,
    output: MaterialId,
    conditions: Vec<Condition>,
}
impl Rule {
    pub fn transformed(&self, grid: &Grid, cell: Cell, index: usize) -> Option<Cell> {
        if !self.input.matches(&grid.ruleset, cell) {
            return None;
        }
        if !self
            .conditions
            .iter()
            .all(|condition| condition.matches(grid.neighbors(index), &grid.ruleset))
        {
            return None;
        }
        Some(Cell::new(self.output))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Northwest,
    North,
    Northeast,
    West,
    East,
    Southwest,
    South,
    Southeast,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ConditionVariant {
    Directional(Vec<Direction>),
    Count(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Condition {
    variant: ConditionVariant,
    pattern: Pattern,
}
impl Condition {
    fn matches(&self, neighbors: CellNeighbors, ruleset: &Ruleset) -> bool {
        match &self.variant {
            ConditionVariant::Directional(directions) => directions.iter().any(|&dir| {
                neighbors
                    .in_direction(dir)
                    .is_some_and(|cell| self.pattern.matches(ruleset, cell))
            }),
            ConditionVariant::Count(counts) => counts.contains(&neighbors.count()),
        }
    }
}
