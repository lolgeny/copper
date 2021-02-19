pub use crate::datapack::Datapack;
pub use crate::core::{Identifier, sel::*};
pub use crate::minecraft::{Block, Item, Entity};
pub use crate::id;
pub use crate::loc;
/// Stable shorthand for `Default::default`. Useful in lots of datapack config structs.
pub fn default<T: Default>() -> T {Default::default()}