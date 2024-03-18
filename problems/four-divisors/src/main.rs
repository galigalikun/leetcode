fn main() {
    assert_eq!(Solution::sum_four_divisors(vec![21, 4, 7]), 32);
    assert_eq!(Solution::sum_four_divisors(vec![21, 21]), 64);
    assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
}

struct Solution;
impl Solution {
    fn get_divisors_sum(num: i32) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        for i in 1..=num {
            if num % i == 0 {
                sum += i;
                count += 1;
            }
            if count > 4 {
                return 0;
            }
        }
        return if count == 4 { sum } else { 0 };
    }
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for num in nums {
            sum += Self::get_divisors_sum(num);
        }
        return sum;
    }
}
