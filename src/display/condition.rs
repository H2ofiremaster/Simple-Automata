use vizia::prelude::*;

use crate::{
    condition::{Condition, ConditionIndex, ConditionVariant, Direction, Operator},
    display::{
        lens, pattern,
        style::{self, svg},
    },
    events::ConditionEvent,
    AppData,
};

pub fn condition(this: &Condition, cx: &mut Context, index: ConditionIndex) {
    HStack::new(cx, move |cx| {
        VStack::new(cx, move |cx| {
            Button::new(cx, move |cx| {
                Svg::new(cx, svg::NUMBERIC_CONDITION).class(style::SVG)
            })
            .size(Pixels(50.0))
            .toggle_class(
                style::PRESSED_BUTTON,
                AppData::screen
                    .then(lens::ScreenToConditionVariant(index))
                    .map(move |variant| matches!(variant, ConditionVariant::Count(_))),
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
        variant(&this.variant, cx, index);
        Button::new(cx, |cx| {
            if this.inverted {
                Svg::new(cx, svg::NOT_EQUAL).class(style::SVG)
            } else {
                Svg::new(cx, svg::EQUAL).class(style::SVG)
            }
        })
        .class(style::CONDITION_INVERT_BUTTON)
        .on_press(move |cx| cx.emit(ConditionEvent::Inverted(index)));
        pattern::pattern(this.pattern, cx, move |cx, selected_index| {
            cx.emit(ConditionEvent::PatternSet(index, selected_index));
        });
        VStack::new(cx, |cx| {
            Button::new(cx, |cx| Svg::new(cx, svg::COPY).class(style::SVG))
                .on_press(move |cx| cx.emit(ConditionEvent::Copied(index)))
                .size(Pixels(50.0));
            Button::new(cx, |cx| Svg::new(cx, svg::TRASH).class(style::SVG))
                .class(style::TRASH_BUTTON)
                .on_press(move |cx| cx.emit(ConditionEvent::Deleted(index)))
                .size(Pixels(50.0));
        })
        .space(Pixels(15.0))
        .min_size(Auto)
        .size(Auto);
    })
    .class(style::CONDITION_EDITOR);
}

pub fn variant(this: &ConditionVariant, cx: &mut Context, index: ConditionIndex) {
    match this {
        ConditionVariant::Directional(_) => display_directional(cx, index),
        ConditionVariant::Count(variant) => display_count(variant, cx, index),
    }
}
fn display_directional(cx: &mut Context, index: ConditionIndex) {
    HStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            direction_button(cx, index, svg::ARROW_NORTHWEST, Direction::Northwest);
            direction_button(cx, index, svg::ARROW_WEST, Direction::West);
            direction_button(cx, index, svg::ARROW_SOUTHWEST, Direction::Southwest);
        })
        .size(Stretch(1.0))
        .min_size(Auto);
        VStack::new(cx, |cx| {
            direction_button(cx, index, svg::ARROW_NORTH, Direction::North);
            direction_button(cx, index, svg::DIRECTIONAL_CONDITION, Direction::North)
                .background_color(Color::transparent())
                .border_color(Color::transparent())
                .hoverable(false);
            direction_button(cx, index, svg::ARROW_SOUTH, Direction::South);
        })
        .size(Stretch(1.0))
        .min_size(Auto);
        VStack::new(cx, |cx| {
            direction_button(cx, index, svg::ARROW_NORTHEAST, Direction::Northeast);
            direction_button(cx, index, svg::ARROW_EAST, Direction::East);
            direction_button(cx, index, svg::ARROW_SOUTHEAST, Direction::Southeast);
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
            matches!(variant, ConditionVariant::Directional(ref vec) if vec.contains(&direction))
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
        AppData::screen
            .then(lens::ScreenToConditionVariant(index))
            .map(move |variant| {
                let ConditionVariant::Count(operator) = variant else {
                    return String::new();
                };
                operator.to_string()
            }),
    )
    .on_submit(move |cx, text, _| {
        cx.emit(ConditionEvent::CountUpdated(index, text));
    })
    .top(Stretch(1.0))
    .bottom(Stretch(1.0));
}
