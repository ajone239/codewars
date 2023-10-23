// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

use std::fmt::Display;

#[allow(dead_code)]
fn to_postfix(_infix: &str) -> String {
    let _tokens = tokenize(_infix);

    unimplemented!()
}

fn tokenize(expr: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();

    let mut char_iter = expr.chars().enumerate();

    while let Some((i, c)) = char_iter.next() {
        match c {
            '0'..='9' => {
                tokens.push(Token::Number(c.to_digit(10).unwrap() as i32));
            }
            '+' | '-' | '*' | '/' | '^' => {
                tokens.push(Token::Operator(Operator::new(c)));
            }
            '(' => {
                let closing_paren = find_closing_paren(&mut char_iter);
                tokens.push(Token::ParenStatement(&expr[i + 1..closing_paren]));
            }
            ')' => {
                unreachable!()
            }
            _ => panic!(),
        }
    }
    tokens
}

fn find_closing_paren(char_iter: &mut std::iter::Enumerate<std::str::Chars>) -> usize {
    let mut count = 1;
    while let Some((i, c)) = char_iter.next() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        }
        if count == 0 {
            return i;
        }
    }
    unreachable!()
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Number(i32),
    Operator(Operator),
    ParenStatement(&'a str),
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

const OPERATORS: &str = "+-*/^";
const EMDAS: &[&str] = &["^", "*/", "+-"];
impl Operator {
    fn new(op: char) -> Self {
        match op {
            '+' => Self::Add,
            '-' => Self::Subtract,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            '^' => Self::Exponent,
            _ => unreachable!(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Exponent => write!(f, "^"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::simple("()", 0, 1)]
    #[case::jump("(())", 0, 3)]
    #[case::double_jump("(()())", 0, 5)]
    #[case::inner("(())", 1, 2)]
    #[case::data("(12+45)", 0, 6)]
    fn test_find_closing_paren(#[case] expr: &str, #[case] i: usize, #[case] expected: usize) {
        let mut char_iter = expr.chars().enumerate();

        for _ in 0..(i + 1) {
            char_iter.next();
        }

        let actual = find_closing_paren(&mut char_iter);

        assert_eq!(actual, expected);

        if let Some((i, _)) = char_iter.next() {
            assert_eq!(i, actual + 1);
        }
    }

    #[rstest]
    #[case(
            "2+7*5",
            vec![
                Token::Number(2),
                Token::Operator(Operator::new('+')),
                Token::Number(7),
                Token::Operator(Operator::new('*')),
                Token::Number(5),
            ]
        )
    ]
    #[case(
            "3*3/(7+1)",
            vec![
                Token::Number(3),
                Token::Operator(Operator::new('*')),
                Token::Number(3),
                Token::Operator(Operator::new('/')),
                Token::ParenStatement("7+1"),
            ]
        )
    ]
    fn test_tokenize(#[case] expr: &str, #[case] expected: Vec<Token>) {
        assert_eq!(tokenize(expr), expected);
    }

    fn do_test(actual: &str, expected: &str) {
        assert_eq!(
            actual, expected,
            "\nYour answer (left) is not the correct answer (right)"
        )
    }

    #[rstest]
    #[case("2+7*5", "275*+")]
    #[case("1^2^3", "123^^")]
    #[case("1+2*3/4-5", "1234/*5-+")]
    fn test_simple(#[case] expr: &str, #[case] expected: &str) {
        do_test(&to_postfix(expr), expected);
    }

    #[rstest]
    #[case("3*3/(7+1)", "33*71+/")]
    #[case("5+(6-2)*9+3^(7-1)", "562-9*+371-^+")]
    #[case("(5-4-1)+9/5/2-7/1/7", "54-1-95/2/+71/7/-")]
    fn fixed_tests(#[case] expr: &str, #[case] expected: &str) {
        do_test(&to_postfix(expr), expected);
    }
}
