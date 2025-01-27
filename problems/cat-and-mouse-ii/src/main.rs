fn main() {
    assert_eq!(
        Solution::can_mouse_win(
            vec![
                "####F".to_string(),
                "#C...".to_string(),
                "M....".to_string()
            ],
            1,
            2
        ),
        true
    );
    assert_eq!(
        Solution::can_mouse_win(vec!["M.C...F".to_string()], 1, 4),
        true
    );
    assert_eq!(
        Solution::can_mouse_win(vec!["M.C...F".to_string()], 1, 3),
        false
    );
}

struct Solution;
impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let grid = grid
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut dp = vec![vec![vec![vec![vec![0; 2]; grid[0].len()]; grid.len()]; 2]; 2];
        let mut mouse = (0, 0);
        let mut cat = (0, 0);
        let mut food = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'M' {
                    mouse = (i, j);
                } else if grid[i][j] == 'C' {
                    cat = (i, j);
                } else if grid[i][j] == 'F' {
                    food = (i, j);
                }
            }
        }
        let mut turn = 0;
        for i in (0..grid.len()).rev() {
            for j in (0..grid[0].len()).rev() {
                for k in 0..2 {
                    for l in 0..2 {
                        dp[turn][i][j][k][l] = 2;
                        if k == 0 && l == 0 {
                            dp[turn][i][j][k][l] = 0;
                        }
                        if k == 1 && l == 1 {
                            dp[turn][i][j][k][l] = 1;
                        }
                        if k == 0 && l == 1 {
                            dp[turn][i][j][k][l] = 1;
                        }
                        if k == 1 && l == 0 {
                            dp[turn][i][j][k][l] = 0;
                        }
                    }
                }
            }
        }
        while dp[turn][mouse.0][mouse.1][cat.0][cat.1] == 2 {
            let mut next = vec![vec![vec![vec![vec![0; 2]; grid[0].len()]; grid.len()]; 2]; 2];
            for i in (0..grid.len()).rev() {
                for j in (0..grid[0].len()).rev() {
                    for k in 0..2 {
                        for l in 0..2 {
                            next[turn][i][j][k][l] = 2;
                            if k == 0 && l == 0 {
                                next[turn][i][j][k][l] = 0;
                            }
                            if k == 1 && l == 1 {
                                next[turn][i][j][k][l] = 1;
                            }
                            if k == 0 && l == 1 {
                                next[turn][i][j][k][l] = 1;
                            }
                            if k == 1 && l == 0 {
                                next[turn][i][j][k][l] = 0;
                            }
                            if grid[i][j] == '#' {
                                continue;
                            }
                            if k == 0 {
                                let mut win = true;
                                for m in 0..=mouse_jump {
                                    let x = i as i32 + m;
                                    if x >= grid.len() as i32 {
                                        break;
                                    }
                                    if grid[x as usize][j] == '#' {
                                        break;
                                    }
                                    if (x as usize, j) == cat {
                                        win = false;
                                        break;
                                    }
                                }
                                if win {
                                    for m in 0..=mouse_jump {
                                        let x = j as i32 + m;
                                        if x >= grid[0].len() as i32 {
                                            break;
                                        }
                                        if grid[i][x as usize] == '#' {
                                            break;
                                        }
                                        if (i, x as usize) == cat {
                                            win = false;
                                            break;
                                        }
                                    }
                                }
                                if win {
                                    let mut lose = true;
                                    for m in 0..=cat_jump {
                                        let x = i as i32 + m;
                                        if x >= grid.len() as i32 {
                                            break;
                                        }
                                        if grid[x as usize][j] == '#' {
                                            break;
                                        }
                                        if (x as usize, j) == mouse {
                                            lose = false;
                                            break;
                                        }
                                    }
                                    if lose {
                                        for m in 0..=cat_jump {
                                            let x = j as i32 + m;
                                            if x >= grid[0].len() as i32 {
                                                break;
                                            }
                                            if grid[i][x as usize] == '#' {
                                                break;
                                            }
                                            if (i, x as usize) == mouse {
                                                lose = false;
                                                break;
                                            }
                                        }
                                    }
                                    if lose {
                                        next[turn][i][j][k][l] = 0;
                                    }
                                }
                            } else {
                                let mut win = true;
                                for m in 0..=cat_jump {
                                    let x = i as i32 + m;
                                    if x >= grid.len() as i32 {
                                        break;
                                    }
                                    if grid[x as usize][j] == '#' {
                                        break;
                                    }
                                    if (x as usize, j) == food {
                                        win = false;
                                        break;
                                    }
                                }
                                if win {
                                    for m in 0..=cat_jump {
                                        let x = j as i32 + m;
                                        if x >= grid[0].len() as i32 {
                                            break;
                                        }
                                        if grid[i][x as usize] == '#' {
                                            break;
                                        }
                                        if (i, x as usize) == food {
                                            win = false;
                                            break;
                                        }
                                    }
                                }
                                if win {
                                    let mut lose = true;
                                    for m in 0..=mouse_jump {
                                        let x = i as i32 + m;
                                        if x >= grid.len() as i32 {
                                            break;
                                        }
                                        if grid[x as usize][j] == '#' {
                                            break;
                                        }
                                        if (x as usize, j) == cat {
                                            lose = false;
                                            break;
                                        }
                                    }
                                    if lose {
                                        for m in 0..=mouse_jump {
                                            let x = j as i32 + m;
                                            if x >= grid[0].len() as i32 {
                                                break;
                                            }
                                            if grid[i][x as usize] == '#' {
                                                break;
                                            }
                                            if (i, x as usize) == cat {
                                                lose = false;
                                                break;
                                            }
                                        }
                                    }
                                    if lose {
                                        next[turn][i][j][k][l] = 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            dp[turn][..].clone_from_slice(&next[turn]);
            turn ^= 1;
        }
        if dp[turn][mouse.0][mouse.1][cat.0][cat.1] == 0 {
            return true;
        }
        return false;
    }
}
