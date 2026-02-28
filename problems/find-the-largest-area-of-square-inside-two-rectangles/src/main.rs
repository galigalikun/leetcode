fn main() {
    assert_eq!(
        Solution::largest_square_area(
            vec![[1,1],[2,2],[3,1]].iter().map(|v| v.to_vec()).collect(),
            vec![[3,3],[4,4],[6,6]].iter().map(|v| v.to_vec()).collect()
        ),
        1
    );
    assert_eq!(
        Solution::largest_square_area(
            vec![[1,1],[1,3],[1,5]].iter().map(|v| v.to_vec()).collect(),
            vec![[5,5],[5,7],[5,9]].iter().map(|v| v.to_vec()).collect()
        ),
        4
    );
    assert_eq!(
        Solution::largest_square_area(
            vec![[1,1],[2,2],[1,2]].iter().map(|v| v.to_vec()).collect(),
            vec![[3,3],[4,4],[3,4]].iter().map(|v| v.to_vec()).collect()
        ),
        1
    );
    assert_eq!(
        Solution::largest_square_area(
            vec![[1,1],[3,3],[3,1]].iter().map(|v| v.to_vec()).collect(),
            vec![[2,2],[4,4],[4,2]].iter().map(|v| v.to_vec()).collect()
        ),
        0
    );
}

struct Solution;
impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;
        for i in 0..bottom_left.len() {
            for j in 0..top_right.len() {
                let x1 = bottom_left[i][0];
                let y1 = bottom_left[i][1];
                let x2 = top_right[j][0];
                let y2 = top_right[j][1];
                if x1 < x2 && y1 < y2 {
                    ans = ans.max((x2 - x1).min(y2 - y1) as i64);
                }
            }
        }
        ans * ans
    }
}
