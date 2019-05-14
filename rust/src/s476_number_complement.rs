/*
 * @lc app=leetcode id=476 lang=rust
 *
 * [476] Number Complement
 */
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = !0;
        while (num & mask) > 0 {
            mask <<= 1;
        }
        !mask & !num
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_complement(5), 2);
    }
}
