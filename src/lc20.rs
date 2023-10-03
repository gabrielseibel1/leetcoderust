pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::<char>::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            _ => match stack.pop() {
                Some('(') if c == ')' => (),
                Some('{') if c == '}' => (),
                Some('[') if c == ']' => (),
                _ => return false,
            },
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_valid(String::from("["));
        assert!(!result);
        let result = is_valid(String::from("()"));
        assert!(result);
        let result = is_valid(String::from("()()"));
        assert!(result);
        let result = is_valid(String::from("(]"));
        assert!(!result);
        let result = is_valid(String::from("{}[]{}"));
        assert!(result);
        let result = is_valid(String::from("{()[]}"));
        assert!(result);
        let result = is_valid(String::from("({)}"));
        assert!(!result);
        let result = is_valid(String::from("()}"));
        assert!(!result);
        let result = is_valid(String::from("(){}}{"));
        assert!(!result)
    }
}
