fn main() {
    assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
    assert_eq!(Solution::maximum_sum(vec![1, -2, -2, 3]), 3);
    assert_eq!(Solution::maximum_sum(vec![-1, -1, -1, -1]), -1);
}

struct Solution;
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        
        // no_del[i] = max sum ending at i with no deletion
        // one_del[i] = max sum ending at i with exactly one deletion
        let mut no_del = arr[0];
        let mut one_del: Option<i32> = None;
        let mut result = arr[0];
        
        for i in 1..n {
            // Update one_del first (needs old no_del)
            let new_one_del = match one_del {
                None => Some(no_del),
                Some(val) => Some(std::cmp::max(no_del, val + arr[i])),
            };
            
            // Update no_del
            // Option 1: start new subarray from current element
            // Option 2: extend no_del with current element
            no_del = std::cmp::max(arr[i], no_del + arr[i]);
            
            one_del = new_one_del;
            
            // Update result
            result = std::cmp::max(result, no_del);
            if let Some(val) = one_del {
                result = std::cmp::max(result, val);
            }
        }
        
        result
    }
}
