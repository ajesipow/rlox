use crate::token::Token;

#[derive(Debug)]
pub(crate) enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal(String)
}

pub(crate) trait Visitor<T> {
    
    fn visit_expr(&mut self, expr: &Expr) -> T;
}
