#![allow(missing_docs)]

use std::fmt::Display;

include!(concat!(env!("OUT_DIR"), "/blocks.rs"));
include!(concat!(env!("OUT_DIR"), "/items.rs"));
include!(concat!(env!("OUT_DIR"), "/entity.rs"));
include!(concat!(env!("OUT_DIR"), "/effect.rs"));