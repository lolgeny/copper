#![feature(trace_macros)]

use std::path::Path;

use copper::{core::GameMode, minecraft::Effect, prelude::*};

#[test]
pub fn test() {
    let pack = Datapack::new(Path::new(".").join("out").join("Test"));
    let mut foo = pack.function(id!(test:foo));
    foo.give(at_a(), Item::Dirt, 1);
    foo.give(at_s().tag("foo", true), Item::Dispenser, 50);
    foo.setblock(loc!(~0 ~0 ~0), Block::DiamondBlock);
    foo.setblock(loc!(^ ^ ^5), Block::Air);
    foo.kill(Some(at_a().level(3, 5)));
    foo.kill(None);
    foo.clear(at_s(), None);
    foo.clear(at_a().game_mode(GameMode::Survival, true), Some((Item::Stone, Some(3))));
    foo.effect_give(at_s(), Effect::Regeneration, 1000000, 255, true);
    foo.effect_give(at_a(), Effect::Blindness, 30, 0, false);
    let mut x = foo.score("#x", "global");
    let y = foo.score("#y", "global").set_to(5);
    x = x + 5 - &y;
}