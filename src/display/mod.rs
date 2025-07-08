use vizia::prelude::*;

use crate::{
    display::grid::GridDisplay,
    events::{
        EditorEvent, GridEvent, GroupEvent, MaterialEvent, RuleEvent, RulesetEvent, UpdateEvent,
    },
    grid::{Cell, Grid},
    id::Identifiable,
    material::Material,
    ruleset::Ruleset,
    AppData,
};

pub mod condition;
pub mod grid;
mod lens;
pub mod material;
pub mod pattern;
pub mod ruleset;
mod style;

pub fn ruleset_editor(cx: &mut Context) {
    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            toolbar(cx);
            tabs(cx);
        })
        .class(style::EDITOR_PANEL)
        .height(Auto)
        .row_between(Pixels(5.0));

        // Materials
        HStack::new(cx, |cx| {
            material_editor(cx);
            group_editor(cx);
        })
        .display(AppData::selected_tab.map(|&tab| tab == EditorTab::Materials));
        // Rules
        HStack::new(cx, rule_editor)
            .display(AppData::selected_tab.map(|&tab| tab == EditorTab::Rules));
    })
    .class(style::BACKGROUND);
}

fn toolbar(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Dummy")).display(false);
        Button::new(cx, |cx| Label::new(cx, "Back")).on_press(|cx| cx.emit(EditorEvent::Disabled));

        ComboBox::new(
            cx,
            AppData::rulesets.map(|rulesets| {
                rulesets
                    .iter()
                    .map(|r| r.name.clone())
                    .collect::<Vec<String>>()
            }),
            AppData::selected_ruleset,
        )
        .on_select(|cx, index| cx.emit(RulesetEvent::Selected(index)));

        Textbox::new(
            cx,
            AppData::screen
                .then(lens::ScreenToRuleset)
                .map(|s| s.name.clone()),
        )
        .on_submit(|cx, text, _| {
            cx.emit(RulesetEvent::Renamed(text));
        })
        .min_width(Pixels(100.0));

        Button::new(cx, |cx| Label::new(cx, "New")).on_press(|cx| cx.emit(RulesetEvent::Created));

        Button::new(cx, |cx| Label::new(cx, "Save"))
            .on_press(|cx| cx.emit(RulesetEvent::Saved))
            .disabled(AppData::selected_ruleset.map(|&index| index == 0));

        Button::new(cx, |cx| Label::new(cx, "Reload"))
            .on_press(|cx| cx.emit(RulesetEvent::Reloaded));
    })
    .height(Auto);
}

fn tabs(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Materials"))
            .on_press(|cx| cx.emit(EditorEvent::TabSwitched(EditorTab::Materials)))
            .toggle_class(
                style::PRESSED_BUTTON,
                AppData::selected_tab.map(|&tab| tab == EditorTab::Materials),
            )
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
        Button::new(cx, |cx| Label::new(cx, "Rules"))
            .on_press(|cx| cx.emit(EditorEvent::TabSwitched(EditorTab::Rules)))
            .toggle_class(
                style::PRESSED_BUTTON,
                AppData::selected_tab.map(|&tab| tab == EditorTab::Rules),
            )
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
    })
    .height(Auto);
}

fn material_editor(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, true, true, move |cx| {
            Binding::new(
                cx,
                AppData::screen.then(lens::ScreenToRuleset),
                |cx, ruleset| {
                    let ruleset = ruleset.get(cx);
                    let materials: Vec<(usize, &Material)> =
                        ruleset.materials.iter().enumerate().collect();
                    VStack::new(cx, |cx| {
                        for chunk in materials.chunks(style::EDITOR_ROW_LENGTH) {
                            editor_material_row(cx, chunk, &ruleset);
                        }
                    })
                    .row_between(Pixels(15.0))
                    .left(Stretch(0.1))
                    .right(Stretch(0.1))
                    .size(Auto);
                },
            );
        })
        .space(Percentage(1.0));
        Button::new(cx, |cx| Label::new(cx, "New Material"))
            .on_press(|cx| cx.emit(MaterialEvent::Created))
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
    })
    .class(style::EDITOR_PANEL);
}

