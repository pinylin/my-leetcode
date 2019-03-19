/*
 * @lc app=leetcode id=38 lang=rust
 *
 * [38] Count and Say
 *
 * https://leetcode.com/problems/count-and-say/description/
 *
 * algorithms
 * Easy (39.60%)
 * Total Accepted:    263.1K
 * Total Submissions: 663.1K
 * Testcase Example:  '1'
 *
 * The count-and-say sequence is the sequence of integers with the first five
 * terms as following:
 * 
 * 
 * 1.     1
 * 2.     11
 * 3.     21
 * 4.     1211
 * 5.     111221
 * 
 * 
 * 1 is read off as "one 1" or 11.
 * 11 is read off as "two 1s" or 21.
 * 21 is read off as "one 2, then one 1" or 1211.
 * 
 * Given an integer n where 1 ≤ n ≤ 30, generate the n^th term of the
 * count-and-say sequence.
 * 
 * Note: Each term of the sequence of integers will be represented as a
 * string.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: 1
 * Output: "1"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 4
 * Output: "1211"
 * 
 */
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        match n {
            1 => "1".to_owned(),
            _ => {
                let s = Solution::count_and_say(n-1);
                let s = s.as_bytes();
                let mut ret = String::new();
                let (mut prev, mut cnt) = (s[0], 1);
                for &c in &s[1..] {
                    if c != prev {
                        ret.push_str(&cnt.to_string());
                        ret.push(prev as char);
                        cnt = 0;
                    }
                    prev = c;
                    cnt += 1;
                }
                if cnt != 0 {
                    ret.push_str(&cnt.to_string());
                    ret.push(prev as char);
                }
                ret
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}