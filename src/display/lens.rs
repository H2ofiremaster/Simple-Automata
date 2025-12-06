use vizia::binding::{Lens, LensValue};

use crate::{
    condition::{ConditionIndex, ConditionVariant},
    display::Screen,
    grid::Grid,
    material::MaterialMap,
    ruleset::Ruleset,
};

#[derive(Debug, Hash, Clone, Copy)]
pub struct ScreenToRuleset;
impl Lens for ScreenToRuleset {
    type Source = Screen;

    type Target = Ruleset;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Borrowed(source.ruleset()))
    }
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct ScreenToGrid;
impl Lens for ScreenToGrid {
    type Source = Screen;

    type Target = Grid;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        if let Screen::Grid(grid) = source {
            Some(LensValue::Borrowed(grid))
        } else {
            None
        }
    }
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct ScreenToMaterialMap;
impl Lens for ScreenToMaterialMap {
    type Source = Screen;

    type Target = MaterialMap;

    fn view<'a>(
        &self,
        source: &'a Self::Source,
    ) -> Option<vizia::binding::LensValue<'a, Self::Target>> {
        Some(LensValue::Borrowed(&source.ruleset().materials))
    }
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct ScreenToConditionVariant(pub ConditionIndex);
impl Lens for ScreenToConditionVariant {
    type Source = Screen;

    type Target = ConditionVariant;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        let variant = &self.0.condition(source.ruleset()).variant;
        Some(LensValue::Borrowed(variant))
    }
}
