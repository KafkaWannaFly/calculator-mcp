pub mod models;
use anyhow::{anyhow, bail};
use bigdecimal::BigDecimal;
pub use models::*;
use num_traits::{ToPrimitive, Zero};
use std::convert::TryFrom;

fn tokenize(input: &str) -> anyhow::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            c if is_paren(c) => tokens.push(to_paren(c)),
            c if c.is_whitespace() => {}
            c if is_op(c) => tokens.push(Token::Op(c.into())),
            c if c.is_ascii_digit() => {
                // normal number, decimals, scientific notation
                let mut num_str = String::new();
                num_str.push(c);

                // Consume the rest of the numbers
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit()
                        || next_char == '.'
                        // Scientific notation
                        || (next_char.eq_ignore_ascii_case(&'e') && !num_str.contains(|c: char| c.eq_ignore_ascii_case(&'e')))
                    {
                        num_str.push(next_char);
                        chars.next(); // Consume the character

                        // Handle sign for scientific notation
                        if next_char.eq_ignore_ascii_case(&'e')
                            && let Some(&sign) = chars.peek()
                            && (sign == '+' || sign == '-')
                        {
                            num_str.push(sign);
                            chars.next();
                        }
                    } else {
                        break;
                    }
                }
                let num = num_str.parse()?;
                tokens.push(Token::Number(num));
            }
            _ if c.is_ascii_alphabetic() => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() {
                        ident.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let math_const = MathConst::try_from(ident.as_str())?;
                tokens.push(Token::Ident(math_const));
            }
            _ => {
                bail!("Unexpected character: {}", c);
            }
        }
    }

    Ok(tokens)
}

fn shunting_yard(tokens: &[Token]) -> anyhow::Result<Vec<Token>> {
    let mut output = Vec::new();
    let mut stack: Vec<Token> = Vec::new();
    let mut expect_operand = true;

    for token in tokens {
        match token {
            Token::Number(_) | Token::Ident(_) => {
                output.push(token.clone());
                expect_operand = false;
            }
            Token::Op(op) => {
                let mut current_op = *op;
                if expect_operand {
                    if current_op == Operator::Sub {
                        current_op = Operator::UnarySub;
                    } else {
                        bail!("Unexpected operator placement");
                    }
                }

                while let Some(stack_top) = stack.last() {
                    let should_pop = match stack_top {
                        Token::Op(stack_op) => should_pop_operator(*stack_op, current_op),
                        Token::LParenthesis => false,
                        _ => false,
                    };

                    if should_pop {
                        if let Some(popped) = stack.pop() {
                            output.push(popped);
                        }
                    } else {
                        break;
                    }
                }
                stack.push(Token::Op(current_op));
                expect_operand = true;
            }
            Token::LParenthesis => {
                stack.push(Token::LParenthesis);
                expect_operand = true;
            }
            Token::RParenthesis => {
                let mut found_left = false;
                while let Some(popped) = stack.pop() {
                    match popped {
                        Token::LParenthesis => {
                            found_left = true;
                            break;
                        }
                        Token::Op(_) => output.push(popped),
                        _ => {}
                    }
                }
                if !found_left {
                    bail!("Mismatched parentheses");
                }
                expect_operand = false;
            }
        }
    }

    while let Some(token) = stack.pop() {
        match token {
            Token::LParenthesis | Token::RParenthesis => bail!("Mismatched parentheses"),
            _ => output.push(token),
        }
    }

    Ok(output)
}

fn eval_rpn(tokens: &[Token]) -> anyhow::Result<BigDecimal> {
    let mut stack: Vec<BigDecimal> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(num.clone()),
            Token::Op(op) => {
                if op.is_unary_sub() {
                    let value = stack
                        .pop()
                        .ok_or_else(|| anyhow!("Not enough operands for operator"))?;
                    let result = apply_unary_operator(value, *op)?;
                    stack.push(result);
                } else {
                    let rhs = stack
                        .pop()
                        .ok_or_else(|| anyhow!("Not enough operands for operator"))?;
                    let lhs = stack
                        .pop()
                        .ok_or_else(|| anyhow!("Not enough operands for operator"))?;
                    let result = apply_operator(lhs, rhs, *op)?;
                    stack.push(result);
                }
            }
            Token::Ident(math_const) => stack.push(BigDecimal::from(*math_const)),
            Token::LParenthesis | Token::RParenthesis => {
                bail!("Parenthesis encountered in RPN stream")
            }
        }
    }

    if stack.len() != 1 {
        bail!("Invalid RPN expression");
    }

    Ok(stack.pop().expect("stack length already validated"))
}

