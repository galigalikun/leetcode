fn main() {
    assert_eq!(Solution::subarrays_div_by_k(vec![4,5,0,-2,-3,1], 5), 7);
    assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
}

struct Solution;
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut counts = vec![0_i32; k as usize];
        counts[0] = 1;

        let mut prefix = 0_i64;
        let mut answer = 0_i64;

        for num in nums {
            prefix += num as i64;
            let rem = ((prefix % k) + k) % k;
            answer += counts[rem as usize] as i64;
            counts[rem as usize] += 1;
        }

        answer as i32
    }
}
