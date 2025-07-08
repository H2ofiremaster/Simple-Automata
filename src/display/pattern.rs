use crate::{pattern::Pattern, AppData};
use vizia::prelude::*;

pub fn pattern<F>(this: Pattern, cx: &mut Context, on_select: F)
where
    F: Fn(&mut EventContext, usize) + 'static,
{
    ComboBox::new(
        cx,
        AppData::screen.map(|screen| screen.ruleset().pattern_values()),
        AppData::screen.map(move |screen| match this {
            Pattern::Material(id) => screen.ruleset().materials.index_of(id).unwrap_or_default(),
            Pattern::Group(id) => screen
                .ruleset()
                .index_of_group(id)
                .map(|index| screen.ruleset().materials.len() + index)
                .unwrap_or_default(),
        }),
    )
    .width(Stretch(1.0))
    .top(Stretch(1.0))
    .bottom(Stretch(1.0))
    .on_select(on_select)
    .class(crate::display::style::LIGHT_COMBOBOX);
}
