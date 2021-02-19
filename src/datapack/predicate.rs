/*!
Contains the [`Predicate`] enum.
Variants are passed to a datapack via [`Datapack::predicate`](crate::datapack::Datapack::predicate).

Note, most predicates use configuration, i.e what you might see in the wiki as "tags common to all ...".
These are represented as structs in this module, each of which implements default.
So when using them, you can just add `..default()` to the end. For example:
```
# use copper::prelude::*;
# use copper::datapack::predicate::*;
EntityPredicate {
    distance: Some(DistancePredicate {
        horizontal: Some(Range {
            min: 0.0,
            max: 10.0
        }),
        ..default()
    }),
    equipment: Some(EquipmentPredicate {
        mainhand: Some(ItemPredicate {
            count: Some(OptionalRange::Exact(32)),
            ..default()
        }),
        ..default()
    }),
    ..default()
};
```
*/
use serde::{Serialize, Serializer, ser::SerializeMap};
use crate::{core::{GameMode, Identifier, TupleMapSerializer}, minecraft::*};
use crate::core::serialize_tuple_map;

use super::item_modifier::{Number, NumberProvider, PlayerContextEntity};

/// Represents a range between 2 numbers
#[derive(Serialize, PartialEq, Clone)]
pub struct Range<N: Number> {
    /// The minimum value
    pub min: N,
    /// The maximum value
    pub max: N
}

/// Represents an optional range; an exact number can be used instead
#[derive(Serialize)]
#[serde(untagged)]
pub enum OptionalRange<N: Number> {
    /// Matches an exact number
    Exact(N),
    /// Matches a range of numbers
    Range(Range<N>)
}

macro_rules! config_struct {
    ($(struct $name:ident $(<$($life:lifetime),+>)? where $structdoc:literal {
        $($({$serializer:literal})? $([$rename:literal])? $field:ident : $ty:ty where $doc:literal),*
    })+) => {
        $(
            #[derive(Default, Serialize)]
            #[doc = $structdoc]
            pub struct $name $(<$($life),+>)? {
                $(
                    #[doc = $doc]
                    #[serde(skip_serializing_if = "Option::is_none" $(, serialize_with = $serializer)? $(, rename = $rename)?)]
                    pub $field : Option<$ty>
                ),*
            }
        )+
    };
}

