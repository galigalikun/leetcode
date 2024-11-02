fn main() {
    assert_eq!(
        Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
        vec![3, 1, 1, 2, 2, 2]
    );
    assert_eq!(
        Solution::frequency_sort(vec![2, 3, 1, 3, 2]),
        vec![1, 3, 3, 2, 2]
    );
    assert_eq!(
        Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
        vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
    );
}

struct Solution;
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut vec: Vec<_> = map.into_iter().collect();
        vec.sort_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        vec.into_iter()
            .flat_map(|(num, count)| vec![num; count])
            .collect()
    }
}
