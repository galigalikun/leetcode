fn main() {
    assert_eq!(
        Solution::check_arithmetic_subarrays(vec![4, 6, 5, 9, 3, 7], vec![0, 0, 2], vec![2, 3, 5]),
        vec![true, false, true]
    );
    assert_eq!(
        Solution::check_arithmetic_subarrays(
            vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
            vec![0, 1, 6, 4, 8, 7],
            vec![4, 4, 9, 7, 9, 10]
        ),
        vec![false, true, false, false, true, true]
    );
}

struct Solution;
impl Solution {
    fn is_arithmetic(arr: &Vec<i32>) -> bool {
        if arr.len() < 2 {
            return false;
        }
        let diff = arr[1] - arr[0];
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] != diff {
                return false;
            }
        }
        true
    }
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut res = vec![];
        for i in 0..l.len() {
            let mut arr = nums[l[i] as usize..=r[i] as usize].to_vec();
            arr.sort();
            res.push(Self::is_arithmetic(&arr));
        }
        res
    }
}
