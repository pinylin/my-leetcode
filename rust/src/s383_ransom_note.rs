/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = vec![0; 26];
        for ch in magazine.bytes() {
            count[(ch - b'a') as usize] += 1;
        }
        for ch in ransom_note.bytes() {
            count[(ch - b'a') as usize] -= 1;
            if count[(ch - b'a') as usize] < 0 {
                return false;
            }
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::can_construct("abcd".to_owned(), "abcdz".to_owned()),
            true
        );
    }
}
