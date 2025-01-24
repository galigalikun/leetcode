fn main() {
    assert_eq!(
        Solution::count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]),
        3
    );
    assert_eq!(
        Solution::count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        return rectangles
            .iter()
            .map(|r| r.iter().min().unwrap())
            .collect::<Vec<_>>()
            .iter()
            .filter(|&&x| {
                x == rectangles
                    .iter()
                    .map(|r| r.iter().min().unwrap())
                    .max()
                    .unwrap()
            })
            .count() as i32;
    }
}
