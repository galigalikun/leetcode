fn main() {
    assert_eq!(
        Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]),
        4
    );
    assert_eq!(
        Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]),
        4
    );
    assert_eq!(
        Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 4]),
        4
    );
    assert_eq!(
        Solution::maximize_square_hole_area(1, 5, vec![2], vec![2, 3]),
        4
    );
}

struct Solution;
impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;
        h_bars.push(0);
        h_bars.push(n + 1);
        v_bars.push(0);
        v_bars.push(m + 1);
        h_bars.sort_unstable();
        v_bars.sort_unstable();
        let mut max_area = 0;
        for i in 1..h_bars.len() {
            for j in 1..v_bars.len() {
                let area = (h_bars[i] - h_bars[i - 1]) * (v_bars[j] - v_bars[j - 1]);
                max_area = max_area.max(area);
            }
        }
        max_area
    }
}
