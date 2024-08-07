fn main() {
    assert_eq!(Solution::min_swaps(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]]), 3);
    assert_eq!(Solution::min_swaps(vec![vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0]]), -1);
    assert_eq!(Solution::min_swaps(vec![vec![1,0,0],vec![1,1,0],vec![1,1,1]]), 0);
}

struct Solution;
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut zeros = vec![0; n];
        for i in 0..n {
            let mut j = n - 1;
            while j >= 0 && grid[i][j] == 0 {
                zeros[i] += 1;
                j -= 1;
            }
        }
        let mut ans = 0;
        for i in 0..n {
            let mut target = n - i - 1;
            let mut found = false;
            for j in i..n {
                if zeros[j] >= target {
                    found = true;
                    ans += j - i;
                    zeros.remove(j);
                    zeros.insert(i, target);
                    break;
                }
            }
            if !found {
                return -1;
            }
        }
        ans as i32
    }
}
