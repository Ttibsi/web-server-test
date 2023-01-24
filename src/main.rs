use rusqlite::{Connection, Result};

fn read_from_db() -> Vec<String> {
    let query = "SELECT user_name FROM users;";

    let conn = Connection::open("./db.db").unwrap();
    let mut stmt = conn.prepare(query).unwrap();

    return stmt
        .query_map([], |row| Ok(row.get(0).unwrap()))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
}

fn insert_to_db(name: String) -> Result<usize> {
    let conn = Connection::open("./db.db").unwrap();
    return conn.execute("INSERT INTO users VALUES(?1)", [name]);
}

fn main() {
    println!("{:?}", read_from_db());
}
