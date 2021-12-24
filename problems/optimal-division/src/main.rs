fn main() {
    assert_eq!(
        Solution::optimal_division(vec![1000, 100, 10, 2]),
        "1000/(100/10/2)"
    );
}

// https://www.tutorialspoint.com/optimal-division-in-cplusplus
struct Solution {}
impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() == 0 {
            return "".to_string();
        } else if nums.len() == 1 {
            return format!("{}", nums[0]);
        } else if nums.len() == 2 {
            return format!("{}/{}", nums[0], nums[1]);
        }
        let mut den = String::new();
        for i in 1..nums.len() {
            den = format!("{}{}", den, nums[i]);
            if i != nums.len() - 1 {
                den = format!("{}/", den);
            }
        }
        return format!("{}/({})", nums[0], den);
    }
}
