
pub trait Database {
    fn open_or_create(name: String) -> Self;
    fn create_table(&mut self, tablename: &str, query: &str);
    fn add_to_table(&self, table: &str, keys: Vec<&str>, values: Vec<&str>);
    fn fetch_strings_from_table(
        &self,
        table: &str,
        keys: [&str; 2],
        query: &str,
    ) -> Result<Vec<[String; 2]>, rusqlite::Error>;
    fn execute_query(&self, query: String, values: Vec<&str>);
}

