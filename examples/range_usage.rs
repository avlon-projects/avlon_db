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
