use rusqlite::{Connection, Result};

pub fn read_from_db() -> Vec<String> {
    let query = "SELECT user_name FROM users;";

    let conn = Connection::open("./db.db").unwrap();
    let mut stmt = conn.prepare(query).unwrap();

    return stmt
        .query_map([], |row| Ok(row.get(0).unwrap()))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
}

pub fn insert_to_db(name: String) -> Result<usize> {
    let conn = Connection::open("./db.db").unwrap();
    return conn.execute("INSERT INTO users VALUES(?1)", [name]);
}
