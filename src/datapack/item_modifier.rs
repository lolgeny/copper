/*!
Contains the [`ItemModifier`] enum.
Variants are passed to a datapack via [`Datapack::item_modifier`](crate::datapack::Datapack::item_modifier).
*/

use serde::{Serialize, Serializer, ser::SerializeMap};

use crate::{core::{Color, Identifier}, minecraft::*};


/// A general context entity
#[derive(Serialize, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ContextEntity {
    /// Represents "this" entity
    This,
    /// Represents the "killer" entity
    Killer,
    /// Represents the "direct_killer" entity
    DirectKiller,
    /// Represents the "player_killer" entity
    PlayerKiller
}

/// A context entity for nbt use
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NbtContextEntity {
    /// Represents "this" entity
    This,
    /// Represents the "killer" entity
    Killer,
    /// Represents the "killer_player" entity
    KillerPlayer,
    /// Represents the "block_entity" entity
    BlockEntity
}

/// A context entity for nbt use
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerContextEntity {
    /// Represents "this" entity
    This,
    /// Represents the "killer" entity
    Killer,
    /// Represents the "killer_player" entity
    KillerPlayer
}


#[doc(hidden)]
pub trait Number: Serialize + PartialEq + Clone {}
impl Number for i64 {}
impl Number for f64 {}
impl<N: Number> Number for NumberProvider<'_, N> {}

/// Represents a score target used in a [`NumberProvider`]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScoreTarget<'a> {
    /// Represents a fixed name
    Fixed(&'a str),
    /// Represents a context entity's score
    Context(ContextEntity)
}
impl Serialize for ScoreTarget<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        match self {
            Self::Context(context) => context.serialize(serializer),
            Self::Fixed(name) => {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("type", "fixed")?;
                map.serialize_entry("name", name)?;
                map.end()
            }
        }
    }
}

/// A number provider. This is implemented for `f64`, `i64`, and other providers defined in this module.
#[derive(Clone, PartialEq)]
pub enum NumberProvider<'a, N: Number> {
    /// A constant number provider, `{"type": "constant"}`
    Constant (N),
    /// A uniformly random number provider, `{"type": "uniform"}`
    Uniform {
        /// The minimum value to choose
        min: Box<NumberProvider<'a, N>>,
        /// The maximum value to choose
        max: Box<NumberProvider<'a, N>>
    },
    /// A binomially random number provider, `{"type": "binomial"}`
    Binomial {
        /// The number of trials
        n: Box<NumberProvider<'a, i64>>,
        /// The probability of success of an induvidual trial
        p: Box<NumberProvider<'a, f64>>
    },
    /// A score number provider, `{"type": "score"}`
    Score {
        /// The score's target
        target: ScoreTarget<'a>,
        /// The score objective
        score: &'a str,
        /// The scale to multiply the score by
        scale: f64
    }
}
impl<N: Number> Serialize for NumberProvider<'_, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        use NumberProvider::*;
        if let Constant(n) = self {
            n.serialize(serializer)
        } else {
            let mut map = serializer.serialize_map(None)?;
            match self {
                Uniform{min, max} => {
                    map.serialize_entry("type", "uniform")?;
                    map.serialize_entry("min", min)?;
                    map.serialize_entry("max", max)?;
                }
                Binomial{n, p} => {
                    map.serialize_entry("type", "binomial")?;
                    map.serialize_entry("n", n)?;
                    map.serialize_entry("p", p)?;
                }
                Score {target, score, scale} => {
                    map.serialize_entry("type", "score")?;
                    map.serialize_entry("target", target)?;
                    map.serialize_entry("score", score)?;
                    if scale.ne(&1.0) {
                        map.serialize_entry("scale", scale)?;
                    }
                }
                Constant(..) => unreachable!()
            }
            map.end()
        }
    }
}

/// A formula for an apply bonus item modifier.
#[allow(missing_docs)]
pub enum ApplyBonusFormula {
    /// Binomial Distribution (`n = level + extra`, `p = probability`)
    BinomialWithBonusCount {extra: i64, probability: f64},
    /// Uniform distribution (from 0 to `level*bonusMultiplier`)
    UniformBonusCount {bonus_multiplier: f64},
    /// Special function for ore drops
    OreDrops
}
impl Serialize for ApplyBonusFormula {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut map = serializer.serialize_map(None)?;
        match self {
            ApplyBonusFormula::BinomialWithBonusCount { extra, probability } => {
                map.serialize_entry("extra", extra)?;
                map.serialize_entry("probability", probability)?;
            }
            ApplyBonusFormula::UniformBonusCount { bonus_multiplier} => {
                map.serialize_entry("bonusMultiplier", bonus_multiplier)?;
            }
            ApplyBonusFormula::OreDrops => {}
        }
        map.end()
    }
}
fn serialize_apply_bonus<S: Serializer>(enchantment: &Enchant, formula: &ApplyBonusFormula, serializer: S) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(None).unwrap();
    map.serialize_entry("enchantment", enchantment)?;
    map.serialize_entry("formula", match formula {
        ApplyBonusFormula::BinomialWithBonusCount{..} => "binomial_with_bonus_count",
        ApplyBonusFormula::UniformBonusCount{..} => "uniform_bonus_count",
        ApplyBonusFormula::OreDrops => "ore_drops",
    })?;
    map.serialize_entry("parameters", formula)?;
    map.end()
}

