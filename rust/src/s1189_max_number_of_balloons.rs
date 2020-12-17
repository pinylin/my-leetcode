/*
 * @lc app=leetcode.cn id=1189 lang=rust
 *
 * [1189] “气球” 的最大数量
 */

// @lc code=start
impl Solution {
    // 287 ns/iter (+/- 23)
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        for ch in text.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }
        let mut res = 0;
        loop {
            for ch in "balloon".to_string().chars() {
                if map.get(&ch) == None {
                    return res;
                } else if map[&ch] < 1 {
                    return res;
                } else {
                    *map.get_mut(&ch).unwrap() -= 1;
                }
            }
            res += 1;
        }
    }
    //  41 ns/iter (+/- 4) 
    pub fn max_number_of_balloons_2(text: String) -> i32 {
        let mut word = vec![0; 5];
        for ch in text.chars() {
            match ch {
                'b' => word[0] += 1,
                'a' => word[1] += 1,
                'l' => word[2] += 1,
                'o' => word[3] += 1,
                'n' => word[4] += 1,
                _ => (),
            }
        }
        word[2] /= 2;
        word[3] /= 2;
        *word.iter().min().unwrap()
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_owned()), 1);
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".to_owned()),
            2
        );
        assert_eq!(Solution::max_number_of_balloons("leetcode".to_owned()), 0);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn hash_map(b: &mut Bencher) {
        b.iter(|| Solution::max_number_of_balloons("nlaebolko".to_owned()));
        b.iter(|| Solution::max_number_of_balloons("loonbalxballpoon".to_owned()));
        b.iter(|| Solution::max_number_of_balloons("leetcode".to_owned()));
    }

    #[bench]
    fn vec(b: &mut Bencher) {
        b.iter(|| Solution::max_number_of_balloons_2("nlaebolko".to_owned()));
        b.iter(|| Solution::max_number_of_balloons_2("loonbalxballpoon".to_owned()));
        b.iter(|| Solution::max_number_of_balloons_2("leetcode".to_owned()));
    }
}
