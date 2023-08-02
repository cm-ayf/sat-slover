use std::str::FromStr;

mod header;
pub use header::*;
mod clause;
pub use clause::*;
mod entry;
pub use entry::*;

#[derive(Debug, Clone)]
pub struct Cnf {
    pub header: Header,
    pub clauses: Vec<Clause>,
}

impl FromStr for Cnf {
    type Err = ParseCnfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();

        let header: Header = loop {
            let Some(line) = iter.next() else {
                return Err(ParseCnfError::NoHeader);
            };
            let Some(first) = first(line) else {
                return Err(ParseCnfError::NoHeader);
            };
            match first {
                "c" => continue,
                "p" => break line.parse()?,
                _ => return Err(ParseCnfError::NoHeader),
            }
        };

        let mut clauses = Vec::with_capacity(header.clause_count);
        for (i, line) in iter.take(header.clause_count).enumerate() {
            let clause: Clause = line
                .parse()
                .map_err(|e| ParseCnfError::InvalidClause(e, i))?;
            if let Some((j, entry)) = clause.invalid_var(header) {
                return Err(ParseCnfError::InvalidVar(
                    header.var_count,
                    entry.var(),
                    i,
                    j,
                ));
            }
            clauses.push(clause);
        }

        if clauses.len() != header.clause_count {
            return Err(ParseCnfError::InsufficientClauses(
                header.clause_count,
                clauses.len(),
            ));
        }

        Ok(Self { header, clauses })
    }
}

fn first(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

#[derive(Debug, thiserror::Error)]
pub enum ParseCnfError {
    #[error("No header")]
    NoHeader,
    #[error("Invalid header: {0}")]
    InvalidHeader(#[from] ParseHeaderError),
    #[error("Invalid {1}th clause: {0}")]
    InvalidClause(ParseClauseError, usize),
    #[error("Invalid {3}th var on {2}th clause: expected 1..={0}, got {1}")]
    InvalidVar(usize, Var, usize, usize),
    #[error("Invalid clause count: expected {0}, got {1}")]
    InsufficientClauses(usize, usize),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
