use std::collections::HashMap;

pub fn balanced<T: AsRef<str>>(input: T) -> bool {
    let mut stack = Vec::new();

    let mut matches = HashMap::new();
    matches.insert(')', '(');
    matches.insert(']', '[');
    matches.insert('}', '{');

    for c in input.as_ref().chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                match (matches.get(&c), stack.pop()) {
                    (Some(curr), Some(prev)) if *curr == prev => (),
                    _ => return false,
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::balanced;

    #[test]
    fn it_works() {
        assert_eq!(balanced(""), true);
        assert_eq!(balanced("()"), true);
        assert_eq!(balanced("("), false);
        assert_eq!(balanced("()[]{}"), true);
        assert_eq!(balanced("{()[]{}[]}"), true);
        assert_eq!(balanced("{(())[[{}]]{}[]}"), true);
        assert_eq!(balanced("{(())[[{}]]{}[]}hello"), false);
    }

    #[test]
    fn matching_opening_closing_bracket() {
        assert_eq!(balanced("(}"), false);
        assert_eq!(balanced("{(())]"), false);
    }
}
