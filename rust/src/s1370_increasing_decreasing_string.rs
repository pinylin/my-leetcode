/*
 * @lc app=leetcode.cn id=1370 lang=rust
 *
 * [1370] 上升下降字符串
 */

// @lc code=start
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut arr = vec![0; 26];
        let mut res = Vec::new();
        for c in s.chars() {
            arr[c as usize - 'a' as usize] += 1;
        }
        while res.len() < s.len() {
            for i in 0..26 {
                if arr[i] > 0 {
                    res.push(i as u8 + b'a');
                    arr[i] -= 1;
                }
            }
            for i in (0..26).rev() {
                if arr[i] > 0 {
                    res.push(i as u8 + b'a');
                    arr[i] -= 1;
                }
            }
        }

        String::from_utf8(res).unwrap()
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::sort_string("aaaabbbbcccc".to_owned()),
            "abccbaabccba".to_owned()
        )
    }
}
