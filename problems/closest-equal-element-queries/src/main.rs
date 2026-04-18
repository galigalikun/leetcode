fn main() {
    assert_eq!(
        Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]),
        vec![2, -1, 3]
    );
    assert_eq!(
        Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]),
        vec![-1, -1, -1, -1]
    );
}

struct Solution;
impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for q in queries {
            let mut idx = None;
            for (i, n) in nums.iter().enumerate() {
                if *n == q {
                    idx = Some(i);
                    break;
                }
            }
            if let Some(i) = idx {
                let mut next_idx = None;
                for (j, n) in nums.iter().enumerate().skip(i + 1) {
                    if *n == q {
                        next_idx = Some(j);
                        break;
                    }
                }
                ans.push(next_idx.map_or(-1, |j| (j - i) as i32));
            } else {
                ans.push(-1);
            }
        }
        ans
    }
}
