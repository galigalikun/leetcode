fn main() {
    assert_eq!(Solution::min_changes(vec![1, 2, 0, 3, 0], 1), 3);
    assert_eq!(Solution::min_changes(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
    assert_eq!(Solution::min_changes(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
}

struct Solution;
impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut f = vec![std::i32::MAX as i128; 1 << 10];
        f[0] = 0;
        for i in 0..k {
            let mut cnt = std::collections::HashMap::new();
            let mut size = 0;
            for j in (i..n).step_by(k) {
                *cnt.entry(nums[j]).or_insert(0) += 1;
                size += 1;
            }
            let g = f.iter().map(|&x| x + size).collect::<Vec<_>>();
            for mask in 0..1024 {
                for (&x, &cntx) in cnt.iter() {
                    f[mask ^ x as usize] = f[mask ^ x as usize].min(g[mask] - cntx);
                }
            }
        }
        f[0] as i32
    }
}
