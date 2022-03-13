fn main() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
        9
    );
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
}

struct Solution {}
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut work = nums;
        work.sort_unstable();
        work.dedup();
        let mut prev = work[0];
        let mut result = 1;
        let mut count = 1;
        for i in 1..work.len() {
            if prev + 1 == work[i] {
                count += 1;
            } else {
                count = 1;
            }
            println!("debug {}", count);
            result = std::cmp::max(count, result);
            prev = work[i];
        }
        return result;
    }
}
