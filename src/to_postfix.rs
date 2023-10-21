// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

use std::cell::RefCell;
use std::fmt::Display;
use std::{collections::VecDeque, rc::Rc};

#[allow(dead_code)]
fn to_postfix(_infix: &str) -> String {
    let tree = Tree::new(_infix);

    let post_fix = tree.preorder_print();

    post_fix
}

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

type Node<'a> = Rc<RefCell<NodeData<'a>>>;

enum NodeData<'a> {
    Number(i32),
    Op {
        op: Operator,
        left: Node<'a>,
        right: Node<'a>,
    },
    Unprocessed(&'a str),
}

impl NodeData<'_> {
    fn preorder_print(&self) -> String {
        match self {
            NodeData::Number(n) => format!("{}", n),
            NodeData::Op { op, left, right } => {
                format!(
                    "{}{}{}",
                    left.borrow().preorder_print(),
                    right.borrow().preorder_print(),
                    op
                )
            }
            NodeData::Unprocessed(expr) => expr.to_string(),
        }
    }
}

fn process<'a>(node: Node<'a>, unprocessed: &mut VecDeque<Node<'a>>) {
    let expr = match *node.borrow() {
        NodeData::Unprocessed(expr) => expr,
        _ => panic!("This is already processed"),
    };

    for ops in EMDAS.iter().rev() {
        for (i, c) in expr.chars().enumerate() {
            if ops.contains(c) {
                let op = Operator::new(c);
                let left = Rc::new(RefCell::new(NodeData::Unprocessed(&expr[..i])));
                let right = Rc::new(RefCell::new(NodeData::Unprocessed(&expr[i + 1..])));

                unprocessed.push_back(left.clone());
                unprocessed.push_back(right.clone());

                *node.borrow_mut() = NodeData::Op { op, left, right };

                return;
            }
        }
    }
    if "1234567890".contains(&expr) {
        *node.borrow_mut() = NodeData::Number(expr.parse().unwrap());
    }
}

struct Tree<'a> {
    root: Node<'a>,
}

impl<'a> Tree<'a> {
    fn new(expression: &'a str) -> Self {
        let mut tree = Tree {
            root: Rc::new(RefCell::new(NodeData::Unprocessed(expression))),
        };

        let mut unprocessed = VecDeque::new();
        unprocessed.push_back(tree.root.clone());

        while let Some(node) = unprocessed.pop_front() {
            process(node, &mut unprocessed);
        }

        tree
    }
    fn preorder_print(&self) -> String {
        self.root.borrow().preorder_print()
    }
}

#[cfg(test)]
mod tests {
    use super::to_postfix;

    fn do_test(actual: &str, expected: &str) {
        assert_eq!(
            actual, expected,
            "\nYour answer (left) is not the correct answer (right)"
        )
    }

    #[test]
    fn test_simple() {
        do_test(&to_postfix("2+7*5"), "275*+");
        do_test(&to_postfix("1+2*3/4-5"), "1234/*5-+");
    }

    #[test]
    fn fixed_tests() {
        do_test(&to_postfix("2+7*5"), "275*+");
        do_test(&to_postfix("3*3/(7+1)"), "33*71+/");
        do_test(&to_postfix("5+(6-2)*9+3^(7-1)"), "562-9*+371-^+");
        do_test(&to_postfix("(5-4-1)+9/5/2-7/1/7"), "54-1-95/2/+71/7/-");
        do_test(&to_postfix("1^2^3"), "123^^");
    }
}
