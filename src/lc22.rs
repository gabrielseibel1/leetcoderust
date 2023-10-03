pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut v = vec![];
    gen(n, &mut String::from(""), &mut v, 0, 0);
    v
}

fn gen(n: i32, s: &mut String, set: &mut Vec<String>, o: i32, c: i32) {
    if o == n && c == n {
        set.push(s.clone());
        return;
    }
    if o > c && c < n {
        s.push(')');
        gen(n, s, set, o, c + 1);
        s.pop();
    }
    if o < n {
        s.push('(');
        gen(n, s, set, o + 1, c);
        s.pop();
    }
}