import flask
import requests

app = flask.Flask(__name__)


@app.route("/")
def hello_world():
    return "<p>Hello, World!</p>"


@app.route("/users")
def serve_users():
    # Call API
    resp = requests.get("http://localhost:9988/users")
    return resp.json()
    # pull in html and fill the template
    # Return html


if __name__ == "__main__":
    app.run(port=9876)