fn serialize_copy_name<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(None)?;
    map.serialize_entry("source", "block_entity")?;
    map.end()
}

/// The source for a copy nbt item modifier.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyNbtSource<'a, 'b> {
    /// Use an nbt storage
    Storage {
        /// The path to the storage
        source: Identifier<'a, 'b>
    },
    /// Use nbt from one of the context's entities
    Context {
        /// The entity to copy from
        target: NbtContextEntity
    }
}

/// Represents an nbt operation for a copy nbt item modifier
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyNbtOperationType {
    /// Replace existing contents of target
    Replace,
    /// Append to a list
    Append,
    /// Merge into a compound tag
    Merge
}

/// Copies nbt to the item's `tag` tag
#[derive(Serialize)]
pub struct CopyNbtOperation<'a> {
    /// The nbt path to copy from
    pub source: &'a str,
    /// The nbt path to copy to, starting at the item's `tag` tag.
    pub target: &'a str,
    /// The operation to do
    pub op: CopyNbtOperationType
}

/// A range between 2 ints, used by [`LimitCountRange`](ItemModifier::LimitCountRange)
#[derive(Serialize)]
pub struct Range<'a> {
    /// The minimum value
    pub min: NumberProvider<'a, i64>,
    /// The maximum value
    pub max: NumberProvider<'a, i64>
}

/// An attribute operation
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeOperation {
    /// Adds the amount to the base value
    Addition,
    /// Multiplies the amount to the base
    MultiplyBase,
    /// Multiplies with the total
    MultiplyTotal
}

/// A slot to apply an attribute modifier to
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum AttributeSlot {
    Mainhand, Offhand, Legs, Feet, Chest, Head
}

/// An attribute modifier, used by [`SetAttributes`](ItemModifier::SetAttributes)
#[derive(Serialize)]
pub struct AttributeModifier<'a> {
    /// The name of the modifier
    pub name: &'a str,
    /// The name of the attribute to act upon
    pub attribute: &'a str,
    /// The operation to do
    pub operation: AttributeOperation,
    /// The amount of the modifier
    pub amount: NumberProvider<'a, f64>,
    /// The uuid to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// The slot to apply the attribute to
    pub slot: AttributeSlot
}

/// A banner pattern, used by [`SetBannerPattern`](ItemModifier::SetBannerPattern)
#[derive(Serialize)]
pub struct BannerPattern<'a> {
    /// The pattern type
    pub pattern: &'a str,
    /// The colour of the pattern
    pub color: Color
}

