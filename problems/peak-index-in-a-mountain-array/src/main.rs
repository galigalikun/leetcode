fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
}

struct Solution;
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = arr.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if arr[mid] < arr[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn finds_peak_in_small_mountain() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    }

    #[test]
    fn finds_peak_in_larger_mountain() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 5, 3, 2, 0]), 1);
    }
}
