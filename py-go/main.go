package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	_ "github.com/mattn/go-sqlite3"
)

func main() {
	r := chi.NewRouter()
	r.Use(middleware.RequestID)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)

	r.Use(middleware.Timeout(60 * time.Second))

	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Hello World"))
	})

	r.Get("/users", getUsers)

	fmt.Println("Serving on localhost:9988")
	http.ListenAndServe(":9988", r)
}

func getUsers(w http.ResponseWriter, r *http.Request) {
	db, err := sql.Open("sqlite3", "./db.db")
	if err != nil {
		fmt.Println("Error:" + err.Error())
	}
	rows, err := db.Query("SELECT user_name FROM users;")
	if err != nil {
		fmt.Println("Error:" + err.Error())
	}
	defer rows.Close()

	var users []string

	for rows.Next() {
		var u string
		err := rows.Scan(&u)
		if err != nil {
			fmt.Println("Error:" + err.Error())
		}
		users = append(users, u)
	}

	db.Close()

	resp, err := json.Marshal(users)
	if err != nil {
		fmt.Println("Error:" + err.Error())
	}
	w.Write(resp)
}
