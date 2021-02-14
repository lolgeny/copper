use std::{fs::File, fs, path::Path};
use std::io::Write;

use crate::{core::{Coordinates, Identifier, Selector, sel::at_s}, score::ScoreValue};
use crate::minecraft::*;

pub struct Function {
    out: File
}
impl Function {
    pub (super) fn new(path: impl AsRef<Path>, id: Identifier) -> Self {
        let mut functions = path.as_ref().join(id.namespace).join("functions");
        for folder in id.folders {
            functions = functions.join(folder);
        }
        fs::create_dir_all(&functions).unwrap();
        let mut out_path = functions.join(id.id);
        out_path.set_extension("mcfunction");
        Self {
            out: File::create(out_path).unwrap()
        }
    }
    pub fn score<'a>(&mut self, name: &'a str, objective: &'a str) -> ScoreValue<'a> {
        ScoreValue::new(name, objective, self.out.try_clone().unwrap())
    }
    pub fn give(&mut self, target: Selector<'_>, item: Item, count: u64) {
        write!(self.out, "give {} {}", target, item).unwrap();
        if count != 1 {
            write!(self.out, " {}", count).unwrap();
        }
        write!(self.out, "\n").unwrap();
    }
    pub fn setblock(&mut self, target: Coordinates, block: Block) {
        writeln!(self.out, "setblock {} {}", target, block).unwrap();
    }
    pub fn kill(&mut self, target: Option<Selector<'_>>) {
        write!(self.out, "kill").unwrap();
        if let Some(target) = target {
            write!(self.out, " {}", target).unwrap();
        }
        writeln!(self.out).unwrap();
    }
    pub fn clear(&mut self, target: Selector<'_>, item: Option<(Item, Option<u64>)>) {
        write!(self.out, "clear").unwrap();
        if target != at_s() {
            write!(self.out, " {}", target).unwrap();
            if let Some((item, count)) = item {
                write!(self.out, " {}", item).unwrap();
                if let Some(count) = count {
                    write!(self.out, " {}", count).unwrap();
                }
            }
        }
        writeln!(self.out).unwrap();
    }
    pub fn effect_give(&mut self, target: Selector<'_>, effect: Effect, seconds: u64, amplifier: u64, hide_particles: bool) {
        write!(self.out, "effect give {} {}", target, effect).unwrap();
        let mut variation = 0;
        if seconds != 30 {variation = 1};
        if amplifier != 0 {variation = 2};
        if hide_particles {variation = 3};
        if variation >= 1 {write!(self.out, " {}", seconds).unwrap();}
        if variation >= 2 {write!(self.out, " {}", amplifier).unwrap();}
        if variation >= 3 {write!(self.out, " {}", hide_particles).unwrap();}
        writeln!(self.out).unwrap();
    }
}