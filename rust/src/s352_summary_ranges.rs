/*
 * @lc app=leetcode.cn id=352 lang=rust
 *
 * [352] 将数据流变为多个不相交区间
 */

// @lc code=start
use std::collections::BTreeMap;
use std::collections::HashSet;
struct SummaryRanges {
    data: BTreeMap<i32, i32>,
    seen: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let data = BTreeMap::new();
        let seen = HashSet::new();
        SummaryRanges { data, seen }
    }

    fn add_num(&mut self, val: i32) {
        if !self.seen.insert(val) {
            return;
        }
        let mut l = val;
        let mut r = val;
        if let Some(&right) = self.data.get(&(val + 1)) {
            r = right;
        }
        if let Some((&left, &limit)) = self.data.range(..val).rev().next() {
            if limit == val - 1 {
                l = left;
            }
        }
        if l < val {
            self.data.remove(&l);
        }
        if r > val {
            self.data.remove(&(val + 1));
        }
        self.data.insert(l, r);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.data.iter().map(|(&k, &v)| vec![k, v]).collect()
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(val);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */
// @lc code=end

#[cfg(test)]
mod test {
    use super::SummaryRanges;
    #[test]
    fn it_works() {
        let mut obj = SummaryRanges::new();
        obj.add_num(1);
        obj.add_num(3);
        obj.add_num(7);
        obj.add_num(2);
        obj.add_num(6);
        assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    }
}
