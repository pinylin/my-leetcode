/*
 * @lc app=leetcode.cn id=1380 lang=rust
 *
 * [1380] 矩阵中的幸运数
 */

// @lc code=start
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }
        let mut res = Vec::new();
        // 1. 两层循环
        // for h in 0..matrix.len() {
        //     let min_in_row = matrix[h].iter().enumerate().min_by_key(|x| x.1).unwrap();
        //     let mut ok = true;
        //     for hh in 0..matrix.len() {
        //         if matrix[hh][min_in_row.0] > *min_in_row.1 {
        //             ok = false;
        //             break;
        //         }
        //     }
        //     if ok {
        //         res.push(*min_in_row.1);
        //     }
        // }

        // 2. hashset
        let mut set = std::collections::HashSet::new();
        for row in matrix.iter() {
            set.insert(row.iter().min().unwrap());
        }
        for col in 0..matrix[0].len() {
            let max = matrix.iter().map(|row| row[col]).collect::<Vec<i32>>();

            if let Some(val) = set.get(&max.iter().max().unwrap()) {
                res.push(**val);
            }
        }
        res
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
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17],]),
            vec![15]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
    }
}
