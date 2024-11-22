use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use vizia::{
    binding::{LensExt, Map, Wrapper},
    context::{Context, EventContext},
    view::Handle,
    views::ComboBox,
};

use crate::{
    grid::Cell,
    id::{Identifiable, UniqueId},
    material::{GroupId, MaterialId},
    ruleset::Ruleset,
    AppData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Material(MaterialId),
    Group(GroupId),
}
impl Pattern {
    pub fn display_editor<F>(self, cx: &mut Context, on_select: F)
    where
        F: Fn(&mut EventContext, usize) + 'static,
    {
        ComboBox::new(
            cx,
            AppData::screen.map(|screen| screen.ruleset().pattern_values()),
            AppData::screen.map(move |screen| match self {
                Self::Material(id) => screen
                    .ruleset()
                    .materials
                    .index_of(id)
                    .expect("Displayed pattern should match the current ruleset."),
                Self::Group(id) => screen
                    .ruleset()
                    .index_of_group(id)
                    .map(|index| screen.ruleset().materials.len() + index)
                    .expect("Displayed pattern should match the current ruleset."),
            }),
        )
        .on_select(on_select);
    }

    pub fn matches(self, ruleset: &Ruleset, target: Cell) -> bool {
        match self {
            Self::Material(id) => id == target.material_id,
            Self::Group(id) => ruleset
                .group(id)
                .is_some_and(|group| group.contains(target.material_id)),
        }
    }

    pub fn from_index(ruleset: &Ruleset, index: usize) -> Option<Self> {
        ruleset
            .materials
            .get_at(index)
            .map(|m| Self::Material(m.id()))
            .or_else(|| {
                ruleset
                    .groups
                    .get(index - ruleset.materials.len())
                    .map(|g| Self::Group(g.id()))
            })
    }
}
impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(PatternVisitor)
    }
}
struct PatternVisitor;
impl<'de> Visitor<'de> for PatternVisitor {
    type Value = Pattern;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "enum Pattern")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let (id, suffix) = v.split_at(v.len() - 1);
        let id = id
            .parse()
            .map_err(|_| de::Error::invalid_type(de::Unexpected::Str(id), &self))?;
        match suffix {
            "m" => Ok(Pattern::Material(UniqueId::new_unchecked(id))),
            "g" => Ok(Pattern::Group(UniqueId::new_unchecked(id))),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Str(suffix),
                &"either 'm' or 'g'",
            )),
        }
    }
}
impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = match self {
            Self::Material(id) => format!("{id}m"),
            Self::Group(id) => format!("{id}g"),
        };
        serializer.serialize_str(&string)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id::UniqueId,
        material::{Material, MaterialGroup, MaterialMap},
    };

    // Wrapper struct because for some reason toml doesn't want to directly deserialize patterns.
    // If it works in this, it should in a Ruleset as well.
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct W<T> {
        v: T,
    }
    impl<T> W<T> {
        const fn new(v: T) -> Self {
            Self { v }
        }
    }

    use super::*;
    #[allow(clippy::unwrap_used)]
    #[test]
    fn serde_pattern() {
        let material_pattern = W::new(Pattern::Material(UniqueId::new(&[])));
        let group_pattern = W::new(Pattern::Group(UniqueId::new(&[])));

        dbg!(&material_pattern);
        dbg!(&group_pattern);

        let material_string = toml::to_string(&material_pattern).unwrap();
        let group_string = toml::to_string(&group_pattern).unwrap();

        println!("Material:\n```\n{material_string:?}\n```\nGroup:\n```\n{group_string:?}\n```");

        let new_material_pattern: W<Pattern> = toml::from_str(&material_string).unwrap();
        let new_group_pattern: W<Pattern> = toml::from_str(&group_string).unwrap();

        dbg!(&new_material_pattern);
        dbg!(&new_group_pattern);

        assert_eq!(material_pattern, new_material_pattern);
        assert_eq!(group_pattern, new_group_pattern);
    }

    #[test]
    fn from_index() {
        const fn ida<T: Identifiable>(v: u32) -> UniqueId<T> {
            UniqueId::new_unchecked(v)
        }
        fn m(id: u32) -> Material {
            Material::new_unchecked(ida(id))
        }
        fn g(id: u32, m_id: u32) -> MaterialGroup {
            MaterialGroup::new_unchecked(ida(id), vec![ida(m_id)])
        }

        let materials: Vec<Material> = vec![m(1), m(2), m(3)];
        let map = MaterialMap::new_unchecked(materials);
        let groups: Vec<MaterialGroup> = vec![g(10, 1), g(20, 2), g(30, 3)];
        let ruleset = Ruleset {
            name: String::from("Test"),
            rules: vec![],
            materials: map,
            groups,
        };

        assert_eq!(
            Pattern::from_index(&ruleset, 0),
            Some(Pattern::Material(ida(1)))
        );
        assert_eq!(
            Pattern::from_index(&ruleset, 1),
            Some(Pattern::Material(ida(2)))
        );
        assert_eq!(
            Pattern::from_index(&ruleset, 2),
            Some(Pattern::Material(ida(3)))
        );
        assert_eq!(
            Pattern::from_index(&ruleset, 3),
            Some(Pattern::Group(ida(10)))
        );
        assert_eq!(
            Pattern::from_index(&ruleset, 4),
            Some(Pattern::Group(ida(20)))
        );
        assert_eq!(
            Pattern::from_index(&ruleset, 5),
            Some(Pattern::Group(ida(30)))
        );
        assert_eq!(Pattern::from_index(&ruleset, 6), None);
    }
}
