fn main() {
    assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
    assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    assert_eq!(Solution::max_frequency_elements(vec![15]), 1);
}

struct Solution;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freq = std::collections::HashMap::new();
        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }
        let mut max_freq = 0;
        for &count in freq.values() {
            if count > max_freq {
                max_freq = count;
            }
        }
        let mut count_max_freq = 0;
        for &count in freq.values() {
            if count == max_freq {
                count_max_freq += 1;
            }
        }
        if count_max_freq > 1 {
            return max_freq * count_max_freq;
        }
        return 0;
    }
}
