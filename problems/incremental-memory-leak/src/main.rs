fn main() {
    assert_eq!(Solution::mem_leak(2, 2), vec![3, 1, 0]);
    assert_eq!(Solution::mem_leak(8, 11), vec![6, 0, 4]);
}

struct Solution;
impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        return vec![memory1 + memory2, if memory1 > memory2 { 0 } else { 1 }];
    }
}
