fn main() {
}

struct Bank {
    pub accounts: Vec<Account>,
}

#[derive(Debug)]
struct Account {
    pub balance: u64,
}

impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        let mut accounts = Vec::<Account>::new();

        for i in balance {
            accounts.push(Account { balance: i as u64});
        }

        Self {
            accounts,
        }
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = (account1 - 1) as usize;
        let account2 = (account2 - 1) as usize;

        let money = money as u64;
        
        if account1 > self.accounts.len() || self.accounts[account1].balance < money || account2 > self.accounts.len() {
            return false;
        }
        
        self.accounts[account1].balance -= money;
        self.accounts[account2].balance += money;
        true 
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account as usize > self.accounts.len() {
            return false;
        }
        self.accounts[(account - 1) as usize].balance += money as u64;
        true
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = (account - 1) as usize;
        let money = money as u64;
        if account > self.accounts.len() || self.accounts[account].balance < money {
            return false;
        }

        self.accounts[account].balance -= money;
        true
    }
}