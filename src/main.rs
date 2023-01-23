use rusqlite::Connection;

// https://docs.rs/rusqlite/latest/rusqlite/struct.Statement.html#method.query_map
fn read_from_db() -> Vec<String> {
    let mut ret = vec![]; 
    let query = "SELECT user_name FROM users;";

    let conn = Connection::open("./db.db").unwrap();
    let mut stmt = conn.prepare(query).unwrap();
    let result = stmt.query_map([], |row| row.get(0)).unwrap();

    for name in result { ret.push(name.unwrap());}

    return ret;
}

fn main() {
    println!("{:?}", read_from_db());
}
