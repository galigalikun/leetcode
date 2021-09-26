fn main() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2
    );
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
        4
    );
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
        2
    );
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![-2147483648, 2147483647]]),
        1
    );
}

pub struct Solution {}
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut pp = points;
        pp.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut possible_end:Option<i32> = None;
        let mut result = 0;
        for p in pp {
            if Some(p[0]) > possible_end {
                possible_end = Some(p[1]);
                result += 1;
            }
        }
        return result;
    }
}