config_struct! {
    struct DamagePredicate<'a, 'b> where "A predicate for checking damage sources" {
        bypasses_armor: bool where "Checks if the damage bypassed the armor of the player (e.g suffocation)",
        bypasses_invulnerability: bool where "Checks if the damage bypassed invulnerability (e.g `/kill`)",
        bypasses_magic: bool where "Checks if the damage was caused by starvation",
        is_explosion: bool where "Checks if the damage originated from an explosion",
        is_fire: bool where "Checks if the damage originated from fire",
        is_magic: bool where "Checks if the damage originated from magic",
        is_projectile: bool where "Checks if the damage originated from a projectile",
        is_lightning: bool where "Checks if the damage originated from lightning",
        direct_entity: EntityPredicate<'a, 'b> where "The entity that was the direct cause of the damage",
        source_entity: EntityPredicate<'a, 'b> where "Checks the entity that was the source of the damage (for example: The skeleton that shot the arrow)"
    }
    struct DistancePredicate where "A predicate for checking distances" {
        absolute: Range<f64> where "The absolute distance",
        horizontal: Range<f64> where "The horizontal distance",
        x: Range<f64> where "The distance in x",
        y: Range<f64> where "The distance in y",
        z: Range<f64> where "The distance in z"
    }
    struct EffectPredicate where "A predicate for checking active effects" {
        ambient: bool where "Whether the effect was from a beacon",
        amplifier: OptionalRange<i64> where "The effect amplifier",
        duration: OptionalRange<i64> where "The effect duration in ticks",
        visible: bool where "Whether the effect has visible particles"
    }
    struct EquipmentPredicate<'a, 'b> where "Checks an entity's equipment" {
        mainhand: ItemPredicate<'a, 'b> where "Checks the item in the entity's mainhand",
        offhand: ItemPredicate<'a, 'b> where "Checks the item in the entity's offhand",
        head: ItemPredicate<'a, 'b> where "Checks the item in the entity's head",
        chest: ItemPredicate<'a, 'b> where "Checks the item in the entity's chest",
        legs: ItemPredicate<'a, 'b> where "Checks the item in the entity's legs",
        feet: ItemPredicate<'a, 'b> where "Checks the item in the entity's feet"
    }
    struct EntityFlags where "Certain flags to check on an entity" {
        is_on_fire: bool where "Tests whether the entity is on fire",
        is_sneaking: bool where "Tests whether the entity is sneaking",
        is_sprinting: bool where "Tests whether the entity is sprinting",
        is_swimming: bool where "Tests whether the entity is swimming",
        is_baby: bool where "Tests whether the entity is a baby variant"
    }
    struct PlayerPredicate<'a, 'b> where "Checks properties of a player" {
        {"serialize_advancements"} advancements: &'a [(Identifier<'b, 'b>, AdvancementPredicate<'b, 'b>)]
            where "A list of advancements in the form `(name, predicate)`",
        gamemode: GameMode where "The gamemode of the player",
        level: OptionalRange<i64> where "The experience level of the player",
        {"serialize_tuple_map"} recipes: &'a [(Identifier<'b, 'b>, bool)] where "A map of recipes to check",
        stats: &'a [StatisticPredicate<'b, 'b>] where "List of statistics to match"
    }
    struct EntityPredicate<'a, 'b> where "A predicate for checking entities" {
        distance: DistancePredicate where "The distance between the target entity and the location",
        {"serialize_tuple_map"} effects: &'a [(Effect, EffectPredicate)] where "A list of status effects",
        equipment: EquipmentPredicate<'a, 'b> where "Equipment to check on the entity",
        flags: EntityFlags where "Predicate flags to be checked",
        location: LocationPredicate<'a, 'b> where "Checks the entity's location",
        nbt: &'a str where "Checks the entity's nbt",
        player: PlayerPredicate<'a, 'b> where "Player properties to check. Fails if the entity is not a player",
        team: &'a str where "The team the entity belongs to",
        ["type"] ty: Entity where "The entity's type",
        targeted_entity: Box<EntityPredicate<'a, 'b>> where "The entity which this entity is targeting for attacks",
        vehicle: Box<EntityPredicate<'a, 'b>> where "The vehicle that this entity is riding on"
    }
    struct ItemPredicate<'a, 'b> where "A predicate for checking items" {
        count: OptionalRange<i64> where "Amount of the item",
        durability: OptionalRange<i64> where "The item's durability",
        enchantments: &'a [EnchantmentPredicate] where "List of enchantments",
        stored_enchantments: &'a [EnchantmentPredicate] where "List of stored enchantments (i.e an enchanted book)",
        item: Item where "An item id",
        nbt: &'a str where "An nbt string",
        potion: Potion where "A potion id",
        tag: Identifier<'a, 'b> where "An item tag"
    }
    struct BlockPredicate<'a, 'b> where "Checks a block" {
        block: Block where "The block to check",
        tag: Identifier<'a, 'b> where "A block tag",
        nbt: &'a str where "The block nbt",
        {"serialize_tuple_map"} state: &'a [(&'b str, BlockstateValue<'b>)] where "Block states to check"
    }
    struct FluidPredicate<'a, 'b> where "Checks a fluid" {
        fluid: Identifier<'a, 'b> where "The fluid to check",
        tag: Identifier<'a, 'b> where "A block tag",
        {"serialize_tuple_map"} state: &'a [(&'b str, BlockstateValue<'b>)] where "Block (fluid) states to check"
    }
    struct PositionPredicate where "Checks a position" {
        x: OptionalRange<i64> where "Tests the x",
        y: OptionalRange<i64> where "Tests the y",
        z: OptionalRange<i64> where "Tests the z"
    }
    struct LocationPredicate<'a, 'b> where "Checks a location" {
        biome: Identifier<'a, 'b> where "The biome the location is in",
        block: BlockPredicate<'a, 'b> where "The block at the location",
        dimension: Identifier<'a, 'b> where "The dimension the entity is in",
        feature: Structure where "Tests for a structure",
        fluid: FluidPredicate<'a, 'b> where "The fluid at the location",
        light: OptionalRange<i64> where "The light at the location (calculated via `(max(sky-darkening,block))`)",
        position: PositionPredicate where "Tests the position",
        smokey: bool where "True if the block is closely above a campfire or soul campfire"        
    }
}

/// A predicate for checking advancements
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvancementPredicate<'a, 'b> {
    /// Checks if the whole advancement is complete
    Complete(bool),
    /// Checks each criteria to see if it's complete, in the form `(criterion, completeness)`
    Criteria(&'a [(&'b str, bool)])
}

fn serialize_advancements<S>(advancements: &Option<&[(Identifier<'_, '_>, AdvancementPredicate<'_, '_>)]>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut map = serializer.serialize_map(None)?;
        for (name, pred) in advancements.unwrap() {
            match pred {
                AdvancementPredicate::Complete(positive) =>
                    map.serialize_entry(name, positive)?,
                AdvancementPredicate::Criteria(criteria) =>
                    map.serialize_entry(name, &TupleMapSerializer(*criteria))?
            }
        }
        map.end()
}

/// A predicate to check statistics against
#[derive(Serialize)]
pub struct StatisticPredicate<'a, 'b> {
    /// The statistic type (e.g `minecraft:custom`)
    #[serde(rename = "type")] pub ty: Identifier<'a, 'b>,
    /// The statistic id for this type
    pub stat: Identifier<'a, 'b>,
    /// The value to check for the statistic
    pub value: OptionalRange<i64>
}

