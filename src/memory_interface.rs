struct Account {
    id: u32,
    service: String,
    username: String,
    password: String,
    notes: String,
}

let mut user_accounts = Vec::new();

pub fn create_account(service: String, username: String, password: String, notes: String) {
    let new_account = Account {
        id: user_accounts.len(),
        service: service,
        username: username,
        password: password,
        notes: notes,
    }
}
