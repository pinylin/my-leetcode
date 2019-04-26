/*
 * @lc app=leetcode id=389 lang=rust
 *
 * [389] Find the Difference
 */
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut res: u8 = 0;
        for i in s.chars(){
            res ^= i as u8;
        }
        for i in t.chars(){
            res ^= i as u8;
        }
        res as char
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_owned(), "abcdz".to_owned()),
            'z'
        );
    }
}
