/*
 * @lc app=leetcode id=507 lang=rust
 *
 * [507] Perfect Number
 */
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        // 100%
        // return num == 6 || num == 28 || num == 496 || num == 8128 || num == 33550336;
        let mut sum = 1;
        let end = f64::from(num).sqrt().ceil() as i32;
        for i in 2..end {
            if num % i == 0 {
                sum += i + (num / i);
            }
        }
        return num != 1 && sum == num;
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::check_perfect_number(6), true);
        assert_eq!(Solution::check_perfect_number(28), true);
    }
}
