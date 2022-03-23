fn main() {
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]), true);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 4, 4, 5]), false);
}

struct Solution {}
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut ans: Vec<(i32, i32)> = vec![];
        for n in nums {
            if let Some(mut v) = ans.pop() {
                if v.1 == n - 1 {
                    v.1 = n;
                } else {
                    ans.push(v);
                    ans.push((n, n));
                }
            } else {
                ans.push((n, n));
            }
        }
        println!("debug {:?}", ans);
        return false;
    }
}
