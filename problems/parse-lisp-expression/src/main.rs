use std::collections::HashMap;

fn main() {
    assert_eq!(
        Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string()),
        14
    );
    assert_eq!(Solution::evaluate("(let x 3 x 2 x)".to_string()), 2);
    assert_eq!(
        Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string()),
        5
    );
    assert_eq!(Solution::evaluate("(add 1 2)".to_string()), 3);
    assert_eq!(Solution::evaluate("(mult 3 (add 2 3))".to_string()), 15);
    assert_eq!(Solution::evaluate("(let x -2 y x y)".to_string()), -2);
}

struct Solution {}
impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        fn eval(expr: &str, scope: &mut HashMap<String, Vec<i32>>) -> i32 {
            if !expr.starts_with('(') {
                if let Ok(value) = expr.parse::<i32>() {
                    return value;
                }
                return *scope
                    .get(expr)
                    .and_then(|values| values.last())
                    .expect("variable should exist in scope");
            }

            let inner = &expr[1..expr.len() - 1];
            if let Some(rest) = inner.strip_prefix("add ") {
                let parts = split_top_level(rest);
                return eval(parts[0], scope) + eval(parts[1], scope);
            }
            if let Some(rest) = inner.strip_prefix("mult ") {
                let parts = split_top_level(rest);
                return eval(parts[0], scope) * eval(parts[1], scope);
            }

            let rest = inner
                .strip_prefix("let ")
                .expect("expression must be add/mult/let");
            let parts = split_top_level(rest);
            let mut assigned_names: Vec<&str> = Vec::new();
            let mut i = 0;

            while i + 1 < parts.len() {
                let name = parts[i];
                let value = eval(parts[i + 1], scope);
                scope.entry(name.to_string()).or_default().push(value);
                assigned_names.push(name);
                i += 2;
            }

            let result = eval(parts[parts.len() - 1], scope);
            for name in assigned_names {
                if let Some(values) = scope.get_mut(name) {
                    values.pop();
                    if values.is_empty() {
                        scope.remove(name);
                    }
                }
            }
            result
        }

        fn split_top_level(s: &str) -> Vec<&str> {
            let bytes = s.as_bytes();
            let mut depth = 0;
            let mut start = 0;
            let mut parts = Vec::new();
            let mut i = 0;

            while i < bytes.len() {
                match bytes[i] as char {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    ' ' if depth == 0 => {
                        if start < i {
                            parts.push(&s[start..i]);
                        }
                        start = i + 1;
                    }
                    _ => {}
                }
                i += 1;
            }

            if start < s.len() {
                parts.push(&s[start..]);
            }
            parts
        }

        let mut scope: HashMap<String, Vec<i32>> = HashMap::new();
        eval(&expression, &mut scope)
    }
}
