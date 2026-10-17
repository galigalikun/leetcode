fn main() {
    assert_eq!(Solution::grid_illumination(5, vec![vec![0,0],vec![4,4]], vec![vec![1,1],vec![1,0]]), vec![1,0]);
    assert_eq!(Solution::grid_illumination(5, vec![vec![0,0],vec![4,4]], vec![vec![1,1],vec![1,1]]), vec![1,1]);
    assert_eq!(Solution::grid_illumination(5, vec![vec![0,0],vec![0,4]], vec![vec![0,4],vec![0,1],vec![1,4]]), vec![1,1,0]);
}

struct Solution;
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};

        fn dec_count(map: &mut HashMap<i64, i32>, key: i64) {
            if let Some(count) = map.get_mut(&key) {
                *count -= 1;
                if *count == 0 {
                    map.remove(&key);
                }
            }
        }

        let n64 = n as i64;
        let mut on_lamps = HashSet::new();
        let mut row_count: HashMap<i64, i32> = HashMap::new();
        let mut col_count: HashMap<i64, i32> = HashMap::new();
        let mut diag_count: HashMap<i64, i32> = HashMap::new(); // r - c
        let mut anti_diag_count: HashMap<i64, i32> = HashMap::new(); // r + c

        for lamp in lamps {
            let r = lamp[0] as i64;
            let c = lamp[1] as i64;
            let key = r * n64 + c;
            if on_lamps.insert(key) {
                *row_count.entry(r).or_insert(0) += 1;
                *col_count.entry(c).or_insert(0) += 1;
                *diag_count.entry(r - c).or_insert(0) += 1;
                *anti_diag_count.entry(r + c).or_insert(0) += 1;
            }
        }

        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let r = query[0] as i64;
            let c = query[1] as i64;

            let illuminated = row_count.contains_key(&r)
                || col_count.contains_key(&c)
                || diag_count.contains_key(&(r - c))
                || anti_diag_count.contains_key(&(r + c));
            result.push(if illuminated { 1 } else { 0 });

            for dr in -1..=1 {
                for dc in -1..=1 {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr < 0 || nr >= n64 || nc < 0 || nc >= n64 {
                        continue;
                    }

                    let key = nr * n64 + nc;
                    if on_lamps.remove(&key) {
                        dec_count(&mut row_count, nr);
                        dec_count(&mut col_count, nc);
                        dec_count(&mut diag_count, nr - nc);
                        dec_count(&mut anti_diag_count, nr + nc);
                    }
                }
            }
        }

        result
    }
}
