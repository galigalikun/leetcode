fn main() {
    assert_eq!(
        Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![[1, 0, 0], [0, 1, 0], [1, 1, 1]]
    );
    assert_eq!(
        Solution::flip_and_invert_image(vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0]
        ]),
        vec![[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]]
    );
}

struct Solution {}
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        for mut v in image {
            v.reverse();
            let mut a = vec![0;v.len()];
            for (i, b) in v.iter().enumerate() {
                a[i] = if b == &1 {
                    0
                } else {
                    1
                };
            }
            ans.push(a);
        }
        return ans;
    }
}
