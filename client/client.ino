/* Includes */
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <ESP8266HTTPClient.h>

/* Environment variables */
const char* SSID = "YourWifiNetworkName";
const char* PASSWORD = "YourWifiNetworkPassword";
const char* PING_URL = "YourPingUrlEndpoint";
 
/* Global variables */
ESP8266WiFiMulti wifi_multi;
WiFiClient client;
HTTPClient http;
uint8_t led_status = LOW; 

void setup()
{
  // Built-In LED 1 indicator
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(115200);

  // Wait for 2 seconds so we can read the serial message log entirely
  delay(2000);

  digitalWrite(LED_BUILTIN, HIGH);
  Serial.println("Initializing");
  
  for(uint8_t i = 0; i < 4; i++)
  {
    Serial.println(".");
    delay(500);
  }

  // Set the WiFi mode to "Station mode"
  WiFi.mode(WIFI_STA);
  wifi_multi.addAP(SSID, PASSWORD);

  wl_status_t status = wifi_multi.run();
  if(status != WL_CONNECTED)
  {
    Serial.printf("Initilization failed. Status code: %d\n", status);
    Serial.flush();
  }
  else
  {
    Serial.println("Initialization finished successfully");
  }

  digitalWrite(LED_BUILTIN, LOW);
}

void loop() 
{
  // The while is used just for the "continue" statements so we can skip code execution
  while(true)
  {
    delay(5000);

    // Check if the WiFi connection is still alive
    if(wl_status_t status = wifi_multi.run(); status != WL_CONNECTED)
    {
      Serial.printf("Error while using WiFi. Status code: %d\n", status);
      Serial.flush();
      continue;
    }

    if(!http.begin(client, PING_URL))
    {
      Serial.println("Endpoint not reachable");
      continue;
    }

    if(int http_status_code = http.GET(); http_status_code != HTTP_CODE_OK)
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
}