use std::cmp::Ordering;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use super::Header;

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    sign: Sign,
    var: Var,
}

impl Entry {
    pub fn new(sign: Sign, var: Var) -> Self {
        Self { sign, var }
    }

    pub fn var(&self) -> Var {
        self.var
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.sign.fmt(f)?;
        self.var.fmt(f)
    }
}

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: isize = s.parse()?;

        match entry.cmp(&0) {
            Ordering::Greater => Ok(Self::new(Sign::Pos, Var(entry as usize))),
            Ordering::Equal => Err(ParseEntryError::Zero),
            Ordering::Less => Ok(Self::new(Sign::Neg, Var((-entry) as usize))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Pos,
    Neg,
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Neg = self {
            f.write_str("-")?;
        }
        Ok(())
    }
}

impl Sign {
    pub fn apply(self, b: bool) -> bool {
        match self {
            Self::Pos => b,
            Self::Neg => !b,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Var(usize);

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Var {
    pub fn new(i: usize) -> Self {
        Self(i)
    }

    pub fn is_valid(self, header: Header) -> bool {
        self.0 <= header.var_count
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseEntryError {
    #[error("Invalid int: {0}")]
    InvalidInt(#[from] ParseIntError),
    #[error("Var is zero")]
    Zero,
}
