fn main() {
    assert_eq!(Solution::cut_off_tree(vec![vec![1,2,3],vec![0,0,4],vec![7,6,5]]), 6);
    assert_eq!(Solution::cut_off_tree(vec![vec![1,2,3],vec![0,0,0],vec![7,6,5]]), -1);
    assert_eq!(Solution::cut_off_tree(vec![vec![2,3,4],vec![0,0,5],vec![8,7,6]]), 6);
}

// https://ttzztt.gitbooks.io/lc/content/cut-off-trees-for-golf-event.html
struct Solution{}
impl Solution {
    fn helper(forest: Vec<Vec<i32>>, sx:usize, sy:usize, dx:usize, dy:usize) -> i32 {
        let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let m =  forest.len();
        let n = forest[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut queue = vec![(sx, sy)];
        let mut steps = 0;
        while !queue.is_empty() {
            println!("debug {:?}", queue);
            let mut cur_nodes = queue.len();
            while cur_nodes > 0 {
                let node = queue[0];
                queue.pop();
                let cx = node.0;
                let cy = node.1;
                if cx == dx && cy == dy {
                    return steps;
                }

                for i in 0..4 {
                    let x = cx as i32 + dir[i].0;
                    let y = cy as i32 + dir[i].1;
                    if x < 0 || x == n as i32 || y < 0 || y == m as i32 || forest[y as usize][x as usize] == 0 || visited[y as usize][x as usize] {
                        continue;
                    }
                    visited[x as usize][y as usize] = true;
                    queue.push((x as usize, y as usize));
                }
                cur_nodes -= 1;
            }
            steps += 1;
        }

        return -1;
    }
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut trees = vec![];
        for x in 0..forest.len() {
            for y in 0..forest[x].len() {
                trees.push((forest[x][y], x, y));
            }
        }
        trees.sort_by(|a, b| a.0.cmp(&b.0));

        let mut sx = 0;
        let mut sy = 0;
        let mut total_steps = 0;
        for i in 0..trees.len() {
            let dx = trees[i].1;
            let dy = trees[i].2;

            let incr_steps = Solution::helper(forest.clone(), sx, sy, dx, dy);
            if incr_steps == -1 {
                return -1;
            }
            total_steps += incr_steps;
            sx = dx;
            sy = dy;
        }
        return total_steps;
    }
}
