fn main() {
    assert_eq!(Solution::equations_possible(vec!["a==b".to_string(),"b!=a".to_string()]), false);
    assert_eq!(Solution::equations_possible(vec!["b==a".to_string(),"a==b".to_string()]), true);
}

struct Solution;
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        for i in 0..equations.len() {
            for j in 0..equations.len() {
                if i == j {
                    continue;
                }
                let eq1 = &equations[i];
                let eq2 = &equations[j];
                if eq1.starts_with('!') && eq2.starts_with('!') {
                    continue;
                }
                if eq1.starts_with('!') {
                    if eq1[2..].eq(&eq2[0..1]) {
                        return false;
                    }
                } else if eq2.starts_with('!') {
                    if eq1[0..1].eq(&eq2[2..]) {
                        return false;
                    }
                } else {
                    if eq1[0..1].eq(&eq2[0..1]) {
                        continue;
                    }
                    if eq1[2..].eq(&eq2[2..]) {
                        continue;
                    }
                    if eq1[0..1].eq(&eq2[2..]) {
                        continue;
                    }
                    if eq1[2..].eq(&eq2[0..1]) {
                        continue;
                    }
                    return false;
                }
            }
        }
        true
    }
}
