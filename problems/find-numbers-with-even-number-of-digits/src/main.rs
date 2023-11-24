fn main() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
}

struct Solution;
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for num in nums {
            if num.to_string().len() % 2 == 0 {
                count += 1;
            }
        }
        return count;
    }
}
