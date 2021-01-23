/*
 * @lc app=leetcode.cn id=1319 lang=rust
 *
 * [1319] 连通网络的操作次数
 */

// @lc code=start
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        // init ufs
        let mut ufs = UnionFindSet::new(n as usize);

        let mut rest = 0;
        let mut computers = 0;
        for i in 0..connections.len() {
            if !ufs.union(connections[i][0] as usize, connections[i][1] as usize) {
                rest += 1;
            } else {
                computers += 1;
            }
        }
        let remain = n - computers - 1;
        if remain > rest {
            -1
        } else {
            remain
        }
    }
}
struct UnionFindSet {
    parent: Vec<usize>,
}

impl<'a> UnionFindSet {
    fn new(n: usize) -> Self {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        Self { parent: vec }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false; 
        }
        self.parent[root_y] = root_x;
        true
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
            Solution::make_connected(
                6,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
            ),
            2
        );
    }
}
