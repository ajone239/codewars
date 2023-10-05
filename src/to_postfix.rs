// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[allow(dead_code)]
fn to_postfix(_infix: &str) -> String {
    todo!()
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
    fn fixed_tests() {
        do_test(&to_postfix("2+7*5"), "275*+");
        do_test(&to_postfix("3*3/(7+1)"), "33*71+/");
        do_test(&to_postfix("5+(6-2)*9+3^(7-1)"), "562-9*+371-^+");
        do_test(&to_postfix("(5-4-1)+9/5/2-7/1/7"), "54-1-95/2/+71/7/-");
        do_test(&to_postfix("1^2^3"), "123^^");
    }
}
