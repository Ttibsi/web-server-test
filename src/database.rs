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

pub fn insert_to_db(name: String) -> usize {
    let conn = Connection::open("./db.db").unwrap();
    println!("Trying to insert");
    let entered = match conn.execute("INSERT INTO users VALUES(NULL, ?1)", [name]) {
        Ok(x) => x,
        Err(_) => 0 
    };
    println!("{} inserted into db", entered);
    return entered;
}
