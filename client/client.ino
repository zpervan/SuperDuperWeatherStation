/* Includes */
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <ESP8266HTTPClient.h>
#include <DHT.h>

/* Web environment variables */
#define SSID     "YOUR-WIFI-NAME"
#define PASSWORD "YOUR-WIFI-PASS"

/* Endpoints - defined in the server code */
#define URL      "http://yourserveraddress:3500"
#define PING_URL URL "/ping"
#define ADD_URL  URL "/add"

/* Sensor configuration variables*/
#define DHT_PIN  2 // The data pin is connected to the ESP01 GPIO2 pin
#define DHT_TYPE DHT22

/* Global variables */
ESP8266WiFiMulti wifi_multi;
WiFiClient client;
HTTPClient http;
DHT dht(DHT_PIN, DHT_TYPE);
String temperature{"nan"};
String humidity{"nan"};

/* Forward declarations */
void ErrorBlink();

/* Bootstrap */
void setup()
{
  // Built-In LED 1 indicator
  pinMode(LED_BUILTIN, OUTPUT);

  Serial.begin(115200);

  // Wait for 2 seconds so we can read the serial message log entirely
  delay(2000);

  Serial.println("Initializing");
  
  /* WiFi connection setup */

  // Set the WiFi mode to "Station mode"
  WiFi.mode(WIFI_STA);
  wifi_multi.addAP(SSID, PASSWORD);

  wl_status_t status = wifi_multi.run();
  if(status != WL_CONNECTED)
  {
    Serial.printf("Connection failed. Status code: %d\n", status);
    Serial.flush();

    ErrorBlink();
  }
  else
  {
    Serial.println("Connection established");
  }
  
  /* Server setup */

  if(!http.begin(client, PING_URL))
  {
    Serial.printf("Connection to server failed");
    Serial.flush();

    ErrorBlink();
  }

  if(int http_status_code = http.GET(); http_status_code != HTTP_CODE_OK)
  {
    Serial.printf("PING request failed. Error: %s", http.errorToString(http_status_code).c_str());
    Serial.flush();

    ErrorBlink();
  }

  /* Sensor setup */

  dht.begin();

  Serial.println("Initialization finished successfully");
}

/* Main loop */
void loop() 
{
  delay(10000);

  // Check if the WiFi connection is still alive
  if(wl_status_t status = wifi_multi.run(); status != WL_CONNECTED)
  {
    Serial.printf("Error while using WiFi. Status code: %d\n", status);
    Serial.flush();
    ErrorBlink();
  }

  // Read sensor data
  temperature = dht.readTemperature();
  humidity = dht.readHumidity();

  // Example payload: { "temperature":20.45,"humidity":54.27" }
  String sensor_data{"{\"temperature\":"};
  sensor_data.concat(temperature);
  sensor_data.concat(",\"humidity\":");
  sensor_data.concat(humidity);
  sensor_data.concat("}");

  // Send data to server
  if(!http.begin(client, ADD_URL))
  {
    Serial.println("Endpoint not reachable");
    ErrorBlink();
  }

  http.addHeader("Content-Type", "Content-Type: application/json");

  if(int http_status_code = http.POST(sensor_data); http_status_code != HTTP_CODE_OK)
  {
    Serial.printf("GET request failed. Error: %s", http.errorToString(http_status_code).c_str());
    Serial.flush();
  }
  else
  {
    Serial.println(http.getString());
  }

  http.end();
}

/// @brief Blinks the built-in LED where one half-period lasts for 500ms, and this way allows to immidieately see whether something went wrong.
void ErrorBlink()
{
  while(true)
  {
    digitalWrite(LED_BUILTIN, HIGH);
    delay(500);
    digitalWrite(LED_BUILTIN, LOW);
    delay(500);
  }
}