/*
 * @lc app=leetcode id=504 lang=rust
 *
 * [504] Base 7
 */
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num < 0 {
            return "-".to_owned() + &Solution::convert_to_base7(-num);
        }
        if num < 7 {
            return num.to_string();
        }
        Solution::convert_to_base7(num / 7) + &(num % 7).to_string()
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::convert_to_base7(100), "202".to_owned());
        assert_eq!(Solution::convert_to_base7(-7), "-10".to_owned());
    }
}
