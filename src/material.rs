use std::vec;

use rand::Rng;
use serde::{Deserialize, Serialize};
use vizia::style::RGBA;

use crate::ruleset::Ruleset;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub color: MaterialColor,
}
impl Material {
    pub fn new(ruleset: &Ruleset) -> Self {
        Self {
            id: MaterialId::get_unique(&ruleset.materials.0),
            name: String::from("Empty"),
            color: MaterialColor::DEFAULT,
        }
    }

    pub fn blank() -> Self {
        Self {
            id: MaterialId::get_unique(&[]),
            name: String::from("Blank"),
            color: MaterialColor::BLANK,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize)]
pub struct MaterialColor {
    r: u8,
    g: u8,
    b: u8,
}
impl MaterialColor {
    const DEFAULT: Self = Self::new(0, 0, 0);
    const BLANK: Self = Self::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const fn to_rgba(&self) -> RGBA {
        RGBA::rgb(self.r, self.g, self.b)
    }
}
impl Serialize for MaterialColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialId(u32);
impl MaterialId {
    fn get_unique(current: &[Material]) -> Self {
        if current.len() as u32 >= u32::MAX {
            panic!("Material list should not exceed u32::MAX");
        }
        let mut random = rand::thread_rng();
        loop {
            let candidate = Self(random.gen());
            if !current.iter().any(|m| m.id == candidate) {
                return candidate;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialMap(Vec<Material>);
impl MaterialMap {
    pub fn new(default: Material) -> Self {
        let materials = vec![default];
        Self(materials)
    }

    pub fn default(&self) -> &Material {
        &self.0[0]
    }

    pub fn get(&self, key: MaterialId) -> Option<&Material> {
        self.0.iter().find(|material| material.id == key)
    }

    pub fn push(&mut self, material: Material) {
        self.0.push(material);
    }

    pub fn iter(&self) -> std::slice::Iter<Material> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialGroup {
    name: String,
    materials: Vec<MaterialId>,
}
