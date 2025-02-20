fn main() {
    assert_eq!(Solution::min_abs_difference(vec![5, -7, 3, 5], 6), 0);
    assert_eq!(Solution::min_abs_difference(vec![7, -9, 15, -2], -5), 1);
    assert_eq!(Solution::min_abs_difference(vec![1, 2, 3], -7), 7);
}

struct Solution;
impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(0);
        let mut left = HashSet::new();
        let mut right = HashSet::new();
        let n = nums.len();
        for i in 0..n / 2 {
            let mut new_set = HashSet::new();
            for &num in set.iter() {
                new_set.insert(num);
                new_set.insert(num + nums[i]);
                left.insert(num + nums[i]);
            }
            set = new_set;
        }
        let mut set = HashSet::new();
        set.insert(0);
        for i in n / 2..n {
            let mut new_set = HashSet::new();
            for &num in set.iter() {
                new_set.insert(num);
                new_set.insert(num + nums[i]);
                right.insert(num + nums[i]);
            }
            set = new_set;
        }
        let mut left = left.into_iter().collect::<Vec<_>>();
        let mut right = right.into_iter().collect::<Vec<_>>();
        left.sort();
        right.sort();
        let mut ans = i32::MAX;
        let mut i = 0;
        let mut j = right.len() as i32 - 1;
        while i < left.len() && j >= 0 {
            let sum = left[i] + right[j as usize];
            ans = ans.min((goal - sum).abs());
            if sum < goal {
                i += 1;
            } else {
                j -= 1;
            }
        }
        ans
    }
}
