fn main() {
    assert_eq!(Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]), 2);
    assert_eq!(Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]), -1);
    assert_eq!(Solution::min_domino_rotations(vec![1, 2, 1, 1, 1], vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(Solution::min_domino_rotations(vec![1, 2, 3], vec![2, 3, 4]), -1);
}

struct Solution;
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        if tops.is_empty() {
            return 0;
        }

        let n = tops.len();
        let mut answer = i32::MAX;

        for value in 1..=6 {
            let mut top_rot = 0;
            let mut bottom_rot = 0;
            let mut valid_top = true;
            let mut valid_bottom = true;

            for i in 0..n {
                if tops[i] == value {
                    // already valid on top side
                } else if bottoms[i] == value {
                    top_rot += 1;
                } else {
                    valid_top = false;
                }

                if bottoms[i] == value {
                    // already valid on bottom side
                } else if tops[i] == value {
                    bottom_rot += 1;
                } else {
                    valid_bottom = false;
                }
            }

            if valid_top {
                answer = answer.min(top_rot);
            }
            if valid_bottom {
                answer = answer.min(bottom_rot);
            }
        }

        if answer == i32::MAX { -1 } else { answer }
    }
}
