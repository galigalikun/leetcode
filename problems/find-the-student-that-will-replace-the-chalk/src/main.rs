fn main() {
    assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
    assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
}

struct Solution;
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        return chalk
            .iter()
            .scan(0, |sum, &x| {
                *sum += x;
                Some(*sum)
            })
            .take_while(|&x| x <= k)
            .count() as i32;
    }
}
