/*
 * @lc app=leetcode id=461 lang=rust
 *
 * [461] Hamming Distance
 */
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut res = 0;
        let mut exec = x ^ y;
        while exec != 0 {
            res += 1;
            exec &= exec - 1;
        }
        res
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }
}
