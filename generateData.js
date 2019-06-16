const uuidv1 = require("uuid/v1");

module.exports = {
  generate: function() {
    let responses = [];
    for (var i = 0; i < this.randomIntFromInterval(10000, 10000); i++) {
      responses.push(this.randomIntFromInterval(0, 10).toString());
    }
    return responses;
  },
  randomIntFromInterval: function(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }
};
