fn main() {
    assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 6]), 1);
    assert_eq!(Solution::count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    assert_eq!(Solution::count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
}

struct Solution;
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        return (0..nums.len())
            .map(|d| {
                let mut count = 0;
                for a in 0..d {
                    for b in a + 1..d {
                        for c in b + 1..d {
                            if nums[a] + nums[b] + nums[c] == nums[d] {
                                count += 1;
                            }
                        }
                    }
                }
                count
            })
            .sum();
    }
}
