import sqlite3

import flask

# from python import webserver

app = flask.Flask(__name__)


@app.route("/")
def hello_world():
    return "<p>Hello, World!</p>"


@app.route("/users")
def users():
    users: list[str] = read_from_db()
    return flask.render_template("users.html", users=users)


@app.route("/users", methods=["POST"])
def new_user():
    text = flask.request.form["user"]
    insert_into_db(text)
    return flask.redirect(flask.url_for("users"))


@app.route("/style.css")
def styles():
    return flask.send_from_directory("templates", "style.css")


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
    cur.execute(f'INSERT INTO users VALUES (NULL, "{name}");')
    con.commit()
