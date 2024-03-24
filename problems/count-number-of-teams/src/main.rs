fn main() {
    assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
    assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
    assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
}

struct Solution;
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..rating.len() {
            let a = rating[i];
            for j in i + 1..rating.len() {
                let b = rating[j];
                for k in j + 1..rating.len() {
                    let c = rating[k];
                    if a < b && b < c {
                        count += 1;
                    } else if a > b && b > c {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }
}
