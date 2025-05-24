fn main() {
    assert_eq!(
        Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]),
        1993
    );
    assert_eq!(
        Solution::maximum_population(vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]]),
        1960
    );
}

struct Solution;
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        return logs
            .iter()
            .fold(vec![0; 2050], |mut acc, log| {
                for year in log[0]..log[1] {
                    acc[year as usize] += 1;
                }
                acc
            })
            .iter()
            .enumerate()
            .max_by_key(|&(_, &count)| count)
            .map(|(year, _)| year as i32)
            .unwrap_or(0);
    }
}
