use serde::{Deserialize, Serialize};

use crate::{
    grid::CellNeighbors,
    id::Identifiable,
    pattern::Pattern,
    ruleset::{Rule, Ruleset},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConditionIndex {
    rule_index: usize,
    condition_index: usize,
}
impl ConditionIndex {
    pub const fn new(rule_index: usize, condition_index: usize) -> Self {
        Self {
            rule_index,
            condition_index,
        }
    }
    pub const fn values(&self) -> (usize, usize) {
        (self.rule_index, self.condition_index)
    }

    pub fn rule<'a>(&self, ruleset: &'a Ruleset) -> &'a Rule {
        ruleset
            .rules
            .get(self.rule_index)
            .expect("invalid rule index")
    }
    pub fn condition<'a>(&self, ruleset: &'a Ruleset) -> &'a Condition {
        self.rule(ruleset)
            .conditions
            .get(self.condition_index)
            .expect("invalid condition index")
    }

    pub fn rule_mut<'a>(&self, ruleset: &'a mut Ruleset) -> &'a mut Rule {
        ruleset
            .rules
            .get_mut(self.rule_index)
            .expect("invalid rule index")
    }
    pub fn condition_mut<'a>(&self, ruleset: &'a mut Ruleset) -> &'a mut Condition {
        self.rule_mut(ruleset)
            .conditions
            .get_mut(self.condition_index)
            .expect("invalid condition index")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    List(Vec<u8>),
    Greater(u8),
    Less(u8),
}
impl Operator {
    fn contains(&self, element: u8) -> bool {
        match self {
            Self::List(vec) => vec.contains(&element),
            Self::Greater(bound) => ((bound + 1)..=8).contains(&element),
            Self::Less(bound) => (0..*bound).contains(&element),
        }
    }
    pub fn with_elements(&self, elements: Vec<u8>) -> Self {
        match self {
            Self::List(_) => Self::List(elements),
            Self::Greater(_) => Self::Greater(elements.into_iter().min().unwrap_or(0)),
            Self::Less(_) => Self::Less(elements.into_iter().max().unwrap_or(0)),
        }
    }
}
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(vec) => {
                let string = vec.iter().map(u8::to_string).collect::<Vec<_>>().join(" ");
                write!(f, "{string}")
            }
            Self::Greater(value) | Self::Less(value) => write!(f, "{value}"),
        }
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
pub enum ConditionVariant {
    Directional(Vec<Direction>),
    Count(Operator),
}
impl ConditionVariant {
    pub fn directions(&mut self) -> Option<&mut Vec<Direction>> {
        match self {
            Self::Directional(vec) => Some(vec),
            Self::Count(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub variant: ConditionVariant,
    pub pattern: Pattern,
    pub inverted: bool,
}
impl Condition {
    pub fn new(ruleset: &Ruleset) -> Self {
        Self {
            variant: ConditionVariant::Count(Operator::List(vec![0])),
            pattern: Pattern::Material(ruleset.materials.default().id()),
            inverted: false,
        }
    }
    pub fn matches(&self, neighbors: CellNeighbors, ruleset: &Ruleset) -> bool {
        let matches = match &self.variant {
            ConditionVariant::Directional(directions) => directions.iter().any(|&dir| {
                neighbors
                    .in_direction(dir)
                    .is_some_and(|cell| self.pattern.matches(ruleset, cell))
            }),
            ConditionVariant::Count(counts) => {
                counts.contains(neighbors.count_matching(ruleset, self.pattern))
            }
        };
        matches != self.inverted
    }
}
