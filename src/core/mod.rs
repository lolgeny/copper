use std::{fmt::{Display, Write}, path::{Path, PathBuf}};
use crate::minecraft::Entity;
use serde::Serialize;

/// Represents an identifier, of the form `namespace:folders.../id`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Identifier<'a, 'b> {
    /// The namespace the identifier is in
    pub namespace: &'a str,
    /// The folders leading up to the target
    pub folders: &'a [&'b str],
    /// The actual id of the identifier
    pub id: &'a str
}
impl<'a, 'b> Identifier<'a, 'b> {
    /// Create an identifier from a namespace and parts.
    /// It's recommended to use the macro [`id!`] in most cases, however.
    pub fn new(namespace: &'a str, parts: &'a [&'b str]) -> Self {
        Self {
            namespace,
            folders: &parts[..parts.len()-1],
            id: parts[parts.len()-1]
        }
    }
    pub (crate) fn join(&self, path: impl AsRef<Path>, folder: &str, extension: &str) -> PathBuf {
        let mut path = path.as_ref().join(self.namespace).join(folder);
        for folder in self.folders {
            path = path.join(folder);
        }
        path = path.join(self.id);
        path.set_extension(extension);
        path
    }
}
impl Serialize for Identifier<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut out = String::from(self.namespace);
        for folder in self.folders {
            write!(out, "{}/", folder).unwrap();
        }
        write!(out, "{}", self.id).unwrap();
        serializer.serialize_str(&out)
    }
}

/// Create an [`Identifier`]. For `minecraft` namespaces, this may be left out.
/// ```
/// # use copper::{id, core::Identifier};
/// # fn main() {
/// assert_eq!(id!(foo:bar/quux), Identifier::new("foo", &["bar", "quux"]));
/// assert_eq!(id!(golden_carrot), Identifier::new("minecraft", &["golden_carrot"]));
/// # }
/// ```
#[macro_export]
macro_rules! id {
    ($namespace:ident : $($part:ident)/+) => {
        $crate::core::Identifier::new(stringify!($namespace), &[$(stringify!($part)),+]);
    };
    ($($part:ident)/+) => {
        id!(minecraft:$($part)/+)
    };
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SelectorType {
    S, P, E, A, R
}
impl Default for SelectorType {
    fn default() -> Self {
        Self::A
    }
}
impl Display for SelectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SelectorType::*;
        write!(f, "{}", match self {
            S => "s",
            P => "p",
            E => "e",
            A => "a",
            R => "r"
        })
    }
}

/// Different sorts used in selectors, e.g `sort=nearest` => `SelectorSort::Nearest`
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SelectorSort {
    #[doc = "Represents `sort=nearest`"] Nearest,
    #[doc = "Represents `sort=furthest`"] Furthest,
    #[doc = "Represents `sort=arbritrary`"] Arbritrary,
    #[doc = "Represents `sort=random`"] Random
}
impl Display for SelectorSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SelectorSort::*;
        write!(f,"{}",match self {
            Nearest => "nearest",
            Furthest => "furthest",
            Arbritrary => "arbritrary",
            Random => "random"
        })
    }
}

/// Represents a game mode, used in selectors.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameMode {
    #[doc = "Represents `gamemode=creative`"] Creative,
    #[doc = "Represents `gamemode=survival`"] Survival,
    #[doc = "Represents `gamemode=spectator`"] Spectator,
    #[doc = "Represents `gamemode=adventure`"] Adventure
}
impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GameMode::*;
        write!(f, "{}", match self {
            Creative => "creative",
            Survival => "survival",
            Spectator => "spectator",
            Adventure => "adventure"
        })
    }
}

