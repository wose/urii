use serde::de::Deserialize;
use serde::Serialize;
use serde_json;
use rusqlite;

#[derive(Debug)]
pub enum StoreError {
    Sql(rusqlite::Error),
    Serde(serde_json::Error)
}

pub struct Store {
    pub db: rusqlite::Connection,
}

impl Store {
    pub fn new(db: &str) -> Self {
        let connection = rusqlite::Connection::open(db).expect("Couldn't open db file");

        connection.execute("CREATE TABLE IF NOT EXISTS bot_store (
                            id INTEGER PRIMARY KEY ASC,
                            namespace TEXT,
                            key TEXT,
                            value TEXT)",
                           &[])
            .expect("Failed to create db schema");

        Store { db: connection }
    }

    pub fn get<T: Deserialize>(&self, namespace: &str, key: &str) -> Result<T, StoreError> {
        let value: String = self.db
            .query_row("SELECT value FROM bot_store WHERE namespace = ? AND key = ?",
                       &[&namespace, &key],
                       |row| row.get(0)).map_err(|e| { StoreError::Sql(e)})?;

        match serde_json::from_str(&value) {
            Ok(value) => Ok(value),
            Err(e) => Err(StoreError::Serde(e))
        }
    }

    pub fn set<T: Serialize>(&self,
                             namespace: &str,
                             key: &str,
                             value: T)
                             -> Result<(), StoreError> {
        self.db
            .execute("DELETE FROM bot_store WHERE namespace = ? AND key = ?",
                     &[&namespace, &key])
            .unwrap();

        self.db
            .execute("INSERT INTO bot_store ( namespace, key, value ) VALUES ( ?, ?, ? )",
                     &[&namespace, &key, &serde_json::to_string(&value).unwrap()])
            .unwrap();
        Ok(())
    }
}
