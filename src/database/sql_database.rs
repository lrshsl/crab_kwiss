use super::traits::Database;

fn to_sql_string_values(values: &[&str]) -> String {
	values
		.iter()
		.map(|s| format!("'{}'", s))
		.collect::<Vec<String>>()
		.join(",")
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
	pub name: String,
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
			.unwrap()
			.execute([]).unwrap();
	}

	fn fetch_strings_from_table(
		&self,
		table: &str,
		keys: [&str; 2],
		condition: &str,
		) -> Result<Vec<[String; 2]>, rusqlite::Error> {
		let query = if condition.is_empty() {
			format!("SELECT {} FROM {}", keys.join(","), table)
		} else {
			format!("SELECT {} FROM {} WHERE {}", keys.join(","), table, condition)
		};
		self.connection
			.prepare(&query)?
			.query([])?
			.mapped(|row| {
				Ok([
				   row.get::<usize, String>(0)?.to_string(),
				   row.get::<usize, String>(1)?.to_string(),
				])
			})
		.collect::<Result<Vec<[String; 2]>, rusqlite::Error>>()
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
