#![allow(dead_code)]
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

#[rustfmt::skip]
    pub mod svg {
        pub const ARROW_NORTHWEST: &str = include_str!("../../resources/svg/arrows/northwest.svg");
        pub const ARROW_NORTH: &str = include_str!("../../resources/svg/arrows/north.svg");
        pub const ARROW_NORTHEAST: &str = include_str!("../../resources/svg/arrows/northeast.svg");
        pub const ARROW_WEST: &str = include_str!("../../resources/svg/arrows/west.svg");
        pub const ARROW_EAST: &str = include_str!("../../resources/svg/arrows/east.svg");
        pub const ARROW_SOUTHWEST: &str = include_str!("../../resources/svg/arrows/southwest.svg");
        pub const ARROW_SOUTH: &str = include_str!("../../resources/svg/arrows/south.svg");
        pub const ARROW_SOUTHEAST: &str = include_str!("../../resources/svg/arrows/southeast.svg");

        pub const ARROW_UP: &str = include_str!("../../resources/svg/arrows/up.svg");
        pub const ARROW_DOWN: &str = include_str!("../../resources/svg/arrows/down.svg");

        pub const COPY: &str = include_str!("../../resources/svg/copy.svg");
        pub const TRASH: &str = include_str!("../../resources/svg/trash.svg");
        pub const TRASH_OPEN: &str = include_str!("../../resources/svg/trash_open.svg");
        pub const TRANSFORM_ARROW: &str = include_str!("../../resources/svg/transform_arrow.svg");
        pub const EQUAL: &str = include_str!("../../resources/svg/equal.svg");
        pub const NOT_EQUAL: &str = include_str!("../../resources/svg/not_equal.svg");
        pub const LESS: &str = include_str!("../../resources/svg/less.svg");
        pub const GREATER: &str = include_str!("../../resources/svg/greater.svg");

        pub const DIRECTIONAL_CONDITION: &str = include_str!("../../resources/svg/directional_condition.svg");
        pub const NUMBERIC_CONDITION: &str = include_str!("../../resources/svg/numeric_condition.svg");
    }
