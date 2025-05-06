fn main() {
    assert_eq!(Solution::get_maximum_xor(vec![0,1,1,3], 2), vec![0,3,2,3]);
    assert_eq!(Solution::get_maximum_xor(vec![2,3,4,7], 3), vec![5,2,6,5]);
    assert_eq!(Solution::get_maximum_xor(vec![0,1,2,2,5,7], 3), vec![4,3,6,4,6,7]);
}

struct Solution;
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        return (0..nums.len())
            .map(|_i| nums.iter().fold(0, |acc, &num| acc ^ num) ^ ((1 << maximum_bit) - 1))
            .collect();
    }
}
