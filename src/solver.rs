use std::collections::HashMap;

use crate::Entry;

use super::Assignment;
use super::Cnf;
use super::Partial;
use super::PartialAssign;
use super::Var;

#[derive(Debug)]
pub struct Solver {
    stack: Vec<StackItem>,
}

#[derive(Debug)]
struct StackItem {
    cnf: Cnf,
    assignment: Assignment,
}

impl Solver {
    pub fn new(cnf: Cnf) -> Self {
        let item = StackItem {
            cnf,
            assignment: Assignment::new(),
        };
        Self { stack: vec![item] }
    }

    pub fn solve(&mut self) -> Result<Solution, SolverError> {
        while let Some(StackItem { cnf, assignment }) = self.stack.pop() {
            if let Some(entry) = Self::find_unit(&cnf) {
                let var = entry.var();
                let b = entry.sign().apply(true);
                if let Some(res) = self.extend_push(&cnf, &assignment, var, b) {
                    return Ok(Solution::Satisfiable(res));
                } else {
                    continue;
                }
            } else {
                let var = Self::pick_var(&cnf).ok_or(SolverError::NoUnassignedVar)?;
                for b in [true, false] {
                    if let Some(res) = self.extend_push(&cnf, &assignment, var, b) {
                        return Ok(Solution::Satisfiable(res));
                    }
                }
            }
        }

        Ok(Solution::Unsatisfiable)
    }

    fn extend_push(
        &mut self,
        cnf: &Cnf,
        assignment: &Assignment,
        var: Var,
        b: bool,
    ) -> Option<Assignment> {
        let assignment = assignment.extend(var, b);
        match cnf.partial_assign(&assignment) {
            Partial::Definite(true) => return Some(assignment),
            Partial::Definite(false) => (),
            Partial::Indefinite(cnf) => self.stack.push(StackItem { cnf, assignment }),
        }
        None
    }

    fn find_unit<'a>(cnf: &Cnf) -> Option<&Entry> {
        cnf.clauses.iter().find_map(|c| {
            if c.entries.len() == 1 {
                c.entries.first()
            } else {
                None
            }
        })
    }

    fn pick_var(cnf: &Cnf) -> Option<Var> {
        let vars = cnf
            .clauses
            .iter()
            .flat_map(|c| c.entries.iter().map(|e| e.var()));

        let mut counts = HashMap::<_, usize>::new();
        for var in vars {
            *counts.entry(var).or_default() += 1;
        }

        counts.into_iter().max_by_key(|(_, c)| *c).map(|(v, _)| v)
    }
}

#[derive(Debug, Clone)]
pub enum Solution {
    Satisfiable(Assignment),
    Unsatisfiable,
}

#[derive(Debug, thiserror::Error)]
pub enum SolverError {
    #[error("Empty stack")]
    EmptyStack,
    #[error("No unassigned var")]
    NoUnassignedVar,
}
