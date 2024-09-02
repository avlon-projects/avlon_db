use serde::{Serialize, Deserialize};
use sled::{self, Db};

/// A struct representing a wrapper around a Sled database.
pub struct AvlonDB {
    pub db_name: String,
    pub client: Db,
}

impl AvlonDB {
    /// Creates a new instance of AvlonDB with the specified database name.
    ///
    /// # Arguments
    ///
    /// * `db_name` - The name of the database file.
    ///
    /// # Returns
    ///
    /// A new `AvlonDB` instance.
    pub fn new(db_name: String) -> Self {
        let client = sled::open(&db_name).expect("Failed to open Sled database!");
        AvlonDB { db_name, client }
    }

    /// Saves a value to the database under the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to associate with the value.
    /// * `value` - The value to store in the database.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn save<T>(&self, key: String, value: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let serialized_data = serde_json::to_vec(&value)?;
        self.client.insert(key, serialized_data)?;
        Ok(())
    }

    /// Loads a value from the database associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to load.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the value, or `None` if the key does not exist.
    pub fn load<T>(&self, key: &str) -> Result<Option<T>, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(data) = self.client.get(key)? {
            let value: T = serde_json::from_slice(&data)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Removes a value from the database associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to remove.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn remove(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client.remove(key)?;
        Ok(())
    }

    /// Updates the value associated with the given key in the database.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to update.
    /// * `new_value` - The new value to store in the database.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. Returns an error if the key does not exist.
    pub fn update<T>(&self, key: &str, new_value: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        if self.client.contains_key(key)? {
            let serialized_data = serde_json::to_vec(&new_value)?;
            self.client.insert(key, serialized_data)?;
            Ok(())
        } else {
            Err(format!("Key '{}' does not exist in the database.", key).into())
        }
    }
}
