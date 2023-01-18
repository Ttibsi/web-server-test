package main

import (
	"fmt"

	"github.com/Ttibsi/web-server-test/pkg"
)

func main() {
	users := pkg.Read_from_db()
	fmt.Println(users)
	pkg.Serve()
}