/// Represents a selector.
/// Create a selector using one of the `at_` functions, like `at_s()`.
/// Then modify it using the builder pattern. Each attribute for selectors has a method.
/// Since many selector attributes have an optional `!`, these are represented with tuples, the second element being positiveness.
/// Examples:
/// `at_a().tag("foo").game_mode((GameMode::Creative, true))` == `@a[tag=foo,gamemode=creative]`
/// `at_e().type("cow").sort(SelectorSort::Nearest).limit(1)` == `@e[type=cow,sort=nearest,limit=1]`
/// `at_s()` == `@s`
/// `at_a().game_mode((GameMode::Spectator, false))` == `@a[gamemode=!spectator]`
#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Selector<'a> {
    sel: SelectorType,
    #[doc = "Represents `limit=`"] pub limit: Option<u64>,
    #[doc = "Represents `sort=`"] pub sort: Option<SelectorSort>,
    #[doc = "Represents `level=`"] pub level: Option<(u64, u64)>,
    #[doc = "Represents `gamemode=`"] pub game_mode: Option<(GameMode, bool)>,
    #[doc = "Represents `name=`"] pub name: Option<(&'a str, bool)>,
    #[doc = "Represents `x_rotation=`"] pub x_rot: Option<(u64, u64)>,
    #[doc = "Represents `y_rotation=`"] pub y_rot: Option<(u64, u64)>,
    #[doc = "Represents `type=`"] pub ty: Option<(Entity, bool)>,
    #[doc = "Represents `tag=`"] pub tag: Option<(&'a str, bool)>,
    // TODO: Add other complex stuff
}
impl<'a> Selector<'a> {
    fn new(sel: SelectorType) -> Self {
        Self {sel, ..Self::default()}
    }
    /// Sets the `limit` of this selector.
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
    /// Sets the `sort` of this selector.
    pub fn sort(mut self, sort: SelectorSort) -> Self {
        self.sort = Some(sort);
        self
    }
    /// Sets the `level` of this selector.
    pub fn level(mut self, min: u64, max: u64) -> Self {
        self.level = Some((min, max));
        self
    }
    /// Sets the `game_mode` of this selector.
    pub fn game_mode(mut self, game_mode: GameMode, positive: bool) -> Self {
        self.game_mode = Some((game_mode, positive));
        self
    }
    /// Sets the `name` of this selector.
    pub fn name(mut self, name: &'a str, positive: bool) -> Self {
        self.name = Some((name, positive));
        self
    }
    /// Sets the `x_rot` of this selector.
    pub fn x_rot(mut self, min: u64, max: u64) -> Self {
        self.x_rot = Some((min, max));
        self
    }
    /// Sets the `y_rot` of this selector.
    pub fn y_rot(mut self, min: u64, max: u64) -> Self {
        self.y_rot = Some((min, max));
        self
    }
    /// Sets the `entity` of this selector.
    pub fn entity(mut self, ty: Entity, positive: bool) -> Self {
        self.ty = Some((ty, positive));
        self
    }
    /// Sets the `tag` of this selector.
    pub fn tag(mut self, tag: &'a str, positive: bool) -> Self {
        self.tag = Some((tag, positive));
        self
    }
}
impl Display for Selector<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn pos(positive: bool) -> &'static str {
            if positive {""} else {"!"}
        }
        let list_start;
        let list_end;
        if self == &Self::new(self.sel) {
            list_start = ""; list_end = "";
        } else {
            list_start = "["; list_end = "]";
        }
        write!(f, "@{}{}", self.sel, list_start)?;
        if let Some(limit) = self.limit {write!(f,"limit={}",limit)?;}
        if let Some(sort) = self.sort {write!(f,"sort={}",sort)?;}
        if let Some((min, max)) = self.level {write!(f,"level={}..{}",min,max)?;}
        if let Some((mode,positive)) = self.game_mode {write!(f,"gamemode={}{}",pos(positive),mode)?;}
        if let Some((name,positive)) = self.name {write!(f,"name={}{}",pos(positive),name)?;}
        if let Some((min,max)) = self.x_rot {write!(f,"x_rotation={}..{}",min,max)?;}
        if let Some((min,max)) = self.y_rot {write!(f,"y_rotation={}..{}",min,max)?;}
        if let Some((ty,positive)) = self.ty {write!(f,"type={}{}",pos(positive),ty)?;}
        if let Some((tag,positive)) = self.tag {write!(f,"tag={}{}",pos(positive),tag)?;}
        write!(f, "{}", list_end)?;
        Ok(())
    }
}
/// Contains methods to create diferent [`Selector`]s.
pub mod sel {
    use super::{Selector, SelectorType};
    /// Creates an `@s` selector
    pub fn at_s<'a>() -> Selector<'a> {
        Selector::new(SelectorType::S)
    }
    /// Creates an `@p` selector
    pub fn at_p<'a>() -> Selector<'a> {
        Selector::new(SelectorType::P)
    }
    /// Creates an `@e` selector
    pub fn at_e<'a>() -> Selector<'a> {
        Selector::new(SelectorType::E)
    }
    /// Creates an `@a` selector
    pub fn at_a<'a>() -> Selector<'a> {
        Selector::new(SelectorType::A)
    }
    /// Creates an `@r` selector
    pub fn at_r<'a>() -> Selector<'a> {
        Selector::new(SelectorType::R)
    }
}

