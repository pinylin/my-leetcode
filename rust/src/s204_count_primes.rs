/*
 * @lc app=leetcode id=204 lang=rust
 *
 * [204] Count Primes
 *
 * https://leetcode.com/problems/count-primes/description/
 *
 * algorithms
 * Easy (28.30%)
 * Total Accepted:    226.4K
 * Total Submissions: 789.9K
 * Testcase Example:  '10'
 *
 * Count the number of prime numbers less than a non-negative number, n.
 * 
 * Example:
 * 
 * 
 * Input: 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 * 
 * 
 */
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 { return 0 }
        let mut nums = vec![true; n as usize];
        for i in 2..((n as f64).sqrt() as usize) + 1 {
            if nums[i] {
                for j in (i..n as usize).take_while(|x| x * i < n as usize) {
                    nums[i * j] = false;
                }
            }
        }
        nums.into_iter().filter(|&x| x == true).count() as i32 - 2
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_primes(10), 4
        );
    }
}
