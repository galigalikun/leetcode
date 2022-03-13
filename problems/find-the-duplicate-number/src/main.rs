fn main() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    assert_eq!(Solution::find_duplicate(vec![1, 1]), 1);
    assert_eq!(Solution::find_duplicate(vec![1, 1, 2]), 1);
}

struct Solution {}
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut work = nums;
        work.sort();
        let mut num: Option<i32> = None;
        for n in &work {
            if num == Some(*n) {
                return *n;
            }
            num = Some(*n);
        }
        return 0;
    }
}
