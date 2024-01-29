fn main() {
    assert_eq!(
        Solution::max_students(vec![
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['.', '#', '#', '#', '#', '.'],
            vec!['#', '.', '#', '#', '.', '#']
        ]),
        4
    );
    assert_eq!(
        Solution::max_students(vec![
            vec!['.', '#'],
            vec!['#', '#'],
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['.', '#']
        ]),
        3
    );
    assert_eq!(
        Solution::max_students(vec![
            vec!['#', '.', '.', '.', '#'],
            vec!['.', '#', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.'],
            vec!['#', '.', '.', '.', '#']
        ]),
        10
    );
}

struct Solution;
impl Solution {
    fn check(seats: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        let mut j = j;
        for k in (0..seats[0].len()).rev() {
            if seats[i][k] == '#' {
                j >>= 1;
            } else if seats[i][k] == '.' {
                if j & 1 == 1 {
                    return false;
                }
                j >>= 1;
            }
        }
        return true;
    }
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let mut dp = vec![vec![0; 1 << seats[0].len()]; seats.len() + 1];
        for i in 0..seats.len() {
            for j in 0..(1 << seats[0].len()) {
                if Self::check(&seats, i, j) {
                    for k in 0..(1 << seats[0].len()) {
                        if (j & (k >> 1) == 0) && ((j >> 1) & k == 0) {
                            dp[i + 1][j] = dp[i + 1][j].max(dp[i][k] + j.count_ones() as i32);
                        }
                    }
                }
            }
        }
        return dp[seats.len()].iter().max().unwrap().to_owned();
    }
}
