use vizia::prelude::*;

use crate::{
    grid::{Cell, Grid},
    id::Identifiable,
    ruleset::Ruleset,
    AppData, AppEvent,
};

pub fn ruleset_editor(cx: &mut Context) {
    VStack::new(cx, |cx| {
        toolbar(cx);
        tabs(cx);

        Binding::new(cx, AppData::screen, |cx, screen| {
            let screen = screen.get(cx);
            let ruleset = screen.ruleset().clone();
            Binding::new(cx, AppData::selected_tab, move |cx, tab| {
                //
                match tab.get(cx) {
                    EditorTab::Materials => {
                        HStack::new(cx, |cx| {
                            material_editor(cx, &ruleset);
                            group_editor(cx);
                        });
                    }
                    EditorTab::Rules => rule_editor(cx),
                }
            });
        });
    })
    .class("background");
}

fn toolbar(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Back"))
            .on_press(|cx| cx.emit(AppEvent::ToggleEditor(false)))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0));

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
        .on_select(|cx, index| cx.emit(AppEvent::SelectRuleset(index)))
        .top(Stretch(1.0))
        .bottom(Stretch(1.0));
        Button::new(cx, |cx| Label::new(cx, "New"))
            .on_press(|cx| cx.emit(AppEvent::StartNewRuleset))
            .display(AppData::displayed_input.map(|input| *input != InputName::Ruleset))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0));

        Textbox::new(cx, AppData::new_object_name)
            .min_width(Pixels(100.0))
            .on_submit(|cx, text, enter_pressed| {
                if enter_pressed {
                    cx.emit(AppEvent::NewRuleset(text));
                }
            })
            .display(AppData::displayed_input.map(|input| *input == InputName::Ruleset))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0));

        Button::new(cx, |cx| Label::new(cx, "Save"))
            .on_press(|cx| cx.emit(AppEvent::SaveRuleset))
            .height(Stretch(1.0));

        Button::new(cx, |cx| Label::new(cx, "Reload"))
            .on_press(|cx| cx.emit(AppEvent::ReloadRulesets))
            .height(Stretch(1.0));
    })
    .height(Percentage(5.0));
}
fn tabs(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Materials"))
            .on_press(|cx| cx.emit(AppEvent::SwitchTab(EditorTab::Materials)))
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
        Button::new(cx, |cx| Label::new(cx, "Rules"))
            .on_press(|cx| cx.emit(AppEvent::SwitchTab(EditorTab::Rules)))
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
    })
    .height(Percentage(5.0));
}
fn material_editor(cx: &mut Context, ruleset: &Ruleset) {
    VStack::new(cx, |cx| {
        for (index, material) in ruleset.materials.iter().enumerate() {
            material.display_editor(cx, index, ruleset);
        }
    });
}
fn group_editor(cx: &mut Context) {
    Element::new(cx);
}
fn rule_editor(cx: &mut Context) {
    Element::new(cx);
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
            cx.emit(AppEvent::UpdateWindowSize);
        }
    })
    .class(style::BACKGROUND);
}

fn left_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {
        editor_button(cx);
        controls(cx);
        speed_controls(cx);
        material_tooltip(cx);
    })
    .class(style::SIDE_PANEL);
}

fn editor_button(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Edit Ruleset"))
            .on_press(|cx| cx.emit(AppEvent::ToggleEditor(true)));
    })
    .class(style::MENU_ELEMENT);
}
fn controls(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| {
            Label::new(
                cx,
                AppData::running.map(|runnning| if *runnning { "Stop" } else { "Start" }),
            )
        })
        .on_press(|cx| cx.emit(AppEvent::ToggleRunning))
        .class(style::BUTTON);
        Button::new(cx, |cx| Label::new(cx, "Step"))
            .on_press(|cx| cx.emit(AppEvent::Step))
            .class(style::BUTTON);
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
            .on_changing(|cx, progress| cx.emit(AppEvent::SetSpeed(progress)));
        Textbox::new(cx, AppData::speed.map(|speed| format!("{speed:.2}")))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0))
            .space(Stretch(0.05))
            .on_edit(|cx, text| {
                if let Ok(speed) = text.parse() {
                    cx.emit(AppEvent::SetSpeed(speed));
                }
            });
    })
    .class(style::MENU_ELEMENT);
}
fn material_tooltip(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, AppData::tooltip);
    })
    .size(Stretch(10.));
}

fn center_panel(cx: &mut Context) {
    Binding::new(cx, AppData::screen, |cx, screen| {
        ZStack::new(cx, |cx| {
            if let Screen::Grid(grid) = screen.get(cx) {
                grid.display(cx);
            }
        })
        .size(AppData::window_size.map(|bounds| Pixels(margined_square_size(bounds))))
        .class(style::CENTER_PANEL);
    });
}

fn right_panel(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        ScrollView::new(cx, 0., 0., true, true, |cx| {
            VStack::new(cx, |cx| {
                Binding::new(cx, AppData::screen, |cx, screen| {
                    let Screen::Grid(grid) = screen.get(cx) else {
                        return;
                    };
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
            let border_color = border_color(cell.color(ruleset));
            cell.display(cx, ruleset)
                .on_press(move |cx| {
                    cx.emit(AppEvent::MaterialSelected(cell.material_id));
                })
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

fn margined_square_size(bounds: &BoundingBox) -> f32 {
    let max_width =
        bounds.width() * style::BACKGROUND_PADDING.mul_add(-2.0, style::CENTER_MARGIN_FACTOR);
    let max_height = bounds.height() * style::BACKGROUND_PADDING.mul_add(-2.0, 1.0);

    max_width.min(max_height)
    // bounds
    //     .height()
    //     .min(bounds.width() * style::CENTER_MARGIN_FACTOR)
    //     - (bounds.width().max(bounds.height()) * style::BACKGROUND_PADDING)
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum InputName {
    None,
    Ruleset,
    Group,
}

pub mod style {
    pub const SIDE_PANEL: &str = "side-panel";
    pub const CENTER_PANEL: &str = "center-panel";
    pub const CELL: &str = "cell";
    pub const MATERIAL_DISPLAY: &str = "material-display";
    pub const MATERIAL_ROW: &str = "material-row";
    pub const BACKGROUND: &str = "background";
    pub const BUTTON: &str = "button";
    pub const MENU_ELEMENT: &str = "menu-element";

    /// The maximum percentage of the screen the center square can take up.
    pub const CENTER_MARGIN_FACTOR: f32 = 0.6;
    /// Mirrors '.backround/child-space' in 'style.css'.
    pub const BACKGROUND_PADDING: f32 = 0.01;
    /// How much darker the corners of a cell should be compared to the center, as a munber from 0.-255.
    pub const CELL_GRADIENT_DARKEN: u8 = 92;
    /// How many materials display per row on the right panel.
    pub const MATERIAL_ROW_LENGTH: usize = 3;
}
