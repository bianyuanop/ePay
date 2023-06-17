from flask import Flask, request

app = Flask(__name__)

@app.route('/', methods=['GET', 'POST'])
def receiver():
    if request.method == 'POST':
        data = request.data
        print(data)
    else:
        print("not posting")

if __name__ == "__main__":
    app.run(port=9000)
