PLOT = document.getElementById("plot");
Plotly.newPlot(PLOT, [{ x: [], y: [] }], {
  title: "Oliebol",
  xaxis: { title: "Tijd" },
  yaxis: { title: "Temperatuur [°C]" },
}, { responsive: true });

// IP for the ESP, with same port as Rust code websocket
websocket = new WebSocket("ws://192.168.1.15:3012");
websocket.onmessage = (event) => {
  event.data.bytes().then((bytes) => {
    timestamp = toInt(bytes.slice(0, 8));
    temp = toInt(bytes.slice(8, 10)) / 10;
    time = formatTime(new Date(timestamp * 1000));

    Plotly.extendTraces(PLOT, { x: [[time]], y: [[temp]] }, [0], 120);
  });
};

function toInt(byteArray) {
  result = 0;
  for (let i = 0; i < byteArray.length; i++) {
    result += byteArray[i] * Math.pow(256, byteArray.length - i - 1);
  }
  return result;
}

function formatTime(date) {
  hour = ("0" + date.getHours()).slice(-2);
  minute = ("0" + date.getMinutes()).slice(-2);
  second = ("0" + date.getSeconds()).slice(-2);
  return hour + ":" + minute + ":" + second;
}
