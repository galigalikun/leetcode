fn main() {
    assert_eq!(Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
    assert_eq!(Solution::max_area(5, 4, vec![3, 1], vec![1]), 6);
    assert_eq!(Solution::max_area(5, 4, vec![3], vec![3]), 9);
}

struct Solution;
impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal_cuts = horizontal_cuts;
        let mut vertical_cuts = vertical_cuts;
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        horizontal_cuts.sort();
        vertical_cuts.sort();
        let mut max_h = 0;
        let mut max_v = 0;
        for h in 1..horizontal_cuts.len() {
            max_h = max_h.max(horizontal_cuts[h] - horizontal_cuts[h - 1]);
        }
        for v in 1..vertical_cuts.len() {
            max_v = max_v.max(vertical_cuts[v] - vertical_cuts[v - 1]);
        }
        let modulo = 1000000007;
        return ((max_h as i64) * (max_v as i64) % modulo) as i32;
    }
}
