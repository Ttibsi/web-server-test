import sqlite3

from flask import Flask
from flask import render_template
from flask import send_from_directory

# from python import webserver

app = Flask(__name__)


@app.route("/")
def hello_world():
    return "<p>Hello, World!</p>"


@app.route("/users")
def users():
    users: list[str] = read_from_db()
    return render_template("users.html", users=users)


@app.route("/style.css")
def styles():
    return send_from_directory("templates", "style.css")


def read_from_db() -> list[str]:
    con = sqlite3.connect("db.db")
    cur = con.cursor()
    res = cur.execute("SELECT user_name FROM users;")

    ret = []
    for val in res.fetchall():
        ret.append(val[0])

    return ret


def insert_into_db(name: str) -> None:
    con = sqlite3.connect("db.db")
    cur = con.cursor()
    cur.execute(f"INSERT INTO users VALUES (NULL, {name})")
    con.commit()


def main() -> int:
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
