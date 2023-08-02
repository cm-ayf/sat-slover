use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub var_count: usize,
    pub clause_count: usize,
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p cnf {} {}", self.var_count, self.clause_count)
    }
}

impl FromStr for Header {
    type Err = ParseHeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match iter.next() {
            Some("p") => {}
            _ => return Err(ParseHeaderError::NotCnf),
        }
        match iter.next() {
            Some("cnf") => {}
            _ => return Err(ParseHeaderError::NotCnf),
        }
        let var_count = iter
            .next()
            .ok_or(ParseHeaderError::NotCnf)?
            .parse()
            .map_err(ParseHeaderError::InvalidVarCount)?;
        let clause_count = iter
            .next()
            .ok_or(ParseHeaderError::NotCnf)?
            .parse()
            .map_err(ParseHeaderError::InvalidClauseCount)?;
        Ok(Self {
            var_count,
            clause_count,
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseHeaderError {
    #[error("Not marked as CNF")]
    NotCnf,
    #[error("Invalid var count: {0}")]
    InvalidVarCount(ParseIntError),
    #[error("Invalid clause count: {0}")]
    InvalidClauseCount(ParseIntError),
}
