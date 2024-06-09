struct BrowserHistory {
    urls: Vec<String>,
    current: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            urls: vec![homepage],
            current: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.urls.push(url);
    }

    fn back(&mut self, steps: i32) -> String {
        let mut current = self.current - steps;
        if current < 0 {
            current = 0;
        }
        self.current = current;
        self.urls[current as usize].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let mut current = self.current + steps;
        if current >= self.urls.len() as i32 {
            current = self.urls.len() as i32 - 1;
        }
        self.current = current;
        self.urls[current as usize].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
fn main() {
    let mut obj = BrowserHistory::new("leetcode.com".to_string());
    obj.visit("google.com".to_string());
    obj.visit("facebook.com".to_string());
    obj.visit("youtube.com".to_string());
    let ret_2: String = obj.back(1);
    assert_eq!(ret_2, "facebook.com".to_string());
    let ret_3: String = obj.back(1);
    assert_eq!(ret_3, "google.com".to_string());
    let ret_4: String = obj.forward(1);
    assert_eq!(ret_4, "facebook.com".to_string());
    obj.visit("linkedin.com".to_string());
    let ret_5: String = obj.forward(2);
    assert_eq!(ret_5, "linkedin.com".to_string());
    let ret_6: String = obj.back(2);
    assert_eq!(ret_6, "google.com".to_string());
    let ret_7: String = obj.back(7);
    assert_eq!(ret_7, "leetcode.com".to_string());
}
