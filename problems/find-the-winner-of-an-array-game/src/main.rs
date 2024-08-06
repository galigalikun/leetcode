fn main() {
    assert_eq!(Solution::get_winner(vec![2,1,3,5,4,6,7], 2), 5);
    assert_eq!(Solution::get_winner(vec![3,2,1], 10), 3);
}

struct Solution;
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut winner = arr[0];
        let mut win_count = 0;
        let mut i = 1;
        while i < arr.len() {
            if arr[i] > winner {
                winner = arr[i];
                win_count = 1;
            } else {
                win_count += 1;
            }
            if win_count == k {
                return winner;
            }
            i += 1;
        }
        return 0;
    }
}
