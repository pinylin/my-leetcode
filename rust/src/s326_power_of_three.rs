/*
 * @lc app=leetcode id=326 lang=rust
 *
 * [326] Power of Three
 */
impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n != 0 && n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

