/*
 * @lc app=leetcode.cn id=1299 lang=rust
 *
 * [1299] 将每个元素替换为右侧最大元素
 */

// @lc code=start
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut max = i32::MIN;
        for num in arr.iter().rev() {
            if res.len() == 0 {
                res.push(-1);
                max = *num;
            } else {
                res.push(max);
                max = std::cmp::max(max, *num);
            }
        }
        res.reverse();
        res
    }

    pub fn replace_elements_insert(arr: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut max = i32::MIN;
        for num in arr.iter().rev() {
            if res.len() == 0 {
                res.push(-1);
                max = *num;
            } else {
                res.insert(0, max);
                max = std::cmp::max(max, *num);
            }
        }
        res
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );
    }
}

mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn reverse(b: &mut Bencher) {
        b.iter(|| Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]));
    }

    #[bench]
    fn insert(b: &mut Bencher) {
        b.iter(|| Solution::replace_elements_insert(vec![17, 18, 5, 4, 6, 1]));
    }
}
