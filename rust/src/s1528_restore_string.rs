/*
 * @lc app=leetcode.cn id=1528 lang=rust
 *
 * [1528] 重新排列字符串
 */

// @lc code=start
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut arr = vec![' '; indices.len()];
        // let s_char: Vec<char> = s.chars().collect();
        // for (i, v) in indices.into_iter().enumerate() {
        //     arr[v as usize] = s_char[i];
        // }
        // arr.into_iter().collect()

        indices
            .iter()
            .zip(s.chars())
            .for_each(|(&x, ch)| arr[x as usize] = ch);
        arr.into_iter().collect()
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    // use crate::utils::vec2d;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0]),
            "nihao".to_string()
        );
        assert_eq!(
            Solution::restore_string("aaiougrt".to_string(), vec![4, 0, 2, 6, 7, 3, 1, 5]),
            "arigatou".to_string()
        );
        assert_eq!(
            Solution::restore_string("art".to_string(), vec![1, 0, 2]),
            "rat".to_string()
        );
    }
}
