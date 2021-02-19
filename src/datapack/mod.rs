use std::{fs::File, path::{Path, PathBuf}};
use std::fs;

use fs::create_dir_all;
use function::Function;
use item_modifier::ItemModifier;
use predicate::Predicate;

use crate::core::Identifier;

pub mod function;
pub mod item_modifier;
pub mod predicate;

/// A datapack. This struct creates and handles a datapack.
pub struct Datapack {
    data: PathBuf
}
impl Datapack {
    /// Create a [`Datapack`] from a [`Path`]
    pub fn new(out: impl AsRef<Path>) -> Self {
        let _ = fs::remove_dir_all(out.as_ref());
        fs::create_dir_all(out.as_ref().join("data")).unwrap();
        fs::write(out.as_ref().join("pack.mcmeta"), 
r#"{
    "pack": {
        "pack_format": 7,
        "description": "Pack generated with Copper"
    }
}
"#
        ).unwrap();
        Self {
            data: out.as_ref().join("data")
        }
    }
    /// Create a function file
    pub fn function(&self, location: Identifier<'_, '_>) -> Function {
        Function::new(&self.data, location)
    }
    /// Create an item modifier
    pub fn item_modifier(&self, location: Identifier<'_, '_>, item_modifier: ItemModifier<'_, '_>) {
        let _ = create_dir_all(self.data.join(location.namespace).join("item_modifiers"));
        let out = File::create(location.join(&self.data, "item_modifiers", "json")).unwrap();
        serde_json::to_writer(out, &item_modifier).unwrap();
    }
    /// Create a predicate
    pub fn predicate(&self, location: Identifier<'_, '_>, predicate: Predicate<'_, '_>) {
        let _ = create_dir_all(self.data.join(location.namespace).join("predicates"));
        let out = File::create(location.join(&self.data, "predicates", "json")).unwrap();
        serde_json::to_writer(out, &predicate).unwrap();
    }
}