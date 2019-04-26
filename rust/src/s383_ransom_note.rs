/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = vec![0; 26];
        for &c in magazine.as_bytes() {
            count[(c - 97) as usize] += 1;
        }
        for &c in ransom_note.as_bytes() {
            count[(c - 97) as usize] -= 1;
            if count[(c - 97) as usize] < 0 {
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
