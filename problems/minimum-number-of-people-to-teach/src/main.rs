fn main() {
    assert_eq!(
        Solution::minimum_teachings(
            2,
            vec![vec![1], vec![2], vec![1, 2]],
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        ),
        1
    );
    assert_eq!(
        Solution::minimum_teachings(
            3,
            vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
            vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]]
        ),
        2
    );
}

struct Solution;
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut lang = vec![0; n as usize];
        let mut users = vec![vec![0; n as usize]; languages.len()];
        for i in 0..languages.len() {
            for &l in &languages[i] {
                lang[(l - 1) as usize] += 1;
                users[i][(l - 1) as usize] = 1;
            }
        }
        let mut res = languages.len() as i32;
        for _i in 0..n {
            let mut need = vec![0; n as usize];
            for j in 0..friendships.len() {
                let mut flag = false;
                for &l in &friendships[j] {
                    if l > users[j].len() as i32 {
                        continue;
                    }
                    if users[j][(l - 1) as usize] == 1 {
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    for &l in &friendships[j] {
                        if l > need.len() as i32 {
                            continue;
                        }
                        need[(l - 1) as usize] = 1;
                    }
                }
            }
            let mut cnt = 0;
            for j in 0..n as usize {
                if need[j] == 1 && lang[j] > 0 {
                    cnt += lang[j];
                }
            }
            res = res.min(cnt);
        }
        res
    }
}
