/*
 * @lc app=leetcode.cn id=1252 lang=rust
 *
 * [1252] 奇数值单元格的数目
 */

// @lc code=start
impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        // 方法一：  61 ns/iter (+/- 3)
        let mut res = 0;
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                let mut last = 0;
                for point in indices.iter() {
                    if point[0] == i {
                        last += 1;
                    }
                    if point[1] == j {
                        last += 1;
                    }
                }
                if last % 2 != 0 {
                    res += 1;
                }
            });
        });
        res
    }

    pub fn odd_cells_2(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        // 方法二：107 ns/iter (+/- 8)
        // 构建一个n*m的矩阵，循环indices，得到每个点indice
        // 循环indice(0..N,Mx)和indice(Nx,0..M)，每个点都加上1
        // 循环矩阵的每个点，统计奇数个数
        let mut matrix = vec![vec![0; m as usize]; n as usize];
        for i in 0..indices.len() {
            for j in 0..n as usize {
                matrix[j][indices[i][1] as usize] += 1;
            }
            for k in 0..m as usize {
                matrix[indices[i][0] as usize][k] += 1;
            }
        }
        matrix
            .iter()
            .map(|r| r.iter().filter(|&&p| p % 2 != 0).count() as i32)
            .sum()
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
        assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn traverse(b: &mut Bencher) {
        b.iter(|| Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]));
        b.iter(|| Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]));
    }

    #[bench]
    fn matrix(b: &mut Bencher) {
        b.iter(|| Solution::odd_cells_2(2, 3, vec![vec![0, 1], vec![1, 1]]));
        b.iter(|| Solution::odd_cells_2(2, 2, vec![vec![1, 1], vec![0, 0]]));
    }
}
