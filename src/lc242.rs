pub fn is_anagram(s: String, t: String) -> bool {
    const SIZE: usize = 26;
    let mut a = [0; SIZE];
    for c in s.bytes() {
        a[c as usize % SIZE] += 1;
    }
    for c in t.bytes() {
        a[c as usize % SIZE] -= 1;
    }
    a.iter().all(|x| *x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_anagram(String::from("a"), String::from("a"));
        assert!(result);
        let result = is_anagram(String::from("aa"), String::from("aa"));
        assert!(result);
        let result = is_anagram(String::from("ab"), String::from("ba"));
        assert!(result);
        let result = is_anagram(String::from("banana"), String::from("abanan"));
        assert!(result);
        let result = is_anagram(String::from("rat"), String::from("car"));
        assert!(!result);
    }
}
