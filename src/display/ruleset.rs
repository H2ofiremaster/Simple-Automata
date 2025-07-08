use vizia::prelude::*;

use crate::{
    display::{
        condition::condition,
        lens::{self},
        pattern,
        style::{self, svg},
    },
    events::{ConditionEvent, RuleEvent},
    material::MaterialMap,
    ruleset::{Rule, RuleIndex},
    AppData,
};

pub fn rule(this: &Rule, cx: &mut Context, index: RuleIndex) {
    let output = this.output;
    VStack::new(cx, move |cx| {
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Svg::new(cx, svg::COPY).class(style::SVG))
                .on_press(move |cx| cx.emit(RuleEvent::Copied(index)))
                .size(Pixels(50.0))
                .top(Stretch(1.0))
                .right(Pixels(15.0))
                .bottom(Stretch(1.0));

            pattern::pattern(this.input, cx, move |cx, selected| {
                cx.emit(RuleEvent::InputSet(index, selected));
            });
            ZStack::new(cx, |cx| {
                Svg::new(cx, svg::TRANSFORM_ARROW)
                    .size(Percentage(80.0))
                    .space(Stretch(1.0));
            })
            .size(Pixels(80.0));
            ComboBox::new(
                cx,
                AppData::screen
                    .then(lens::ScreenToMaterialMap)
                    .map(MaterialMap::names),
                AppData::screen
                    .then(lens::ScreenToMaterialMap)
                    .map(move |materials| materials.index_of(output).unwrap_or_default()),
            )
            .class(style::LIGHT_COMBOBOX)
            .width(Stretch(1.0))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0))
            .on_select(move |cx, selected| {
                cx.emit(RuleEvent::OutputSet(index, selected));
            });

            Button::new(cx, |cx| Svg::new(cx, svg::TRASH).class(style::SVG))
                .class(style::TRASH_BUTTON)
                .on_press(move |cx| cx.emit(RuleEvent::Deleted(index)))
                .size(Pixels(50.0))
                .top(Stretch(1.0))
                .left(Pixels(15.0))
                .bottom(Stretch(1.0));
        })
        .top(Pixels(-5.0))
        .height(Auto);
        VStack::new(cx, move |cx| {
            for (condition_index, condition) in this.conditions.iter().enumerate() {
                self::condition(condition, cx, index.with_condition(condition_index));
            }
            Button::new(cx, |cx| Label::new(cx, "New Condition").space(Stretch(1.0)))
                .width(Stretch(1.0))
                .on_press(move |cx| cx.emit(ConditionEvent::Created(index)));
        })
        .class(style::CONDITION_CONTAINER);
    })
    .class(style::BASE_EDITOR)
    .width(Percentage(50.0));
}
