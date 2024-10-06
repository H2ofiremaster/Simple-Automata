use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::material::{Material, MaterialGroup, MaterialMap};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    rules: Vec<Rule>,
    pub materials: MaterialMap,
    groups: Vec<MaterialGroup>,
}
impl Ruleset {
    pub fn blank() -> Self {
        Self {
            name: String::from("Blank"),
            rules: Vec::new(),
            materials: MaterialMap::new(Material::blank()),
            groups: vec![],
        }
    }
    pub fn save(&self) -> Result<(), String> {
        let string = toml::to_string(self).map_err(|err| {
            format!("Could not save ruleset '{self:?}'; serialization failed: {err}")
        })?;
        let mut path = PathBuf::from(crate::RULESET_PATH);
        path.push(&self.name);
        path.set_extension("toml");
        fs::write(path, string)
            .map_err(|err| format!("Could not save ruleset '{self:?}'; file IO failed: {err}"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Rule {}
