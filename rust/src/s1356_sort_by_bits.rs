/*
 * @lc app=leetcode.cn id=1356 lang=rust
 *
 * [1356] 根据数字二进制下 1 的数目排序
 */

// @lc code=start
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        use std::iter;
        let mut result: Vec<Vec<i32>> = iter::repeat(vec![]).take(16).collect();
        arr.iter()
            .for_each(|n| result[n.count_ones() as usize].push(*n));
        result
            .into_iter()
            .map(|mut s| {
                s.sort();
                s
            })
            .flatten()
            .collect()
    }

    pub fn sort_by_bits_2(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| {
            Solution::count1(*a)
                .cmp(&Solution::count1(*b))
                .then(a.cmp(b))
        });
        arr
    }

    pub fn count1(num: i32) -> i32 {
        let (mut num, mut res) = (num, 0);
        while num > 0 {
            if (num & 1) == 1 {
                res += 1
            }
            num >>= 1;
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
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![10000, 10000]),
            vec![10000, 10000]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]),
            vec![2, 3, 5, 17, 7, 11, 13, 19]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![10, 100, 1000, 10000]),
            vec![10, 100, 10000, 1000]
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;
    #[bench]
    fn add_binary(b: &mut Bencher) {
        b.iter(|| Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]));
        b.iter(|| Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]));
        b.iter(|| Solution::sort_by_bits(vec![1111,7644,1107,6978,8742,1,7403,7694,9193,4401,377,8641,5311,624,3554,6631]));
    }

    #[bench]
    fn add_binary_2(b: &mut Bencher) {
        b.iter(|| Solution::sort_by_bits_2(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]));
        b.iter(|| Solution::sort_by_bits_2(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]));
        b.iter(|| Solution::sort_by_bits_2(vec![1111,7644,1107,6978,8742,1,7403,7694,9193,4401,377,8641,5311,624,3554,6631]));
    }
}
