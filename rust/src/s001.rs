use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result: Vec<i32> = vec![0, 0];

        for (i, n) in nums.iter().enumerate() {
            match map.get(&(target - n)) {
                Some(j) => {
                    result[0] = *j as i32;
                    result[1] = i as i32;
                    break;
                }
                None => {
                    map.insert(n, i);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    }
}
