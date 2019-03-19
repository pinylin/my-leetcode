/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 *
 * https://leetcode.com/problems/sqrtx/description/
 *
 * algorithms
 * Easy (30.74%)
 * Total Accepted:    338.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '4'
 *
 * Implement int sqrt(int x).
 * 
 * Compute and return the square root of x, where x is guaranteed to be a
 * non-negative integer.
 * 
 * Since the return type is an integer, the decimal digits are truncated and
 * only the integer part of the result is returned.
 * 
 * Example 1:
 * 
 * 
 * Input: 4
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since 
 * the decimal part is truncated, 2 is returned.
 * 
 * 
 */
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut size = x;
        let mut base = 1;
        if x <= 1{
            return x;
        }
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if mid <= x / mid {
                base = mid;
            }
            size -= half;
        }
        base
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}