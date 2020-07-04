// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut cv = s.chars().collect::<Vec<char>>();
        let cl = cv.len();
        for start in (0..cl).step_by((k + k) as usize) {
            let mut i = start;
            let mut j = std::cmp::min(start + k as usize - 1, cl - 1);
            while i < j {
                cv.swap(i, j);
                i += 1;
                j -= 1
            }
        }
        cv.iter().collect::<String>()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse_str("abcdefg".to_string(), 2),
            "bacdfeg".to_string()
        )
    }
}
