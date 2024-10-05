use crate::material::{Material, MaterialGroup, MaterialMap};

#[derive(Debug, Clone, PartialEq)]
pub struct Ruleset {
    rules: Vec<Rule>,
    pub materials: MaterialMap,
    groups: Vec<MaterialGroup>,
}
impl Ruleset {
    pub fn blank() -> Self {
        Self {
            rules: Vec::new(),
            materials: MaterialMap::new(Material::blank()),
            groups: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {}
