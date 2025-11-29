use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(BigDecimal),
    Const(String),
    Op(Operator),
    LParenthesis,
    RParenthesis,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            '/' => Operator::Div,
            '^' => Operator::Pow,
            _ => panic!("Invalid character for operator: {}", c),
        }
    }
}

pub fn is_op(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/' | '^')
}

pub fn is_paren(ch: char) -> bool {
    matches!(ch, '(' | ')')
}

pub fn to_paren(ch: char) -> Token {
    match ch {
        '(' => Token::LParenthesis,
        ')' => Token::RParenthesis,
        _ => panic!("Invalid character for parenthesis: {}", ch),
    }
}

#[derive(Debug)]
pub enum Assoc {
    Left,
    Right,
}

pub fn operator_precedence(op: Operator) -> u8 {
    match op {
        Operator::Add | Operator::Sub => 1,
        Operator::Mul | Operator::Div => 2,
        Operator::Pow => 3,
    }
}

pub fn operator_associativity(op: Operator) -> Assoc {
    match op {
        Operator::Pow => Assoc::Right,
        Operator::Add | Operator::Sub | Operator::Mul | Operator::Div => Assoc::Left,
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
