const HELP_MENU: &str = "
USAGE:
    kwiss COMMAND [ARGS] [OPTIONS]

Commands:
    create <set>                            create a new set called <set>
    add <word> <definition> to <set>        add a new entry to <set>
    start <MODE> <set>                      start a new game in <MODE> mode     [Not yet implemented]
    help                                    show this help message

Modes:   [Not yet implemented]
    learning                                normal mode
    test                                    test mode

Options:
    -h, --help                              show this help message
    -c, --config <path>                     specify a custom config file        [Not yet implemented]
";

fn to_sql_string_values(values: &[&str]) -> String {
    values
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",")
}

fn throw_error(msg: &str, code: i32) {
    eprintln!("{}", msg);
    std::process::exit(code);
}

pub struct WordPair {
    pub word: String,
    pub definition: String,
}

pub trait Database {
    fn open_or_create(name: String) -> Self;
    fn create_table(&mut self, tablename: &str, query: &str);
    fn add_to_table(&self, table: &str, keys: Vec<&str>, values: Vec<&str>);
    fn fetch_strings_from_table(
        &self,
        table: &str,
        keys: [&str; 2],
        query: &str,
    ) -> Option<Vec<[String; 2]>>;
    fn execute_query(&self, query: String, values: Vec<&str>);
}

pub fn sql_values_from_vec(vec: Vec<&str>) -> String {
    let string = vec
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    format!("({})", string)
}

pub struct SqliteDatabase {
    name: String,
    connection: rusqlite::Connection,
}

impl Database for SqliteDatabase {
    fn open_or_create(name: String) -> Self {
        let connection = rusqlite::Connection::open(name.clone()).unwrap();
        Self { name, connection }
    }

    fn create_table(&mut self, tablename: &str, query: &str) {
        self.connection.execute(query, []).unwrap();
    }

    fn add_to_table(&self, table: &str, keys: Vec<&str>, values: Vec<&str>) {
        self.connection
            .prepare(&format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table,
                to_sql_string_values(&keys),
                to_sql_string_values(&values)
            ))
            .unwrap();
    }

    fn fetch_strings_from_table(
        &self,
        table: &str,
        keys: [&str; 2],
        condition: &str,
    ) -> Option<Vec<[String; 2]>> {
        Some(
            self.connection
                .prepare(&format!(
                    "SELECT {} FROM {} WHERE {}",
                    keys.join(","),
                    table,
                    condition
                ))
                .ok()?
                .query_map(keys, |row| Ok([row.get(0)?, row.get(1)?]))
                .ok()?
                .map(|r| r.unwrap())
                .collect::<Vec<_>>(),
        )
    }

    fn execute_query(&self, query: String, values: Vec<&str>) {
        self.connection
            .execute(
                &query,
                rusqlite::params_from_iter(values.iter().map(|s| s.to_string())),
            )
            .unwrap();
    }
}

enum KwissDatabase {
    Sqlite(SqliteDatabase),
}

impl KwissDatabase {
    pub fn get_instance() -> Self {
        Self::Sqlite(SqliteDatabase::open_or_create(
            "kwiss_default_sqlite.db".to_string(),
        ))
    }

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

    pub fn get_entries(&self, set_name: &str) -> Option<Vec<WordPair>> {
        match self {
            Self::Sqlite(db) => Some(
                db.fetch_strings_from_table(set_name, ["word", "definition"], "")?
                    .iter()
                    .map(|r| WordPair {
                        word: r[0].to_string(),
                        definition: r[1].to_string(),
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let KwissDatabase::Sqlite(SqliteDatabase { name, .. }) = KwissDatabase::get_instance();
    println!("db: {}", name);
    match args[1..].iter().map(|s| s.as_str()).collect::<Vec<_>>()[..] {
        ["add", word, def, "to", set_name] => {
            add_to_set(set_name, word, def);
        }
        ["create", set_name] => {
            create_set(set_name);
        }
        ["start", mode, set_name] => {
            for e in KwissDatabase::get_instance().get_entries(set_name).unwrap() {
                println!("{}", e.word);
            }
        }
        ["-h"] | ["--help"] => throw_error(HELP_MENU, 0),
        _ => throw_error(HELP_MENU, 1),
    }
}

fn add_to_set(set_name: &str, word: &str, definition: &str) {
    KwissDatabase::get_instance().add_to_set(set_name, word, definition);
    println!("adding {}, {} to {}", word, definition, set_name);
}

fn create_set(name: &str) {
    KwissDatabase::get_instance().create_set(name);
    println!("creating set {}", name);
}
