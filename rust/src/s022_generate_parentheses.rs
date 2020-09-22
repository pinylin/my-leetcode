/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = Vec::new();
        Solution::helper(&mut ret, n, n, "".to_string());
        ret
    }

    fn helper(ret: &mut Vec<String>, c: i32, o: i32, cur: String) {
        if c > o {
            return;
        }
        if c == 0 && o == 0 {
            ret.push(cur);
            return;
        }

        if c > 0 {
            Solution::helper(ret, c - 1, o, cur.clone() + "(");
        }
        if o > 0 {
            Solution::helper(ret, c, o - 1, cur + ")");
        }
    }
}
// @lc code=end

impl Solution {
    // 和这个大佬说的dp很像
    // https://leetcode.com/problems/generate-parentheses/discuss/209410/c%2B%2B-dynamic-programming-(0ms)
    // 设 a[n] = generate_parenthesis(n)
    // a[0] = ""
    // a[n] = "(" + a[x] + ")" + a[y], x + y = n - 1
    pub fn generate_parenthesis_cn(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_owned()];
        }
        let mut ret = vec![];
        for c in 0..n {
            for left in Solution::generate_parenthesis_cn(c) {
                for right in Solution::generate_parenthesis_cn(n - c - 1) {
                    ret.push(format!("({}){}", left, right));
                }
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_string;

    #[test]
    fn test() {
        let mut ret = Solution::generate_parenthesis(3);
        ret.sort_unstable();
        assert_eq!(
            ret,
            vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test_cn() {
        let mut ret = Solution::generate_parenthesis_cn(3);
        ret.sort_unstable();
        assert_eq!(
            ret,
            vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn recursion(b: &mut Bencher) {
        b.iter(|| Solution::generate_parenthesis(4));
    }

    #[bench]
    fn clourse_number(b: &mut Bencher) {
        b.iter(|| Solution::generate_parenthesis_cn(4));
    }
}
