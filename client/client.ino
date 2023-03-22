/* Includes */
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <ESP8266HTTPClient.h>

/* Environment variables */
const char* SSID = "WifiNetworkName";
const char* PASSWORD = "WifiNetowrkPassword";
 
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
    delay(1000);
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
    if(wl_status_t status = wifi_multi.run(); status != WL_CONNECTED)
    {
      Serial.printf("Error while using WiFi. Status code: %d\n", status);
    }
    else 
    {
      Serial.println("Do some work");
    }

    delay(4000);
}