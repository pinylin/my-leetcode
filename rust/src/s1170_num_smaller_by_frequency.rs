/*
 * @lc app=leetcode.cn id=1170 lang=rust
 *
 * [1170] 比较字符串最小字母出现频次
 */
// 
// refer: https://github.com/leshow/exercism/blob/add43ff577246f318d0218b4a36d86c3236c730b/hackerrank/src/compare_strings_by_freq_of_smallest_char.rs 
// @lc code=start
impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        if queries.is_empty() || words.is_empty() {
            return vec![];
        }
        let q = queries.iter().map(|s| Solution::freq(&s[..]));
        let w = words
            .iter()
            .map(|s| Solution::freq(&s[..]))
            .collect::<Vec<_>>();
        let mut ret = Vec::new();
        for count in q {
            ret.push(w.iter().filter(|&&c| c > count).count() as i32);
        }
        ret
    }

    pub fn freq(s: &str) -> usize {
        use std::cmp::Ordering;
        let (count, _smallest) = s.chars().fold((0, 'z'), |(cnt, x), c| match c.cmp(&x) {
            Ordering::Less => (1, c),
            Ordering::Equal => (cnt + 1, x),
            Ordering::Greater => (cnt, x),
        });
        count
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    use crate::vec_string;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::num_smaller_by_frequency(
                vec_string!["bbb", "cc"],
                vec_string!["a", "aa", "aaa", "aaaa"]
            ),
            vec![1, 2]
        );
        assert_eq!(
            Solution::num_smaller_by_frequency(vec_string!["cbd"], vec_string!["zaaaz"]),
            vec![1]
        );
    }
}
