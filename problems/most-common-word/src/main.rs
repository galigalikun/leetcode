fn main() {
    assert_eq!(
        Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec!["hit".to_string()]
        ),
        "ball"
    );

    assert_eq!(Solution::most_common_word("a.".to_string(), vec![]), "a");

    assert_eq!(
        Solution::most_common_word("Bob!".to_string(), vec!["hit".to_string()]),
        "bob"
    );

    assert_eq!(Solution::most_common_word("L, P! X! C; u! P? w! P. G, S? l? X? D. w? m? f? v, x? i. z; x' m! U' M! j? V; l. S! j? r, K. O? k? p? p, H! t! z' X! v. u; F, h; s? X? K. y, Y! L; q! y? j, o? D' y? F' Z; E? W; W' W! n! p' U. N; w? V' y! Q; J, o! T? g? o! N' M? X? w! V. w? o' k. W. y, k; o' m! r; i, n. k, w; U? S? t; O' g' z. V. N? z, W? j! m? W! h; t! V' T! Z? R' w, w? y? y; O' w; r? q. G, V. x? n, Y; Q. s? S. G. f, s! U? l. o! i. L; Z' X! u. y, Q. q; Q, D; V. m. q. s? Y, U; p? u! q? h? O. W' y? Z! x! r. E, R, r' X' V, b. z, x! Q; y, g' j; j. q; W; v' X! J' H? i' o? n, Y. X! x? h? u; T? l! o? z. K' z' s; L? p? V' r. L? Y; V! V' S. t? Z' T' Y. s? i? Y! G? r; Y; T! h! K; M. k. U; A! V? R? C' x! X. M; z' V! w. N. T? Y' w? n, Z, Z? Y' R; V' f; V' I; t? X? Z; l? R, Q! Z. R. R, O. S! w; p' T. u? U! n, V, M. p? Q, O? q' t. B, k. u. H' T; T? S; Y! S! i? q! K' z' S! v; L. x; q; W? m? y, Z! x. y. j? N' R' I? r? V! Z; s, O? s; V, I, e? U' w! T? T! u; U! e? w? z; t! C! z? U, p' p! r. x; U! Z; u! j; T! X! N' F? n! P' t, X. s; q'".to_string(),
vec!["m".to_string(),"i".to_string(),"s".to_string(),"w".to_string(),"y".to_string(),"d".to_string(),"q".to_string(),"l".to_string(),"a".to_string(),"p".to_string(),"n".to_string(),"t".to_string(),"u".to_string(),"b".to_string(),"o".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"c".to_string(),"x".to_string()]), "z");
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut map = HashMap::new();
        for s in paragraph.split(&[' ', ',', '.', '!', '?', ';', ':', '"', '\''][..]) {
            let key = s.to_ascii_lowercase();
            if key.len() > 0 {
                if let Some(m) = map.get_mut(&key) {
                    *m += 1;
                } else {
                    map.insert(key, 1);
                }
            }
        }
        for s in banned {
            map.remove(&s);
        }
        let mut ans = String::new();
        let mut m = std::i32::MIN;
        for (s, i) in map {
            if i >= m {
                ans = s;
                m = i;
            }
        }
        return ans;
    }
}
