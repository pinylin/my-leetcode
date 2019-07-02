impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::VecDeque;
        let mut index = HashMap::new();
        let mut next_larger = vec![-1; nums2.len()];
        let mut stack = VecDeque::new();
        for i in 0..nums2.len() {
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

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
