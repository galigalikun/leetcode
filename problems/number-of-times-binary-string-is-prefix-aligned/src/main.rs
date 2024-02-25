fn main() {
    assert_eq!(Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]), 2);
    assert_eq!(Solution::num_times_all_blue(vec![4, 1, 2, 3]), 1);
    assert_eq!(Solution::num_times_all_blue(vec![2, 1, 4, 3, 6, 5]), 3);
}

struct Solution;
impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut max = 0;
        for i in 0..flips.len() {
            max = max.max(flips[i]);
            if max == i as i32 + 1 {
                res += 1;
            }
        }
        return res;
    }
}
