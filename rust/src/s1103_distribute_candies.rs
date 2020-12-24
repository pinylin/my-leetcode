/*
 * @lc app=leetcode.cn id=1103 lang=rust
 *
 * [1103] 分糖果 II
 */

// @lc code=start
impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut res = vec![0; num_people as usize];
        let mut cur = 0;
        loop {
            for idx in 0..num_people as usize {
                cur += 1;

                if candies - cur < 0 {
                    res[idx] += candies;
                    return res;
                } else {
                    res[idx] += cur
                }
                candies -= cur;
            }
        }
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::distribute_candies(10, 3), vec![5, 2, 3]);
    }
}
