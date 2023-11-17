struct CombinationIterator {
    characters: String,
    combination_length: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combination_length: i32) -> Self {
        CombinationIterator { 
            characters: characters,
            combination_length: combination_length,
         }
    }
    
    fn next(&self) -> String {
        return self.characters
            .chars()
            .take(self.combination_length as usize)
            .collect::<String>();
    }
    
    fn has_next(&self) -> bool {
        return self.characters.len() >= self.combination_length as usize;
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
fn main() {
    let obj = CombinationIterator::new("abc".to_string(), 2);
    assert_eq!(obj.next(), "ab");
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), "ac");
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), "bc");
    assert_eq!(obj.has_next(), false);
}
