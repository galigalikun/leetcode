fn main() {
    assert_eq!(
        Solution::can_form_array(vec![15, 88], vec![vec![88], vec![15]]),
        true
    );
    assert_eq!(
        Solution::can_form_array(vec![49, 18, 16], vec![vec![16, 18, 49]]),
        false
    );
    assert_eq!(
        Solution::can_form_array(vec![91, 4, 64, 78], vec![vec![78], vec![4, 64], vec![91]]),
        true
    );
}

struct Solution;
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;
        while i < arr.len() {
            let mut found = false;
            for piece in &pieces {
                if piece[0] == arr[i] {
                    for j in 0..piece.len() {
                        if piece[j] != arr[i] {
                            return false;
                        }
                        i += 1;
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        return true;
    }
}
