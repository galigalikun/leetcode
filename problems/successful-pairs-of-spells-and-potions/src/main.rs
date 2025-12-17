fn main() {
    assert_eq!(
        Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
        vec![4, 0, 3]
    );
    assert_eq!(
        Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
        vec![2, 0, 2]
    );
}

struct Solution;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort_unstable();
        let n = potions.len();
        let mut result = Vec::with_capacity(spells.len());
        for &spell in &spells {
            let target = (success + spell as i64 - 1) / spell as i64;
            let mut left = 0;
            let mut right = n;
            while left < right {
                let mid = left + (right - left) / 2;
                if potions[mid] as i64 >= target {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            result.push((n - left) as i32);
        }
        result
    }
}
