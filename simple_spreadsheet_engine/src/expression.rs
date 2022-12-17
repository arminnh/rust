use crate::formula::Formula;
use crate::function::Function;

#[derive(Debug)]
pub enum Clone {
    Left,
    Right,
    Top,
}

#[derive(Debug)]
pub enum Expression {
    Clone(Clone),
    Function(Function),
    Formula(Formula),
}

impl Expression {
    pub fn from(input: &str) -> Option<Self> {
        if let Some(fun) = Function::from(input) {
            return Some(Expression::Function(fun));
        }
        if let Some(formula) = Formula::from(input) {
            return Some(Expression::Formula(formula));
        }
        None
    }
}
