struct AuthenticationManager {
    time_to_live: i32,
    tokens: std::collections::HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuthenticationManager {

    fn new(timeToLive: i32) -> Self {
        AuthenticationManager { time_to_live: timeToLive, tokens: std::collections::HashMap::new() }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        // Check if the token is already present
        if self.tokens.contains_key(&token_id) {
            // If it is, update the expiration time
            self.tokens.insert(token_id, current_time + self.time_to_live);
        } else {
            // If not, add the token with its expiration time
            self.tokens.insert(token_id, current_time + self.time_to_live);
        }
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        // Check if the token is present and not expired
        if let Some(&expiration_time) = self.tokens.get(&token_id) {
            if current_time < expiration_time {
                // If it is, update the expiration time
                self.tokens.insert(token_id, current_time + self.time_to_live);
            }
        }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        // Count the number of tokens that are not expired
        self.tokens.values().filter(|&&expiration_time| current_time < expiration_time).count() as i32
    }
}

/**
 * Your AuthenticationManager object will be instantiated and called as such:
 * let obj = AuthenticationManager::new(timeToLive);
 * obj.generate(tokenId, currentTime);
 * obj.renew(tokenId, currentTime);
 * let ret_3: i32 = obj.count_unexpired_tokens(currentTime);
 */
fn main() {
    let mut obj = AuthenticationManager::new(5);
    obj.renew("aaa".to_string(), 1);
    obj.generate("aaa".to_string(), 2);
    assert_eq!(obj.count_unexpired_tokens(6), 1);
    obj.generate("bbb".to_string(), 7);
    obj.renew("aaa".to_string(), 8);
    obj.renew("bbb".to_string(), 10);
    assert_eq!(obj.count_unexpired_tokens(15), 0);
}
