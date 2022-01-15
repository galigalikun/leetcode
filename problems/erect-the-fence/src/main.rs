fn main() {
    assert_eq!(Solution::outer_trees(vec![vec![1,1],vec![2,2],vec![2,0],vec![2,4],vec![3,3],vec![4,2]]), vec![[1,1],[2,0],[3,3],[2,4],[4,2]]);
    assert_eq!(Solution::outer_trees(vec![vec![1,2],vec![2,2],vec![4,2]]), vec![[4,2],[2,2],[1,2]]);
}

// https://ttzztt.gitbooks.io/lc/content/jingchiai/erect-the-fence.html
struct Solution{}
use std::cmp::Ordering;
impl Solution {
    fn helper(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
        return (q[1]-p[1]) * (r[0]-q[0])-(q[0]-p[0])*(r[1]-q[1])
    }
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() <= 3 {
            return trees;
        }
        /*
            /// assert_eq!(5.cmp(&10), Ordering::Less);
    /// assert_eq!(10.cmp(&5), Ordering::Greater);
    /// assert_eq!(5.cmp(&5), Ordering::Equal);
        */
        let mut points = trees;
        points.sort_by(|a, b| if a[0] < b[0] || (a[0] == b[0] && a[1] < b[1]) {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        println!("debug {:?}", points);

        let mut ans = vec![];
        for i in 0..points.len() {
            while ans.len() >= 2 && Solution::helper(&ans[ans.len()-2], &ans[ans.len()-1], &points[i]) < 0 {
                ans.pop();
            }
            ans.push(points[i].clone());
        }
        ans.pop();
        for i in (0..points.len()).rev() {
            while ans.len() >= 2 && Solution::helper(&ans[ans.len()-2], &ans[ans.len()-1], &points[i]) < 0 {
                ans.pop();
            }
            ans.push(points[i].clone());
        }
        ans.pop();
        return ans;
    }
}
