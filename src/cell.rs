use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use vizia::{
    context::{Context, EmitContext},
    layout::Units::Stretch,
    modifiers::{ActionModifiers, LayoutModifiers, StyleModifiers},
    style::RGBA,
    view::Handle,
    views::{Button, Element},
};

use crate::{AppData, AppEvent};

pub type StateSet = HashMap<String, Vec<String>>;
pub type State = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    material: Material,
    state: State,
}
impl Cell {
    pub fn new(material: Material) -> Self {
        let state = material.default_state();
        Self { material, state }
    }

    pub fn display<'c>(&self, cx: &'c mut Context) -> Handle<'c, Button> {
        Button::new(cx, |cx| Element::new(cx))
            .background_gradient(self.gradient().as_str())
            .space(Stretch(0.03))
            .size(Stretch(1.0))
            .on_hover_out(|cx| cx.emit(AppEvent::CellUnhovered))
    }
    #[rustfmt::skip]
    fn gradient(&self) -> String {
        let color = self.material.color.to_rgba();
        let dark_color = RGBA::rgb(color.r() - 64, color.g() - 64, color.b() - 64);
        format!(
            "radial-gradient(rgba({}, {}, {}), rgba({}, {}, {}))",
            color.r(),      color.g(),      color.b(),
            dark_color.r(), dark_color.g(), dark_color.b()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    color: CellColor,
    states: StateSet,
}
impl Material {
    pub fn blank() -> Self {
        Material {
            color: CellColor::new(255, 255, 255),
            states: StateSet::new(),
        }
    }
    pub fn default_state(&self) -> State {
        self.states
            .iter()
            .map(|(k, v)| (k.clone(), v[0].clone()))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CellColor {
    r: u8,
    g: u8,
    b: u8,
}
impl CellColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn to_rgba(&self) -> RGBA {
        RGBA::rgb(self.r, self.g, self.b)
    }
}
