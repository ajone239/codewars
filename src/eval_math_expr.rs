//! # Instructions
//!
//! Given a mathematical expression as a string you must return the result as a number.
//!
//! # Numbers
//!
//! Number may be both whole numbers and/or decimal numbers. The same goes for the returned result.
//!
//! # Operators
//!
//! You need to support the following mathematical operators:
//!
//! - Multiplication *
//! - Division / (as floating point division)
//! - Addition +
//! - Subtraction -
//! - Operators are always evaluated from left-to-right, and * and / must be evaluated before + and -.
//!
//! # Parentheses
//!
//! You need to support multiple levels of nested parentheses, ex. (2 / (2 + 3.33) * 4) - -6
//!
//! # Whitespace
//!
//! There may or may not be whitespace between numbers and operators.
//!
//! An addition to this rule is that the minus sign (-) used for negating numbers and parentheses will never be separated by whitespace. I.e all of the following are valid expressions.
//!
//! 1-1    // 0
//! 1 -1   // 0
//! 1- 1   // 0
//! 1 - 1  // 0
//! 1- -1  // 2
//! 1 - -1 // 2
//! 1--1   // 2
//!
//! 6 + -(4)   // 2
//! 6 + -( -4) // 10
//! And the following are invalid expressions
//!
//! 1 - - 1    // Invalid
//! 1- - 1     // Invalid
//! 6 + - (4)  // Invalid
//! 6 + -(- 4) // Invalid
//!
//! # Validation
//!
//! You do not need to worry about validation - you will only receive valid mathematical expressions following the above rules.
//!
//! # Restricted APIs
//!
//! NOTE: std::process::Command is disallowed in your solution.

const OPERATORS: &str = "*/+-";

#[allow(dead_code)]
pub fn calc(expr: &str) -> f64 {
    let parens = find_parens(expr);

    // recurse on parens
    let parens_eval = parens
        .iter()
        .map(|(start, end)| calc(&expr[*start + 1..*end]))
        .collect::<Vec<f64>>();

    // replace the parens
    let mut expr = expr.to_string();

    for ((start, end), parens_eval) in parens.iter().zip(parens_eval.iter()).rev() {
        if start == &0 {
            expr.replace_range(start..&(end + 1), &format!("{}", parens_eval));
        } else if start == &1 && expr.chars().nth(start - 1).unwrap() == '-' {
            expr.replace_range(&(start - 1)..&(end + 1), &format!("{}", -parens_eval));
        } else if start >= &2 && expr.chars().nth(start - 1).unwrap() == '-' {
            if expr.chars().nth(start - 2).unwrap().is_ascii_digit() {
                expr.replace_range(&(start - 1)..&(end + 1), &format!("+{}", -parens_eval));
            } else {
                expr.replace_range(&(start - 1)..&(end + 1), &format!("{}", -parens_eval));
            }
        } else {
            expr.replace_range(start..&(end + 1), &format!("{}", parens_eval));
        }
    }

    // split the expression
    let expressions = split_expr(&expr);

    // evaluate the expressions
    eval_expr(expressions)
}

fn find_parens(expr: &str) -> Vec<(usize, usize)> {
    let mut parens: Vec<(usize, usize)> = vec![];
    let mut last_open: Option<usize> = None;
    let mut open_count = 0;

    for (i, c) in expr.chars().enumerate() {
        if c == '(' {
            match last_open {
                None => last_open = Some(i),
                Some(_) => open_count += 1,
            }
        } else if c == ')' {
            match open_count {
                0 => {
                    // UNWRAP: should be safe on good input
                    parens.push((last_open.unwrap(), i));
                    last_open = None;
                }
                _ => open_count -= 1,
            }
        }
    }

    parens
}

fn split_expr(expr: &str) -> Vec<String> {
    let mut expressions: Vec<String> = vec![];

    let mut last_index = 0;

    for (i, _) in expr
        .chars()
        .enumerate()
        .filter(|(_, c)| OPERATORS.find(*c).is_some())
    {
        if last_index != i {
            // will be an operand
            expressions.push(expr[last_index..i].trim().to_string());
        }
        // will be an operator
        expressions.push(expr[i..i + 1].trim().to_string());
        last_index = i + 1;
    }

    if last_index < expr.len() {
        // will be an operand
        expressions.push(expr[last_index..].trim().to_string());
    }

    expressions
}

