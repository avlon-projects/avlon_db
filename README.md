# A simple embedded struct database wrapper based on Sled.

## basic usage
``` rust
// Define the database name
let db_name = String::from("test_db");

// Create an instance of Account
let account = Account {
    username: String::from("johndoe"),
    password: String::from("secretpassword"),
    age: 30,
};

// Initialize the database
let db = AvlonDB::new(db_name);

// Save the account to the database
db.save(account.username.clone(), account)?;
println!("Account saved to database!");

// Load the account from the database
if let Some(loaded_account) = db.load::<Account>("johndoe")? {
    println!("Account loaded from database: {:?}", loaded_account);
} else {
    println!("Account not found in the database.");
}

// Update the account in the database
let updated_account = Account {
    username: String::from("joker"),
    password: String::from("123654987"),
    age: 99,
};
db.update("johndoe", updated_account)?;

// Load the updated account from the database
if let Some(loaded_account) = db.load::<Account>("johndoe")? {
    println!("Updated account loaded from database: {:?}", loaded_account);
} else {
    println!("Account not found in the database.");
}

// Remove the account from the database
db.remove("johndoe")?;

// Attempt to load the account again after removal
if let Some(loaded_account) = db.load::<Account>("johndoe")? {
    println!("Account loaded from database after removal: {:?}", loaded_account);
} else {
    println!("Account successfully removed from the database.");
}
```

## range usage
``` rust
use serde::{Deserialize, Serialize};
use avlon_db::AvlonDB;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    value: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = AvlonDB::new("test_db".to_string());

    // Insert some data into the database
    for i in 0..10 {
        let data = Data {
            name: format!("name_{}", i),
            value: i,
        };
        db.save(format!("key_{}", i), data)?;
    }

    let data = db.load::<Data>("key_0")?;
    println!("{:?}", data);

    // Load a range of data from the database
    let results = db.load_range::<Data>("key_0", "key_9")?;
    for data in results {
        println!("{:?}", data);
    }

    Ok(())
}
```
