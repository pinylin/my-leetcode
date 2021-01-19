/*
 * @lc app=leetcode.cn id=1584 lang=rust
 *
 * [1584] 连接所有点的最小费用
 */

// @lc code=start
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = Vec::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                edges.push((
                    i,
                    j,
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(),
                ));
            }
        }
        edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));

        let mut parents = (0..10).collect::<Vec<_>>();

        let mut res = 0;
        for edge in edges {
            let mut a = edge.0;
            let mut b = edge.1;
            while a != parents[a] {
                a = parents[a];
            }
            while b != parents[b] {
                b = parents[b];
            }
            if a == b {
                continue;
            }
            parents[b] = a;
            res += edge.2;
        }
        res
    }
}
// @lc code=end

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![2, 2],
                vec![3, 10],
                vec![5, 2],
                vec![7, 0]
            ]),
            20
        );
    }
}
