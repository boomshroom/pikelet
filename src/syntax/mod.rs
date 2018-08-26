//! The syntax of the language

use std::fmt;
use std::ops::{Add, AddAssign};

pub mod concrete;
pub mod core;
pub mod parse;
pub mod pretty;
pub mod raw;
pub mod translation;

/// A universe level
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, BoundTerm)]
pub struct Level(pub u32);

impl Level {
    pub fn succ(self) -> Level {
        Level(self.0 + 1)
    }
}

impl From<u32> for Level {
    fn from(src: u32) -> Level {
        Level(src)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A shift in universe level
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, BoundTerm, BoundPattern)]
pub struct LevelShift(pub u32);

impl From<u32> for LevelShift {
    fn from(src: u32) -> LevelShift {
        LevelShift(src)
    }
}

impl fmt::Display for LevelShift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for LevelShift {
    type Output = LevelShift;

    fn add(self, other: LevelShift) -> LevelShift {
        LevelShift(self.0 + other.0)
    }
}

impl AddAssign for LevelShift {
    fn add_assign(&mut self, other: LevelShift) {
        self.0 += other.0;
    }
}

impl Add<LevelShift> for Level {
    type Output = Level;

    fn add(self, other: LevelShift) -> Level {
        Level(self.0 + other.0)
    }
}

impl AddAssign<LevelShift> for Level {
    fn add_assign(&mut self, other: LevelShift) {
        self.0 += other.0;
    }
}

/// A label that describes the name of a field in a record
///
/// Labels are significant when comparing for alpha-equality
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BoundPattern, BoundTerm)]
pub struct Label(pub String);

impl From<String> for Label {
    fn from(src: String) -> Label {
        Label(src)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
