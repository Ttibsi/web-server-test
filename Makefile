.PHONY: go
go:
	cd golang && go run .

.PHONY: rust
rust:
	cd rust && cargo run .

.PHONY: python
python:
	rm -rf venv python/__pycache__
	virtualenv venv
	venv/bin/pip install flask
	venv/bin/flask --app python/main run

.PHONY: clean
clean:
	rm -rf venv \
		python/__pycache__ \
		Cargo.lock \
