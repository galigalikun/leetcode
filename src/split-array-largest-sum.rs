fn main() {
    assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
    assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
}

pub struct Solution {}
impl Solution {
    fn helper(mid: i32, nums: Vec<i32>, m: i32) -> bool {
        let mut sum = 0;
        let mut count = 0;
        for n in nums {
            if n > mid {
                return false;
            }
            sum += n;
            if sum > mid {
                count += 1;
                sum = n;
            }
        }
        count += 1;
        if count <= m {
            return true;
        }
        return false;
    }
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut start = 1;
        let mut end = 0;
        for n in nums.clone() {
            start = std::cmp::max(start, n);
            end += n;
        }
        let mut result = 0;
        while start <= end {
            let mid = (start + end) / 2;
            if Solution::helper(mid, nums.clone(), m) {
                result = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        return result;
    }
}
