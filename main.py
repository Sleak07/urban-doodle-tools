from flask import Flask

app = Flask(__name__)


@app.route("/")
def hello_world():
    return "<p>Hello,!</p>"


@app.route("/about")
def bye():
    return "<p>red!</p>"


if __name__ == "__main__":
    app.run(debug=True)
