# Temperature graph on ESP

ESP32-C3, PT1000 and MAX31865 based temperature sensor node. The data can be
accessed through a webpage, hosted on the ESP itself, which displays a
convenient live-updating graph. Still a work in progress, as the software was
written before the hardware was complete.

To use this, change the IP in `plot.js` to your ESP's assigned local IP. Create
a `cfg.toml` file with your wifi credentials, see `cfg.toml.example` for the
format. If all goes well, you can run the code with `cargo run` and go to
`http://<ESP's local ip>/` (no https!) in your browser and see the live
temperature graph.

(the crate is called `oliebol` because we are going to use this sensor to fry
traditional Dutch oliebollen on New Year's Eve with high precision :) )
