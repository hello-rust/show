#[cfg(test)]
extern crate rstest;

use std::collections::HashMap;

pub fn balanced(input: impl AsRef<str>) -> bool {
    let mut stack = Vec::new();

    let mut matches = HashMap::new();
    matches.insert(')', '(');
    matches.insert(']', '[');
    matches.insert('}', '{');

    for c in input.as_ref().chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => if matches.get(&c) != stack.pop().as_ref() {
                return false;
            },
            _ => return false,
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::balanced;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(value,
    case(""),
    case("()"),
    case("()[]{}"),
    case("{()[]{}[]}")
    )]
    fn is_balanced(value: &str) {
        assert!(balanced(value))
    }

    #[rstest_parametrize(value,
    case("("),
    case(")"),
    case("{(())[[{}]]{}[]}hello")
    )]
    fn is_not_balanced(value: &str) {
        assert!(!balanced(value))
    }

}