fn main() {
    assert_eq!(Solution::closest_to_target(vec![9, 12, 3, 7, 15], 5), 2);
    assert_eq!(
        Solution::closest_to_target(vec![1000000, 1000000, 1000000], 1),
        999999
    );
    assert_eq!(Solution::closest_to_target(vec![1, 2, 4, 8, 16], 0), 0);
}

struct Solution;
impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut res = i32::MAX;
        let mut s = vec![];
        for &x in arr.iter() {
            let mut t = vec![x];
            for &y in s.iter() {
                t.push(x & y);
            }
            s = t;
            for &y in s.iter() {
                res = res.min((target - y).abs());
            }
        }
        res
    }
}
