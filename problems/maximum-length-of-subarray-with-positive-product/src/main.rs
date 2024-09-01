fn main() {
    assert_eq!(Solution::get_max_len(vec![1,-2,-3,4]), 4);
    assert_eq!(Solution::get_max_len(vec![0,1,-2,-3,-4]), 3);
    assert_eq!(Solution::get_max_len(vec![-1,-2,-3,0,1]), 2);
}

struct Solution;
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut pos = 0;
        let mut neg = 0;
        for num in nums {
            if num > 0 {
                pos += 1;
                if neg > 0 {
                    neg += 1;
                }
            } else if num < 0 {
                let temp = pos;
                pos = neg + 1;
                neg = temp + 1;
            } else {
                pos = 0;
                neg = 0;
            }
            max_len = max_len.max(pos);
        }
        max_len
    }
}
