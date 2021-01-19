/*
 * @lc app=leetcode.cn id=537 lang=rust
 *
 * [537] 复数乘法
 */

// @lc code=start
impl Solution {
    fn parse_string(s: String) -> (i32, i32) {
        let pos = s.find('+').unwrap();
        let real = s[..pos].parse().unwrap();
        let complex = s[pos + 1..s.len() - 1].parse().unwrap();

        (real, complex)
    }

    pub fn complex_number_multiply(a: String, b: String) -> String {
        let (a1, b1) = Self::parse_string(a);
        let (a2, b2) = Self::parse_string(b);
        let (a, b) = (a1 * a2 - b1 * b2, a1 * b2 + a2 * b1);
        format!("{:0}+{:0}i", a, b)
    }
}
// @lc code=end

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            Solution::complex_number_multiply("1+-1i".to_owned(), "1+-1i".to_owned()),
            "0+-2i".to_owned()
        );
    }
}
