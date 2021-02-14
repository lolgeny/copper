use std::{fs::File, io::Write, ops::{Add, Sub}};

pub struct ScoreValue<'a> {
    objective: &'a str,
    name: &'a str,
    out: File
}
impl<'a> ScoreValue<'a> {
    pub (crate) fn new(name: &'a str, objective: &'a str, out: File) -> Self {
        Self {objective, name, out}
    }
    pub fn set_to(mut self, to: i64) -> Self {
        writeln!(self.out, "scoreboard players set {} {} {}", self.name, self.objective, to).unwrap();
        self
    }
    pub fn set(mut self, to: &Self) -> Self {
        writeln!(self.out, "scoreboard players operation {} {} = {} {}", self.name, self.objective, to.name, to.objective).unwrap();
        self
    }
}
impl Add<&Self> for ScoreValue<'_> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} += {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Add<i64> for ScoreValue<'_> {
    type Output = Self;

    fn add(mut self, rhs: i64) -> Self::Output {
        writeln!(self.out, "scoreboard players add {} {} {}", self.name, self.objective, rhs).unwrap();
        self
    }
}
impl Sub<&Self> for ScoreValue<'_> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        writeln!(self.out, "scoreboard players operation {} {} -= {} {}", self.name, self.objective, rhs.name, rhs.objective).unwrap();
        self
    }
}
impl Sub<i64> for ScoreValue<'_> {
    type Output = Self;

    fn sub(mut self, rhs: i64) -> Self::Output {
        writeln!(self.out, "scoreboard players remove {} {} {}", self.name, self.objective, rhs).unwrap();
        self
    }
}