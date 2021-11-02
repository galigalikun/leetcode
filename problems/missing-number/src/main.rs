fn main() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    assert_eq!(Solution::missing_number(vec![1]), 0);
}

pub struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut work = nums;
        work.sort();
        let mut prev = 0;
        for n in work {
            if n != prev {
                return prev;
            }
            prev += 1;
        }
        return prev;
    }
}
