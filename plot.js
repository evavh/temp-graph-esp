websocket = new WebSocket("ws://192.168.1.15:3012");
websocket.onmessage = (event) => {
  console.log(event.data);
};

PLOT = document.getElementById("plot");
Plotly.newPlot(PLOT, [{
  x: [1, 2, 3, 4, 5],
  y: [1, 2, 4, 8, 16],
}], {
  margin: { t: 0 },
});