fn editor_material_row(cx: &mut Context, row: &[(usize, &Material)], ruleset: &Ruleset) {
    HStack::new(cx, |cx| {
        for i in 0..style::EDITOR_ROW_LENGTH {
            if let Some((index, material)) = row.get(i) {
                material::material(material, cx, *index, ruleset);
            } else {
                Element::new(cx).size(Pixels(style::EDITOR_MATERIAL_SIZE));
            }
        }
    })
    .col_between(Pixels(15.0))
    .size(Auto);
}

fn group_editor(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, true, true, move |cx| {
            Binding::new(
                cx,
                AppData::screen.then(lens::ScreenToRuleset),
                |cx, ruleset| {
                    let ruleset = ruleset.get(cx);
                    VStack::new(cx, |cx| {
                        for (index, group) in ruleset.groups.iter().enumerate() {
                            material::group(group, cx, index, &ruleset);
                        }
                    })
                    .row_between(Pixels(5.0))
                    .min_height(Auto);
                },
            );
        })
        .space(Percentage(1.0));
        Button::new(cx, |cx| Label::new(cx, "New Group"))
            .on_press(|cx| cx.emit(GroupEvent::Created))
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
    })
    .class(style::EDITOR_PANEL);
}
fn rule_editor(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
            Binding::new(
                cx,
                AppData::screen.then(lens::ScreenToRuleset),
                |cx, ruleset| {
                    VStack::new(cx, move |cx| {
                        for (index, rule) in ruleset.get(cx).rules.iter().enumerate() {
                            ruleset::rule(rule, cx, index.into());
                        }
                    })
                    .row_between(Pixels(5.0))
                    .bottom(Pixels(150.0))
                    .min_height(Auto);
                },
            );
        });
        Button::new(cx, |cx| Label::new(cx, "New Rule"))
            .on_press(|cx| cx.emit(RuleEvent::Created))
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
    })
    .class(style::EDITOR_PANEL);
}

pub fn game_board(cx: &mut Context) {
    HStack::new(cx, |cx| {
        left_panel(cx);
        center_panel(cx);
        right_panel(cx);
    })
    .on_geo_changed(|cx, changes| {
        if changes.contains(GeoChanged::WIDTH_CHANGED)
            || changes.contains(GeoChanged::HEIGHT_CHANGED)
        {
            cx.emit(UpdateEvent::WindowSizeChanged);
        }
    })
    .class(style::BACKGROUND);
}

fn left_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {
        editor_button(cx);
        step_controls(cx);
        speed_controls(cx);
        size_controls(cx);
        savestate_controls(cx);
        HStack::new(cx, |cx| {
            Checkbox::new(cx, AppData::grid_lines_enabled)
                .on_toggle(|cx| cx.emit(GridEvent::GridLinesToggled));
            Label::new(cx, "Toggle Grid Lines");
        });
        Element::new(cx).height(Stretch(5.0));
    })
    .class(style::SIDE_PANEL);
}

fn editor_button(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Edit Ruleset"))
            .on_press(|cx| cx.emit(EditorEvent::Enabled))
            .class(style::RULESET_BUTTON);
    })
    .class(style::MENU_ELEMENT);
}
fn step_controls(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| {
            Label::new(
                cx,
                AppData::running.map(|runnning| if *runnning { "Stop" } else { "Start" }),
            )
        })
        .on_press(|cx| cx.emit(GridEvent::Toggled))
        .class(style::CONTROL_BUTTON);
        Button::new(cx, |cx| Label::new(cx, "Step"))
            .on_press(|cx| cx.emit(GridEvent::Stepped))
            .class(style::CONTROL_BUTTON);
    })
    .class(style::MENU_ELEMENT);
}
fn speed_controls(cx: &mut Context) {
    HStack::new(cx, |cx: &mut Context| {
        Slider::new(cx, AppData::speed.map(|speed| 0_f32.max(*speed).min(1.0)))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0))
            .space(Stretch(0.05))
            .range(0.01..1.0)
            .on_changing(|cx, progress| cx.emit(GridEvent::SpeedSet(progress)));
        Textbox::new(cx, AppData::speed.map(|speed| format!("{speed:.2}")))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0))
            .space(Stretch(0.05))
            .on_edit(|cx, text| {
                if let Ok(speed) = text.parse() {
                    cx.emit(GridEvent::SpeedSet(speed));
                }
            });
    })
    .class(style::MENU_ELEMENT);
}
fn size_controls(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Label::new(cx, "Grid Size: ")
            .height(Stretch(1.0))
            .child_space(Stretch(1.0));
        Textbox::new(cx, AppData::grid_size.map(|&x| x.to_string())).on_submit(
            |cx, text, enter_pressed| {
                if enter_pressed {
                    if let Ok(size) = text.parse() {
                        cx.emit(GridEvent::Resized(size));
                    }
                }
            },
        );
    })
    .col_between(Pixels(5.0))
    .class(style::MENU_ELEMENT);
}
fn savestate_controls(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Save State"))
            .class(style::CONTROL_BUTTON)
            .on_press(|cx| cx.emit(GridEvent::StateSaved));
        Button::new(cx, |cx| Label::new(cx, "Load State"))
            .class(style::CONTROL_BUTTON)
            .on_press(|cx| cx.emit(GridEvent::StateLoaded))
            .disabled(AppData::saved_state.map(Option::is_none));
    })
    .class(style::MENU_ELEMENT);
}

