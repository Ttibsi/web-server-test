.PHONY: go
go:
	cd golang && go run .

.PHONY: rust 
rust:
	cd rust && cargo run .
