fn main() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    assert_eq!(Solution::first_missing_positive(vec![]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
}

struct Solution {}
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut n = nums;
        n.sort();
        let mut asc = n.into_iter().filter(|&x| x > 0);
        if let Some(f) = asc.next() {
            if f > 1 {
                return 1;
            }
            let mut i = f;
            while let Some(j) = asc.next() {
                i += 1;
                if j > i {
                    return i;
                }
                i = j;
            }
            return i + 1;
        }

        return 1;
    }
}
