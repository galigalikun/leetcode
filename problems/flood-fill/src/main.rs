fn main() {
    assert_eq!(
        Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![[2, 2, 2], [2, 2, 0], [2, 0, 1]]
    );
    assert_eq!(
        Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
        vec![[0, 0, 0], [0, 0, 0]]
    );
}

struct Solution {}
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let mut queue = Vec::new();
        let idx = image[sr as usize][sc as usize];
        queue.push((sr, sc));
        while !queue.is_empty() {
            let (r, c) = queue.pop().unwrap();
            if image[r as usize][c as usize] == color {
                continue;
            }
            if image[r as usize][c as usize] == idx {
                image[r as usize][c as usize] = color;
                if r > 0 {
                    queue.push((r - 1, c));
                }
                if r < image.len() as i32 - 1 {
                    queue.push((r + 1, c));
                }
                if c > 0 {
                    queue.push((r, c - 1));
                }
                if c < image[0].len() as i32 - 1 {
                    queue.push((r, c + 1));
                }
            }
        }
        return image;
    }
}
