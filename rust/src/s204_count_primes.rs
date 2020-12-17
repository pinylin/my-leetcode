/*
 * @lc app=leetcode.cn id=204 lang=rust
 *
 * [204] 计数质数
 */

// @lc code=start
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let mut nums = vec![true; n as usize];
        for i in 2..=(f64::from(n).sqrt() as usize) {
            if nums[i] {
                for j in (i..n as usize).take_while(|x| x * i < n as usize) {
                    nums[i * j] = false;
                }
            }
        }
        nums.into_iter().filter(|&x| x).count() as i32 - 2
    }
}
// @lc code=end
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_primes(10), 4);
    }
}
