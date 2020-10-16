/*
 * @lc app=leetcode id=345 lang=rust
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut indexes: Vec<usize> = Vec::new();

        for (index, ch) in chars.iter().enumerate() {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => indexes.push(index),
                _ => {}
            }
        }

        if indexes.len() == 0 || indexes.len() == 1 {
            return s;
        }

        let mut i = 0_usize;
        let mut j = indexes.len() - 1_usize;
        while i < j {
            chars.swap(indexes[i], indexes[j]);
            i += 1;
            j -= 1;
        }
        chars.iter().collect()
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_vowels("".to_string()), "".to_string());
        assert_eq!(Solution::reverse_vowels("a".to_string()), "a".to_string());
        assert_eq!(Solution::reverse_vowels("aa".to_string()), "aa".to_string());
        assert_eq!(
            Solution::reverse_vowels("aio".to_string()),
            "oia".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("hello".to_string()),
            "holle".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("hEllo".to_string()),
            "hollE".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("   ".to_string()),
            "   ".to_string()
        );
    }
}
