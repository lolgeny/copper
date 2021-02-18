#![allow(non_snake_case)]

/*!
Contains the `Function` struct, created via [`Datapack::function`](super::Datapack::function).

Also contains commands that can be passed to a function, via [`Function::run`].
Each command is a struct containing its various paramters.

There is also a function with the same as each command, which allows for easy importing.
This function creates the command filled with default values, provided the required values.

You can use this constructor function, along with struct update syntax, to specify arguments.

You may notice that structs contain values for optional fields; this is to make it easy to specify these.
When they are set to their default value they are automatically omitted (provided there is no variance afterwards).

For example:
```
# use copper::{id, datapack::function::{Give, Clear, EffectClear, Command}, minecraft::*, core::sel::*, core::Identifier};
# struct DummyPack;
# impl DummyPack {pub fn function(&self, path: Identifier) -> DummyFunc {DummyFunc}}
# struct DummyFunc;
# impl DummyFunc {pub fn run(&self, cmd: impl Command) {}}
# let pack = DummyPack;
let mut func = pack.function(id!(test:func));
func.run(Give{count: 50, ..Give(at_s().tag("foo", true), Item::Dispenser)}); // give @s[tag=foo] dispenser 50
func.run(Clear()); // clear
func.run(EffectClear{effect: Some(Effect::Blindness), ..EffectClear()}); // effect clear @s blindness
```
*/

use std::{fs::File, fs, path::Path};
use std::io::Write;

use crate::{core::{Coordinates, Identifier, Selector, sel::at_s}, score::ScoreValue};
use crate::minecraft::*;

/// A handle to an mcfunction file, created with [`Datapack::function()`](super::Datapack::function)
pub struct Function {
    prefix: String,
    out: File
}
impl Function {
    pub (super) fn new(path: impl AsRef<Path>, id: Identifier<'_, '_>) -> Self {
        let mut functions = path.as_ref().join(id.namespace).join("functions");
        for folder in id.folders {
            functions = functions.join(folder);
        }
        fs::create_dir_all(&functions).unwrap();
        let mut out_path = functions.join(id.id);
        out_path.set_extension("mcfunction");
        Self {
            prefix: String::new(),
            out: File::create(out_path).unwrap()
        }
    }
    /// Run a [`Command`].
    pub fn run(&mut self, cmd: impl Command) {
        write!(self.out, "{}{}", self.prefix, if self.prefix.is_empty() {""} else {" run "}).unwrap();
        cmd.output(&mut self.out);
        writeln!(self.out).unwrap();
    }
    /// Create a [`ScoreValue`], given its name and objective.
    pub fn score<'a>(&mut self, name: &'a str, objective: &'a str) -> ScoreValue<'a> {
        ScoreValue::new(name, objective, self.out.try_clone().unwrap())
    }
}

/// A trait that commands implement
pub trait Command {
    /// Output to a [`Write`]
    fn output(self, out: &mut impl Write);
}

pub use command::*;

#[allow(missing_docs)]
#[doc(inline)]
mod command {
    use super::*;
    /// The `give` command.  
    /// Syntax: `/give <target> <item> <count>`
    pub struct Give<'a> {
        pub target: Selector<'a>,
        pub item: Item,
        pub count: u64
    }
    #[doc(hide)]
    pub fn Give<'a>(target: Selector<'a>, item: Item) -> Give<'a> {
        Give {target, item, count: 1}
    }
    impl Command for Give<'_> {
        fn output(self, out: &mut impl std::io::Write) {
            write!(out, "give {} {}", self.target, self.item).unwrap();
            if self.count != 1 {
                write!(out, " {}", self.count).unwrap();
            }
            write!(out, "\n").unwrap();
        }
    }

    /// The `clear` command.  
    /// Syntax: `clear <target> [<item.0>] [<item.1>]`
    pub struct Clear<'a> {
        pub target: Selector<'a>,
        pub item: Option<(Item, Option<u64>)>
    }
    pub fn Clear<'a>() -> Clear<'a> {
        Clear {
            target: at_s(),
            item: None
        }
    }
    impl Command for Clear<'_> {
        fn output(self, out: &mut impl std::io::Write) {
            write!(out, "clear").unwrap();
            if self.target != at_s() {
                write!(out, " {}", self.target).unwrap();
                if let Some((item, count)) = self.item {
                    write!(out, " {}", item).unwrap();
                    if let Some(count) = count {
                        write!(out, " {}", count).unwrap();
                    }
                }
            }
        }
    }

    /// The `setblock` command.  
    /// Syntax: `setblock <location> <block>`
    pub struct Setblock {
        pub location: Coordinates,
        pub block: Block
    }
    pub fn Setblock(location: Coordinates, block: Block) -> Setblock {
        Setblock {location, block}
    }
    impl Command for Setblock {
        fn output(self, out: &mut impl Write) {
            write!(out, "setblock {} {}", self.location, self.block).unwrap();
        }
    }

    /// The `kill` command.  
    /// Syntax: `kill <target>`
    pub struct Kill<'a> {
        pub target: Selector<'a>
    }
    pub fn Kill<'a>() -> Kill<'a> {
        Kill {target: at_s()}
    }
    impl Command for Kill<'_> {
        fn output(self, out: &mut impl Write) {
            write!(out, "kill").unwrap();
            if self.target != at_s() {
                write!(out, " {}", self.target).unwrap();
            }
        }
    }

    /// The `effect give` subcommand.  
    /// Syntax: `effect give <target> <effect> <seconds> <amplifier> <hide_particles>`
    pub struct EffectGive<'a> {
        pub target: Selector<'a>,
        pub effect: Effect,
        pub seconds: u64,
        pub amplifier: u64,
        pub hide_particles: bool
    }
    pub fn EffectGive<'a>(target: Selector<'a>, effect: Effect) -> EffectGive<'a> {
        EffectGive {target, effect, seconds: 30, amplifier: 1, hide_particles: false}
    }
    impl Command for EffectGive<'_> {
        fn output(self, out: &mut impl Write) {
            write!(out, "effect give {} {}", self.target, self.effect).unwrap();
            let mut variation = 0;
            if self.seconds != 30 {variation = 1};
            if self.amplifier != 0 {variation = 2};
            if self.hide_particles {variation = 3};
            if variation >= 1 {write!(out, " {}", self.seconds).unwrap();}
            if variation >= 2 {write!(out, " {}", self.amplifier).unwrap();}
            if variation >= 3 {write!(out, " {}", self.hide_particles).unwrap();}
        }
    }

    /// The `effect clear` subcommand.  
    /// Syntax: `effect clear <target> [<effect>]`
    pub struct EffectClear<'a> {
        pub target: Selector<'a>,
        pub effect: Option<Effect>
    }
    pub fn EffectClear<'a>() -> EffectClear<'a> {
        EffectClear {target: at_s(), effect: None}
    }
    impl Command for EffectClear<'_> {
        fn output(self, out: &mut impl Write) {
            write!(out, "effect clear").unwrap();
            if self.target != at_s() || self.effect.is_some() {
                write!(out, " {}", self.target).unwrap();
            }
            if let Some(effect) = self.effect {
                write!(out, " {}", effect).unwrap();
            }
        }
    }
}