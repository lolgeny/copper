use std::path::{Path, PathBuf};
use std::fs;

use function::Function;

use crate::core::Identifier;

pub mod function;
pub struct Datapack {
    data: PathBuf
}
impl Datapack {
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
    pub fn function(&self, location: Identifier) -> Function {
        Function::new(&self.data, location)
    }
}