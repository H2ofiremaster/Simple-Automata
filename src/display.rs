use vizia::prelude::*;

use crate::{
    events::{
        EditorEvent, GridEvent, GroupEvent, MaterialEvent, RuleEvent, RulesetEvent, UpdateEvent,
    },
    grid::{Cell, Grid},
    id::Identifiable,
    ruleset::Ruleset,
    AppData,
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
                            material_editor(cx, ruleset.clone());
                            group_editor(cx, ruleset.clone());
                        })
                        .space(Percentage(1.0));
                    }
                    EditorTab::Rules => rule_editor(cx, ruleset.clone()),
                }
            });
        });
    })
    .class("background");
}

fn toolbar(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Back"))
            .on_press(|cx| cx.emit(EditorEvent::Disabled))
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
        .on_select(|cx, index| cx.emit(RulesetEvent::Selected(index)))
        .top(Stretch(1.0))
        .bottom(Stretch(1.0));

        Textbox::new(cx, AppData::screen.map(|s| s.ruleset().name.clone()))
            .min_width(Pixels(100.0))
            .on_submit(|cx, text, _| {
                cx.emit(RulesetEvent::Renamed(text));
            })
            .top(Stretch(1.0))
            .bottom(Stretch(1.0));

        Button::new(cx, |cx| Label::new(cx, "New"))
            .on_press(|cx| cx.emit(RulesetEvent::Created))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0));

        Button::new(cx, |cx| Label::new(cx, "Save"))
            .on_press(|cx| cx.emit(RulesetEvent::Saved))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0));

        Button::new(cx, |cx| Label::new(cx, "Reload"))
            .on_press(|cx| cx.emit(RulesetEvent::Reloaded))
            .top(Stretch(1.0))
            .bottom(Stretch(1.0));
    })
    .height(Percentage(5.0));
}

fn tabs(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Materials"))
            .on_press(|cx| cx.emit(EditorEvent::TabSwitched(EditorTab::Materials)))
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
        Button::new(cx, |cx| Label::new(cx, "Rules"))
            .on_press(|cx| cx.emit(EditorEvent::TabSwitched(EditorTab::Rules)))
            .width(Stretch(1.0))
            .text_align(TextAlign::Center)
            .child_space(Stretch(1.0));
    })
    .height(Percentage(5.0));
}

fn material_editor(cx: &mut Context, ruleset: Ruleset) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, true, true, move |cx| {
            VStack::new(cx, |cx| {
                for (index, material) in ruleset.materials.iter().enumerate() {
                    material.display_editor(cx, index, &ruleset);
                }
            })
            .min_height(Auto);
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

fn group_editor(cx: &mut Context, ruleset: Ruleset) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, true, true, move |cx| {
            VStack::new(cx, |cx| {
                for (index, group) in ruleset.groups.iter().enumerate() {
                    group.display_editor(cx, index, &ruleset);
                }
            })
            .row_between(Pixels(5.0))
            .min_height(Auto);
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
fn rule_editor(cx: &mut Context, ruleset: Ruleset) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
            VStack::new(cx, move |cx| {
                for (index, rule) in ruleset.rules.iter().enumerate() {
                    rule.display_editor(cx, index, &ruleset);
                }
            })
            .row_between(Pixels(5.0))
            .min_height(Auto);
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
        controls(cx);
        speed_controls(cx);
        material_tooltip(cx);
    })
    .class(style::SIDE_PANEL);
}

fn editor_button(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Edit Ruleset"))
            .on_press(|cx| cx.emit(EditorEvent::Enabled));
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
        .on_press(|cx| cx.emit(GridEvent::Toggled))
        .class(style::BUTTON);
        Button::new(cx, |cx| Label::new(cx, "Step"))
            .on_press(|cx| cx.emit(GridEvent::Stepped))
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
                    cx.emit(UpdateEvent::MaterialSelected(cell.material_id));
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

pub mod style {
    pub const SIDE_PANEL: &str = "side-panel";
    pub const CENTER_PANEL: &str = "center-panel";
    pub const CELL: &str = "cell";
    pub const MATERIAL_DISPLAY: &str = "material-display";
    pub const MATERIAL_ROW: &str = "material-row";
    pub const BACKGROUND: &str = "background";
    pub const BUTTON: &str = "button";
    pub const MENU_ELEMENT: &str = "menu-element";
    pub const EDITOR_PANEL: &str = "editor-panel";

    /// The maximum percentage of the screen the center square can take up.
    pub const CENTER_MARGIN_FACTOR: f32 = 0.6;
    /// Mirrors '.backround/child-space' in 'style.css'.
    pub const BACKGROUND_PADDING: f32 = 0.01;
    /// How much darker the corners of a cell should be compared to the center, as a munber from 0.-255.
    pub const CELL_GRADIENT_DARKEN: u8 = 92;
    /// How many materials display per row on the right panel.
    pub const MATERIAL_ROW_LENGTH: usize = 3;
}
