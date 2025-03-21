use std::{fmt::Display, str::FromStr, vec};

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use vizia::{
    binding::LensExt,
    context::{Context, EmitContext},
    layout::Units::{Auto, Percentage, Pixels, Stretch},
    modifiers::{ActionModifiers, LayoutModifiers, StyleModifiers},
    style::RGBA,
    views::{Button, ComboBox, HStack, Label, Svg, Textbox, VStack},
};

use crate::{
    display::style,
    events::{GroupEvent, MaterialEvent},
    grid::Cell,
    id::{Identifiable, UniqueId},
    ruleset::Ruleset,
    AppData,
};

pub type MaterialId = UniqueId<Material>;
pub type GroupId = UniqueId<MaterialGroup>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
    pub fn new_unchecked(id: MaterialId) -> Self {
        Self {
            id,
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

    pub fn display_editor(&self, cx: &mut Context, index: usize, ruleset: &Ruleset) {
        VStack::new(cx, |cx| {
            let cell = Cell::new(self.id);
            let id = self.id;

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

            cell.display(cx, ruleset)
                .size(Pixels(style::EDITOR_MATERIAL_SIZE));

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
}
impl Default for Material {
    fn default() -> Self {
        Self {
            id: UniqueId::new(&[]),
            name: String::from("Empty"),
            color: MaterialColor::DEFAULT,
        }
    }
}
impl Identifiable for Material {
    fn id(&self) -> UniqueId<Self> {
        self.id
    }
}
struct MaterialVisitor;
impl<'de> Visitor<'de> for MaterialVisitor {
    type Value = Material;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct Material")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut id = None;
        let mut name = None;
        let mut color = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "id" => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
                    }
                    let raw_id: u32 = map.next_value()?;
                    id = Some(UniqueId::new_unchecked(raw_id));
                }
                "name" => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = map.next_value()?;
                }
                "color" => {
                    if color.is_some() {
                        return Err(de::Error::duplicate_field("color"));
                    }
                    color = map.next_value()?;
                }
                _ => return Err(de::Error::unknown_field(&key, &["id", "name", "color"])),
            }
        }

        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let color = color.ok_or_else(|| de::Error::missing_field("color"))?;

        Ok(Material { id, name, color })
    }
}
impl<'de> Deserialize<'de> for Material {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Material", &["id", "name", "color"], MaterialVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct MaterialColor {
    r: u8,
    g: u8,
    b: u8,
}
impl MaterialColor {
    pub const DEFAULT: Self = Self::new(0, 0, 0);
    const BLANK: Self = Self::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const fn to_rgba(self) -> RGBA {
        RGBA::rgb(self.r, self.g, self.b)
    }
    #[allow(clippy::cast_possible_truncation)]
    pub const fn invert_grayscale(self) -> Self {
        let avg =
            (((255 - self.r) as u32 + (255 - self.g) as u32 + (255 - self.b) as u32) / 3) as u8;
        Self {
            r: avg,
            g: avg,
            b: avg,
        }
    }
}
impl Display for MaterialColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
impl FromStr for MaterialColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .strip_prefix('#')
            .ok_or_else(|| String::from("str was not prefixed with '#'"))?;
        let mut numbers = numbers
            .as_bytes()
            .chunks(2)
            .map(|bytes| u8::from_str_radix(&String::from_utf8_lossy(bytes), 16));
        let r = numbers
            .next()
            .ok_or_else(|| String::from("Too few numbers. Got '0', expected '3'."))
            .and_then(|result| {
                result.map_err(|err| format!("value for 'r' is invalid hexadecimal. {err}"))
            })?;
        let g = numbers
            .next()
            .ok_or_else(|| String::from("Too few numbers. Got '1', expected '3'."))
            .and_then(|result| {
                result.map_err(|err| format!("value for 'g' is invalid hexadecimal. {err}"))
            })?;
        let b = numbers
            .next()
            .ok_or_else(|| String::from("Too few numbers. Got '2', expected '3'."))
            .and_then(|result| {
                result.map_err(|err| format!("value for 'b' is invalid hexadecimal. {err}"))
            })?;
        if numbers.next().is_some() {
            return Err(String::from("Too many numbers. Expected '3'."));
        }
        Ok(Self::new(r, g, b))
    }
}
impl From<MaterialColor> for vizia::vg::Color {
    fn from(value: MaterialColor) -> Self {
        Self::from_rgb(value.r, value.g, value.b)
    }
}
impl Serialize for MaterialColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
struct MaterialColorVisitor;
impl Visitor<'_> for MaterialColorVisitor {
    type Value = MaterialColor;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct MaterialColor")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(|err| de::Error::custom(&err))
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
    pub const fn new_unchecked(v: Vec<Material>) -> Self {
        Self(v)
    }
    pub fn default(&self) -> &Material {
        &self.0[0]
    }

    pub fn get(&self, key: MaterialId) -> Option<&Material> {
        self.0.iter().find(|material| material.id == key)
    }

    pub fn remove(&mut self, id: MaterialId) {
        if let Some(index) = self.0.iter().position(|m| m.id == id) {
            self.0.remove(index);
        };
    }

    pub fn names(&self) -> Vec<String> {
        self.iter().map(|m| m.name.clone()).collect()
    }

    pub fn index_of(&self, id: MaterialId) -> Option<usize> {
        self.iter().position(|m| m.id == id)
    }

    pub fn get_at(&self, index: usize) -> Option<&Material> {
        self.0.get(index)
    }

    pub fn get_mut_at(&mut self, index: usize) -> Option<&mut Material> {
        self.0.get_mut(index)
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterialGroup {
    id: UniqueId<Self>,
    pub name: String,
    materials: Vec<MaterialId>,
}
impl MaterialGroup {
    pub fn new(ruleset: &Ruleset) -> Self {
        Self {
            id: UniqueId::new(&ruleset.groups),
            name: String::from("New Group"),
            materials: vec![],
        }
    }
    pub fn new_unchecked(id: GroupId, materials: Vec<MaterialId>) -> Self {
        Self {
            id,
            name: String::from("New Group"),
            materials,
        }
    }
    pub fn contains(&self, id: MaterialId) -> bool {
        self.materials.contains(&id)
    }
    pub fn push(&mut self, id: MaterialId) {
        self.materials.push(id);
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut MaterialId> {
        self.materials.get_mut(index)
    }
    pub fn remove_at(&mut self, index: usize) {
        self.materials.remove(index);
    }

    pub fn display_editor(&self, cx: &mut Context, index: usize, ruleset: &Ruleset) {
        let id = self.id;
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

            self.materials
                .iter()
                .enumerate()
                .filter_map(|(index, id)| ruleset.materials.get(*id).map(|_| index))
                .enumerate()
                .for_each(|(entry_index, material_index)| {
                    Self::display_entry(cx, index, material_index, entry_index);
                });
        })
        .width(Percentage(100.0))
        .class(style::BASE_EDITOR);
    }
    fn display_entry(
        cx: &mut Context,
        group_index: usize,
        material_index: usize,
        entry_index: usize,
    ) {
        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "-")).on_press(move |cx| {
                cx.emit(GroupEvent::EntryDeleted {
                    group_index,
                    entry_index,
                });
            });
            ComboBox::new(
                cx,
                AppData::screen.map(|screen| screen.ruleset().materials.names()),
                AppData::screen.map(move |screen| {
                    let Some(group) = screen.ruleset().groups.get(group_index) else {
                        return 0;
                    };
                    let Some(material) = group.materials.get(material_index) else {
                        return 0;
                    };
                    let Some(index) = screen.ruleset().materials.index_of(*material) else {
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
}
impl Identifiable for MaterialGroup {
    fn id(&self) -> UniqueId<Self> {
        self.id
    }
}
struct MaterialGroupVisitor;
impl<'de> Visitor<'de> for MaterialGroupVisitor {
    type Value = MaterialGroup;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct MaterialGroup")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut id = None;
        let mut name = None;
        let mut materials = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "id" => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
                    }
                    let id_raw: u32 = map.next_value()?;
                    id = Some(UniqueId::new_unchecked(id_raw));
                }
                "name" => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                "materials" => {
                    if materials.is_some() {
                        return Err(de::Error::duplicate_field("materials"));
                    }
                    let materials_raw: Vec<u32> = map.next_value()?;
                    materials = Some(
                        materials_raw
                            .into_iter()
                            .map(UniqueId::new_unchecked)
                            .collect(),
                    );
                }
                _ => return Err(de::Error::unknown_field(&key, &["id", "name", "materials"])),
            }
        }

        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let materials = materials.ok_or_else(|| de::Error::missing_field("materials"))?;

        Ok(MaterialGroup {
            id,
            name,
            materials,
        })
    }
}
impl<'de> Deserialize<'de> for MaterialGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "MaterialGroup",
            &["id", "name", "materials"],
            MaterialGroupVisitor,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::unwrap_used)]
    #[test]
    fn serde_material() {
        let material = Material::blank();
        let serialized = toml::to_string(&material);
        if let Err(err) = serialized {
            println!("{err}");
            panic!("'serialized' returned error")
        }
        let deserialized = toml::from_str(&serialized.unwrap());
        if let Err(err) = deserialized {
            println!("{err}");
            panic!("'deserialized' returned error")
        }
        assert_eq!(material, deserialized.unwrap());
    }
}
