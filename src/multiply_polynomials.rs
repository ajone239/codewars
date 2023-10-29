//! # SITUATION
//!
//! One day, as you wake up, you have a terrible realization:
//! You have magically time-traveled, going all the way back to high school.
//!
//! Ignoring the weird scientific implications of this event, you focus on what's truly important: HOMEWORK
//!
//! You see, your teacher has handed out hours' worth of math assignments for you. However, you have better things to do, such as trying to figure out how you even ended up in this situation.
//!
//! Using your retained knowledge, in order to save time, you decide to create a program to do your homework for you, even if it takes way longer than just being a normal person and doing it yourself.
//!
//! # TASK
//!
//! Write a function / method called polynomial_product(), which takes two strings representing polynomials as input and returns another string representing the product of those polynomials.
//!
//! # INPUT
//!
//! You will be given two strings of the form: ax^n + bx^n-1 + cx^n-2 ... + hx^2 + ix + j.
//!
//! Where a, b, c, ..., i, j and n are integers.
//! For example: 4x^3 - x^2 + 7x - 1 and x^2 - 2x + 1
//!
//! # Special Cases:
//!
//! The variable won't always be x; it may be u, v, A, or any other ascii letter (character code: 65 to 90 and 97 to 122). However, the variable is always consistent between the polynomials.
//! You might get u^2 - 1 and 2u + 1 as well as A^3 - A and 2A^2 + 8, but never 3x^2 - x and 2y + 1.
//! If the coefficient of a term is 1, then it will simply not show up, unless it's the constant term.
//! For example,u^3 - u^2 + u - 1 or u^2 + u + 1
//! If the coefficient of a term is 0, then, the whole term will, simply, not show up.
//! For example,2H^2 + 1 or 3H^3 - 4H
//! OUTPUT
//!
//! Return a string representing a polynomial of the form: ax^n + bx^n-1 + cx^n-2 ... + hx^2 + ix + j.
//!
//! For example, given x^2 + 2x + 1 and x + 1, return x^3+3x^2+3x+1
//!
//! Rules for formatting the answer:
//!
//! Your answer should not contain whitespace; all the terms, must, be together.
//! For example, s^2+2s+1 instead of s^2 + 2s + 1
//! When the coefficient of a term is 1 or -1, it shouldn't show up, unless it's the constant term.
//! For example, -x^2+x-1 instead of -1x^2+1x-1
//! When the coefficient of a term is 0, the whole term shouldn't appear in the answer.
//! For example, 2Y^3+Y instead of 2Y^3+0Y^2+Y+0
//! The terms should be in order from the highest degree to the lowest, from left to right.
//! For example, a^4+2a^2+1 instead of 1+2a^2+a^4
//! The variable used for the answer should be the same as the input; don't go turning every variable into x's.
//! For example, given b^2 + 1 and 9b - 2, return 9b^3-2b^2+9b-2 rather than 9x^3-2x^2+9x-2.
//!
//! # RANDOM TESTS
//!
//! 0 <= exponents <= 4500
//! -85000 <= coefficients <= 85000
//! 20 small tests
//! 30 medium tests
//! 20 big tests
//! 5 really big tests
//!
//! This kata isn't focused on performance. However, don't go writing spaghetti code just because of this; you can still timeout if your code is poorly written.

use regex::{Captures, Match, Regex};

fn polynomial_product(s1: &str, s2: &str) -> String {
    todo!()
}

#[derive(Debug, PartialEq)]
struct Polynomial {
    terms: Vec<Term>,
}

impl Polynomial {
    fn from_string(s: &str) -> Self {
        let re = Regex::new(r"(-?\d+)?([a-zA-Z])?(\^(-?\d+))?").unwrap();

        let mut terms = Vec::new();
        for cap in re.captures_iter(s) {
            if cap.get(0).unwrap().as_str().len() == 0 {
                continue;
            }
            terms.push(Term::from_capture(&cap));
        }
        Self { terms }
    }
}

#[derive(Debug, PartialEq)]
struct Term {
    coefficient: i32,
    power: i32,
}

impl Term {
    fn from_capture(cap: &Captures<'_>) -> Self {
        let coefficient = match cap.get(1) {
            Some(c) => c.as_str().parse::<i32>().unwrap(),
            None => 1,
        };
        let variable = match cap.get(2) {
            Some(v) => v.as_str(),
            None => "",
        };
        let power = match cap.get(4) {
            Some(p) => p.as_str().parse::<i32>().unwrap(),
            None => {
                if !variable.is_empty() {
                    1
                } else {
                    0
                }
            }
        };
        Self { coefficient, power }
    }
    fn add(&self, other: &Self) -> Self {
        if self.power != other.power {
            panic!("Cannot add terms");
        }
        Self {
            coefficient: self.coefficient + other.coefficient,
            power: self.power,
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        Self {
            coefficient: self.coefficient * other.power,
            power: self.power + other.power,
        }
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "x^2 + 2x + 1",
        vec![
            Term {
                coefficient: 1,
                power: 2
            },
            Term {
                coefficient: 2,
                power: 1
            },
            Term {
                coefficient: 1,
                power: 0
            }
        ]
    )]
    fn test_polynomial_from_string(#[case] s: &str, #[case] expected: Vec<Term>) {
        let polynomial = Polynomial::from_string(s);
        assert_eq!(expected, polynomial.terms);
    }

    fn dotest(s1: &str, s2: &str, expected: &str) {
        let actual = polynomial_product(s1, s2);
        assert!(
            actual == expected,
            "With s1 = \"{s1}\", \"{s2}\"\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn example_tests() {
        dotest("u^2 + 2u+1", "u + 1", "u^3+3u^2+3u+1");
        dotest("x^2", "3x - 1", "3x^3-x^2");
        dotest("2", "4y - 4", "8y-8");
        dotest("-4r^2 + 1", "-1", "4r^2-1");
        dotest("1", "p^3", "p^3");
        dotest("1", "-1", "-1");
        dotest("0", "2 - x", "0");
        dotest("-1", "0", "0");
        dotest("v^2 - 1+3v^3", "1+v^2", "3v^5+v^4+3v^3-1");
    }
}
