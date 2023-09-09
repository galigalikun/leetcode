fn main() {
    assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
    assert_eq!(Solution::maximum_sum(vec![1, -2, -2, 3]), 3);
    assert_eq!(Solution::maximum_sum(vec![-1, -1, -1, -1]), -1);
}

struct Solution;
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let mut max = arr[0];
        let mut max_ending_here = arr[0];
        let mut max_so_far = arr[0];
        let mut skip = false;
        for i in 1..arr.len() {
            if arr[i] < 0 {
                if skip {
                    max_ending_here = arr[i];
                    max_so_far = arr[i];
                    skip = false;
                } else {
                    max_ending_here = std::cmp::max(max_ending_here + arr[i], arr[i]);
                    max_so_far = std::cmp::max(max_so_far, max_ending_here);
                }
            } else {
                if skip {
                    max_ending_here = std::cmp::max(max_ending_here + arr[i], arr[i]);
                    max_so_far = std::cmp::max(max_so_far, max_ending_here);
                } else {
                    max_ending_here = std::cmp::max(max_ending_here + arr[i], arr[i]);
                    max_so_far = std::cmp::max(max_so_far, max_ending_here);
                    skip = true;
                }
            }
            max = std::cmp::max(max, arr[i]);
        }
        if max < 0 {
            return max;
        }
        if max_so_far > 0 {
            return max_so_far;
        }
        return max;
    }
}
