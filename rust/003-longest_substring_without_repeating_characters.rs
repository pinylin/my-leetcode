use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let (mut i, mut ans) = (0, 0);
        for (j, c) in s.chars().enumerate() {
            i = i.max(*map.get(&c).unwrap_or(&0));
            ans = ans.max(j - i + 1);
            map.insert(c, j + 1);
        }
        ans as i32
    }
}