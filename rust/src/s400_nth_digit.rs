/*
 * @lc app=leetcode id=400 lang=rust
 *
 * [400] Nth Digit
 */
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        //计算目标数的位数
        let mut n = i64::from(n);
        let mut digits = 1; //单个数的位数
        let mut base = 9; //某位数的个数 ,为了满足题目n的范围要求，设置为i64型
        while n > (base * digits) as i64 {
            n -= base * digits; //不断减去k位数的个数，直到到目标数的位数
            base *= 10;
            digits += 1; //当前位数
        } //跳出循环时，digits为目标数位数,n为剩余需要计数的总位数

        //计算是哪个目标数
        let mut objnum = 10i64.pow(digits as u32 - 1); //当前数位中第一个数，如1、10、100
                                                       // let mut  objnum = firstnum.clone();
        let mut index = n % digits; //index标识要找的数在目标数中哪个位置(从左往右数)
        if index == 0 {
            //说明在目标数的最后一位
            index = digits;
            objnum += n / digits - 1; //注意减1，如100，n = 3，则在100最后一位
        } else {
            objnum += n / digits; //如 n=4,则在101第一位
        }
        //找出是目标数中哪个数字
        for _ in index..digits {
            objnum /= 10; //除到目标位为个位
        }
        objnum as i32 % 10
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(1000000000), 1);
    }
}
