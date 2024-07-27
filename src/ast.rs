use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
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

pub trait Visitor<T> {
    
    fn visit_expr(&mut self, expr: &Expr) -> T;
}

pub struct PrettyPrinter {}

impl PrettyPrinter {
    fn parenthesize<S: AsRef<str>>(&mut self, name: S, expr: &Expr) -> String {
        let s = self.visit_expr(expr);
        format!("( {} {} )", name.as_ref(), s)
    }
}

impl Visitor<String> for PrettyPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                let first = self.parenthesize(operator.lexeme().unwrap_or(""), left);
                self.parenthesize(first, right)
            }
            Expr::Unary { operator, right } => {
                self.parenthesize(operator.lexeme().unwrap_or(""), right)
            }
            Expr::Grouping { expression } => {
                self.parenthesize("group", expression)
            }
            Expr::Literal(s) => {
                s.to_string()
            }
        }
    }
}
