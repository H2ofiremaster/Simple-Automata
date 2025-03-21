use vizia::prelude::*;

use crate::{
    events::{
        EditorEvent, GridEvent, GroupEvent, MaterialEvent, RuleEvent, RulesetEvent, UpdateEvent,
    },
    grid::{Cell, Grid, GridDisplay, VisualGridState},
    id::Identifiable,
    material::Material,
    ruleset::Ruleset,
    AppData,
};

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

        Textbox::new(cx, AppData::screen.map(|s| s.ruleset().name.clone()))
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
            Binding::new(cx, AppData::screen, |cx, screen| {
                let screen = screen.get(cx);
                let materials: Vec<(usize, &Material)> =
                    screen.ruleset().materials.iter().enumerate().collect();
                VStack::new(cx, |cx| {
                    for chunk in materials.chunks(style::EDITOR_ROW_LENGTH) {
                        editor_material_row(cx, chunk, screen.ruleset());
                    }
                })
                .row_between(Pixels(15.0))
                .left(Stretch(0.1))
                .right(Stretch(0.1))
                .size(Auto);
            });
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
                material.display_editor(cx, *index, ruleset);
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
            Binding::new(cx, AppData::screen, |cx, screen| {
                let screen = screen.get(cx);
                VStack::new(cx, |cx| {
                    for (index, group) in screen.ruleset().groups.iter().enumerate() {
                        group.display_editor(cx, index, screen.ruleset());
                    }
                })
                .row_between(Pixels(5.0))
                .min_height(Auto);
            });
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
            Binding::new(cx, AppData::screen, |cx, screen| {
                VStack::new(cx, move |cx| {
                    for (index, rule) in screen.get(cx).ruleset().rules.iter().enumerate() {
                        rule.display_editor(cx, index.into());
                    }
                })
                .row_between(Pixels(5.0))
                .bottom(Pixels(150.0))
                .min_height(Auto);
            });
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
            AppData::screen.map(|screen| {
                if let Screen::Grid(grid) = screen {
                    grid.visual_state()
                } else {
                    VisualGridState::default()
                }
            }),
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
            let border_color = border_color(cell.color(ruleset).to_rgba());
            cell.display(cx, ruleset)
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

#[allow(dead_code)]
pub mod style {
    pub const BACKGROUND: &str = "background";

    pub const SIDE_PANEL: &str = "side-panel";
    pub const CENTER_PANEL: &str = "center-panel";
    pub const CELL: &str = "cell";
    pub const MATERIAL_DISPLAY: &str = "material-display";
    pub const MATERIAL_ROW: &str = "material-row";
    pub const RULESET_BUTTON: &str = "ruleset-button";
    pub const CONTROL_BUTTON: &str = "control-button";
    pub const MATERIAL_TOOLTIP: &str = "material-tooltip";

    pub const PRESSED_BUTTON: &str = "pressed-button";
    pub const TRASH_BUTTON: &str = "trash-button";
    pub const LIGHT_COMBOBOX: &str = "light-combobox";
    pub const MENU_ELEMENT: &str = "menu-element";
    pub const SVG: &str = "svg";

    pub const EDITOR_PANEL: &str = "editor-panel";
    pub const BASE_EDITOR: &str = "base-editor";
    pub const CONDITION_EDITOR: &str = "condition-editor";
    pub const CONDITION_CONTAINER: &str = "condition-container";
    pub const CONDITION_INVERT_BUTTON: &str = "condition-invert-button";

    /// The maximum percentage of the screen the center square can take up.
    pub const CENTER_MARGIN_FACTOR: f32 = 0.6;
    /// Mirrors '.backround/child-space' in 'style.css'.
    pub const BACKGROUND_PADDING: f32 = 10.0;
    /// How much darker the corners of a cell should be compared to the center, as a number from 0-255
    pub const CELL_GRADIENT_DARKEN: u8 = 92;
    /// How many materials display per row on the right panel.
    pub const MATERIAL_ROW_LENGTH: usize = 3;
    /// How many materials display per row in the editor.
    pub const EDITOR_ROW_LENGTH: usize = 5;
    /// How many pixels each material takes up in the editor.
    pub const EDITOR_MATERIAL_SIZE: f32 = 150.0;

    pub mod svg {
        pub const ARROW_NORTHWEST: &str = include_str!("../resources/svg/arrows/northwest.svg");
        pub const ARROW_NORTH: &str = include_str!("../resources/svg/arrows/north.svg");
        pub const ARROW_NORTHEAST: &str = include_str!("../resources/svg/arrows/northeast.svg");
        pub const ARROW_WEST: &str = include_str!("../resources/svg/arrows/west.svg");
        pub const ARROW_EAST: &str = include_str!("../resources/svg/arrows/east.svg");
        pub const ARROW_SOUTHWEST: &str = include_str!("../resources/svg/arrows/southwest.svg");
        pub const ARROW_SOUTH: &str = include_str!("../resources/svg/arrows/south.svg");
        pub const ARROW_SOUTHEAST: &str = include_str!("../resources/svg/arrows/southeast.svg");

        pub const ARROW_UP: &str = include_str!("../resources/svg/arrows/up.svg");
        pub const ARROW_DOWN: &str = include_str!("../resources/svg/arrows/down.svg");

        pub const COPY: &str = include_str!("../resources/svg/copy.svg");
        pub const TRASH: &str = include_str!("../resources/svg/trash.svg");
        pub const TRASH_OPEN: &str = include_str!("../resources/svg/trash_open.svg");
        pub const TRANSFORM_ARROW: &str = include_str!("../resources/svg/transform_arrow.svg");
        pub const EQUAL: &str = include_str!("../resources/svg/equal.svg");
        pub const NOT_EQUAL: &str = include_str!("../resources/svg/not_equal.svg");
        pub const LESS: &str = include_str!("../resources/svg/less.svg");
        pub const GREATER: &str = include_str!("../resources/svg/greater.svg");

        #[rustfmt::skip]
        pub const DIRECTIONAL_CONDITION: &str = include_str!("../resources/svg/directional_condition.svg");
        pub const NUMBERIC_CONDITION: &str = include_str!("../resources/svg/numeric_condition.svg");
    }
}
