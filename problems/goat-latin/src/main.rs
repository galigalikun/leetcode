fn main() {
    assert_eq!(
        Solution::to_goat_latin("I speak Goat Latin".to_string()),
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
    );
    assert_eq!(Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()), "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa");
}

struct Solution {}
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut ans = String::new();
        let mut i = 1;
        for s in sentence.split(" ") {
            let c = s.chars().nth(0).unwrap();
            let a = match c.to_ascii_lowercase() {
                'i' | 'a' | 'e' | 'o' | 'u' => format!("{}ma{}", s, "a".repeat(i)),
                _ => format!("{}{}ma{}", &s[1..], c, "a".repeat(i)),
            };
            i += 1;
            ans = format!("{}{} ", ans, a);
        }
        return ans.trim_end().to_string();
    }
}
