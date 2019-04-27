/*
 * @lc app=leetcode id=387 lang=rust
 *
 * [387] First Unique Character in a String
 */
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // 看了下best solution,发现之前把 char转usize 再减97 是多么弱智的操作呀
        // 得快点改
        let mut band = vec![0; 26];
        for ch in s.bytes() {
            band[(ch - b'a') as usize] += 1;
        }
        for (i, ch) in s.bytes().enumerate() {
            if band[(ch - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::first_uniq_char("abcd".to_owned()), 0);
        assert_eq!(Solution::first_uniq_char("leetcode".to_owned()), 0);
    }
}
