fn main() {
    assert_eq!(Solution::best_rotation(vec![2,3,1,4,0]), 3);
    assert_eq!(Solution::best_rotation(vec![1,3,0,2,4]), 0);
}

struct Solution{}
impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let v1 = &nums[i..];
            let v2 = &nums[0..i];
            println!("debug {:?} {:?}", v1, v2);
        }
        return 0;
    }
}