#[inline]
fn zero(x: f64, f: &mut std::fmt::Formatter<'_>) {if x != 0.0 {write!(f, "{}", x).unwrap();}}

/// Represents a single coordinate, either absolute or relative.
/// Does not contain local coordinates, see [`Coordinates`].
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Coordinate {
    #[doc="Represents an absolute coordinate, e.g `3`"] Absolute(f64),
    #[doc="Represents an absolute coordinate, e.g `~5`"] Relative(f64)
}
impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absolute(x) => write!(f, "{}", x),
            Self::Relative(x) => {let res = write!(f, "~"); zero(*x, f); res}
        }
    }
}

/// Represents a set of coordinates.
/// They may either be mixed, a combination of relative and absolute coordinates, or local.
/// Create coordinates using the [`loc!`] macro (this supports expressions too).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Coordinates {
    /// Represents mixed coordinates, consisting of multiple [`Coordinate`]s
    Mixed(Coordinate, Coordinate, Coordinate),
    /// Represents local coordinates
    Local(f64, f64, f64)
}
impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(x, y, z) => try {
                f.write_char('^')?;
                zero(*x, f);
                f.write_str(" ^")?;
                zero(*y, f);
                f.write_str(" ^")?;
                zero(*z, f);
            },
            Self::Mixed(x, y, z) => write!(f, "{} {} {}", x, y, z)
        }
    }
}

/// Create [`Coordinates`] using the same syntax as minecraft.
/// Currently only literals are supported but expressions should be soon
#[macro_export]
macro_rules! loc {
    (^$x:literal ^$y:literal ^$z:literal) => {$crate::core::Coordinates::Local($x as f64, $y as f64, $z as f64)};
    (^ ^$y:literal ^$z:literal) => {$crate::core::Coordinates::Local(0f64, $y as f64, $z as f64)};
    (^ ^ ^$z:literal) => {$crate::core::Coordinates::Local(0f64, 0f64, $z as f64)};
    (^$x:literal ^ ^$z:literal) => {$crate::core::Coordinates::Local($x as f64, 0f64, $z as f64)};
    (^$x:literal ^ ^) => {$crate::core::Coordinates::Local($x as f64, 0f64, 0f64)};
    (^ ^ ^) => {$crate::core::Coordinates::Local(0f64, 0f64, 0f64)};

    ($x:literal $y:literal $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Absolute($z as f64))};
	($x:literal $y:literal ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Relative($z as f64))};
	($x:literal $y:literal ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Relative(0f64))};
	($x:literal ~$y:literal $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Absolute($z as f64))};
	($x:literal ~$y:literal ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Relative($z as f64))};
	($x:literal ~$y:literal ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Relative(0f64))};
	($x:literal ~ $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Absolute($z as f64))};
	($x:literal ~ ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative($z as f64))};
	($x:literal ~ ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Absolute($x as f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative(0f64))};
	(~$x:literal $y:literal $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Absolute($z as f64))};
	(~$x:literal $y:literal ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Relative($z as f64))};
	(~$x:literal $y:literal ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Relative(0f64))};
	(~$x:literal ~$y:literal $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Absolute($z as f64))};
	(~$x:literal ~$y:literal ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Relative($z as f64))};
	(~$x:literal ~$y:literal ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Relative(0f64))};
	(~$x:literal ~ $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Absolute($z as f64))};
	(~$x:literal ~ ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative($z as f64))};
	(~$x:literal ~ ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative($x as f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative(0f64))};
	(~ $y:literal $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Absolute($z as f64))};
	(~ $y:literal ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Relative($z as f64))};
	(~ $y:literal ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Absolute($y as f64),$crate::core::Coordinate::Relative(0f64))};
	(~ ~$y:literal $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Absolute($z as f64))};
	(~ ~$y:literal ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Relative($z as f64))};
	(~ ~$y:literal ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative($y as f64),$crate::core::Coordinate::Relative(0f64))};
	(~ ~ $z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Absolute($z as f64))};
	(~ ~ ~$z:literal) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative($z as f64))};
	(~ ~ ~) => {$crate::core::Coordinates::Mixed($crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative(0f64),$crate::core::Coordinate::Relative(0f64))};
}

/// Represents a colour
#[derive(Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Color {
    White, Orange, Magenta, LightBlue, Yellow, Lime, Pink, Gray, LightGray, Cyan, Purple, Blue, Brown, Green, Red, Black
}