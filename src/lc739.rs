pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = std::iter::repeat(0).take(temperatures.len()).collect::<Vec<i32>>();
    let mut stk = Vec::<usize>::with_capacity(temperatures.len());
    for i in 0..temperatures.len() {
        while let Some(pos) = stk.last() {
            if temperatures[*pos] < temperatures[i] {
                let p = stk.pop().unwrap();
                ans[p] = (i - p) as i32;
            } else {
                break;
            }
        }
        stk.push(i);
    }
    while let Some(pos) = stk.pop() {
        ans[pos] = 0;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        let result = daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
        let result = daily_temperatures(vec![30, 40, 50, 60]);
        assert_eq!(result, vec![1, 1, 1, 0]);
        let result = daily_temperatures(vec![30, 60, 90]);
        assert_eq!(result, vec![1, 1, 0]);
    }
}