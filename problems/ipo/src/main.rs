fn main() {
    assert_eq!(
        Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
        4
    );
    assert_eq!(
        Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
        6
    );
}

struct Solution {}
// https://programmerall.com/article/40551610623/
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut v = vec![];
        for i in 0..profits.len() {
            v.push((capital[i], profits[i]));
        }

        v.sort();

        let mut result = w;
        for _i in 0..k {
            let mut left = 0;
            let mut right = v.len();
            let mut mx = 0;
            let mut idx = 0;
            while left < right {
                let mid = left + (right - left) / 2;
                if v[mid].0 <= result {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if right > 0 {
                for j in (0..right).rev() {
                    if mx < v[j].1 {
                        mx = v[j].1;
                        idx = j;
                    }
                }
            }
            result += mx;
            if v.lenn() > idx {
                v.remove(idx);
            }
        }
        return result;
    }
}
