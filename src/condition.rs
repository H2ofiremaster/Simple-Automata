use serde::{Deserialize, Serialize};
use vizia::prelude::*;

use crate::{
    display::style::{self, svg},
    events::{ConditionEvent, RuleEvent},
    grid::CellNeighbors,
    id::Identifiable,
    pattern::Pattern,
    ruleset::{Rule, Ruleset},
    AppData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn display_editor(&self, cx: &mut Context, index: ConditionIndex) {
        match self {
            Self::Directional(_) => Self::display_directional(cx, index),
            Self::Count(variant) => Self::display_count(variant, cx, index),
        }
    }
    fn display_directional(cx: &mut Context, index: ConditionIndex) {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Self::direction_button(cx, index, svg::ARROW_NORTHWEST, Direction::Northwest);
                Self::direction_button(cx, index, svg::ARROW_WEST, Direction::West);
                Self::direction_button(cx, index, svg::ARROW_SOUTHWEST, Direction::Southwest);
            })
            .size(Stretch(1.0))
            .min_size(Auto);
            VStack::new(cx, |cx| {
                Self::direction_button(cx, index, svg::ARROW_NORTH, Direction::North);
                Self::direction_button(cx, index, svg::DIRECTIONAL_CONDITION, Direction::North)
                    .background_color(Color::transparent())
                    .border_color(Color::transparent())
                    .hoverable(false);
                Self::direction_button(cx, index, svg::ARROW_SOUTH, Direction::South);
            })
            .size(Stretch(1.0))
            .min_size(Auto);
            VStack::new(cx, |cx| {
                Self::direction_button(cx, index, svg::ARROW_NORTHEAST, Direction::Northeast);
                Self::direction_button(cx, index, svg::ARROW_EAST, Direction::East);
                Self::direction_button(cx, index, svg::ARROW_SOUTHEAST, Direction::Southeast);
            })
            .size(Stretch(1.0))
            .min_size(Auto);
        })
        .size(Pixels(100.0))
        .top(Pixels(15.0))
        .bottom(Pixels(15.0))
        .min_size(Auto);
    }
    fn direction_button<'c>(
        cx: &'c mut Context,
        index: ConditionIndex,
        svg: &'static str,
        direction: Direction,
    ) -> vizia::view::Handle<'c, Button> {
        Button::new(cx, |cx| {
            Svg::new(cx, svg)
                .max_size(Percentage(80.0))
                .space(Stretch(1.0))
        })
        .toggle_class(
            style::PRESSED_BUTTON,
            AppData::screen.map(move |screen| {
                let variant = &index.condition(screen.ruleset()).variant;
                matches!(variant, Self::Directional(ref vec) if vec.contains(&direction))
            }),
        )
        .on_press(move |cx| {
            cx.emit(ConditionEvent::DirectionToggled(index, direction));
        })
        .min_size(Auto)
        .size(Stretch(1.0))
    }
    fn display_count(variant: &Operator, cx: &mut Context, index: ConditionIndex) {
        Button::new(cx, |cx| match variant {
            Operator::List(_) => Svg::new(cx, svg::EQUAL).class(style::SVG),
            Operator::Greater(_) => Svg::new(cx, svg::GREATER).class(style::SVG),
            Operator::Less(_) => Svg::new(cx, svg::LESS).class(style::SVG),
        })
        .on_press(move |cx| cx.emit(ConditionEvent::OperatorChanged(index)))
        .size(Pixels(35.0))
        .top(Stretch(1.0))
        .bottom(Stretch(1.0))
        .right(Pixels(15.0));
        Textbox::new(
            cx,
            AppData::screen.map(move |screen| {
                let condition = index.condition(screen.ruleset());
                let Self::Count(variant) = &condition.variant else {
                    return String::new();
                };
                match variant {
                    Operator::List(vec) => {
                        vec.iter().map(u8::to_string).collect::<Vec<_>>().join(" ")
                    }
                    Operator::Greater(value) | Operator::Less(value) => value.to_string(),
                }
            }),
        )
        .on_submit(move |cx, text, _| {
            cx.emit(ConditionEvent::CountUpdated(index, text));
        })
        .top(Stretch(1.0))
        .bottom(Stretch(1.0));
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
        match &self.variant {
            ConditionVariant::Directional(directions) => directions.iter().any(|&dir| {
                neighbors
                    .in_direction(dir)
                    .is_some_and(|cell| self.pattern.matches(ruleset, cell))
            }),
            ConditionVariant::Count(counts) => {
                counts.contains(neighbors.count_matching(ruleset, self.pattern))
            }
        }
    }

    pub fn display_editor(&self, cx: &mut Context, index: ConditionIndex) {
        HStack::new(cx, move |cx| {
            VStack::new(cx, move |cx| {
                Button::new(cx, move |cx| {
                    Svg::new(cx, svg::NUMBERIC_CONDITION).class(style::SVG)
                })
                .size(Pixels(50.0))
                .toggle_class(
                    style::PRESSED_BUTTON,
                    AppData::screen.map(move |screen| {
                        let variant = &index.condition(screen.ruleset()).variant;
                        matches!(variant, ConditionVariant::Count(_))
                    }),
                )
                .on_press(move |cx| {
                    cx.emit(ConditionEvent::VariantChanged(
                        index,
                        ConditionVariant::Count(Operator::List(vec![0])),
                    ));
                });
                Button::new(cx, move |cx| {
                    Svg::new(cx, svg::DIRECTIONAL_CONDITION)
                        .max_size(Percentage(80.0))
                        .space(Stretch(1.0))
                })
                .size(Pixels(50.0))
                .toggle_class(
                    style::PRESSED_BUTTON,
                    AppData::screen.map(move |screen| {
                        let variant = &index.condition(screen.ruleset()).variant;
                        matches!(variant, ConditionVariant::Directional(_))
                    }),
                )
                .on_press(move |cx| {
                    cx.emit(ConditionEvent::VariantChanged(
                        index,
                        ConditionVariant::Directional(vec![]),
                    ));
                });
            })
            .space(Pixels(15.0))
            .min_size(Auto)
            .size(Auto);
            self.variant.display_editor(cx, index);
            Button::new(cx, |cx| {
                if self.inverted {
                    Svg::new(cx, svg::NOT_EQUAL).class(style::SVG)
                } else {
                    Svg::new(cx, svg::EQUAL).class(style::SVG)
                }
            })
            .class(style::CONDITION_INVERT_BUTTON)
            .on_press(move |cx| cx.emit(ConditionEvent::Inverted(index)));
            self.pattern.display_editor(cx, move |cx, selected_index| {
                cx.emit(ConditionEvent::PatternSet(index, selected_index));
            });
            VStack::new(cx, |cx| {
                Button::new(cx, |cx| Svg::new(cx, style::svg::COPY).class(style::SVG))
                    .on_press(move |cx| cx.emit(ConditionEvent::Copied(index)))
                    .size(Pixels(50.0));
                Button::new(cx, |cx| Svg::new(cx, style::svg::TRASH).class(style::SVG))
                    .on_press(move |cx| cx.emit(ConditionEvent::Deleted(index)))
                    .size(Pixels(50.0));
            })
            .space(Pixels(15.0))
            .min_size(Auto)
            .size(Auto);
        })
        .class(style::CONDITION_EDITOR);
        // .child_top(Stretch(1.0))
        // .child_bottom(Stretch(1.0))
        // .min_height(Auto);
    }
}
