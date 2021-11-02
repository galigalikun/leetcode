fn main() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
    assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
    assert_eq!(
        Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3]),
        true
    );
    assert_eq!(
        Solution::makesquare(vec![10, 6, 5, 5, 5, 3, 3, 3, 2, 2, 2, 2]),
        true
    );
}

pub struct Solution {}
// https://dev.to/seanpgallivan/solution-matchsticks-to-square-2fk8
impl Solution {
    fn btrack(matchsticks: &mut Vec<i32>, side: i32, i: usize, space: i32, done: i32) -> bool {
        if done == 3 {
            return true;
        }
        if i == matchsticks.len() {
            return false;
        }
        let num = matchsticks[i];
        if num > space {
            return Solution::btrack(matchsticks, side, i + 1, space, done);
        }
        matchsticks[i] = side + 1;
        if num == space {
            return Solution::btrack(matchsticks, side, 1, side, done + 1);
        } else {
            if Solution::btrack(matchsticks, side, i + 1, space - num, done) {
                return true;
            }
            matchsticks[i] = num;
            return Solution::btrack(matchsticks, side, i + 1, space, done);
        }
    }
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let periphery = matchsticks.iter().fold(0, |sum, a| sum + a);
        if periphery % 4 == 0 {
            let side = periphery / 4;
            let mut mm = matchsticks;
            mm.sort_by(|a, b| b.cmp(a));
            return Solution::btrack(&mut mm, side, 0, side, 0);
        }

        return false;
    }
}
