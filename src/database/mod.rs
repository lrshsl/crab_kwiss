
mod sql_database;
mod kwiss_db;
mod traits;
mod utils;


pub use kwiss_db::KwissDatabase;
pub use utils::WordPair;


// Database public API
// Exists because of debug printing ;>
pub(crate) struct Database {
    db: KwissDatabase,
}

impl Default for Database {
    fn default() -> Database {
        Self {
            db: KwissDatabase::default()
        }
    }
}

impl Database {
    pub fn new(db_name: &str) -> Self {
        Self {
            db: KwissDatabase::Sqlite::new(db_name),
        }
    }

    fn get_set_as_vec(&self, set_name: &str) -> Result<Vec<WordPair>, rusqlite::Error> {
        self.db.get_entries(set_name)
    }

    fn add_to_set(&mut self, set_name: &str, word: &str, definition: &str) {
        self.db.add_to_set(set_name, word, definition);
        println!("adding {}, {} to {}", word, definition, set_name);
    }

    fn create_set(&mut self, name: &str) {
        self.db.create_set(name);
        println!("creating set {}", name);
    }
}


/*
fn _old_main() {
    let args = std::env::args().collect::<Vec<_>>();

    // Parse Arguments
    match args[1..].iter().map(|s| s.as_str()).collect::<Vec<_>>()[..] {
        ["create", set_name] => create_set(set_name),
        ["add", word, def, "to", set_name] => {
            add_to_set(set_name, word, def);
        }
        ["start", _mode, set_name] => {
            for e in db_instance().get_entries(set_name).unwrap_or_default() {
                println!("{}", e.word);
            }
        }
        ["dump", set_name] => {
            for e in db_instance().get_entries(set_name).unwrap_or_else(|err| {
                println!("Something went wrond with fetching the values: {}", err);
                vec![]}) {
                println!("{} : {}", e.word, e.definition);
            }
        }
        ["-h"] | ["--help"] => throw_error(&HELP_MENU, 0),
        _ => throw_error(&HELP_MENU, 1),
    }
}
*/
