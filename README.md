# SuperDuperWeatherStation #

This is a small weather station project which measures the environment temperature and humidity then sends this data to
a dedicated server and visualized afterwards.
Currently, it was developed on Windows 11, and works on Ubuntu too but additional libraries need to be installed.

## Environment ##

The weather station consists of three components:

### Client (measuring station) ###

The client is where the actual measuring will take in place. The client hardware components consists of the following:

- ESP01 - microcontroller with a built-in Wi-Fi module
- DHT22 - temperature and humidity sensor

### Server ###

The acquired data will be stored on the server in a MongoDB database and is accessible from a web browser or basically
anything that allows to make HTTP requests (i.e. curl, Postman etc.). The server consists of the following:

- Golang 1.20
- MongoDB

The server components are wrapped in Docker files which allows to instantly build and run the server and database
quickly. Make sure the Docker environment is properly installed in your machine.

### Frontend ###

Visualizes the weather data by temperature and humidity on a selected date. The application itself is written in Rust
and is available for desktop.

![desktop_app](assets/visualization_app_preview.png)

## Setup ##

### Client ###

1. Make sure the [Arduino IDE](https://www.arduino.cc/en/software) is installed with the
   accompanying [ESP8266 package](https://randomnerdtutorials.com/how-to-install-esp8266-board-arduino-ide/)
2. Connect the ESP01 module to your PC and make sure that the module is set in flashing/programming mode
3. Select the `Port` and `Board` (label: `Generic ESP8266 board`) inside the IDE (located under tools)
4. Compile (and upload) the code

Currently, there are environment variables located in the `.ino` file which need to be changed for your case, and those
variables are:

- `SSID` - WiFi network name
- `PASSWORD` - WiFi network password
- `URL` - server URL

Example:

```cpp
#define SSID     "MyWifi";
#define PASSWORD "Password1234";
#define URL      "http://192.168.1.1:3500/ping" 
```

Assuming that the server is added to your local network, make sure to obtain the correct IP address from your router.

### Server ###

Navigate your terminal to the root of the project and execute the following command:

```shell
docker-compose up -d --build
```

Also, make sure that you enabled communication on port `3500` in your firewall so the data can be received by the
server.

### Frontend ###

Make sure you change the API endpoint address in `frontend/src/requests.rs` on line `15` to the set local network
address, i.e.

```rust
// Endpoints
const BASE_URL: &str = "http://192.168.1.1:3500";
// Set your server IP address
const FETCH_WEATHER_DATA_BY_DATE_ENDPOINT: &str = "/get/";
const FETCH_DATES_ENDPOINT: &str = "/dates";
const FETCH_LATEST_DATE_ENDPOINT: &str = "/latest";
```

Additionally, Ubuntu 22.04 needs the following libraries in order to compile successfully:

```shell
sudo apt-get update
sudo apt-get install pkg-config libglib2.0-dev libpango1.0-dev libatk1.0-dev libgdk-pixbuf-2.0-dev libssl-dev
```

This is working out-of-the-box on Windows 11, so no special attention needed there.
Once the measuring station and server are **running**, execute the following command inside the `frontend` folder run
the desktop application.

```shell
cargo run .
```

## Schematic ##
<p align="center">
<img src="assets/schematic_weather_station.png" alt="JetBrains Logo" width="635"/>
</p>

## Smoke test ##

After the weather station is wired as shown in the schematic, turn on the power supply and the ESP01 should begin the
initialization process by connecting to the provided Wi-Fi data and send the an initial `ping` request to the server.

Make sure you follow the log output of the server by executing the following command:

```shell
docker logs sdws_server --follow
```

You should see all the activities on the server. Also, once the desktop application is running, click on the **circle
arrow button** to fetch the data.

## TODO ##

* [FRONTEND] Add input text field for server URL address
* [FRONTEND] Realtime weather data tracking
* [FRONTEND] Comparing data from different days
* [ESP8266] Median filter while capturing the environment data (reduce outliers possibility)

## Support ##

Special thanks to JetBrains for providing me with a
free [open-source](https://www.jetbrains.com/community/opensource/?utm_campaign=opensource&utm_content=approved&utm_medium=email&utm_source=newsletter&utm_term=jblogo#support)
license in order to create this project.
<p align="center">
<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo" width="200"/>
</p>