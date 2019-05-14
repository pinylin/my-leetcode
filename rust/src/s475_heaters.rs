/*
 * @lc app=leetcode id=475 lang=rust
 *
 * [475] Heaters
 */
impl Solution {
    //  我没什么好想法
    // refer:  https://github.com/kiyonlin/leetcode/blob/dd2f4e8890bbb50f948c262b96bc954c6e9310b5/algorithms/rust/475.heaters.rs
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort();
        heaters.sort();
        let mut ret = vec![std::i32::MAX; houses.len()];
        let mut i = 0;
        let mut j = 0;
        // 正向遍历，计算每个房子最近的加热器
        while i < houses.len() && j < heaters.len() {
            if houses[i] <= heaters[j] {
                ret[i] = heaters[j] - houses[i];
                i += 1;
            } else {
                j += 1;
            }
        }
        i = houses.len() - 1;
        j = heaters.len() - 1;
        // 逆向遍历，更新每个房子最近的加热器
        loop {
            if houses[i] >= heaters[j] {
                ret[i] = std::cmp::min(ret[i], houses[i] - heaters[j]);
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            } else {
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
        }

        *ret.iter().max().unwrap_or(&0i32)
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
        assert_eq!(Solution::find_radius(vec![1, 5], vec![10]), 9);
        // assert_eq!(Solution::find_radius(vec![1, 5], vec![2]), 3);
        // assert_eq!(Solution::find_radius(vec![617819336,399125485,156091745,356425228], vec![585640194,937186357]), 585640193);   // 这个就奇怪了，， 明明是ack了的 这个test
        assert_eq!(Solution::find_radius(vec![1, 2, 3, 5, 15], vec![2, 30]), 13);
        // assert_eq!(Solution::find_radius(vec![1, 10], vec![2, 9]), 3);
        assert_eq!(Solution::find_radius(vec![1], vec![1, 2, 3, 4]), 0);
        assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![1, 2, 3]), 0);
        assert_eq!(
            Solution::find_radius(
                vec![
                    282475249, 622650073, 984943658, 144108930, 470211272, 101027544, 457850878,
                    458777923
                ],
                vec![
                    823564440, 115438165, 784484492, 74243042, 114807987, 137522503, 441282327,
                    16531729, 823378840, 143542612
                ]
            ),
            161834419
        );
    }
}
