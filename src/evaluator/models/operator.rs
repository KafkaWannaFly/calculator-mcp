use std::fmt;
use variantly::Variantly;

use crate::evaluator::Assoc;

#[derive(Debug, Clone, PartialEq, Copy, Variantly)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    UnarySub,
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            '/' => Operator::Div,
            '%' => Operator::Mod,
            '^' => Operator::Pow,
            _ => panic!("Invalid character for operator: {}", c),
        }
    }
}

pub fn is_op(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/' | '%' | '^')
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::Pow => "^",
            Operator::UnarySub => "u-",
        };
        write!(f, "{symbol}")
    }
}

pub fn operator_precedence(op: Operator) -> u8 {
    match op {
        Operator::Add | Operator::Sub => 1,
        Operator::Mul | Operator::Div | Operator::Mod => 2,
        Operator::UnarySub => 3,
        Operator::Pow => 4,
    }
}

pub fn operator_associativity(op: Operator) -> Assoc {
    match op {
        Operator::Pow | Operator::UnarySub => Assoc::Right,
        Operator::Add | Operator::Sub | Operator::Mul | Operator::Div | Operator::Mod => {
            Assoc::Left
        }
    }
}

pub fn should_pop_operator(stack_op: Operator, incoming: Operator) -> bool {
    let stack_prec = operator_precedence(stack_op);
    let incoming_prec = operator_precedence(incoming);

    if stack_prec > incoming_prec {
        return true;
    }

    if stack_prec == incoming_prec {
        return matches!(operator_associativity(incoming), Assoc::Left);
    }

    false
}
