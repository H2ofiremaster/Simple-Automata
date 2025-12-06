use vizia::prelude::*;

use crate::{
    AppData,
    display::{
        grid,
        lens::{self, ScreenToRuleset},
        style,
    },
    events::{GroupEvent, MaterialEvent},
    grid::Cell,
    id::Identifiable,
    material::{Material, MaterialColor, MaterialGroup, MaterialMap},
    ruleset::Ruleset,
};

pub fn material(this: &Material, cx: &mut Context, index: usize, ruleset: &Ruleset) {
    VStack::new(cx, |cx| {
        let cell = Cell::new(this.id());
        let id = this.id();

        Textbox::new(
            cx,
            AppData::screen.map(move |screen| {
                screen
                    .ruleset()
                    .materials
                    .get_at(index)
                    .expect("The specified index did not contain a material")
                    .name
                    .clone()
            }),
        )
        .width(Stretch(1.0))
        .on_submit(move |cx, text, _| cx.emit(MaterialEvent::Renamed(index, text)));

        grid::cell(cell, cx, ruleset).size(Pixels(style::EDITOR_MATERIAL_SIZE));

        HStack::new(cx, |cx| {
            Textbox::new(
                cx,
                AppData::screen.map(move |screen| {
                    screen
                        .ruleset()
                        .materials
                        .get_at(index)
                        .expect("The specified index did not contain a material")
                        .color
                        .to_string()
                }),
            )
            .width(Stretch(1.0))
            .on_submit(move |cx, text, _| cx.emit(MaterialEvent::Recolored(index, text)))
            .min_height(Pixels(30.0));

            Button::new(cx, |cx| Svg::new(cx, style::svg::TRASH).class(style::SVG))
                .class(style::TRASH_BUTTON)
                .size(Pixels(30.0))
                .on_press(move |cx| cx.emit(MaterialEvent::Deleted(id)))
                .display(AppData::screen.map(|screen| screen.ruleset().materials.len() != 1));
        })
        .width(Stretch(1.0))
        .height(Auto);
    })
    .size(Auto);
}

pub fn group(this: &MaterialGroup, cx: &mut Context, index: usize, ruleset: &Ruleset) {
    let id = this.id();
    VStack::new(cx, move |cx| {
        HStack::new(cx, move |cx| {
            Textbox::new(
                cx,
                AppData::screen.map(move |s| {
                    s.ruleset()
                        .group(id)
                        .map_or_else(|| String::from("ERROR"), |g| g.name.clone())
                }),
            )
            .on_submit(move |cx, text, _| cx.emit(GroupEvent::Renamed(index, text)));
            Button::new(cx, |cx| Label::new(cx, "New Material"))
                .on_press(move |cx| cx.emit(GroupEvent::EntryAdded(index)));
            Button::new(cx, |cx| Label::new(cx, "Delete"))
                .on_press(move |cx| cx.emit(GroupEvent::Deleted(index)));
        })
        .height(Auto);

        this.materials
            .iter()
            .enumerate()
            .filter_map(|(index, id)| ruleset.materials.get(*id).map(|_| index))
            .enumerate()
            .for_each(|(entry_index, material_index)| {
                entry(cx, index, material_index, entry_index);
            });
    })
    .width(Percentage(100.0))
    .class(style::BASE_EDITOR);
}
fn entry(cx: &mut Context, group_index: usize, material_index: usize, entry_index: usize) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "-")).on_press(move |cx| {
            cx.emit(GroupEvent::EntryDeleted {
                group_index,
                entry_index,
            });
        });
        ComboBox::new(
            cx,
            AppData::screen
                .then(lens::ScreenToMaterialMap)
                .map(MaterialMap::names),
            AppData::screen.then(ScreenToRuleset).map(move |ruleset| {
                let Some(group) = ruleset.groups.get(group_index) else {
                    return 0;
                };
                let Some(material) = group.materials.get(material_index) else {
                    return 0;
                };
                let Some(index) = ruleset.materials.index_of(*material) else {
                    return 0;
                };
                index
            }),
        )
        .on_select(move |cx, selected_index| {
            cx.emit(GroupEvent::Edited {
                group_index,
                entry_index: material_index,
                new_material_index: selected_index,
            });
        });
    })
    .height(Auto)
    .width(Stretch(1.0));
}
