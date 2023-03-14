fn main() {
    assert_eq!(Solution::powerful_integers(2, 3, 10), vec![2,3,4,5,7,9,10]);
    assert_eq!(Solution::powerful_integers(3, 5, 15), vec![2,4,6,8,10,14]);
}

struct Solution;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..(10 as f64).log(x as f64) as u32 {
            for j in 0..((10 as f64).log(y as f64) as u32) {
                let a = x.pow(i) + y.pow(j);
                if a <= bound {
                    ans.push(a);
                }
            }
        }
        return ans;
    }
}
