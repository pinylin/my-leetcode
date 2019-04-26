/*
 * @lc app=leetcode id=342 lang=rust
 *
 * [342] Power of Four
 */
impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        while n != 0 && n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

