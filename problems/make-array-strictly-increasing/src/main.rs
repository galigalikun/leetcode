fn main() {
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]),
        1
    );
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]),
        2
    );
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]),
        -1
    );
    assert_eq!(Solution::make_array_increasing(vec![1, 1], vec![]), -1);
    assert_eq!(Solution::make_array_increasing(vec![1, 1], vec![2]), 1);
}

struct Solution;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        arr2.sort_unstable();
        arr2.dedup();

        let mut values = arr1.clone();
        values.extend(arr2.iter().copied());
        values.sort_unstable();
        values.dedup();

        let inf = i32::MAX / 2;
        let mut dp = vec![inf; values.len()];

        let first_index = values.binary_search(&arr1[0]).unwrap();
        dp[first_index] = 0;
        for &value in &arr2 {
            let index = values.binary_search(&value).unwrap();
            dp[index] = dp[index].min(1);
        }

        for i in 1..arr1.len() {
            let mut prefix = vec![inf; values.len() + 1];
            let mut minimum = inf;
            for (index, &cost) in dp.iter().enumerate() {
                prefix[index] = minimum;
                minimum = minimum.min(cost);
            }
            prefix[values.len()] = minimum;

            let mut next = vec![inf; values.len()];
            let keep_index = values.binary_search(&arr1[i]).unwrap();
            next[keep_index] = prefix[keep_index];

            for &value in &arr2 {
                let replace_index = values.binary_search(&value).unwrap();
                let cost = if prefix[replace_index] == inf {
                    inf
                } else {
                    prefix[replace_index] + 1
                };
                next[replace_index] = next[replace_index].min(cost);
            }
            dp = next;
        }

        let answer = dp.into_iter().min().unwrap();
        if answer == inf {
            -1
        } else {
            answer
        }
    }
}