/// A predicate to check an enchantment
#[derive(Serialize)]
pub struct EnchantmentPredicate {
    /// The enchantment to check
    pub enchantment: Enchant,
    /// The levels to allow
    pub levels: OptionalRange<i64>
}

/// Represents a block state value
#[derive(Serialize)]
#[serde(untagged)]
pub enum BlockstateValue<'a> {
    /// Represents a boolean block state value (like `open`)
    Bool(bool),
    /// Represents an int block state value (like `age`)
    Int(i64),
    /// Represents a string block state value (like `facing`)
    Str(&'a str)
}

/// A predicate. Use [`Datapack::predicate`](crate::datapack::Datapack::predicate).
#[derive(Serialize)]
#[serde(tag = "condition", rename_all = "snake_case")]
pub enum Predicate<'a, 'b> {
    /// Joins conditions with or
    Alternative {
        /// A list of conditions to join
        terms: &'a [Predicate<'b, 'b>]
    },
    /// Check properties of a blocks state
    BlockStateProperty {
        /// Test fails if this block doesn't match
        block: Block,
        /// Properties to test against, in the form `(property, expected value)`
        #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_tuple_map")]
        properties: Option<&'a [(&'b str, &'b str)]>
    },
    /// Check properties of the damage source
    DamageSourceProperties {
        /// The predicate to check on the damage source
        predicate: DamagePredicate<'a, 'b>
    },
    /// Test properties of an entity
    EntityProperties {
        /// The entity to test
        entity: PlayerContextEntity,
        /// Predicate applied to the entity
        predicate: EntityPredicate<'a, 'b>
    },
    /// Test an entity's scores
    EntityScores {
        /// The entity to test
        entity: PlayerContextEntity,
        /// The scores to test, in the form `(objective, value)`
        scores: &'a [(&'b str, OptionalRange<NumberProvider<'b, i64>>)]
    },
    /// Inverts a predicate
    Inverted {
        /// The term to be negated
        term: Box<Predicate<'a, 'b>>
    },
    /// Tests the presence of a `killer_player`
    KilledByPlayer {
        /// If true, test the absence
        #[serde(skip_serializing_if = "std::ops::Not::not")] inverse: bool
    },
    /// Tests the current location
    LocationCheck {
        /// An optional offset in the x direction
        #[serde(skip_serializing_if = "Option::is_none", rename = "offsetX")] offset_x: Option<i64>,
        /// An optional offset in the y direction
        #[serde(skip_serializing_if = "Option::is_none", rename = "offsetY")] offset_y: Option<i64>,
        /// An optional offset in the z direction
        #[serde(skip_serializing_if = "Option::is_none", rename = "offsetZ")] offset_z: Option<i64>,
        /// The predicate to be applied to the location
        predicate: LocationPredicate<'a, 'b>
    },
    /// Checks the tool
    MatchTool {
        /// The predicate to check the tool with
        predicate: ItemPredicate<'a, 'b>
    },
    /// Tests if a random number between 0 and 1 is less than the specified value
    RandomChance {
        /// Success rate
        chance: f64
    },
    /// Test if a random number 0.0–1.0 is less than a specified value, affected by the level of Looting on the `killer` entity.
    RandomChanceWithLooting {
        /// Base success rate
        chance: f64,
        /// Looting adjustment to the base success rate. Formula is chance + `(looting_level * looting_multiplier)`
        looting_multiplier: f64
    },
    /// Tests if another predicate passes
    Reference {
        /// The id of the predicate to test
        name: Identifier<'a, 'b>
    },
    /// Returns true with `1/explosion radius` probability
    SurvivesExplosion,
    /// Passes with probability picked from table, indexed by enchantment level.
    TableBonus {
        /// Id of enchantment
        enchantment: i64,
        /// List of probabilities for enchantment level, indexed from 0.
        chances: &'a [f64]
    },
    /// Checks the current time
    TimeCheck {
        /// The time value in ticks
        value: OptionalRange<NumberProvider<'a, i64>>,
        /// If present, time gets modulo-divided by this value
        /// (for example, if set to 24000, value operates on a time period of daytime ticks just like /time query daytime).
        #[serde(skip_serializing_if = "Option::is_none")] period: Option<i64>
    },
    /// Checks for a current weather state
    WeatherCheck {
        /// Tests if it's raining
        #[serde(skip_serializing_if = "Option::is_none")] raining: Option<bool>,
        /// Tests if it's thundering
        #[serde(skip_serializing_if = "Option::is_none")] thundering: Option<bool>
    },
    /// Checks a value
    ValueCheck {
        /// The value to test
        value: NumberProvider<'a, i64>,
        /// The range to check against
        range: OptionalRange<NumberProvider<'a, i64>>
    }
}