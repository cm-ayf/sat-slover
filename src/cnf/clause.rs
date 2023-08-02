use std::collections::HashSet;
use std::str::FromStr;

use super::Entry;
use super::Header;
use super::ParseEntryError;
use super::Var;

#[derive(Debug, Clone)]
pub struct Clause {
    pub entries: Vec<Entry>,
}

impl FromStr for Clause {
    type Err = ParseClauseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();
        let mut vars = HashSet::new();

        for entry in s.split_whitespace() {
            match entry.parse::<Entry>() {
                Ok(entry) => {
                    if !vars.insert(entry.var()) {
                        return Err(ParseClauseError::DuplicateVar(entry.var()));
                    }
                    entries.push(entry);
                }
                Err(ParseEntryError::Zero) => break,
                Err(e) => return Err(ParseClauseError::InvalidEntry(e, entries.len())),
            }
        }

        Ok(Self { entries })
    }
}

impl Clause {
    pub fn invalid_var(&self, header: Header) -> Option<(usize, &Entry)> {
        self.entries
            .iter()
            .enumerate()
            .find(|(_, e)| !e.var().is_valid(header))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseClauseError {
    #[error("Invalid {1}th entry: {0}")]
    InvalidEntry(ParseEntryError, usize),
    #[error("No 0 terminator")]
    NoTerminator,
    #[error("Duplicate var: {0}")]
    DuplicateVar(Var),
}
