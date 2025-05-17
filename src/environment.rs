use std::collections::HashMap;

use crate::ast::Literal;
use crate::error::RunTimeError;

#[derive(Debug)]
pub(crate) struct Environment<'a> {
    values: HashMap<&'a str, Literal<'a>>,
}

impl<'a> Environment<'a> {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub(crate) fn define(
        &mut self,
        name: &'a str,
        value: Literal<'a>,
    ) {
        self.values.insert(name, value);
    }

    pub(crate) fn get(
        &self,
        name: &'a str,
    ) -> Result<Literal<'a>, RunTimeError> {
        self.values
            .get(name)
            .copied()
            .ok_or_else(|| RunTimeError::UndefinedVariable {
                variable: name.to_string(),
            })
    }
}
