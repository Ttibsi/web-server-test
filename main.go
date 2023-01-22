package main

import (
	"github.com/Ttibsi/web-server-test/pkg"
)

func main() {
	users := pkg.Read_from_db()
	pkg.Serve(users)
}
