use std::{fmt::{Display, Write}, path::{Path, PathBuf}};
use crate::minecraft::Entity;

pub struct Identifier<'a, 'b> {
    pub namespace: &'a str,
    pub folders: &'a [&'b str],
    pub id: &'a str
}
impl<'a, 'b> Identifier<'a, 'b> {
    pub fn new(namespace: &'a str, parts: &'a [&'b str]) -> Self {
        Self {
            namespace,
            folders: &parts[..parts.len()-1],
            id: parts[parts.len()-1]
        }
    }
    pub fn join(&self, path: impl AsRef<Path>) -> PathBuf {
        let mut path = path.as_ref().join(self.namespace);
        for part in self.folders {
            path = path.join(part);
        };
        path
    }
}

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
pub enum SelectorType {
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SelectorSort {
    Nearest, Furthest, Arbritrary, Random
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameMode {
    Creative, Survival, Spectator, Adventure
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
#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Selector<'a> {
    sel: SelectorType,
    pub limit: Option<u64>,
    pub sort: Option<SelectorSort>,
    pub level: Option<(u64, u64)>,
    pub game_mode: Option<(GameMode, bool)>,
    pub name: Option<(&'a str, bool)>,
    pub x_rot: Option<(u64, u64)>,
    pub y_rot: Option<(u64, u64)>,
    pub ty: Option<(Entity, bool)>,
    pub tag: Option<(&'a str, bool)>,
    // TODO: Add other complex stuff
}
impl<'a> Selector<'a> {
    fn new(sel: SelectorType) -> Self {
        let mut this = Self::default();
        this.sel = sel;
        this
    }
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn sort(mut self, sort: SelectorSort) -> Self {
        self.sort = Some(sort);
        self
    }
    pub fn level(mut self, min: u64, max: u64) -> Self {
        self.level = Some((min, max));
        self
    }
    pub fn game_mode(mut self, game_mode: GameMode, positive: bool) -> Self {
        self.game_mode = Some((game_mode, positive));
        self
    }
    pub fn name(mut self, name: &'a str, positive: bool) -> Self {
        self.name = Some((name, positive));
        self
    }
    pub fn x_rot(mut self, min: u64, max: u64) -> Self {
        self.x_rot = Some((min, max));
        self
    }
    pub fn y_rot(mut self, min: u64, max: u64) -> Self {
        self.y_rot = Some((min, max));
        self
    }
    pub fn entity(mut self, ty: Entity, positive: bool) -> Self {
        self.ty = Some((ty, positive));
        self
    }
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
pub mod sel {
    use super::{Selector, SelectorType};
    pub fn at_s<'a>() -> Selector<'a> {
        Selector::new(SelectorType::S)
    }
    pub fn at_p<'a>() -> Selector<'a> {
        Selector::new(SelectorType::P)
    }
    pub fn at_e<'a>() -> Selector<'a> {
        Selector::new(SelectorType::E)
    }
    pub fn at_a<'a>() -> Selector<'a> {
        Selector::new(SelectorType::A)
    }
    pub fn at_r<'a>() -> Selector<'a> {
        Selector::new(SelectorType::R)
    }
}

#[inline]
fn zero(x: f64, f: &mut std::fmt::Formatter<'_>) {if x != 0.0 {write!(f, "{}", x).unwrap();}}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Coordinate {
    Absolute(f64),
    Relative(f64)
}
impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absolute(x) => write!(f, "{}", x),
            Self::Relative(x) => {let res = write!(f, "~"); zero(*x, f); res}
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Coordinates {
    Mixed(Coordinate, Coordinate, Coordinate),
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