use std::collections::BinaryHeap;

fn main() {
    assert_eq!(Solution::smallest_range(vec![vec![4,10,15,24,26],vec![0,9,12,20],vec![5,18,22,30]]), vec![20,24]);
    assert_eq!(Solution::smallest_range(vec![vec![1,2,3],vec![1,2,3],vec![1,2,3]]), vec![1,1]);
}

// https://dreamume.medium.com/leetcode-632-smallest-range-covering-elements-from-k-lists-aaeaf56a9425
struct Solution{}
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pq = BinaryHeap::new();
        let mut res = vec![0; 2];
        for n in nums {
            pq.push(n.clone());
            res[1] = std::cmp::max(res[1], n[0]);
        }
        res[0] = *pq.peek().unwrap().first().unwrap();
        let mut current = res.clone();

        let func = |pq:&mut BinaryHeap<Vec<i32>>| {
            let mut its = pq.peek_mut().unwrap();
            *its = vec![its[0]+1, its[1]];

            vec![its[0]+1, its[1]]
        };

        let mut i = 0;
        while !pq.is_empty() {
            let its = func(&mut pq);
            pq.pop();
            if its[0] == its[1] {
                break;
            }
            current[1] = std::cmp::max(current[1], its[0]);
            pq.push(its);
            current[0] = pq.peek().unwrap()[0];
            if current[1] - current[0] < res[1] - res[0] {
                res = current.clone();
            }
            i += 1;
        }
        return res;
    }
}
