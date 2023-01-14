# web-server-test

Setting up and trialling a simple web server in multiple languages:
* Golang (using `net/http`)
* Rust (Using `yew.rs`)
* Python3 (using `Flask`)

The goal here is to see which of the two main langauges (Golang and Rust) are
easier and more suitable to work with for another project I want to work on.

### Definition of Done
* Connect to an SQLite database file and read names from a `users` table
* Serve a simple html file to the localhost listing all of the names in the 
database
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

