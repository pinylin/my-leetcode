/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 *
 * https://leetcode.com/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (44.67%)
 * Total Accepted:    239.8K
 * Total Submissions: 530.2K
 * Testcase Example:  '5'
 *
 * Given a non-negative integer numRows, generate the first numRows of Pascal's
 * triangle.
 * 
 * 
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it.
 * 
 * Example:
 * 
 * 
 * Input: 5
 * Output:
 * [
 * ⁠    [1],
 * ⁠   [1,1],
 * ⁠  [1,2,1],
 * ⁠ [1,3,3,1],
 * ⁠[1,4,6,4,1]
 * ]
 * 
 * 
 */
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = Vec::new();
        if num_rows < 1 { return triangle }
        let mut cur = vec![1];
        for _ in 0..num_rows {
            let mut next = vec![1; cur.len()+1];
            for i in 1..cur.len() {
                next[i] = cur[i-1] + cur[i];
            }
            triangle.push(cur);
            cur = next;
        }
        triangle
    }
}

