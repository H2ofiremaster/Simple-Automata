use std::{fs, ops::RangeInclusive};

use anyhow::{bail, ensure};
use macroquad::color::Color;
use serde::Deserialize;
use toml::{Table, Value};

use crate::logic::{
    cell::{self, Material},
    grid::Direction,
    rules::*,
};

#[derive(Debug, Deserialize)]
struct SimpleRuleset {
    cells: Vec<SimpleCellType>,
    rules: Vec<SimpleRule>,
}
impl SimpleRuleset {
    fn complicate(self) -> anyhow::Result<Ruleset> {
        Ok(Ruleset::new(
            self.cells
                .into_iter()
                .map(|cell| cell.complicate())
                .collect::<anyhow::Result<Vec<_>>>()?,
            self.rules
                .into_iter()
                .map(|rule| rule.complicate())
                .collect::<anyhow::Result<Vec<_>>>()?,
        ))
    }
}

#[derive(Debug, Deserialize)]
struct SimpleCellType {
    name: String,
    color: String,
    states: Table,
}
impl SimpleCellType {
    fn complicate(self) -> anyhow::Result<Material> {
        let color_string = self.color.trim_start_matches('#');
        let color_number = u32::from_str_radix(color_string, 16)?;
        let states = self
            .states
            .into_iter()
            .map(|(key, value)| {
                let states: Vec<String> = match value {
                    Value::Integer(int) => (0..=int).map(|n| n.to_string()).collect(),
                    Value::Array(arr) => arr
                        .into_iter()
                        .map(|value| {
                            let Value::String(string) = value else {
                                bail!("Value in array was not String");
                            };
                            Ok(string)
                        })
                        .collect::<anyhow::Result<Vec<_>>>()?,
                    _ => bail!("Cell state list was not Array or Integer"),
                };
                Ok((key, states))
            })
            .collect::<anyhow::Result<cell::StateSet>>()?;

        Ok(Material::new(
            self.name,
            Color::from_hex(color_number),
            states,
        ))
    }
}

#[derive(Debug, Deserialize)]
struct SimpleRule {
    #[serde(rename = "in")]
    input: String,
    out: String,
    conditions: Vec<SimpleCondition>,
}
impl SimpleRule {
    fn complicate(self) -> anyhow::Result<Rule> {
        Ok(Rule::new(
            self.input.parse()?,
            self.out.parse()?,
            self.conditions
                .into_iter()
                .map(|condition| condition.complicate())
                .collect::<anyhow::Result<Vec<_>>>()?,
        ))
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SimpleCondition {
    Directional {
        dirs: String,
        #[serde(rename = "type")]
        material: String,
    },
    CountExact {
        count: u8,
        #[serde(rename = "type")]
        material: String,
    },
    CountArray {
        count: Vec<u8>,
        #[serde(rename = "type")]
        material: String,
    },
    CountRange {
        count: String,
        #[serde(rename = "type")]
        material: String,
    },
}
impl SimpleCondition {
    fn complicate(self) -> anyhow::Result<Condition> {
        match self {
            SimpleCondition::Directional { dirs, material } => {
                let dirs: anyhow::Result<Vec<Direction>> = dirs
                    .split(' ')
                    .map(|dir| dir.parse::<Direction>())
                    .collect();
                let pattern: Pattern = material.parse()?;
                Ok(Condition::Directional(dirs?, pattern))
            }
            SimpleCondition::CountExact { count, material } => {
                Ok(Condition::CountExact(count, material.parse()?))
            }
            SimpleCondition::CountArray { count, material } => {
                Ok(Condition::CountArray(count, material.parse()?))
            }
            SimpleCondition::CountRange { count, material } => Ok(Condition::CountRange(
                parse_range(&count)?,
                material.parse()?,
            )),
        }
    }
}

pub fn parse_ruleset(path: &str) -> anyhow::Result<Ruleset> {
    let text = fs::read_to_string(path).expect("toml file path should be valid");
    let simple_ruleset: SimpleRuleset = toml::from_str(&text)?;

    simple_ruleset.complicate()
}

fn parse_range(s: &str) -> anyhow::Result<RangeInclusive<u8>> {
    let segments = s.split("..").collect::<Vec<_>>();
    if segments.len() != 2 {
        bail!("Range string '{s}' had too many segments.")
    }
    match (segments[0], segments[1]) {
        ("", "") => bail!("Range string '{s}' had no numbers"),
        (min, "") => {
            let min: u8 = min.parse()?;
            ensure!(
                min <= 8,
                "Range string '{s}' had too high of a number: {min}"
            );
            Ok(min..=8)
        }
        ("", max) => {
            let max: u8 = max.parse()?;
            ensure!(
                max <= 8,
                "Range string '{s}' had too high of a number: {max}"
            );
            Ok(0..=max)
        }
        (min, max) => {
            let min: u8 = min.parse()?;
            let max: u8 = max.parse()?;
            ensure!(
                min <= 8 && max <= 8,
                "Range string '{s}' had too high of numbers: {min}, {max}"
            );
            ensure!(min <= max, "Range string '{s}' had higher min than max.");
            Ok(min..=max)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_parse() {
        let test: Result<Ruleset, anyhow::Error> = parse_ruleset("./test_files/test.toml");
        // let text = fs::read_to_string("./test_files/test.toml").unwrap();
        // let test: Result<SimpleRuleset, toml::de::Error> = toml::from_str(&text);
        println!("{:#?}", test);
        assert!(test.is_ok())
    }
}
