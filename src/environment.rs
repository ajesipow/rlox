use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Literal;
use crate::error::RunTimeError;

#[derive(Debug)]
pub(crate) struct Environment {
    values: HashMap<Rc<str>, Literal>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub(crate) fn define(
        &mut self,
        name: Rc<str>,
        value: Literal,
    ) {
        self.values.insert(name, value.clone());
    }

    pub(crate) fn get(
        &self,
        name: Rc<str>,
    ) -> Result<Literal, RunTimeError> {
        self.values
            .get(&*name)
            .cloned()
            .ok_or_else(|| RunTimeError::UndefinedVariable {
                variable: name.to_string(),
            })
    }

    pub(crate) fn assign(
        &mut self,
        name: Rc<str>,
        value: Literal,
    ) -> Result<(), RunTimeError> {
        match self.values.entry(name).and_modify(|e| *e = value) {
            Entry::Occupied(_) => Ok(()),
            Entry::Vacant(v) => Err(RunTimeError::UndefinedVariable {
                variable: v.key().to_string(),
            }),
        }
    }
}
