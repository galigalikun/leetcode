fn main() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    // assert_eq!(Solution::letter_combinations("".to_string()), vec![]);
    assert_eq!(
        Solution::letter_combinations("2".to_string()),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

pub struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = Vec::new();
        for c in digits.as_str().chars() {
            match c {
                '2' => {
                    let default = vec!["a".to_string(), "b".to_string(), "c".to_string()];
                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                '3' => {
                    let default = vec!["d".to_string(), "e".to_string(), "f".to_string()];

                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                '4' => {
                    let default = vec!["g".to_string(), "h".to_string(), "i".to_string()];
                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                '5' => {
                    let default = vec!["j".to_string(), "k".to_string(), "l".to_string()];
                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                '6' => {
                    let default = vec!["m".to_string(), "n".to_string(), "o".to_string()];
                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                '7' => {
                    let default = vec![
                        "p".to_string(),
                        "q".to_string(),
                        "r".to_string(),
                        "s".to_string(),
                    ];
                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                '8' => {
                    let default = vec!["t".to_string(), "u".to_string(), "v".to_string()];
                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                '9' => {
                    let default = vec![
                        "w".to_string(),
                        "x".to_string(),
                        "y".to_string(),
                        "z".to_string(),
                    ];
                    let mut work = Vec::new();
                    for i in 0..result.len() {
                        let key = format!("{}", result[i]);
                        for j in 0..default.len() {
                            work.push(format!("{}{}", key, default[j]));
                        }
                    }
                    if work.len() == 0 {
                        result.extend(default);
                    } else {
                        result = work;
                    }
                }
                _ => panic!(""),
            }
        }
        return result;
    }
}
