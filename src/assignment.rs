use crate::Clause;
use crate::Cnf;
use crate::Entry;
use crate::Header;

use super::cnf::Var;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Assignment(HashMap<Var, bool>);

impl Assignment {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn extend(&self, var: Var, b: bool) -> Self {
        let map = self
            .0
            .iter()
            .map(|(var, b)| (*var, *b))
            .chain(Some((var, b)))
            .collect();
        Self(map)
    }

    pub fn pick_unassigned(&self, header: Header) -> Option<Var> {
        (1..=header.var_count)
            .map(Var::new)
            .find(|var| !self.0.contains_key(var))
    }
}

pub trait PartialAssign
where
    Self: Sized,
{
    fn partial_assign(&self, assignment: &Assignment) -> Partial<Self>;
}

impl PartialAssign for Entry {
    fn partial_assign(&self, assignment: &Assignment) -> Partial<Self> {
        match assignment.0.get(&self.var()) {
            Some(b) => Partial::Definite(self.sign().apply(*b)),
            None => Partial::Indefinite(*self),
        }
    }
}

impl PartialAssign for Clause {
    fn partial_assign(&self, assignment: &Assignment) -> Partial<Self> {
        let mut entries = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            match entry.partial_assign(assignment) {
                Partial::Definite(b) => {
                    if b {
                        return Partial::Definite(true);
                    }
                }
                Partial::Indefinite(entry) => {
                    entries.push(entry);
                }
            }
        }

        if entries.is_empty() {
            Partial::Definite(false)
        } else {
            Partial::Indefinite(Self { entries })
        }
    }
}

impl PartialAssign for Cnf {
    fn partial_assign(&self, assignment: &Assignment) -> Partial<Self> {
        let mut clauses = Vec::with_capacity(self.clauses.len());

        for clause in &self.clauses {
            match clause.partial_assign(assignment) {
                Partial::Definite(b) => {
                    if !b {
                        return Partial::Definite(false);
                    }
                }
                Partial::Indefinite(clause) => {
                    clauses.push(clause);
                }
            }
        }

        if clauses.is_empty() {
            Partial::Definite(true)
        } else {
            Partial::Indefinite(Self {
                header: self.header,
                clauses,
            })
        }
    }
}

pub enum Partial<T> {
    Definite(bool),
    Indefinite(T),
}
