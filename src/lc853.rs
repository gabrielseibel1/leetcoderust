#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {

    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // sort p and s
        let target = target as f32;
        let mut psv: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        psv.sort_by(|a, b| { a.0.cmp(&b.0).reverse() });
        let (p, s): (Vec<f32>, Vec<f32>) = psv.into_iter().map(|x| { (x.0 as f32, x.1 as f32) }).unzip();
        let (mut p, mut s) = (std::collections::VecDeque::from(p), std::collections::VecDeque::from(s));


        // make all cars reach dest and count fleets
        let mut f = 0;
        while !p.is_empty() {

            // pass some time: time-to-arrival of first car or some time to collision
            let tta0 = (target - p[0]) / s[0];
            let ttc1 = if p.len() >= 2 { (p[0] - p[1]) / (s[1] - s[0]) } else { f32::INFINITY };
            let t = if ttc1 < 0.0 || ttc1.is_nan() || tta0 < ttc1 || p[0] + s[0] * ttc1 > target {
                tta0
            } else {
                ttc1
            };

            // everyone moves t times clamped to the speed of the car ahead
            p[0] += s[0] * t;
            for k in 1..p.len() {
                let would_be_p_k = p[k] + s[k] * t;
                let max_p_k = p[k - 1];
                if would_be_p_k >= max_p_k {
                    p[k] = max_p_k;
                    s[k] = s[k - 1];
                } else {
                    p[k] = would_be_p_k;
                }
            }

            // pop fleet f
            let mut new_fleet = false;
            while !p.is_empty() && p[0] >= target {
                p.pop_front();
                s.pop_front();
                new_fleet = true
            }
            if new_fleet {
                f += 1;
            }
        }
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let fleets = Solution::car_fleet(20, vec![2, 7, 5], vec![3, 1, 2]);
        assert_eq!(fleets, 1);

        let fleets = Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
        assert_eq!(fleets, 3);

        let fleets = Solution::car_fleet(10, vec![3], vec![3]);
        assert_eq!(fleets, 1);

        let fleets = Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]);
        assert_eq!(fleets, 1);

        let fleets = Solution::car_fleet(10, vec![6, 8], vec![3, 2]);
        assert_eq!(fleets, 2);

        let fleets = Solution::car_fleet(10, vec![4, 6], vec![3, 2]);
        assert_eq!(fleets, 1);

        let fleets = Solution::car_fleet(20, vec![6, 2, 17], vec![3, 9, 2]);
        assert_eq!(fleets, 2);

        let fleets = Solution::car_fleet(13, vec![10, 2, 5, 7, 4, 6, 11], vec![7, 5, 10, 5, 9, 4, 1]);
        assert_eq!(fleets, 2);

        let fleets = Solution::car_fleet(17, vec![8, 12, 16, 11, 7], vec![6, 9, 10, 9, 7]);
        assert_eq!(fleets, 4);

        let fleets = Solution::car_fleet(31, vec![5, 26, 18, 25, 29, 21, 22, 12, 19, 6], vec![7, 6, 6, 4, 3, 4, 9, 7, 6, 4]);
        assert_eq!(fleets, 6);
    }
}