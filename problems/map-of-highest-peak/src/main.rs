fn main() {
    assert_eq!(
        Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]),
        vec![[1, 0], [2, 1]]
    );
    assert_eq!(
        Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]),
        vec![[1, 1, 0], [0, 1, 1], [1, 2, 2]]
    );
}

struct Solution;
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; is_water[0].len()]; is_water.len()];
        let mut queue = Vec::new();
        for w in 0..is_water.len() {
            for h in 0..is_water[0].len() {
                if is_water[w][h] == 1 {
                    res[w][h] = 0;
                    queue.push((w, h));
                } else {
                    res[w][h] = -1;
                }
            }
        }
        let mut height = 1;
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            for (w, h) in queue {
                for (i, j) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (nw, nh) = (w as i32 + i, h as i32 + j);
                    if nw >= 0
                        && nh >= 0
                        && nw < is_water.len() as i32
                        && nh < is_water[0].len() as i32
                    {
                        let (nw, nh) = (nw as usize, nh as usize);
                        if res[nw][nh] == -1 {
                            res[nw][nh] = height;
                            next_queue.push((nw, nh));
                        }
                    }
                }
            }
            queue = next_queue;
            height += 1;
        }
        res
    }
}
