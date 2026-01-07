use std::string;

#[derive(Debug)]
struct Account {
     id: u32,
     balance: i32,
     holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank{
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

impl Account {
    fn new(id:u32, holder:String) -> Self {
        Account { id, balance: 0, holder }
    }
}

fn print_account(mut account:Account) -> Account{
    println!("{:#?}",account);
    account.id =  account.id +1;
    account
}
fn main() {
    // let bank = Bank::new();

    // let other_bank = bank;

    let mut account = Account::new(1, String::from("me name"));
    let other_account = account;

    // println!("{:#?}",other_account);
    account = print_account(other_account);
    // account = print_account(other_account);
    // println!("{:#?}",account);
    print_account(account);

}
