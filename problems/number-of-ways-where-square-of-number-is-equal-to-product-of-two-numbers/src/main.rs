fn main() {
    assert_eq!(Solution::num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
    assert_eq!(Solution::num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
    assert_eq!(
        Solution::num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]),
        2
    );
}

struct Solution;
impl Solution {
    fn count_triplets(num: i32, nums: &Vec<i32>) -> i32 {
        let mut res = 0;
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let target = num as i64 * num as i64 / nums[i] as i64;
            if let Some(&v) = map.get(&target) {
                res += v;
            }
            *map.entry(nums[i] as i64).or_insert(0) += 1;
        }
        res
    }
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums1.len() {
            res += Self::count_triplets(nums1[i], &nums2);
        }
        for i in 0..nums2.len() {
            res += Self::count_triplets(nums2[i], &nums1);
        }
        res
    }
}