fn eval_expr(mut expr: Vec<String>) -> f64 {
    // handle unary minus

    for i in 0..expr.len() {
        if expr[i] != "-" {
            continue;
        }
        if i != 0 {
            if !OPERATORS.contains(&expr[i - 1]) {
                continue;
            }
        }
        let new_val = eval_unary(&expr[i..i + 2]);
        expr[i] = "".to_string();
        expr[i + 1] = new_val;
    }

    let mut expr = expr
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    // handle * and /
    for i in 1..expr.len() {
        if expr[i] != "*" && expr[i] != "/" {
            continue;
        }
        let new_val = eval_binary(&expr[i - 1..i + 2]);
        expr[i - 1] = "".to_string();
        expr[i] = "".to_string();
        expr[i + 1] = new_val;
    }

    let mut expr = expr
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    // handle + and -
    for i in 1..expr.len() {
        if expr[i] != "+" && expr[i] != "-" {
            continue;
        }

        let expr_slice = &expr[i - 1..i + 2];

        let new_val = eval_binary(expr_slice);

        expr[i - 1] = "".to_string();
        expr[i] = "".to_string();
        expr[i + 1] = new_val;
    }

    let expr = expr
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    if expr.len() != 1 {
        println!("{:?}", expr);
        panic!("Invalid expression");
    }

    expr[0].parse::<f64>().unwrap()
}

fn eval_unary(expr: &[String]) -> String {
    if expr.len() != 2 {
        println!("{:?}", expr);
        panic!("Invalid unary expression");
    }
    let operator = expr[0].as_str();

    let right = expr[1].parse::<f64>().unwrap();

    match operator {
        "-" => format!("{}", -right),
        _ => unreachable!(),
    }
}

fn eval_binary(expr: &[String]) -> String {
    if expr.len() != 3 {
        println!("{:?}", expr);
        panic!("Invalid binary expression");
    }

    let operator = expr[1].as_str();

    let left = expr[0].parse::<f64>().unwrap();
    let right = expr[2].parse::<f64>().unwrap();

    match operator {
        "*" => format!("{}", left * right),
        "/" => format!("{}", left / right),
        "+" => format!("{}", left + right),
        "-" => format!("{}", left - right),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_expr_numbers() {
        assert_eq!(split_expr("0"), vec!["0"]);
        assert_eq!(split_expr("123"), vec!["123"]);
        assert_eq!(split_expr("123.456"), vec!["123.456"]);
    }

    #[test]
    fn test_split_expr_ops() {
        assert_eq!(split_expr("1+1"), vec!["1", "+", "1"]);
        assert_eq!(split_expr("1-1"), vec!["1", "-", "1"]);
        assert_eq!(split_expr("1*1"), vec!["1", "*", "1"]);
        assert_eq!(split_expr("1/1"), vec!["1", "/", "1"]);
        assert_eq!(split_expr("-1"), vec!["-", "1"]);

        assert_eq!(split_expr("1-1+1"), vec!["1", "-", "1", "+", "1"]);
        assert_eq!(split_expr("1-1--1"), vec!["1", "-", "1", "-", "-", "1"]);
    }

    #[test]
    fn test_find_parens() {
        assert_eq!(find_parens("()"), vec![(0, 1)]);
        assert_eq!(find_parens("(())"), vec![(0, 3)]);
        assert_eq!(find_parens("(())()"), vec![(0, 3), (4, 5)]);
        assert_eq!(find_parens("0-(1+(2+3))+(4+5)"), vec![(2, 10), (12, 16)]);
    }

    #[test]
    fn test_eval_unary() {
        let expr = ["-".to_string(), "1".to_string()];
        let new_val = eval_unary(&expr);
        assert_eq!(new_val, "-1".to_string());
    }

    #[test]
    fn test_eval_binary() {
        let expr = ["1".to_string(), "+".to_string(), "1".to_string()];
        assert_eq!(eval_binary(&expr), "2".to_string());

        let expr = ["1".to_string(), "-".to_string(), "1".to_string()];
        assert_eq!(eval_binary(&expr), "0".to_string());

        let expr = ["2".to_string(), "*".to_string(), "2".to_string()];
        assert_eq!(eval_binary(&expr), "4".to_string());

        let expr = ["2".to_string(), "/".to_string(), "4".to_string()];
        assert_eq!(eval_binary(&expr), "0.5".to_string());
    }

    #[test]
    fn test_eval_expr() {
        let expr = ["-", "1", "+", "2", "*", "3"];
        let expr = expr.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(eval_expr(expr), 5.0);
    }

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
        assert_expr_eq!("350.1", 350.1);
        assert_expr_eq!("350.123", 350.123);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators_hard() {
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }
}
