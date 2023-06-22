package pkg

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"text/template"
)

var User_list []string

type Content struct {
	User_list []string
	Css       string
}

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Println(User_list)
	title := r.URL.Path[1:]
	filename := "http/" + title + ".html"
	css_file, err := os.ReadFile("http/style.css")
	if err != nil {
		fmt.Println("Error:" + err.Error())
	}

	c := Content{User_list: User_list, Css: string(css_file)}
	t, err := template.ParseFiles(filename)
	if err != nil {
		fmt.Println("Error:" + err.Error())
	}
	t.Execute(w, c)
}

func addHandler(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	fmt.Println(r.Form["user"])
	insertIntoDB(r.Form["user"][0])
	User_list = Read_from_db() // This is a bad way to do this
	http.Redirect(w, r, "/users", http.StatusSeeOther)
}

func Serve(users []string) {
	User_list = users
	http.HandleFunc("/", handler)
	http.HandleFunc("/new", addHandler)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
