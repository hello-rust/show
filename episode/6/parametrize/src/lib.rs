extern crate balanced;
#[cfg(test)]
extern crate rstest;

#[cfg(test)]
mod tests {
    use balanced::balanced;
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


