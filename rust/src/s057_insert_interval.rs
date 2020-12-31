/*
 * @lc app=leetcode.cn id=57 lang=rust
 *
 * [57] 插入区间
 */

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    fn lower_bound_by<T: Ord, F: FnMut(&T) -> bool>(values: &[T], mut f: F) -> usize {
        values
            .binary_search_by(move |value| {
                if f(value) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_err()
    }

    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let start = new_interval[0];
        let end = new_interval[1];

        let i = Self::lower_bound_by(&intervals, |interval| interval[1] < start);
        let mut j = Self::lower_bound_by(&intervals[i..], |interval| interval[0] <= end);

        if j == 0 {
            intervals.insert(i, new_interval);
        } else {
            j += i;
            intervals[i][0] = intervals[i][0].min(start);
            intervals[i][1] = intervals[j - 1][1].max(end);

            intervals.drain(i + 1..j);
        }

        intervals
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
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
