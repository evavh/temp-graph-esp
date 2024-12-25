PLOT = document.getElementById("plot");
Plotly.newPlot(PLOT, [{ x: [], y: [] }], { margin: { t: 0 } });

websocket = new WebSocket("ws://192.168.1.15:3012");
websocket.onmessage = (event) => {
  event.data.bytes().then((bytes) => {
    timestamp = toInt(bytes.slice(0, 8));
    temp = toInt(bytes.slice(8, 10)) / 10;
    console.log(bytes);
    console.log(bytes.slice(8, 10));
    console.log(timestamp);
    console.log(temp);

    Plotly.extendTraces(PLOT, { x: [[timestamp]], y: [[temp]] }, [0]);
  });
};

function toInt(byteArray) {
  result = 0;
  for (let i = 0; i < byteArray.length; i++) {
    result += byteArray[i] * Math.pow(256, i - 1);
  }
  return result;
}