fn center_panel(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        GridDisplay::new(
            cx,
            AppData::screen
                .then(lens::ScreenToGrid)
                .map(super::grid::Grid::visual_state),
            AppData::hovered_index,
            AppData::grid_lines_enabled,
        )
        .size(Stretch(1.0))
        .background_color(Color::rgba(255, 0, 0, 128));
        // grid.display(cx);
    })
    .size(Stretch(2.2))
    .min_size(Auto)
    .class(style::CENTER_PANEL);
}

fn right_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, AppData::tooltip.map(|(text, _)| text.clone()))
            .class(style::MATERIAL_TOOLTIP)
            .color(AppData::tooltip.map(|(_, color)| *color));
        ScrollView::new(cx, 0., 0., true, true, |cx| {
            VStack::new(cx, |cx| {
                Binding::new(cx, AppData::screen.then(lens::ScreenToGrid), |cx, grid| {
                    let grid = grid.get(cx);
                    let ruleset = grid.ruleset;
                    let cells: Vec<Cell> = ruleset
                        .materials
                        .iter()
                        .map(|material| Cell::new(material.id()))
                        .collect();
                    cells.chunks(style::MATERIAL_ROW_LENGTH).for_each(|chunk| {
                        material_row(cx, chunk, &ruleset);
                    });
                });
            })
            .min_size(Auto);
        });
    })
    .class(style::SIDE_PANEL);
}

fn material_row(cx: &mut Context, row: &[Cell], ruleset: &Ruleset) {
    HStack::new(cx, |cx| {
        for &cell in row {
            let border_color = border_color(cell.color(ruleset).to_rgba());
            grid::cell(cell, cx, ruleset)
                .on_press(move |cx| {
                    cx.emit(UpdateEvent::MaterialSelected(cell.material_id));
                })
                .on_hover(move |cx| cx.emit(UpdateEvent::MaterialHovered(cell.material_id)))
                .on_hover_out(|cx| cx.emit(UpdateEvent::MaterialUnhovered))
                .border_color(AppData::selected_material.map(move |id| {
                    if *id == cell.material_id {
                        border_color
                    } else {
                        Color::transparent()
                    }
                }))
                .class(style::MATERIAL_DISPLAY);
        }
    })
    .class(style::MATERIAL_ROW);
}

// Utility

fn border_color(color: RGBA) -> Color {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    let avg = (u32::from(r) + u32::from(g) + u32::from(b)) / 3;
    if avg > 128 {
        Color::black()
    } else {
        Color::white()
    }
}

pub fn rect_bounds(bounds: &BoundingBox) -> BoundingBox {
    let target_size = bounds.width().min(bounds.height());
    let left = (bounds.width() / 2.0) - (target_size / 2.0) + bounds.left();
    let top = (bounds.height() / 2.0) - (target_size / 2.0) + bounds.top();
    BoundingBox {
        x: left,
        y: top,
        w: target_size,
        h: target_size,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Data)]
pub enum Screen {
    Grid(Grid),
    Editor(Ruleset),
}
impl Screen {
    pub const fn ruleset(&self) -> &Ruleset {
        match self {
            Self::Grid(grid) => &grid.ruleset,
            Self::Editor(ruleset) => ruleset,
        }
    }
    pub fn ruleset_mut(&mut self) -> &mut Ruleset {
        match self {
            Self::Grid(grid) => &mut grid.ruleset,
            Self::Editor(ruleset) => ruleset,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum EditorTab {
    Materials,
    Rules,
}
