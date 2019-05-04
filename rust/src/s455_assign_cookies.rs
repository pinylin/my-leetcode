/*
 * @lc app=leetcode id=455 lang=rust
 *
 * [455] Assign Cookies
 */
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut j = 0;
        for i in 0..s.len() {
            if j < g.len() && s[i] >= g[j] {
                j += 1;
            }
        }
        j as i32
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }
}
