const rust = import("./pkg/fetch");

rust
  .then(r => {
    js_run();
    let startTime = new Date();
    return r.run().then(data => {
      let endTime = new Date();
      data.time = endTime - startTime;
      console.log("rust");
      console.log(data);
    });
  })
  .catch(console.error);

function js_run() {
  let startTime = new Date();
  fetch("http://localhost:3000").then(resp => {
    resp.json().then(function(data) {
      // do work
      let distribution = {};
      let mostCommon = {};
      let average = {};
      const dates = Object.keys(data.all);
      dates.forEach(date => {
        distribution[date] = {};
        let temp = 0;
        let sum = 0;
        let count = data.all[date].length;
        data.all[date].forEach(response => {
          if (response in distribution[date]) {
            distribution[date][response]++;
          } else {
            distribution[date][response] = 1;
          }
          if (distribution[date][response] > temp) {
            temp = parseFloat(distribution[date][response]);
            mostCommon[date] = response;
          }
          sum = sum + parseFloat(response);
        });
        average[date] = sum / count;
      });
      var results = {
        distribution: distribution,
        most_common: mostCommon,
        average: average
      };
      // end work
      let endTime = new Date();
      results.time = endTime - startTime;
      console.log("js");
      console.log(results);
    });
  });
}
