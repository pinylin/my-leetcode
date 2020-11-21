/*
 * @lc app=leetcode.cn id=1374 lang=rust
 *
 * [1374] 生成每种字符都是奇数个的字符串
 */

// @lc code=start
impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 != 0 {
            (0..n).map(|_| 'x').collect()
        } else {
            let mut s: String = (0..(n - 1)).map(|_| 'y').collect();
            s.push('z');
            s
        }
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::generate_the_string(3), "xxx".to_owned())
    }
}
