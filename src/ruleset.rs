use std::{fs, path::PathBuf};

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use vizia::{
    binding::{Data, LensExt},
    context::{Context, EmitContext},
    layout::Units::{Auto, Percentage, Pixels, Stretch},
    modifiers::{ActionModifiers, LayoutModifiers, StyleModifiers, TextModifiers},
    views::{Button, ComboBox, HStack, Label, Svg, VStack, ZStack},
};

use crate::{
    condition::{Condition, ConditionIndex},
    display::style::{self, svg},
    events::RuleEvent,
    grid::{Cell, Grid},
    id::{Identifiable, UniqueId},
    material::{GroupId, Material, MaterialGroup, MaterialId, MaterialMap},
    pattern::Pattern,
    AppData,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    pub rules: Vec<Rule>,
    pub materials: MaterialMap,
    pub groups: Vec<MaterialGroup>,
}

impl Data for Ruleset {
    fn same(&self, other: &Self) -> bool {
        self.name == other.name
            && self.rules == other.rules
            && self.materials == other.materials
            && self.groups == other.groups
    }
}
impl Ruleset {
    pub const PATH: &str = "./rulesets/";

    pub fn new() -> Self {
        Self {
            name: String::from("New Ruleset"),
            rules: vec![],
            materials: MaterialMap::new(Material::default()),
            groups: vec![],
        }
    }

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
        let mut path = PathBuf::from(Self::PATH);
        path.push(&self.name);
        path.set_extension("toml");
        fs::write(path, string)
            .map_err(|err| format!("Could not save ruleset '{self:?}'; file IO failed: {err}"))?;
        Ok(())
    }
    pub fn load_all() -> Result<Vec<Self>, String> {
        let path = PathBuf::from(Self::PATH);
        let paths = path
            .read_dir()
            .map_err(|err| format!("Could not load rulesets; directory reading failed: {err}"))?
            .filter_map(|file| {
                if let Ok(file) = file {
                    if file.path().extension().is_some_and(|e| e == "toml") {
                        return Some(file);
                    }
                } else {
                    println!("Could not read file: {file:?}");
                }
                None
            });
        let mut rulesets = vec![Self::blank()];
        for path in paths {
            let text = fs::read_to_string(path.path()).map_err(|err| {
                format!("Could not load rulesets; could not read file '{path:?}': {err}")
            })?;
            let ruleset = toml::from_str(&text).map_err(|err| {
                format!(
                    "Could not load rulesets; deserialization failed for file '{path:?}': {err}"
                )
            })?;
            rulesets.push(ruleset);
        }
        Ok(rulesets)
    }

    pub fn group(&self, id: GroupId) -> Option<&MaterialGroup> {
        self.groups.iter().find(|group| group.id() == id)
    }

    pub fn index_of_group(&self, id: GroupId) -> Option<usize> {
        self.groups.iter().position(|group| group.id() == id)
    }

    pub fn pattern_values(&self) -> Vec<String> {
        let material_names = self.materials.iter().map(|m| m.name.clone());
        let group_names = self.groups.iter().map(|g| format!("#{}", g.name.clone()));
        material_names.chain(group_names).collect()
    }
}
impl Default for Ruleset {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleIndex {
    index: usize,
}
impl RuleIndex {
    const fn new(rule_index: usize) -> Self {
        Self { index: rule_index }
    }

    pub const fn value(self) -> usize {
        self.index
    }
    const fn with_condition(self, condition_index: usize) -> ConditionIndex {
        ConditionIndex::new(self.index, condition_index)
    }
    pub fn rule(self, ruleset: &Ruleset) -> &Rule {
        ruleset.rules.get(self.index).expect("invalid rule index")
    }

    pub fn rule_mut(self, ruleset: &mut Ruleset) -> &mut Rule {
        ruleset
            .rules
            .get_mut(self.index)
            .expect("invalid rule index")
    }
}
impl From<usize> for RuleIndex {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Rule {
    pub input: Pattern,
    pub output: MaterialId,
    pub conditions: Vec<Condition>,
}
impl Rule {
    pub fn new(ruleset: &Ruleset) -> Self {
        Self {
            input: Pattern::Material(ruleset.materials.default().id()),
            output: ruleset.materials.default().id(),
            conditions: Vec::new(),
        }
    }

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

