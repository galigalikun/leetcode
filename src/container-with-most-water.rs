fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
    assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    assert_eq!(Solution::max_area(vec![2, 1]), 1);
}

pub struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        for i in 0..height.iter().count() {
            if let Some(h) = height.iter().nth(i) {
                let mut with = 0;
                for j in i + 1..height.iter().count() {
                    if let Some(h2) = height.iter().nth(j) {
                        if h2 >= h {
                            let w = j - i;
                            if w > with {
                                with = w;
                            }
                        }
                    }
                }
                for j in (0..i).rev() {
                    if let Some(h2) = height.iter().nth(j) {
                        if h2 >= h {
                            let w = i - j;
                            if w > with {
                                with = w;
                            }
                        }
                    }
                }
                let a = with as i32 * h;
                if a > area {
                    area = a;
                }
            }
        }
        return area;
    }
}
