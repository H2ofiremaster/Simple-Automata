use serde::{Deserialize, Serialize};
use vizia::{binding::LensExt, context::Context, views::ComboBox};

use crate::{
    grid::Cell,
    id::Identifiable,
    material::{GroupId, MaterialId},
    ruleset::Ruleset,
    AppData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pattern {
    Material(MaterialId),
    Group(GroupId),
}
impl Pattern {
    pub fn display_editor(self, cx: &mut Context) {
        let names = AppData::grid.map(|grid| grid.ruleset.clone());
        ComboBox::new(
            cx,
            names.map(Ruleset::pattern_values),
            names.map(move |ruleset| match self {
                Self::Material(id) => ruleset
                    .materials
                    .iter()
                    .enumerate()
                    .find(|(_, material)| material.id() == id)
                    .map(|(index, _)| index)
                    .expect("Displayed pattern should match the current ruleset."),
                Self::Group(id) => ruleset
                    .groups
                    .iter()
                    .enumerate()
                    .find(|(_, group)| group.id() == id)
                    .map(|(index, _)| ruleset.materials.len() + index + 1)
                    .expect("Displayed pattern should match the current ruleset."),
            }),
        );
    }

    pub fn matches(self, ruleset: &Ruleset, target: Cell) -> bool {
        match self {
            Self::Material(id) => id == target.material_id,
            Self::Group(id) => ruleset
                .group(id)
                .is_some_and(|group| group.contains(target.material_id)),
        }
    }
}
