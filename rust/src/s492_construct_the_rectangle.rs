/*
 * @lc app=leetcode id=492 lang=rust
 *
 * [492] Construct the Rectangle
 */
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut r = 1;
        let mut i = 1;
        while i * i <= area {
            if area % i == 0 {
                r = i;
            }
            i += 1;
        }
        vec![area / r, r]
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::construct_rectangle(1), vec![1, 1]);
        assert_eq!(Solution::construct_rectangle(5), vec![5, 1]);
    }
}
