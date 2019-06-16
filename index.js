const express = require("express");
const generateData = require("./generateData");

const app = express();
const port = 3000;

app.use(function(req, res, next) {
  res.header("Access-Control-Allow-Origin", "*");
  res.header(
    "Access-Control-Allow-Headers",
    "Origin, X-Requested-With, Content-Type, Accept"
  );
  next();
});

app.get("/", (req, res) =>
  res.send({
    all: {
      "2019-01-01T00:00:0": generateData.generate(),
      "2019-02-01T00:00:0": generateData.generate(),
      "2019-03-01T00:00:0": generateData.generate()
    }
  })
);

app.listen(port, () => console.log(`Example app listening on port ${port}!`));
