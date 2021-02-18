/*!
Contains item modifiers.
*/

use serde::{Serialize, Serializer, ser::SerializeMap};



/// A context entity
#[derive(Serialize)]
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

#[doc(hidden)]
pub trait Number: Serialize {}
impl Number for i64 {}
impl Number for f64 {}

/// Represents a score target used in a [`NumberProvider`]
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
