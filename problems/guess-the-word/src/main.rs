use std::cell::Cell;
use std::collections::HashSet;

fn main() {
    {
        let words = vec!["acckzz", "ccbazz", "eiowzz", "abcczz"]
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let master = Master::new(words.clone(), "acckzz", 10);

        Solution::find_secret_word(words, &master);

        assert!(master.solved_within_limit());
    }
    {
        let words = vec![
            "gaxckt", "trlccr", "jxwhkz", "ycbfps", "peayuf", "yiejjw", "ldzccp", "nqsjoa",
            "qrjasy", "pcldos", "acrtag", "buyeia", "ubmtpj", "drtclz", "zqderp", "snywek",
            "caoztp", "ibpghw", "evtkhl", "bhpfla", "ymqhxk", "qkvipb", "tvmued", "rvbass",
            "axeasm", "qolsjg", "roswcb", "vdjgxx", "bugbyv", "zipjpc", "tamszl", "osdifo",
            "dvxlxm", "iwmyfb", "wmnwhe", "hslnop", "nkrfwn", "puvgve", "rqsqpq", "jwoswl",
            "tittgf", "evqsqe", "aishiv", "pmwovj", "sorbte", "hbaczn", "coifed", "hrctvp",
            "vkytbw", "dizcxz", "arabol", "uywurk", "ppywdo", "resfls", "tmoliy", "etriev",
            "oanvlx", "wcsnzy", "loufkw", "onnwcy", "novblw", "mtxgwe", "rgrdbt", "ckolob",
            "kxnflb", "phonmg", "egcdab", "cykndr", "lkzobv", "ifwmwp", "jqmbib", "mypnvf",
            "lnrgnj", "clijwa", "kiioqr", "syzebr", "rqsmhg", "sczjmz", "hsdjfp", "mjcgvm",
            "ajotcx", "olgnfv", "mjyjxj", "wzgbmg", "lpcnbj", "yjjlwn", "blrogv", "bdplzs",
            "oxblph", "twejel", "rupapy", "euwrrz", "apiqzu", "ydcroj", "ldvzgq", "zailgu",
            "xgqpsr", "wxdyho", "alrplq", "brklfk",
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        let master = Master::new(words.clone(), "hbaczn", 10);

        Solution::find_secret_word(words, &master);

        assert!(master.solved_within_limit());
    }
    println!("You guessed the secret word correctly.");
}

struct Solution;

struct Master {
    words: HashSet<String>,
    secret: String,
    allowed_guesses: usize,
    guesses_used: Cell<usize>,
    found: Cell<bool>,
}

impl Master {
    fn new(words: Vec<String>, secret: &str, allowed_guesses: usize) -> Self {
        Self {
            words: words.into_iter().collect(),
            secret: secret.to_string(),
            allowed_guesses,
            guesses_used: Cell::new(0),
            found: Cell::new(false),
        }
    }

    fn guess(&self, word: String) -> i32 {
        let used = self.guesses_used.get() + 1;
        self.guesses_used.set(used);

        if !self.words.contains(&word) {
            return -1;
        }

        let score = match_count(&word, &self.secret) as i32;
        if score == 6 {
            self.found.set(true);
        }
        score
    }

    fn solved_within_limit(&self) -> bool {
        self.found.get() && self.guesses_used.get() <= self.allowed_guesses
    }
}

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        if words.is_empty() {
            return;
        }

        let mut candidates = (0..words.len()).collect::<Vec<_>>();
        let pairwise = build_pairwise_matches(&words);

        while !candidates.is_empty() {
            let guess_index = pick_minimax_guess(&candidates, &pairwise);
            let guess_word = words[guess_index].clone();
            let score = master.guess(guess_word);

            if score == 6 || score < 0 {
                return;
            }

            let score = score as usize;
            candidates.retain(|&idx| pairwise[guess_index][idx] == score);
        }
    }
}

fn build_pairwise_matches(words: &[String]) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; words.len()]; words.len()];
    for i in 0..words.len() {
        for j in i..words.len() {
            let value = match_count(&words[i], &words[j]);
            matrix[i][j] = value;
            matrix[j][i] = value;
        }
    }
    matrix
}

fn pick_minimax_guess(candidates: &[usize], pairwise: &[Vec<usize>]) -> usize {
    let mut best_index = 0;
    let mut best_worst_bucket = usize::MAX;

    for &guess_idx in candidates {
        let mut buckets = [0usize; 7];
        for &candidate_idx in candidates {
            let m = pairwise[guess_idx][candidate_idx];
            buckets[m] += 1;
        }
        let worst_bucket = buckets.iter().copied().max().unwrap_or(0);
        if worst_bucket < best_worst_bucket {
            best_worst_bucket = worst_bucket;
            best_index = guess_idx;
        }
    }

    best_index
}

fn match_count(a: &str, b: &str) -> usize {
    a.bytes().zip(b.bytes()).filter(|(x, y)| x == y).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example_within_limit() {
        let words = vec!["acckzz", "ccbazz", "eiowzz", "abcczz"]
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let master = Master::new(words.clone(), "acckzz", 10);

        Solution::find_secret_word(words, &master);

        assert!(master.solved_within_limit());
    }

    #[test]
    fn solves_multiple_secrets() {
        let words = vec![
            "wichbx", "oahwep", "tpulot", "trbzyb", "hwayay", "kqkzfj", "cchhvm", "mcmqvp",
            "qcyxgn", "lsgyuk", "wzdrfk", "exxkzm", "jykrnv", "tqubwh", "uacncn", "rjtbqm",
            "hbvcyq", "nslmrd", "cooyyl", "rjibgb",
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

        for secret in &words {
            let master = Master::new(words.clone(), secret, 10);
            Solution::find_secret_word(words.clone(), &master);
            assert!(master.solved_within_limit(), "secret={secret}");
            assert!(master.guesses_used.get() <= 10, "secret={secret}");
        }
    }
}
