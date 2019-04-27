/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 *
 * https://leetcode.com/problems/length-of-last-word/description/
 *
 * algorithms
 * Easy (32.15%)
 * Total Accepted:    249.6K
 * Total Submissions: 776K
 * Testcase Example:  '"Hello World"'
 *
 * Given a string s consists of upper/lower-case alphabets and empty space
 * characters ' ', return the length of last word in the string.
 *
 * If the last word does not exist, return 0.
 *
 * Note: A word is defined as a character sequence consists of non-space
 * characters only.
 *
 * Example:
 *
 * Input: "Hello World"
 * Output: 5
 *
 *
 */
impl Solution {
    // 81 ns/iter (+/- 1)
    pub fn length_of_last_word(s: String) -> i32 {
        s.bytes()
            .rev()
            .skip_while(|&c| c == b' ')
            .take_while(|&c| c != b' ')
            .count() as i32
    }
    // 221 ns/iter (+/- 42)
    pub fn length_of_last_word_builtin(s: String) -> i32 {
        s.trim()
            .split_whitespace()
            .last()
            .map(str::len)
            .unwrap_or(0) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("a ".to_owned()), 1);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn custom(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word("Hello World Hello World Hello World Hello World Hello World Hello World Hello World".to_owned()));
    }

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word_builtin("Hello World Hello World Hello World Hello World Hello World Hello World Hello World".to_owned()));
    }
}