    pub fn display_editor(&self, cx: &mut Context, index: RuleIndex) {
        let output = self.output;
        VStack::new(cx, move |cx| {
            HStack::new(cx, move |cx| {
                self.input.display_editor(cx, move |cx, selected| {
                    cx.emit(RuleEvent::InputSet(index, selected));
                });
                ZStack::new(cx, |cx| {
                    Svg::new(cx, svg::TRANSFORM_ARROW)
                        .size(Percentage(80.0))
                        .space(Stretch(1.0));
                })
                .size(Pixels(80.0));
                // .background_color("green");
                ComboBox::new(
                    cx,
                    AppData::screen.map(|screen| screen.ruleset().materials.names()),
                    AppData::screen.map(move |screen| {
                        screen
                            .ruleset()
                            .materials
                            .index_of(output)
                            .expect("Output material should exist in the current ruleset.")
                    }),
                )
                .width(Stretch(1.0))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0))
                .on_select(move |cx, selected| {
                    cx.emit(RuleEvent::OutputSet(index, selected));
                });
            })
            // .background_color("red")
            .top(Pixels(-5.0))
            .height(Auto);
            VStack::new(cx, move |cx| {
                for (condition_index, condition) in self.conditions.iter().enumerate() {
                    condition.display_editor(cx, index.with_condition(condition_index));
                }
                Button::new(cx, |cx| Label::new(cx, "New Condition").space(Stretch(1.0)))
                    .width(Stretch(1.0))
                    .on_press(move |cx| cx.emit(RuleEvent::ConditionCreated(index)));
            })
            // .background_color("red")
            .class(style::CONDITION_CONTAINER);
            // .height(Auto)
            // .child_space(Percentage(5.0))
            // .row_between(Pixels(15.0));
        })
        .class(style::BASE_EDITOR)
        .width(Percentage(50.0));
    }
}
struct RuleVisitor;
impl<'de> Visitor<'de> for RuleVisitor {
    type Value = Rule;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct Rule")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut input = None;
        let mut output = None;
        let mut conditions = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "input" => {
                    if input.is_some() {
                        return Err(de::Error::duplicate_field("input"));
                    }
                    input = Some(map.next_value()?);
                }
                "output" => {
                    if output.is_some() {
                        return Err(de::Error::duplicate_field("output"));
                    }
                    let raw_id = map.next_value()?;
                    output = Some(UniqueId::new_unchecked(raw_id));
                }
                "conditions" => {
                    if conditions.is_some() {
                        return Err(de::Error::duplicate_field("conditions"));
                    }
                    conditions = Some(map.next_value()?);
                }
                _ => {
                    return Err(de::Error::unknown_field(
                        &key,
                        &["input", "output", "conditions"],
                    ))
                }
            }
        }

        let input = input.ok_or_else(|| de::Error::missing_field("input"))?;
        let output = output.ok_or_else(|| de::Error::missing_field("output"))?;
        let conditions = conditions.ok_or_else(|| de::Error::missing_field("conditions"))?;

        Ok(Rule {
            input,
            output,
            conditions,
        })
    }
}
impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Rule", &["input", "output", "conditions"], RuleVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        condition::{ConditionVariant, CountVariant, Direction},
        id::UniqueId,
        ruleset::Rule,
    };

    use super::*;

    #[allow(clippy::unwrap_used)]
    #[test]
    fn serde_rule() {
        let rule = Rule {
            input: Pattern::Material(UniqueId::new_unchecked(10)),
            output: UniqueId::new_unchecked(100),
            conditions: vec![
                Condition {
                    variant: ConditionVariant::Count(CountVariant::List(vec![1, 2, 3])),
                    pattern: Pattern::Group(UniqueId::new_unchecked(20)),
                },
                Condition {
                    variant: ConditionVariant::Directional(vec![
                        Direction::North,
                        Direction::South,
                    ]),
                    pattern: Pattern::Group(UniqueId::new_unchecked(200)),
                },
            ],
        };

        dbg!(&rule);

        let rule_string = toml::to_string(&rule).unwrap();
        println!("{rule_string:?}");

        let new_rule: Rule = toml::from_str(&rule_string).unwrap();

        dbg!(&new_rule);

        assert_eq!(rule, new_rule);
    }
}
