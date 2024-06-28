fn main() {
    assert_eq!(
        Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5),
        true
    );
    assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
    assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
}

struct Solution;
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];
        for num in arr {
            let num = num as usize % k as usize;
            count[num] += 1;
        }
        if count[0] % 2 != 0 {
            return false;
        }
        for i in 1..k as usize {
            if count[i] != count[k as usize - i] {
                return false;
            }
        }

        return true;
    }
}
