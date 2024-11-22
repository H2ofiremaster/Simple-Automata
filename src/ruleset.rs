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
    style::Color,
    vg::Handle,
    view::View,
    views::{Button, ComboBox, Element, HStack, Label, Textbox, VStack},
};

use crate::{
    display::style,
    events::RuleEvent,
    grid::{Cell, CellNeighbors, Grid},
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

    pub fn display_editor(&self, cx: &mut Context, index: usize, ruleset: &Ruleset) {
        let output = self.output;
        VStack::new(cx, move |cx| {
            HStack::new(cx, move |cx| {
                self.input.display_editor(cx, move |cx, selected| {
                    cx.emit(RuleEvent::InputSet(index, selected));
                });
                Label::new(cx, "=>")
                    .font_size("x-large")
                    .space(Stretch(0.05));
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
                .on_select(move |cx, selected| {
                    cx.emit(RuleEvent::OutputSet(index, selected));
                });
            })
            .height(Auto);
            VStack::new(cx, move |cx| {
                for (condition_index, condition) in self.conditions.iter().enumerate() {
                    condition.display_editor(cx, index, condition_index, ruleset);
                }
                Button::new(cx, |cx| Label::new(cx, "New Condition"))
                    .on_press(move |cx| cx.emit(RuleEvent::ConditionCreated(index)));
            })
            .height(Auto);
        })
        .left(Stretch(1.0))
        .right(Stretch(1.0))
        .width(Percentage(50.0))
        .height(Auto);
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CountVariant {
    List(Vec<u8>),
    Greater(u8),
    Less(u8),
}
impl CountVariant {
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
    Count(CountVariant),
}
impl ConditionVariant {
    pub fn directions(&mut self) -> Option<&mut Vec<Direction>> {
        match self {
            Self::Directional(vec) => Some(vec),
            Self::Count(_) => None,
        }
    }

    fn display_editor(&self, cx: &mut Context, rule_index: usize, condition_index: usize) {
        match self {
            Self::Directional(_) => Self::display_directional(cx, rule_index, condition_index),
            Self::Count(variant) => Self::display_count(variant, cx, rule_index, condition_index),
        }
    }
    fn display_directional(cx: &mut Context, rule_index: usize, condition_index: usize) {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"↖"*/ "+",
                    Direction::Northwest,
                );
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"←"*/ "",
                    Direction::West,
                );
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"↙"*/ "+",
                    Direction::Southwest,
                );
            })
            .background_color("red")
            .size(Stretch(1.0))
            .min_size(Auto);
            VStack::new(cx, |cx| {
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"↑"*/ "",
                    Direction::North,
                );
                Element::new(cx).min_size(Auto).size(Stretch(1.0));
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"↓"*/ "",
                    Direction::South,
                )
                .translate((Pixels(0.0), Pixels(3.0)));
            })
            .background_color("green")
            .size(Stretch(1.0))
            .min_size(Auto);
            VStack::new(cx, |cx| {
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"↗"*/ "+",
                    Direction::Northeast,
                );
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"→"*/ "",
                    Direction::East,
                );
                Self::direction_button(
                    cx,
                    rule_index,
                    condition_index,
                    /*"↘"*/ "+",
                    Direction::Southeast,
                );
            })
            .background_color("blue")
            .size(Stretch(1.0))
            .min_size(Auto);
        })
        // .child_space(Stretch(1.0))
        .background_color("purple")
        .size(Pixels(150.0))
        .min_size(Auto);
    }
    fn direction_button<'c>(
        cx: &'c mut Context,
        rule_index: usize,
        condition_index: usize,
        char: &str,
        direction: Direction,
    ) -> vizia::view::Handle<'c, Button> {
        Button::new(cx, |cx| {
            Label::new(cx, char)
                .min_size(Auto)
                .space(Stretch(1.0))
                .background_color("white")
        })
        .background_color(AppData::screen.map(move |screen| {
            let condition = screen
                .ruleset()
                .rules
                .get(rule_index)
                .expect("Ruleset should contain Rule")
                .conditions
                .get(condition_index)
                .expect("Rule should contain condition");
            match condition.variant.clone() {
                Self::Directional(vec) => {
                    if vec.contains(&direction) {
                        style::PRESSED_BUTTON_COLOR
                    } else {
                        style::BUTTON_COLOR
                    }
                }
                Self::Count(_) => style::BUTTON_COLOR,
            }
        }))
        .on_press(move |cx| {
            cx.emit(RuleEvent::ConditionDirectionToggled(
                rule_index,
                condition_index,
                direction,
            ));
        })
        .border_width(Pixels(2.0))
        .border_color(Color::black())
        // .space(Stretch(1.0))
        .min_size(Auto)
        // .size(Pixels(100.0))
        .size(Stretch(1.0))
    }
    fn display_count(
        variant: &CountVariant,
        cx: &mut Context,
        rule_index: usize,
        condition_index: usize,
    ) {
        Button::new(cx, |cx| match variant {
            CountVariant::List(_) => Label::new(cx, "="),
            CountVariant::Greater(_) => Label::new(cx, ">"),
            CountVariant::Less(_) => Label::new(cx, "<"),
        });
        Textbox::new(
            cx,
            AppData::screen.map(move |screen| {
                let condition = screen
                    .ruleset()
                    .rules
                    .get(rule_index)
                    .expect("Ruleset should contain Rule")
                    .conditions
                    .get(condition_index)
                    .expect("Rule should contain condition");
                let Self::Count(variant) = &condition.variant else {
                    return String::new();
                };
                match variant {
                    CountVariant::List(vec) => {
                        vec.iter().map(u8::to_string).collect::<Vec<_>>().join(" ")
                    }
                    CountVariant::Greater(value) | CountVariant::Less(value) => value.to_string(),
                }
            }),
        )
        .on_submit(move |cx, text, _| {
            cx.emit(RuleEvent::ConditionCountUpdated(
                rule_index,
                condition_index,
                text,
            ));
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub variant: ConditionVariant,
    pub pattern: Pattern,
}
impl Condition {
    pub fn new(ruleset: &Ruleset) -> Self {
        Self {
            variant: ConditionVariant::Count(CountVariant::List(vec![0])),
            pattern: Pattern::Material(ruleset.materials.default().id()),
        }
    }
    fn matches(&self, neighbors: CellNeighbors, ruleset: &Ruleset) -> bool {
        match &self.variant {
            ConditionVariant::Directional(directions) => directions.iter().any(|&dir| {
                neighbors
                    .in_direction(dir)
                    .is_some_and(|cell| self.pattern.matches(ruleset, cell))
            }),
            ConditionVariant::Count(counts) => counts.contains(neighbors.count()),
        }
    }

    fn display_editor(
        &self,
        cx: &mut Context,
        rule_index: usize,
        condition_index: usize,
        ruleset: &Ruleset,
    ) {
        HStack::new(cx, move |cx| {
            VStack::new(cx, move |cx| {
                Button::new(cx, move |cx| Label::new(cx, "123")).on_press(move |cx| {
                    cx.emit(RuleEvent::ConditionVariantChanged(
                        rule_index,
                        condition_index,
                        ConditionVariant::Count(CountVariant::List(vec![0])),
                    ));
                });
                Button::new(cx, move |cx| Label::new(cx, "↑↓"))
                    .width(Stretch(1.0))
                    .on_press(move |cx| {
                        cx.emit(RuleEvent::ConditionVariantChanged(
                            rule_index,
                            condition_index,
                            ConditionVariant::Directional(vec![]),
                        ));
                    });
            })
            .background_color("aqua")
            .min_size(Auto)
            .height(Auto)
            .width(Auto);
            self.variant.display_editor(cx, rule_index, condition_index);
            Label::new(cx, "=")
                .background_color("gray")
                .font_size("x-large")
                .space(Stretch(0.05))
                .height(Stretch(1.0))
                .min_size(Auto);
            self.pattern.display_editor(cx, move |cx, selected_index| {
                cx.emit(RuleEvent::ConditionPatternSet {
                    rule_index,
                    condition_index,
                    pattern_index: selected_index,
                });
            });
        })
        .background_color("yellow")
        .child_space(Stretch(1.0))
        // .height(Pixels(200.0));
        .min_height(Auto);
    }
}

#[cfg(test)]
mod tests {
    use crate::id::UniqueId;

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
