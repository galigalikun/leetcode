fn main() {
    assert_eq!(Solution::regions_by_slashes(vec![" /".to_string(),"/ ".to_string()]), 2);
    assert_eq!(Solution::regions_by_slashes(vec![" /".to_string(),"  ".to_string()]), 1);
    assert_eq!(Solution::regions_by_slashes(vec!["/\\".to_string(),"\\/".to_string()]), 5);
}

struct Solution;
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let total = 4 * n * n;
        let mut parent = (0..total).collect::<Vec<usize>>();

        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }

        fn union(parent: &mut Vec<usize>, a: usize, b: usize) {
            let root_a = find(parent, a);
            let root_b = find(parent, b);
            if root_a != root_b {
                parent[root_b] = root_a;
            }
        }

        let idx = |r: usize, c: usize, k: usize| 4 * (r * n + c) + k;

        for r in 0..n {
            for c in 0..n {
                let base = idx(r, c, 0);
                let cell = grid[r].as_bytes()[c] as char;

                match cell {
                    ' ' => {
                        union(&mut parent, base, base + 1);
                        union(&mut parent, base + 1, base + 2);
                        union(&mut parent, base + 2, base + 3);
                    }
                    '/' => {
                        union(&mut parent, base, base + 3);
                        union(&mut parent, base + 1, base + 2);
                    }
                    '\\' => {
                        union(&mut parent, base, base + 1);
                        union(&mut parent, base + 2, base + 3);
                    }
                    _ => {}
                }

                if c + 1 < n {
                    union(&mut parent, base + 1, idx(r, c + 1, 3));
                }
                if r + 1 < n {
                    union(&mut parent, base + 2, idx(r + 1, c, 0));
                }
            }
        }

        let mut regions = 0;
        for i in 0..total {
            if find(&mut parent, i) == i {
                regions += 1;
            }
        }

        regions as i32
    }
}
