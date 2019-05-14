/*
 * @lc app=leetcode id=485 lang=rust
 *
 * [485] Max Consecutive Ones
 */
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        let mut res = 0;
        for item in nums.iter() {
            if item == &1 {
                counter += 1;
            } else {
                res = std::cmp::max(res, counter);
                counter = 0;
            }
        }
        std::cmp::max(res, counter)
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![0, 3, 1, 5, 52, 1, 1]),
            2
        );
    }
}
