fn main() {
    assert_eq!(Solution::magical_sum(5, 5, vec![1,10,100,10000,1000000]), 991600007);
    assert_eq!(Solution::magical_sum(2, 2, vec![5,4,3,2,1]), 170);
    assert_eq!(Solution::magical_sum(1, 1, vec![28]), 28);
}

struct Solution;
impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        return nums.iter()
            .filter(|&&x| x % m == k)
            .sum();
    }
}
