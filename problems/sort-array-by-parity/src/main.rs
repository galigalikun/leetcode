fn main() {
    assert_eq!(
        Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
        vec![2, 4, 3, 1]
    );
    assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
}

struct Solution;
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut even = vec![];
        let mut odd = vec![];
        for n in nums {
            if n % 2 == 0 {
                even.push(n);
            } else {
                odd.push(n);
            }
        }
        even.append(&mut odd);
        return even;
    }
}
