use super::traits::Database;
use super::sql_database::SqliteDatabase;
use super::utils::WordPair;


pub enum KwissDatabase {
    Sqlite(SqliteDatabase),
}

impl Default for KwissDatabase {
    fn default() -> Self {
        Self::Sqlite(SqliteDatabase::open_or_create(
            "kwiss_default_sqlite.db".to_string(),
        ))
    }
}

impl KwissDatabase {

    pub fn add_to_set(&mut self, set_name: &str, word: &str, definition: &str) {
        match self {
            Self::Sqlite(db) => {
                db.add_to_table(set_name, vec!["word", "definition"], vec![word, definition]);
            }
        }
    }

    pub fn create_set(&mut self, set_name: &str) {
        match self {
            Self::Sqlite(db) => {
                db.create_table(
                    set_name,
                    format!(
                        "
                CREATE TABLE IF NOT EXISTS {} (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    word TEXT NOT NULL,
                    definition TEXT NOT NULL
                );",
                        set_name
                    )
                    .as_str(),
                );
            }
        }
    }

    pub fn get_entries(&self, set_name: &str) -> Result<Vec<WordPair>, rusqlite::Error> {
        match self {
            Self::Sqlite(db) => Ok(
                db.fetch_strings_from_table(set_name, ["word", "definition"], "")?
                .iter()
                .map(|r| WordPair {
                    word: r[0].to_string(),
                    definition: r[1].to_string(),
                })
                .collect::<Vec<_>>()
                )
        }
    }
}

