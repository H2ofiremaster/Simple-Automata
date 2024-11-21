use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use vizia::{
    binding::{LensExt, Map, Wrapper},
    context::Context,
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

type ScreenWrapper = Wrapper<crate::app_data_derived_lenses::screen>;
type StringVecMap = Map<ScreenWrapper, Vec<String>>;
type UsizeMap = Map<ScreenWrapper, usize>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Pattern {
    Material(MaterialId),
    Group(GroupId),
}
impl Pattern {
    pub fn display_editor(
        self,
        cx: &mut Context,
    ) -> Handle<'_, ComboBox<StringVecMap, UsizeMap, String>> {
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
                    .map(|index| screen.ruleset().materials.len() + index + 1)
                    .expect("Displayed pattern should match the current ruleset."),
            }),
        )
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
                    .get(index + 1 - ruleset.materials.len())
                    .map(|g| Self::Group(g.id()))
            })
    }
}
impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(PatternVisitor)
    }
}
struct PatternVisitor;
impl<'de> Visitor<'de> for PatternVisitor {
    type Value = Pattern;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "enum Pattern")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut pattern = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "Material" => {
                    if pattern.is_some() {
                        return Err(de::Error::duplicate_field("Material"));
                    }
                    let raw_id = map.next_value()?;
                    pattern = Some(Pattern::Material(UniqueId::new_unchecked(raw_id)));
                }
                "Group" => {
                    if pattern.is_some() {
                        return Err(de::Error::duplicate_field("Group"));
                    }
                    let raw_id = map.next_value()?;
                    pattern = Some(Pattern::Group(UniqueId::new_unchecked(raw_id)));
                }
                _ => return Err(de::Error::unknown_field(&key, &["Material", "Group"])),
            }
        }

        pattern.ok_or_else(|| de::Error::missing_field("Material or Group"))
    }
}

#[cfg(test)]
mod tests {
    use crate::id::UniqueId;

    use super::*;
    #[allow(clippy::unwrap_used)]
    #[test]
    fn serde_pattern() {
        let material_pattern = Pattern::Material(UniqueId::new(&[]));
        let group_pattern = Pattern::Group(UniqueId::new(&[]));

        dbg!(material_pattern);
        dbg!(group_pattern);

        let material_string = toml::to_string(&material_pattern).unwrap();
        let group_string = toml::to_string(&group_pattern).unwrap();

        println!("Material:\n```\n{material_string:?}\n```\nGroup:\n```\n{group_string:?}\n```");

        let new_material_pattern: Pattern = toml::from_str(&material_string).unwrap();
        let new_group_pattern: Pattern = toml::from_str(&group_string).unwrap();

        dbg!(new_material_pattern);
        dbg!(new_group_pattern);

        assert_eq!(material_pattern, new_material_pattern);
        assert_eq!(group_pattern, new_group_pattern);
    }
}
