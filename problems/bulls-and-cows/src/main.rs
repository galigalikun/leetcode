fn main() {
    assert_eq!(
        Solution::get_hint("1807".to_string(), "7810".to_string()),
        "1A3B"
    );

    assert_eq!(
        Solution::get_hint("1123".to_string(), "0111".to_string()),
        "1A1B"
    );

    assert_eq!(Solution::get_hint("1".to_string(), "0".to_string()), "0A0B");

    assert_eq!(Solution::get_hint("1".to_string(), "1".to_string()), "1A0B");

    assert_eq!(
        Solution::get_hint("11".to_string(), "10".to_string()),
        "1A0B"
    );
}

pub struct Solution {}
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut i = 0;
        let mut a = 0;
        let mut b = 0;
        let mut gg = guess.chars().map(|x| (-1, x)).collect::<Vec<_>>();

        for s in secret.chars() {
            let (c, d) = gg[i];
            if s == d {
                a += 1;
                if c == 2 {
                    let mut div = -1;
                    for j in 0..gg.len() {
                        let (e, f) = gg[j];
                        if s == f && e == -1 {
                            div = 0;
                            gg[j] = (2, f);
                            break;
                        }
                    }
                    b += div;
                }
                gg[i] = (1, d);
            } else {
                for j in 0..gg.len() {
                    let (e, f) = gg[j];
                    if s == f && e == -1 {
                        b += 1;
                        gg[j] = (2, f);
                        break;
                    }
                }
            }
            i += 1;
        }
        return format!("{}A{}B", a, b).to_string();
    }
}
