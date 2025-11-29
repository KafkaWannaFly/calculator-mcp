use bigdecimal::BigDecimal;
use std::fmt;

use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(BigDecimal),
    MathConst(String),
    Op(Operator),
    LParenthesis,
    RParenthesis,
}

pub struct TokenList<'a>(pub &'a [Token]);

impl<'a> From<&'a [Token]> for TokenList<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        TokenList(tokens)
    }
}

impl<'a> From<&'a Vec<Token>> for TokenList<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
        TokenList(tokens.as_slice())
    }
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(num) => write!(f, "{}", num),
            Token::MathConst(name) => write!(f, "{}", name),
            Token::Op(op) => write!(f, "{}", op),
            Token::LParenthesis => write!(f, "("),
            Token::RParenthesis => write!(f, ")"),
        }
    }
}

impl<'a> fmt::Display for TokenList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, token) in self.0.iter().enumerate() {
            if idx > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}
