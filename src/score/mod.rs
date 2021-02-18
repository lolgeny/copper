use std::{fs::File, io::Write, ops::*};

/// A representation of a scoreboard value
/// Create using `Datapack::score()`
pub struct ScoreValue<'a> {
    objective: &'a str,
    name: &'a str,
    out: File
}
impl<'a> ScoreValue<'a> {
    pub (crate) fn new(name: &'a str, objective: &'a str, out: File) -> Self {
        Self {objective, name, out}
    }
    /// Set the scoreboard value to a constant
    #[must_use]
    pub fn set_to(mut self, to: i64) -> Self {
        writeln!(self.out, "scoreboard players set {} {} {}", self.name, self.objective, to).unwrap();
        self
    }
    /// Set the scoreboard value to another score
    #[must_use]
    pub fn set(mut self, to: &Self) -> Self {
        writeln!(self.out, "scoreboard players operation {} {} = {} {}", self.name, self.objective, to.name, to.objective).unwrap();
        self
    }
}
impl Add<&Self> for ScoreValue<'_> {
    type Output = Self;

    #[must_use]
    fn add(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} += {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Add<i64> for ScoreValue<'_> {
    type Output = Self;

    #[must_use]
    fn add(mut self, rhs: i64) -> Self::Output {
        writeln!(self.out, "scoreboard players add {} {} {}", self.name, self.objective, rhs).unwrap();
        self
    }
}
impl Sub<&Self> for ScoreValue<'_> {
    type Output = Self;

    #[must_use]
    fn sub(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} -= {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Sub<i64> for ScoreValue<'_> {
    type Output = Self;

    #[must_use]
    fn sub(mut self, rhs: i64) -> Self::Output {
        writeln!(self.out, "scoreboard players remove {} {} {}", self.name, self.objective, rhs).unwrap();
        self
    }
}
impl Mul<&Self> for ScoreValue<'_> {
    type Output = Self;

    #[must_use]
    fn mul(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} *= {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Div<&Self> for ScoreValue<'_> {
    type Output = Self;

    #[must_use]
    fn div(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} /= {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Rem<&Self> for ScoreValue<'_> {
    type Output = Self;

    fn rem(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} *= {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Shl<&Self> for ScoreValue<'_> {
    type Output = Self;

    fn shl(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} < {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Shr<&Self> for ScoreValue<'_> {
    type Output = Self;

    fn shr(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} > {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl BitOrAssign for ScoreValue<'_> {
    fn bitor_assign(&mut self, rhs: Self) {
        writeln!(self.out, "scoreboard players operation {} {} >< {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
    }
}