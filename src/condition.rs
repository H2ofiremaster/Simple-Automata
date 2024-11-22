use serde::{Deserialize, Serialize};
use vizia::prelude::*;

use crate::{
    display::style,
    events::RuleEvent,
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

    fn display_editor(&self, cx: &mut Context, index: ConditionIndex) {
        match self {
            Self::Directional(_) => Self::display_directional(cx, index),
            Self::Count(variant) => Self::display_count(variant, cx, index),
        }
    }
    fn display_directional(cx: &mut Context, index: ConditionIndex) {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Self::direction_button(cx, index, /*"↖"*/ "+", Direction::Northwest);
                Self::direction_button(cx, index, /*"←"*/ "", Direction::West);
                Self::direction_button(cx, index, /*"↙"*/ "+", Direction::Southwest);
            })
            .background_color("red")
            .size(Stretch(1.0))
            .min_size(Auto);
            VStack::new(cx, |cx| {
                Self::direction_button(cx, index, /*"↑"*/ "", Direction::North);
                Element::new(cx).min_size(Auto).size(Stretch(1.0));
                Self::direction_button(cx, index, /*"↓"*/ "", Direction::South)
                    .translate((Pixels(0.0), Pixels(3.0)));
            })
            .background_color("green")
            .size(Stretch(1.0))
            .min_size(Auto);
            VStack::new(cx, |cx| {
                Self::direction_button(cx, index, /*"↗"*/ "+", Direction::Northeast);
                Self::direction_button(cx, index, /*"→"*/ "", Direction::East);
                Self::direction_button(cx, index, /*"↘"*/ "+", Direction::Southeast);
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
        index: ConditionIndex,
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
            let condition = index.condition(screen.ruleset());
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
            cx.emit(RuleEvent::ConditionDirectionToggled(index, direction));
        })
        .border_width(Pixels(2.0))
        .border_color(Color::black())
        // .space(Stretch(1.0))
        .min_size(Auto)
        // .size(Pixels(100.0))
        .size(Stretch(1.0))
    }
    fn display_count(variant: &CountVariant, cx: &mut Context, index: ConditionIndex) {
        Button::new(cx, |cx| match variant {
            CountVariant::List(_) => Label::new(cx, "="),
            CountVariant::Greater(_) => Label::new(cx, ">"),
            CountVariant::Less(_) => Label::new(cx, "<"),
        });
        Textbox::new(
            cx,
            AppData::screen.map(move |screen| {
                let condition = index.condition(screen.ruleset());
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
            cx.emit(RuleEvent::ConditionCountUpdated(index, text));
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
    pub fn matches(&self, neighbors: CellNeighbors, ruleset: &Ruleset) -> bool {
        match &self.variant {
            ConditionVariant::Directional(directions) => directions.iter().any(|&dir| {
                neighbors
                    .in_direction(dir)
                    .is_some_and(|cell| self.pattern.matches(ruleset, cell))
            }),
            ConditionVariant::Count(counts) => counts.contains(neighbors.count()),
        }
    }

    pub fn display_editor(&self, cx: &mut Context, index: ConditionIndex) {
        HStack::new(cx, move |cx| {
            VStack::new(cx, move |cx| {
                Button::new(cx, move |cx| Label::new(cx, "123")).on_press(move |cx| {
                    cx.emit(RuleEvent::ConditionVariantChanged(
                        index,
                        ConditionVariant::Count(CountVariant::List(vec![0])),
                    ));
                });
                Button::new(cx, move |cx| Label::new(cx, "↑↓"))
                    .width(Stretch(1.0))
                    .on_press(move |cx| {
                        cx.emit(RuleEvent::ConditionVariantChanged(
                            index,
                            ConditionVariant::Directional(vec![]),
                        ));
                    });
            })
            .background_color("aqua")
            .min_size(Auto)
            .height(Auto)
            .width(Auto);
            self.variant.display_editor(cx, index);
            Label::new(cx, "=")
                .background_color("gray")
                .font_size("x-large")
                .space(Stretch(0.05))
                .height(Stretch(1.0))
                .min_size(Auto);
            self.pattern.display_editor(cx, move |cx, selected_index| {
                cx.emit(RuleEvent::ConditionPatternSet(index, selected_index));
            });
        })
        .background_color("yellow")
        .child_space(Stretch(1.0))
        // .height(Pixels(200.0));
        .min_height(Auto);
    }
}
