fn main() {
    assert_eq!(Solution::count_triplets(vec![2, 1, 3]), 12);
    assert_eq!(Solution::count_triplets(vec![0, 0, 0]), 27);
}

struct Solution;
impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let max_num = nums.iter().copied().max().unwrap_or(0) as u32;
        let max_bit = if max_num == 0 { 0 } else { max_num.ilog2() as usize + 1 };

        if max_bit <= 16 {
            let limit = 1usize << max_bit;
            let mut freq = vec![0i64; limit];
            for &num in &nums {
                freq[num as usize] += 1;
            }

            let mut superset_count = freq;
            for bit in 0..max_bit {
                let step = 1usize << bit;
                for mask in 0..limit {
                    if mask & step != 0 {
                        let prev = mask ^ step;
                        superset_count[prev] += superset_count[mask];
                    }
                }
            }

            let mut non_zero_triplets = 0i64;
            for mask in 1..limit {
                let count = superset_count[mask];
                if count == 0 {
                    continue;
                }
                let triple_count = count.pow(3);
                if mask.count_ones() % 2 == 1 {
                    non_zero_triplets += triple_count;
                } else {
                    non_zero_triplets -= triple_count;
                }
            }

            return (n.pow(3) - non_zero_triplets) as i32;
        }

        let mut ans = 0i32;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                let mask = nums[i] & nums[j];
                for k in 0..nums.len() {
                    if (mask & nums[k]) == 0 {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}

