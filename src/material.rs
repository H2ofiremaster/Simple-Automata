use std::vec;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use vizia::style::RGBA;

use crate::{
    id::{Identifiable, UniqueId},
    ruleset::Ruleset,
};

pub type MaterialId = UniqueId<Material>;
pub type GroupId = UniqueId<MaterialGroup>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Material {
    id: UniqueId<Self>,
    pub name: String,
    pub color: MaterialColor,
}
impl Material {
    pub fn new(ruleset: &Ruleset) -> Self {
        Self {
            id: UniqueId::new(&ruleset.materials.0),
            name: String::from("Empty"),
            color: MaterialColor::DEFAULT,
        }
    }

    pub fn blank() -> Self {
        Self {
            id: UniqueId::new(&[]),
            name: String::from("Blank"),
            color: MaterialColor::BLANK,
        }
    }
}
impl Identifiable for Material {
    fn id(&self) -> UniqueId<Self> {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
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
    pub const fn to_rgba(self) -> RGBA {
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
struct MaterialColorVisitor;
impl<'de> Visitor<'de> for MaterialColorVisitor {
    type Value = MaterialColor;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct MaterialColor")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let numbers = v
            .strip_prefix('#')
            .ok_or_else(|| de::Error::custom("str was not prefixed with '#'"))?;
        let mut numbers = numbers
            .as_bytes()
            .chunks(2)
            .map(|bytes| u8::from_str_radix(&String::from_utf8_lossy(bytes), 16));
        let r = numbers
            .next()
            .ok_or_else(|| de::Error::custom("Too few numbers. Got '0', expected '3'."))
            .and_then(|result| {
                result.map_err(|err| {
                    de::Error::custom(format!("value for 'r' is invalid hexadecimal. {err}"))
                })
            })?;
        let g = numbers
            .next()
            .ok_or_else(|| de::Error::custom("Too few numbers. Got '1', expected '3'."))
            .and_then(|result| {
                result.map_err(|err| {
                    de::Error::custom(format!("value for 'g' is invalid hexadecimal. {err}"))
                })
            })?;
        let b = numbers
            .next()
            .ok_or_else(|| de::Error::custom("Too few numbers. Got '2', expected '3'."))
            .and_then(|result| {
                result.map_err(|err| {
                    de::Error::custom(format!("value for 'b' is invalid hexadecimal. {err}"))
                })
            })?;
        if numbers.next().is_some() {
            return Err(de::Error::custom("Too many numbers. Expected '3'."));
        }
        Ok(MaterialColor::new(r, g, b))
    }
}
impl<'de> Deserialize<'de> for MaterialColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MaterialColorVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialGroup {
    id: UniqueId<Self>,
    pub name: String,
    materials: Vec<MaterialId>,
}
impl MaterialGroup {
    pub fn new(name: String, ruleset: &Ruleset) -> Self {
        Self {
            id: UniqueId::new(&ruleset.groups),
            name,
            materials: vec![],
        }
    }
    pub fn contains(&self, id: MaterialId) -> bool {
        self.materials.contains(&id)
    }
}
impl Identifiable for MaterialGroup {
    fn id(&self) -> UniqueId<Self> {
        self.id
    }
}
