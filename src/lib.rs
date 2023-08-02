mod assignment;
mod cnf;
mod solver;

use std::path::PathBuf;

use assignment::*;
pub use cnf::Cnf;
use cnf::*;
pub use solver::*;

pub fn parse(path: &PathBuf) -> Result<Cnf, ParseError> {
    let s = std::fs::read_to_string(path).map_err(|e| ParseError::IoError(e, path.clone()))?;
    s.parse()
        .map_err(|e| ParseError::ParseCnfError(e, path.clone()))
}

pub fn solve(cnf: Cnf) -> Result<Solution, SolverError> {
    Solver::new(cnf).solve()
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid cnf on file {1}: {0}")]
    ParseCnfError(ParseCnfError, PathBuf),
    #[error("IO error on file {1}: {0}")]
    IoError(std::io::Error, PathBuf),
}
