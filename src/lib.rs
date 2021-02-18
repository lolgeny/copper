#![feature(try_blocks)]
#![deny(rust_2018_idioms, deprecated_in_future, missing_docs)]
#![warn(rustdoc)]
#![allow(missing_doc_code_examples)]

/*! # Copper
Copper is a library to generate minecraft datapacks.
*/
/// Import `copper::prelude::*` to import the most common types.
pub mod prelude;
/// Contains minecraft's definitions
pub mod minecraft;
/// Contains classes used to make datapacks
pub mod datapack;
/// Contains core classes often used in commands/throught datapacks
pub mod core;
/// Contains the `Score` class, which can be used to manipulate scores in a friendly way.
pub mod score;