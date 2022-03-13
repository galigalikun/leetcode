fn main() {
    assert_eq!(
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
    assert_eq!(
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
        -1
    );
}

// https://programmerstart.com/article/46181979628/
struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut result = vec![0; n];

        let mut res = 0;
        for i in 0..n {
            result[i] = gas[i] - cost[i];
            res += gas[i] - cost[i];
        }
        if res < 0 {
            return -1;
        }
        for i in 0..n {
            let mut start = (i + 1) % n;
            let end = i;
            let mut total = result[i];
            if total < 0 {
                continue;
            }
            while start != end {
                total += result[start];
                if total < 0 {
                    break;
                }
                start = (start + 1) % n;
            }
            if total >= 0 {
                return i as i32;
            }
        }
        return -1;
    }
}
