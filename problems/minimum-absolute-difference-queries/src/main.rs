fn main() {
    assert_eq!(
        Solution::min_difference(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]]
        ),
        vec![2, 1, 4, 1]
    );
    assert_eq!(
        Solution::min_difference(
            vec![4, 5, 2, 2, 7, 10],
            vec![vec![2, 3], vec![0, 2], vec![0, 5], vec![3, 5]]
        ),
        vec![-1, 1, 1, 3]
    );
}

struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        return queries
            .iter()
            .map(|query| {
                let (l, r) = (query[0] as usize, query[1] as usize);
                let mut subarray = nums[l..=r].to_vec();
                subarray.sort_unstable();

                let mut min_diff = i32::MAX;
                for i in 1..subarray.len() {
                    min_diff = min_diff.min(subarray[i] - subarray[i - 1]);
                }

                if min_diff == i32::MAX { -1 } else { min_diff }
            })
            .collect();
    }
}
