use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for n in nums {
        if set.contains(&n) {
            return true;
        }
        set.insert(n);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = contains_duplicate(vec![1, 2, 3, 1]);
        assert!(result);
        let result = contains_duplicate(vec![1, 2, 3, 4]);
        assert!(!result);
    }
}
