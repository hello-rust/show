use std::collections::HashMap;

pub fn balanced<T: Into<String>>(input: T) -> bool {
    let mut stack = Vec::new();

    let mut matches = HashMap::new();
    matches.insert(')', '(');
    matches.insert(']', '[');
    matches.insert('}', '{');

    for c in input.into().chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let prev = stack.pop();
                match matches.get(&c) {
                    Some(prev) => (),
                    _ => unreachable!(),
                }
            }
            _ => return false,
        }
    }
    stack.len() == 0
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
}
