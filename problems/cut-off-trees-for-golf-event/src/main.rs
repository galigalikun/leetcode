fn main() {
    assert_eq!(Solution::cut_off_tree(vec![vec![1,2,3],vec![0,0,4],vec![7,6,5]]), 6);
    assert_eq!(Solution::cut_off_tree(vec![vec![1,2,3],vec![0,0,0],vec![7,6,5]]), -1);
    assert_eq!(Solution::cut_off_tree(vec![vec![2,3,4],vec![0,0,5],vec![8,7,6]]), 6);
}

struct Solution{}
impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        for x in 0..forest.len() {
            for y in 0..forest[x].len() {
                let mut tree_height = forest[x][y];
            }
        }
        return -1;
    }
}
