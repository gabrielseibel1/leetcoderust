#[cfg(test)]
use std::cmp::Ordering;

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }
        if !has_rotation(&nums) {
            return binary_search(&nums, target, 0, nums.len() - 1);
        }
        let p = find_pivot(&nums);
        return if target >= nums[0] && target <= nums[p.floor() as usize] {
            // target is in left side
            binary_search(&nums, target, 0, p.floor() as usize)
        } else {
            // target is in right side
            binary_search(&nums, target, p.ceil() as usize, nums.len() - 1)
        };
    }
}

#[cfg(test)]
fn has_rotation(n: &[i32]) -> bool {
    return n[0] > n[n.len() - 1];
}

#[cfg(test)]
fn find_pivot(n: &[i32]) -> f32 {
    let (mut l, mut r) = (0, n.len() - 1);
    while r != l + 1 {
        let m = (l + r) as f32 / 2f32;
        if n[l] > n[m.floor() as usize] {
            // there is a pivot between i and m
            r = m.ceil() as usize;
        } else {
            // there is a pivot between m and j
            l = m.floor() as usize;
        }
    }
    return (l + r) as f32 / 2f32;
}

#[cfg(test)]
fn binary_search(n: &[i32], t: i32, l: usize, r: usize) -> i32 {
    if n[l] > t || t > n[r] {
        return -1;
    }
    if l == r {
        return l as i32;
    }
    let m = (l + r) / 2;
    match t.cmp(&n[m]) {
        Ordering::Equal => binary_search(n, t, m, m),
        Ordering::Less => binary_search(n, t, l, m - 1),
        Ordering::Greater => binary_search(n, t, m + 1, r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        assert_eq!(-1, Solution::search(vec![1], 0));
        assert_eq!(-1, Solution::search(vec![1, 3, 5], 2));
    }
}