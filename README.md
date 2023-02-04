# web-server-test

Setting up and trialling a simple web server in multiple languages:
* Golang (using `net/http`)
* Rust (Using `actix_web`)
* Python3 (using `Flask`)

The goal here is to see which of the two main langauges (Golang and Rust) are
easier and more suitable to work with for another project I want to work on.

### To run
* `Go` - Run `go run .` and open `localhost:8080/users` in your web browser
* `Rust` - Run `cargo run` and open `localhost:5656/users` in your web browser
    * This will require the installation of `libsqlilte-dev`
* `Python` - This is more complex than just a single command as I use a
virtualenv here.

```bash
python3 -m virtualenv venv && \
. venv/bin/activate && \
pip install flask && \
flask --app python/main run
```

Run these commands (put together in a single command to just copy the block)
and open `localhost:5000/users` in your web browser


### Definition of Done
* Connect to an SQLite database file and read names from a `users` table
* Serve a simple html file to the localhost listing all of the names in the
database
* Adjust the displayed webpage with some basic CSS
* Display a textbox for the user to enter a new name and a button that - when
pressed - will enter the text from the textbox into the atabase as a new
username record

I'll also be recording my thoughts as I work through this project in each
language for future reference. Please see `thoughts.md` for more.

Note that Rust and Golang are my focus with this - I will add Python later on
as an experiment to see how easy it is to achieve the same result in that
language too

---

Creating the database:

```sql
CREATE TABLE users(user_id INTEGER PRIMARY KEY AUTOINCREMENT, user_name CHAR(50) NOT NULL UNIQUE);
INSERT INTO users VALUES(NULL, 'Tom'), (NULL, 'Lily'), (NULL, 'Isla'), (NULL, 'Damian');
```

---

### Future expansion
I'd like to experiment around with trying the same thing in lua one day because
it's a pretty simple scripting language, although I think this will require a
dockerfile environment for it's requirements.
