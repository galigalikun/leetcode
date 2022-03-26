fn main() {
    assert_eq!(
        Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );
    assert_eq!(
        Solution::image_smoother(vec![
            vec![100, 200, 100],
            vec![200, 50, 200],
            vec![100, 200, 100]
        ]),
        vec![[137, 141, 137], [141, 138, 141], [137, 141, 137]]
    );
}

struct Solution {}
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; img[0].len()]; img.len()];
        for x in 0..img.len() as i32 {
            for y in 0..img[x as usize].len() as i32 {
                let mut sum = 0;
                let mut count = 0;
                for i in -1..2 as i32 {
                    for j in -1..2 as i32 {
                        if x + i < 0
                            || x + i >= img.len() as i32
                            || y + j < 0
                            || y + j >= img[x as usize].len() as i32
                        {
                            continue;
                        }
                        sum += img[(x + i) as usize][(y + j) as usize];
                        count += 1;
                    }
                }
                res[x as usize][y as usize] = sum / count;
            }
        }
        return res;
    }
}
