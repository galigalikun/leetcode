fn main() {
    assert_eq!(
        Solution::is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4),
        true
    );
    assert_eq!(
        Solution::is_possible_divide(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3),
        true
    );
    assert_eq!(Solution::is_possible_divide(vec![1, 2, 3, 4], 3), false);
}

struct Solution;
impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        let mut nums = nums;
        nums.sort();
        let mut map = std::collections::HashMap::new();
        for num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }
        for num in &nums {
            if let Some(count) = map.get_mut(&num) {
                if *count == 0 {
                    continue;
                }
                for i in 0..k {
                    if let Some(count) = map.get_mut(&(num + i)) {
                        if *count == 0 {
                            return false;
                        }
                        *count -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}
