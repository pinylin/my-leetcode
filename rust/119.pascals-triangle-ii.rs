/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 *
 * https://leetcode.com/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (42.17%)
 * Total Accepted:    193.8K
 * Total Submissions: 453.2K
 * Testcase Example:  '3'
 *
 * Given a non-negative index k where k ≤ 33, return the k^th index row of the
 * Pascal's triangle.
 * 
 * Note that the row index starts from 0.
 * 
 * 
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it.
 * 
 * Example:
 * 
 * 
 * Input: 3
 * Output: [1,3,3,1]
 * 
 * 
 * Follow up:
 * 
 * Could you optimize your algorithm to use only O(k) extra space?
 * 
 */
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut triangle = vec![1;(row_index+1) as usize];
        for i in 1..row_index + 1 {
            let mut prev = 1;
            for j in 1..i {
                let temp = triangle[j as usize];
                triangle[j as usize] = temp + prev;
                prev = temp;
            }
        }
        triangle
    }
    /// best solution 
    // pub fn get_row(row_index: i32) -> Vec<i32> {
    //     if (row_index < 0) {
    //         return Vec::new();
    //     }
        
    //     let mut row = Solution::get_row(row_index - 1);
    //     for i in (1..row.len()).rev() {
    //         row[i] = row[i] + row[i - 1];
    //     }
    //     row.push(1);        
        
    //     row        
    // }
}

