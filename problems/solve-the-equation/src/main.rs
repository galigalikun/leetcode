fn main() {
    assert_eq!(Solution::solve_equation("x+5-3+x=6+x-2".to_string()), "x=2");
    assert_eq!(
        Solution::solve_equation("x=x".to_string()),
        "Infinite solutions"
    );
    assert_eq!(Solution::solve_equation("2x=x".to_string()), "x=0");
    assert_eq!(Solution::solve_equation("x=x+2".to_string()), "No solution");
    assert_eq!(Solution::solve_equation("3x=33+22+11".to_string()), "x=22");
}

struct Solution {}
impl Solution {
    fn parse(s: &str) -> (i32, i32) {
        let mut op = '+';
        let mut num = 0;
        let mut param: Option<String> = None;
        let mut x_num = 0;
        for c in s.chars() {
            match c {
                'x' => {
                    if let Some(p) = param {
                        let n = p.parse::<i32>().unwrap();
                        x_num += if op == '+' { n } else { -n };
                    } else {
                        x_num += if op == '+' { 1 } else { -1 };
                    }
                    op = '+';
                    param = None;
                }
                '+' | '-' => {
                    if let Some(p) = param {
                        let n = p.parse::<i32>().unwrap();
                        num += if op == '+' { n } else { -n };
                    }
                    op = c;
                    param = None;
                }
                _ => {
                    if let Some(p) = param {
                        param = Some(format!("{}{}", p, c));
                    } else {
                        param = Some(format!("{}", c));
                    }
                }
            }
        }

        if let Some(p) = param {
            let n = p.parse::<i32>().unwrap();
            num += if op == '+' { n } else { -n };
        }

        return (x_num, num);
    }
    pub fn solve_equation(equation: String) -> String {
        let s = equation.split('=').collect::<Vec<_>>();
        let (left, right) = (s[0], s[1]);
        let (l_x_num, l_num) = Solution::parse(left);
        let (r_x_num, r_num) = Solution::parse(right);

        let x_num = l_x_num - r_x_num;
        let num = r_num - l_num;

        if num == 0 && x_num == 0 {
            return "Infinite solutions".to_string();
        } else if x_num == 0 {
            return "No solution".to_string();
        }
        return format!("x={}", num / x_num);
    }
}
