/*
 * @lc app=leetcode id=521 lang=rust
 *
 * [521] Longest Uncommon Subsequence I 
 */
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b { 
            -1
        } else {
            std::cmp::max(a.len(), b.len()) as i32
        }
        
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_lu_slength("aba".to_owned(), "cdc".to_owned()), 3);
    }
}