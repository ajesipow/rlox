use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Literal;
use crate::error::RunTimeError;
use crate::token::Token;

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
        name: &str,
    ) -> Result<Literal, RunTimeError> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| RunTimeError::UndefinedVariable {
                variable: name.to_string(),
            })
    }
    
    pub(crate) fn assign(&mut self, name: Rc<str>, value: Literal) -> Result<(), RunTimeError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            Err(RunTimeError::UndefinedVariable { variable: name.to_string() })
        }
    }
}
