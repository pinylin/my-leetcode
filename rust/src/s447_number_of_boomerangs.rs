/*
 * @lc app=leetcode id=447 lang=rust
 *
 * [447] Number of Boomerangs
 */
impl Solution {
    // best
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut res = 0;
        for i in 0..points.len() {
            // all dists start from point i.
            let mut dists: HashMap<i32, i32> = HashMap::new();
            for j in 0..points.len() {
                if j == i {
                    continue;
                }
                let x = points[i][0] - points[j][0];
                let y = points[i][1] - points[j][1];
                let dist = x * x + y * y;
                let cnt = dists.entry(dist).or_insert(0);
                if *cnt > 0 {
                    res += *cnt * 2;
                }
                *cnt += 1;
            }
        }

        res
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::number_of_boomerangs(vec![
                vec![0, 0],
                vec![1, 0],
                vec![-1, 0],
                vec![0, 1],
                vec![0, -1],
            ]),
            20
        );
    }
}
