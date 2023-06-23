package main

import (
	"fmt"

	"github.com/Ttibsi/web-server-test/pkg"
)

func main() {
	users := pkg.Read_from_db()
	fmt.Println("Serving on localhost:8080")
	pkg.Serve(users)
}
