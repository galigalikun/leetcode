struct Bank {
    balances: Vec<i64>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Bank {
            balances: balance,
        }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let acc1 = (account1 - 1) as usize;
        let acc2 = (account2 - 1) as usize;
        if acc1 >= self.balances.len() || acc2 >= self.balances.len() {
            return false;
        }
        if self.balances[acc1] < money {
            return false;
        }
        self.balances[acc1] -= money;
        self.balances[acc2] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let acc = (account - 1) as usize;
        if acc >= self.balances.len() {
            return false;
        }
        self.balances[acc] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let acc = (account - 1) as usize;
        if acc >= self.balances.len() {
            return false;
        }
        if self.balances[acc] < money {
            return false;
        }
        self.balances[acc] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
fn main() {
    let mut obj = Bank::new(vec![10, 100, 20, 50, 30]);
    assert_eq!(true, obj.withdraw(3, 10));
    assert_eq!(true, obj.transfer(5, 1, 20));
    assert_eq!(true, obj.deposit(5, 20));
    assert_eq!(false, obj.transfer(3, 4, 15));
    assert_eq!(false, obj.withdraw(10, 50));
}
