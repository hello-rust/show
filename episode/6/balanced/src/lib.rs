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

    macro_rules! parameterized {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(expected, balanced(input));
                }
            )*
        }
    }

    parameterized! {
        empty: ("", true),
        simple_parenthesis: ("()", true),
        multiple_brackets: ("()[]{}", true),
        nested_brackets: ("{()[]{}[]}", true),
    }

    parameterized! {
        missing_close: ("(", false),
        missing_open: (")", false),
        non_bracket_input: ("{(())[[{}]]{}[]}hello", false),
    }

}