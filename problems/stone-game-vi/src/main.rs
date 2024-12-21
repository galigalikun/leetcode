fn main() {
    assert_eq!(Solution::stone_game_vi(vec![1, 3], vec![2, 1]), 1);
    assert_eq!(Solution::stone_game_vi(vec![1, 2], vec![3, 1]), 0);
    assert_eq!(Solution::stone_game_vi(vec![2, 4, 3], vec![1, 6, 7]), -1);
}

struct Solution;
impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let n = alice_values.len();
        let mut sum = vec![];
        for i in 0..n {
            sum.push((alice_values[i] + bob_values[i], i));
        }
        sum.sort_by(|a, b| b.0.cmp(&a.0));
        let mut alice = 0;
        let mut bob = 0;
        for i in 0..n {
            if i % 2 == 0 {
                alice += alice_values[sum[i].1];
            } else {
                bob += bob_values[sum[i].1];
            }
        }
        if alice > bob {
            return 1;
        } else if alice < bob {
            return -1;
        }
        return 0;
    }
}
