use crate::cell::Material;

#[derive(Debug, Clone, PartialEq)]
pub struct Ruleset {
    rules: Vec<Rule>,
    materials: Vec<Material>,
}
impl Ruleset {
    pub fn blank() -> Self {
        Self {
            rules: Vec::new(),
            materials: vec![Material::blank()],
        }
    }

    pub fn default_material(&self) -> &Material {
        &self.materials[0]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {}
