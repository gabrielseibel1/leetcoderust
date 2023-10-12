#[cfg(test)]
use std::cmp::Ordering;

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        if piles.len() == 1 {
            return (piles[0] as f64 / h as f64).ceil() as i32;
        }
        match piles.len().cmp(&(h as usize)) {
            Ordering::Less => binary_search(h, *piles.iter().max().unwrap(), &piles),
            Ordering::Equal => *piles.iter().max().unwrap(),
            Ordering::Greater => panic!("can't eat that many bananas in such short amount of time")
        }
    }
}

#[cfg(test)]
fn binary_search(h: i32, max: i32, piles: &[i32]) -> i32 {
    let (mut i, mut j) = (1, max);
    loop {
        let x = (i + j) as f64 / 2f64;
        let d = x.floor();
        match piles.iter().fold(0i64, |a, b| { a + (*b as f64 / d).ceil() as i64 }).cmp(&(h as i64)) {
            Ordering::Equal | Ordering::Less => {
                // try to find a lower m, or break if its impossible
                if i == j {
                    break;
                }
                j = x.floor() as i32;
            }
            Ordering::Greater => {
                // try to find a greater m
                if i == j {
                    break;
                }
                i = x.ceil() as i32;
            }
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_eating_speed() {
        let res = Solution::min_eating_speed(vec![3, 6, 7, 11], 8);
        assert_eq!(res, 4);

        let res = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
        assert_eq!(res, 30);

        let res = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6);
        assert_eq!(res, 23);

        let res = Solution::min_eating_speed(
            vec![332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673, 679580077, 337406589, 290818316, 877337160, 901728858, 679284947, 688210097, 692137887, 718203285, 629455728, 941802184],
            823855818,
        );
        assert_eq!(res, 14);

        let res = Solution::min_eating_speed(vec![1000000000], 2);
        assert_eq!(res, 500000000);

        let res = Solution::min_eating_speed(vec![312884470], 312884469);
        assert_eq!(res, 2);

        let res = Solution::min_eating_speed(vec![1000000000, 1000000000], 3);
        assert_eq!(res, 1000000000)
    }
}