fn apply_operator(lhs: BigDecimal, rhs: BigDecimal, op: Operator) -> anyhow::Result<BigDecimal> {
    let result = match op {
        Operator::Add => lhs + rhs,
        Operator::Sub => lhs - rhs,
        Operator::Mul => lhs * rhs,
        Operator::Div => {
            if rhs.is_zero() {
                bail!("Division by zero");
            }
            lhs / rhs
        }
        Operator::Mod => {
            if rhs.is_zero() {
                bail!("Modulo by zero");
            }
            lhs % rhs
        }
        Operator::Pow => {
            if !rhs.is_integer() {
                bail!("Exponent must be an integer for power operation");
            }
            let exponent = rhs
                .to_i64()
                .ok_or_else(|| anyhow!("Exponent is out of range for power operation"))?;
            lhs.powi(exponent)
        }
        Operator::UnarySub => bail!("Unary operator cannot be applied in binary context"),
    };

    Ok(result)
}

fn apply_unary_operator(value: BigDecimal, op: Operator) -> anyhow::Result<BigDecimal> {
    match op {
        Operator::UnarySub => Ok(-value),
        _ => bail!("Unsupported unary operator"),
    }
}

pub fn eval(input: &str) -> anyhow::Result<BigDecimal> {
    let tokens = tokenize(input)?;
    let rpn = shunting_yard(&tokens)?;
    eval_rpn(&rpn)
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_eval_int() {
        assert_eq!(eval("3 + 4").unwrap(), BigDecimal::from(7));
        assert_eq!(eval("3 * 4").unwrap(), BigDecimal::from(12));
        assert_eq!(eval("3 ^ 4").unwrap(), BigDecimal::from(81));

        assert_eq!(eval("-5 * 4").unwrap(), BigDecimal::from(-20));
        assert_eq!(eval("-5 + (-5)").unwrap(), BigDecimal::from(-10));
        assert_eq!(eval("-(-3 * 2)").unwrap(), BigDecimal::from(6));
        assert_eq!(eval("--5").unwrap(), BigDecimal::from(5));
        assert_eq!(eval("-5 * -2").unwrap(), BigDecimal::from(10));

        assert_eq!(eval("3 + 4 * 5").unwrap(), BigDecimal::from(23));
        assert_eq!(eval("(3 + 4) * 5").unwrap(), BigDecimal::from(35));
        assert_eq!(eval("3 + 4 * 5 / 2").unwrap(), BigDecimal::from(13));
        assert_eq!(eval("2^3 + 1").unwrap(), BigDecimal::from(9));
        assert_eq!(eval("2^(3 + 1)").unwrap(), BigDecimal::from(16));
        assert_eq!(eval("1/2 * 10 * 2^2 + 1").unwrap(), BigDecimal::from(21));

        assert_eq!(eval("10 % 3").unwrap(), BigDecimal::from(1));
        assert_eq!(eval("10 % 3 * 2").unwrap(), BigDecimal::from(2));
    }

    #[test]
    fn test_eval_float() {
        assert_eq!(eval("3 / 4").unwrap(), BigDecimal::from_f64(0.75).unwrap());
        assert_eq!(
            eval("2.5 * 5.2 / 3.1").unwrap().round(2).to_plain_string(),
            "4.19"
        );
        assert_eq!(eval("2.5 ^ 2").unwrap().round(2).to_string(), "6.25");
        assert_eq!(eval("(-2.5) ^ 2").unwrap().round(2).to_string(), "6.25");
        assert_eq!(
            eval("2.5 ^ (2 + 2)").unwrap().round(4).to_string(),
            "39.0625"
        );
        assert_eq!(
            eval("(3 + 4) * 5 / 2").unwrap(),
            BigDecimal::from_f64(17.5).unwrap()
        );
        assert_eq!(eval("1.2e3").unwrap(), BigDecimal::from(1200));
        assert_eq!(
            eval("4.2e-2").unwrap(),
            BigDecimal::from_str("0.042").unwrap()
        );
        assert_eq!(
            eval("1.5e2 + 2.5e-1").unwrap(),
            BigDecimal::from_str("150.25").unwrap()
        );
    }

    #[test]
    fn test_eval_math_const() {
        assert_eq!(eval("pi").unwrap(), BigDecimal::from(MathConst::Pi));
        assert_eq!(
            eval("pi * 2").unwrap(),
            BigDecimal::from(MathConst::Pi) * BigDecimal::from(2)
        );
        assert_eq!(eval("tau").unwrap(), BigDecimal::from(MathConst::Tau));
        assert_eq!(eval("e").unwrap(), BigDecimal::from(MathConst::E));
        assert_eq!(eval("phi").unwrap(), BigDecimal::from(MathConst::Phi));
        assert_eq!(eval("c").unwrap(), BigDecimal::from(MathConst::C));
        assert_eq!(eval("h").unwrap(), BigDecimal::from(MathConst::H));
        assert_eq!(eval("g").unwrap(), BigDecimal::from(MathConst::G));
        assert_eq!(eval("r").unwrap(), BigDecimal::from(MathConst::R));
        assert_eq!(eval("na").unwrap(), BigDecimal::from(MathConst::Na));
        assert_eq!(eval("kb").unwrap(), BigDecimal::from(MathConst::Kb));
        assert_eq!(eval("ec").unwrap(), BigDecimal::from(MathConst::Ec));
        assert_eq!(eval("tau / pi").unwrap(), BigDecimal::from(2));
    }
}
