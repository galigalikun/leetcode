fn main() {
    assert_eq!(Solution::closest_room(vec![vec![2,2],vec![1,2],vec![3,2]], vec![vec![3,1],vec![3,3],vec![5,2]]), vec![3,-1,3]);
    assert_eq!(Solution::closest_room(vec![vec![1,4],vec![2,3],vec![3,5],vec![4,1],vec![5,2]], vec![vec![2,3],vec![2,4],vec![2,5]]), vec![2,1,3]);
}

struct Solution;
impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![-1; queries.len()];
        let mut queries: Vec<_> = queries.into_iter().enumerate().collect();
        queries.sort_by(|a, b| a.1[1].cmp(&b.1[1]));
        let mut rooms: Vec<_> = rooms.into_iter().enumerate().collect();
        rooms.sort_by(|a, b| a.1[0].cmp(&b.1[0]));
        let mut j = 0;
        let mut heap = std::collections::BinaryHeap::new();
        for (i, (id, room)) in rooms.iter().enumerate() {
            while j < queries.len() && queries[j].1[1] >= room[0] {
                let (idx, query) = queries[j];
                if query[0] <= id {
                    heap.push((query[0], idx));
                }
                j += 1;
            }
            while let Some((_, idx)) = heap.pop() {
                ans[idx] = *id as i32;
            }
        }
        for (i, (idx, query)) in queries.iter().enumerate() {
            if ans[*idx] == -1 {
                ans[*idx] = -1;
            }
        }
        ans.sort_by(|a, b| a.cmp(b));
        ans.reverse();
        ans.truncate(queries.len());
        return ans;
    }
}
