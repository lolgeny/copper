#![feature(trace_macros, box_syntax)]

use std::path::Path;

use copper::{core::GameMode, datapack::{function::*, item_modifier::{ContextEntity, NumberProvider, ScoreTarget}}, minecraft::Effect, prelude::*};

#[test]
pub fn test() {
    let pack = Datapack::new(Path::new(".").join("out").join("Test"));
    let mut foo = pack.function(id!(test:foo));
    foo.run(Give(at_a(), Item::Dirt));
    foo.run(Give{count: 50, ..Give(at_s().tag("foo", true), Item::Dispenser)});
    foo.run(Setblock(loc!(~0 ~0 ~0), Block::DiamondBlock));
    foo.run(Setblock(loc!(^ ^ ^5), Block::Air));
    foo.run(Kill{target: at_a().level(3, 5)});
    foo.run(Kill());
    foo.run(Clear());
    foo.run(Clear{target: at_a().game_mode(GameMode::Survival, true), item: Some((Item::Stone, Some(3)))});
    foo.run(EffectGive{target: at_s(), effect: Effect::Regeneration, seconds: 1000000, amplifier: 255, hide_particles: true});
    foo.run(EffectGive(at_a(), Effect::Blindness));
    foo.run(EffectGive{amplifier: 3, ..EffectGive(at_s(), Effect::Absorption)});
    foo.run(EffectClear());
    foo.run(EffectClear{effect: Some(Effect::Blindness), ..EffectClear()});
    let mut x = foo.score("#x", "global");
    let y = foo.score("#y", "global").set_to(5);
    x = x + 5 - &y;

    let num = NumberProvider::Score::<i64> {
        target: ScoreTarget::Fixed("foo"),
        score: "bar",
        scale: 1.0
    };
    println!("{}", serde_json::to_string(&num).unwrap());
}