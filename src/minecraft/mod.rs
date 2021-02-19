#![allow(missing_docs)]

use std::fmt::Display;
use serde::Serialize;

include!(concat!(env!("OUT_DIR"), "/blocks.rs"));
include!(concat!(env!("OUT_DIR"), "/items.rs"));
include!(concat!(env!("OUT_DIR"), "/entity.rs"));
include!(concat!(env!("OUT_DIR"), "/effect.rs"));
include!(concat!(env!("OUT_DIR"), "/enchant.rs"));
include!(concat!(env!("OUT_DIR"), "/structures.rs"));
include!(concat!(env!("OUT_DIR"), "/potions.rs"));