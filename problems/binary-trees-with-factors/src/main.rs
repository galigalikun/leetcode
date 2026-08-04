fn main() {
    assert_eq!(Solution::num_factored_binary_trees(vec![2,4]), 3);
    assert_eq!(Solution::num_factored_binary_trees(vec![2,4,5,10]), 7);
}

struct Solution{}
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut sorted = arr;
        sorted.sort_unstable();

        let mut index_by_value = std::collections::HashMap::new();
        for (index, &value) in sorted.iter().enumerate() {
            index_by_value.insert(value, index);
        }

        let mut ways = vec![1_i64; sorted.len()];

        for i in 0..sorted.len() {
            for j in 0..i {
                let left = sorted[j];
                let root = sorted[i];

                if root % left != 0 {
                    continue;
                }

                let right = root / left;
                if let Some(&k) = index_by_value.get(&right) {
                    if k < i {
                        ways[i] = (ways[i] + (ways[j] * ways[k]) % MOD) % MOD;
                    }
                }
            }
        }

        (ways.iter().fold(0_i64, |acc, &count| (acc + count) % MOD)) as i32
    }
}
