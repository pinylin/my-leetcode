/*
 * @lc app=leetcode id=506 lang=rust
 *
 * [506] Relative Ranks
 */
impl Solution {
    // best
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        struct Map {
            key: usize,
            value: i32,
        }
        let mut re: Vec<String> = Vec::with_capacity(nums.len() as usize);
        let mut mapvec: Vec<Map> = Vec::with_capacity(nums.len() as usize);
        for (i, a) in nums.iter().enumerate() {
            re.push("".to_string());
            mapvec.push(Map { key: i, value: *a });
        }

        // sort
        mapvec.sort_by(|a, b| b.value.cmp(&a.value));
        for (rank, m) in mapvec.iter().enumerate() {
            match rank as i32 {
                0 => {
                    re[m.key] = "Gold Medal".to_string();
                }
                1 => {
                    re[m.key] = "Silver Medal".to_string();
                }
                2 => {
                    re[m.key] = "Bronze Medal".to_string();
                }
                _ => {
                    re[m.key] = (rank as u32 + 1).to_string();
                }
            }
        }
        re
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
    }
}
