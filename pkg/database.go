package pkg

import (
	"database/sql"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

func Handle_error(e error) {
	if e != nil {
		log.Panicln(e.Error())
	}
}

func insertIntoDB(name string) {
	// This doesn't have any validation
	db, err := sql.Open("sqlite3", "./db.db")
	Handle_error(err)
	_, e := db.Exec("INSERT INTO users VALUES(NULL, ?);", name)
	Handle_error(e)
	db.Close()
}

func Read_from_db() []string {
	db, err := sql.Open("sqlite3", "./db.db")
	Handle_error(err)
	rows, err := db.Query("SELECT user_name FROM users;")
	Handle_error(err)
	defer rows.Close()

	var users []string

	for rows.Next() {
		var u string
		err := rows.Scan(&u)
		Handle_error(err)
		users = append(users, u)
	}

	db.Close()
	return users
}