fn serialize_set_enchantments<'a, S: Serializer>
(enchantments: &&[(Enchant, NumberProvider<'a, i64>)], add: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    struct KeyEnchantments<'a, 'b> (&'a [(Enchant, NumberProvider<'b, i64>)]);
    impl Serialize for KeyEnchantments<'_, '_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut map = serializer.serialize_map(None)?;
            for (enchant, level) in self.0 {
                map.serialize_entry(enchant, level)?;
            }
            map.end()
        }
    }
    let mut map = serializer.serialize_map(None)?;
    map.serialize_entry("enchantments", &KeyEnchantments(*enchantments))?;
    if *add { map.serialize_entry("add", add)?; }
    map.end()
}

/// A status effect, used in [`SetStewEffect`](ItemModifier::SetStewEffect)
#[derive(Serialize)]
pub struct StatusEffect<'a> {
    /// The effect to use
    pub ty: Effect,
    /// The duration of the effect
    pub duration: NumberProvider<'a, i64>
}

/// An item modifier. Use [`DataPack::item_modifier`](crate::datapack::Datapack::item_modifier)
#[derive(Serialize)]
#[serde(tag = "function")]
pub enum ItemModifier<'a, 'b> {
    /// Apply a bonus enchantment to the item
    #[serde(serialize_with = "serialize_apply_bonus")]
    ApplyBonus {
        /// Enchantment used for level calculation
        enchantment: Enchant,
        /// Formula to use
        formula: ApplyBonusFormula
    },
    /// Copy the item's name from a block entity
    #[serde(serialize_with = "serialize_copy_name")]
    CopyName,
    /// Copy nbt from a context entity/storage to the item
    CopyNbt {
        /// Specifies the source of the nbt
        source: CopyNbtSource<'a, 'b>,
        /// List of copy operations to do
        ops: &'a [CopyNbtOperation<'b>]
    },
    /// Copies state from dropped block to the item's `BlockStateTag` tag
    CopyState {
        /// The block which has these states; fails if it doesn't match
        block: Block,
        /// A list of property names to copy
        properties: &'a [&'b str]
    },
    /// Enchants the item with one randomly-selected enchantment. The level of the enchantment, if applicable, is random.
    EnchantRandomly {
        /// List of enchantments to choose from. If omitted, all enchantments equippable to the item are possible.
        #[serde(skip_serializing_if = "Option::is_none")]
        enchantments: Option<&'a [Enchant]>
    },
    /// Enchants the item, with the specified enchantment level (roughly equivalent to using an enchantment table at that level).
    EnchantWithLevels {
        /// Determines whether treausre enchantments are allowed on this item
        treasure: bool,
        /// Specifices the exact enchantment level to use
        levels: NumberProvider<'a, i64>
    },
    /// Converts an empty map into an explorer map leading to a nearby generated structure.
    ExplorationMap {
        /// The type of generated structure to locate
        destination: Structure,
        /// The icon used to mark the destination on the map
        decoration: &'a str,
        /// The zoom level on the resulting map
        zoom: i64,
        /// The size, in chunks, of the area to search for structures.
        /// The area checked is square, not circular.
        /// Radius 0 causes only the current chunk to be searched,
        /// radius 1 causes the current chunk and eight adjacent chunks to be searched, and so on.
        search_radius: i64,
        /// Don't search in chunks that have already been generated
        skip_existing_chunks: bool
    },
    /// For loot tables of type 'block', removes some items from a stack, if there was an explosion.
    /// Each item has a chance of 1/explosion radius to be lost.
    ExplosionDecay,
    /// Smelts the item as it would be in a furnace.
    /// Used in combination with the entity_properties condition to cook food from animals on death.
    FurnaceSmelt,
    /// Adds required item tags of a player head
    FillPlayerHead {
        /// The player to set the head from
        entity: PlayerContextEntity
    },
    /// Limits the count of every item stack to an exact number
    #[serde(rename = "limit_count")]
    LimitCountExact {
        /// The number to limit the stack size to
        limit: NumberProvider<'a, i64>
    },
    /// Limits the count of every item stack to a range
    #[serde(rename = "limit_count")]
    LimitCountRange {
        /// The range to limit the stack size to
        limit: Range<'a>
    },
    /// Adjusts the stack size based on the level of the Looting enchantment on the killer entity.
    LootingEnchant {
        /// Specifies the number of additional items per level of looting.
        /// Note the number may be fractional, rounded after multiplying by the looting level.
        count: NumberProvider<'a, i64>,
        /// Specifies the maximum amount of items in the stack after the looting calculation.
        /// If the value is 0, no limit is applied.
        limit: i64
    },
    /// Add attribute modifiers to the item
    SetAttributes {
        /// A list of modifiers to add
        modifiers: &'a [AttributeModifier<'b>]
    },
    /// Sets tags needed for banner patterns
    SetBannerPattern {
        /// Whether to add patterns to existing ones
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        append: bool,
        /// A list of patterns to set
        patterns: &'a [BannerPattern<'b>]
    },
    /// For loot tables of type 'block', sets the contents of a container block item to a list of entries.
    SetContents {
        /// The entries to use as contents
        entries: &'a [&'b str]
    },
    /// Sets the stack size
    SetCount {
        /// Specifies the stack size to set
        count: NumberProvider<'a, i64>,
        /// If true, change will be relative to the current count
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        add: bool
    },
    /// Sets the item's damage value (durability) for tools.
    SetDamage {
        /// Specifies the damage fraction to set
        damage: NumberProvider<'a, f64>,
        /// If true, change will be relative to the current damage
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        add: bool
    },
    /// Sets the item's enchantments
    #[serde(serialize_with = "serialize_set_enchantments")]
    SetEnchantments {
        /// The list of enchantments to change
        enchantments: &'a [(Enchant, NumberProvider<'b, i64>)],
        /// If true, change will be relative to the current level
        add: bool
    },
    /// Sets the loot table for a container
    SetLootTable {
        /// The loot table to use
        name: Identifier<'a, 'b>,
        /// The seed to use (if omitted, generate a random seed)
        #[serde(skip_serializing_if = "Option::is_none")]
        seed: Option<i64>
    },
    /// Adds lore to the item
    SetLore {
        /// A list of JSON components that make up the lore
        // TODO: port this to use a proper typed struct
        lore: &'a [&'b str],
        /// The entity to use as `@s` in the lore
        entity: PlayerContextEntity,
        /// Whether to add these lines to the existing lore
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        replace: bool
    },
    /// Adds display name of the item
    SetName {
        /// A JSON name
        name: &'a str,
        /// The entity to use as `@s` in the lore
        entity: PlayerContextEntity
    },
    /// Adds nbt data to the item
    SetNbt {
        /// The tag to add
        tag: &'a str
    },
    /// Sets the status effects for suspicious stew
    SetStewEffect {
        /// The effects to apply
        effects: &'a [StatusEffect<'b>]
    }
}