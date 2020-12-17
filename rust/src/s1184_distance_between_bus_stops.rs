/*
 * @lc app=leetcode.cn id=1184 lang=rust
 *
 * [1184] 公交站间的距离
 */

// @lc code=start
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let sum: i32 = distance.iter().sum();
        let mut res = 0;
        let (mut from, mut to) = (start, destination);
        if start > destination {
            from = destination;
            to = start;
        }
        for n in from..to {
            res += distance[n as usize];
        }

        res.min(sum - res)
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1),
            1
        );
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2),
            3
        );
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3),
            4
        );
    }
}
