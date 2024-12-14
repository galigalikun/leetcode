fn main() {
    assert_eq!(Solution::interpret("G()(al)".to_string()), "Goal");
    assert_eq!(Solution::interpret("G()()()()(al)".to_string()), "Gooooal");
    assert_eq!(
        Solution::interpret("(al)G(al)()()G".to_string()),
        "alGalooG"
    );
}

struct Solution;
impl Solution {
    pub fn interpret(command: String) -> String {
        return command.replace("()", "o").replace("(al)", "al");
    }
}
