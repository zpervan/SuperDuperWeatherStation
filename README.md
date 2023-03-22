# SuperDuperWeatherStation #

This is a small weather station project which consists which measures the temperature and moisture then sends this data to a dedicated server and which data is accessible from a web browser. Currently, it works on Windows.

## Environment ##

The weather station consists of two components:

### Client (measuring station) ###

The client is where the actual measuring will take in place. The client hardware components are the following:
- ESP01 WiFi module - brain of the client which will acquire the sensor data and send it afterwards to the server
- DHT22 - temperature and humidity sensor

### Server ###

The measured data will be stored on the server in a database and will be accessible from a web browser or basically anything that allows to make CRUD requests (i.e. curl). The server consists of the following:
- Golang 1.20
- MongoDb

## Build ##

### Client ###
0. Make sure you have installed the [Arduino IDE](https://www.arduino.cc/en/software) and accompanying [ESP8266 package](https://randomnerdtutorials.com/how-to-install-esp8266-board-arduino-ide/)
1. Connect the ESP01 module to your PC and make sure that the module is set in flashing/programming mode 
2. Select the `Port` and `Board` (label: `Generic ESP8266 board`) inside the IDE (located under tools)
3. Compile (and upload) the code

Currently, there are environment variables located in the `.ino` file which are:
- `SSID` - WiFi network name
- `PASSWORD` - WiFi network password
- `PING_URL` - endpoint URL which contains the "ping" functionality to check whether the connection is alive

Make sure that you adjust those variables to suit your needs.
Example:
```cpp
const char* SSID = "MyWifi";
const char* PASSWORD = "Password1234";
const char* PING_URL = "http://192.168.1.1:3500/ping" // Assuming that the server is on your local network, make sure to obtain the correct IP address
```
### Server ###

0. Make sure you have [Go 1.20](https://go.dev/doc/install) installed 
1. Navigate to the `server` folder
2. Update the dependencies and imports by executing `go mod tidy`
3. Build the project by executing `go build . -o server`

## Run ##

1. Run the server by executing the executable or running the command `go run .`
2. Power on the ESP01 module - hook it up to a power supply or a USB port (allows us to use the serial monitor)

## Schematic ##
TODO