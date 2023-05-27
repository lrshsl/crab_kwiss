struct Database {
    name: String,
    connection: sqlite::Connection,
}

impl Database {
    pub fn new(name: String) -> Database {
        Database {
            name: name.clone(),
            connection: sqlite::open(name).unwrap(),
        }
    }
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args[1..].iter().map(|s| s.as_str()).collect::<Vec<_>>()[..] {
        ["add", word, def, "to", set_name] => {
            add_to_set(set_name, word, def);
        }
        _ => {
            println!("Usage: {} add <word> <definition> to <set_name>", args[0]);
            std::process::exit(1);
        }
    }
}

fn add_to_set(set_name: &str, word: &str, definition: &str) {
    println!("adding {}, {} to {}", word, definition, set_name);
}

fn create_set(db: &mut Database, name: &str) {
    println!("creating set {}", name);
}

