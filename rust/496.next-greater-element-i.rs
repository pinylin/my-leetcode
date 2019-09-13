/*
 * @lc app=leetcode id=496 lang=rust
 *
 * [496] Next Greater Element I
 */
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::VecDeque;
        let n = nums2.len();
        let mut index = HashMap::new();
        let mut next_larger = vec![-1; n];
        let mut stack = VecDeque::new();
        for i in 0..n {
            index.insert(nums2[i], i);
            while !stack.is_empty() && nums2[*stack.back().unwrap()] < nums2[i] {
                let index: usize = stack.pop_back().unwrap();
                next_larger[index] = nums2[i];
            }
            stack.push_back(i)
        }
        nums1
            .into_iter()
            .map(|i| next_larger[*index.get(&i).unwrap()])
            .collect()
    }
}

