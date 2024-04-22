use std::{collections::HashMap, fmt::Debug};

use macroquad::color::Color;

pub type State = HashMap<String, String>;
pub type StateSet = HashMap<String, Vec<String>>;

#[derive(Clone)]
pub struct Cell {
    pub(super) material: Material,
    pub(super) state: State,
}
impl Cell {
    pub fn new(material: Material, state: State) -> Self {
        Self { material, state }
    }
    pub fn new_default(material: Material) -> Self {
        let state = material.default_states();
        Self { material, state }
    }
    pub fn is_material(&self, material: &str) -> bool {
        self.material.name == material
    }
    pub fn has_states(&self, states: &State) -> bool {
        states
            .iter()
            .all(|(key, value)| self.state.get(key).is_some_and(|v| v == value))
    }
    pub fn get_state(&self) -> &HashMap<String, String> {
        &self.state
    }
    pub fn get_color(&self) -> Color {
        self.material.color
    }
    pub fn get_name(&self) -> &str {
        &self.material.name
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state: String = self.state.iter().fold(String::new(), |acc, pair| {
            acc + &format!("({}: {}), ", pair.0, pair.1)
        });
        let color = {
            let c = self.get_color();
            ((c.r * 255.) as u8, (c.g * 255.) as u8, (c.b * 255.) as u8)
        };

        write!(
            f,
            "Cell{{{}, #{:02X}{:02X}{:02X}: {}}}",
            self.get_name(),
            color.0,
            color.1,
            color.2,
            state
        )
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub color: Color,
    states: StateSet,
}
impl Material {
    pub fn new(name: String, color: Color, states: StateSet) -> Self {
        Self {
            name,
            color,
            states,
        }
    }
    pub fn default_states(&self) -> State {
        self.states
            .iter()
            .map(|(namespace, set)| (namespace.clone(), set[0].clone()))
            .collect()
    }
}

pub fn merge_states(base: &State, other: &State) -> State {
    let mut new = base.clone();
    other
        .clone()
        .into_iter()
        .filter(|(key, _)| base.contains_key(key))
        .for_each(|(key, value)| {
            new.insert(key, value);
        });
    new
}
