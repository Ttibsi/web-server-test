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
	Css string
}

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Println(User_list)
	title := r.URL.Path[1:]
	filename := "http_go/" + title + ".html"
	css_file, _ := os.ReadFile("http_go/style.css")

	c := Content{User_list: User_list, Css: string(css_file)}
	t, _ := template.ParseFiles(filename)
	t.Execute(w, c)
}

func Serve(users []string) {
	User_list = users
	http.HandleFunc("/", handler)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
