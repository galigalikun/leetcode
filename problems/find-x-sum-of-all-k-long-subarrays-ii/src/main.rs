fn main() {
    assert_eq!(
        Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
        vec![6, 10, 12]
    );
    assert_eq!(
        Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
        vec![11, 15, 15, 15, 12]
    );
}

struct Solution;
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let n = nums.len();
        for i in 0..n - k as usize + 1 {
            let mut sum = 0i64;
            for j in i..i + k as usize {
                sum += nums[j] as i64;
            }
            if sum == x as i64 {
                let mut res = vec![];
                for m in 0..n - k as usize + 1 {
                    let mut sub_sum = 0i64;
                    for j in m..m + k as usize {
                        sub_sum += nums[j] as i64;
                    }
                    res.push(sub_sum);
                }
                return res;
            }
        }
        return vec![];
    }
}
