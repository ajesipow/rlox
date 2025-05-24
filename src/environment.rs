use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Literal;
use crate::error::RunTimeError;

#[derive(Debug, Clone, Default)]
pub(crate) struct Environment {
    values: HashMap<Rc<str>, Literal>,
    enclosing: Option<Box<Self>>,
}

impl Environment {
    pub(crate) fn new(enclosing: Option<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: enclosing.map(Box::new),
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
            .or_else(|| {
                self.enclosing
                    .as_ref()
                    .and_then(|e| e.get(name.clone()).ok())
            })
            .ok_or_else(|| RunTimeError::UndefinedVariable {
                variable: name.to_string(),
            })
    }

    pub(crate) fn assign(
        &mut self,
        name: Rc<str>,
        value: Literal,
    ) -> Result<(), RunTimeError> {
        match self.values.entry(name).and_modify(|e| *e = value.clone()) {
            Entry::Occupied(_) => (),
            Entry::Vacant(v) => {
                self.enclosing
                    .as_mut()
                    .map(|parent| parent.assign(v.key().clone(), value))
                    .ok_or_else(|| RunTimeError::UndefinedVariable {
                        variable: v.key().to_string(),
                    })??;
            }
        };
        Ok(())
    }

    pub(crate) fn take_enclosing(&mut self) -> Option<Self> {
        self.enclosing.take().map(|e| *e)
    }
}
