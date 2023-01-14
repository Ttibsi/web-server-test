package pkg

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

func read_from_db() []string {
	db, err := sql.Open("sqlite3", "./db.db")
	handle_error(err)
	rows, err := db.Query("SELECT user_name FROM users;")
	handle_error(err)
	defer rows.Close()

	var users []string 

	for rows.Next() {
		var u string
		err := rows.Scan(&u)
		handle_error(err)
		users = append(users, u)
	}
	
	return users
}

func handle_error(e error) {
	if e != nil {
		log.Panicln(e.Error())
	}
}

func Init() int {
	fmt.Println("marker")
	users := read_from_db()
	fmt.Println(users)
	return 0
}
