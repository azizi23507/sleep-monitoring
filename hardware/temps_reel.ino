#include <DHT.h>
#define DHTPIN 2
#define DHTTYPE DHT11
#define PIR_PIN 3
#define SOUND_PIN A0
DHT dht(DHTPIN, DHTTYPE);

void setup() {
  Serial.begin(9600);
  pinMode(PIR_PIN, INPUT);
  dht.begin();
}

void loop() {
  float temp = dht.readTemperature();
  float hum = dht.readHumidity();
  bool motion = digitalRead(PIR_PIN);
  int sound = analogRead(SOUND_PIN);
  
  Serial.print("{\"temp\":"); Serial.print(temp,1);
  Serial.print(",\"hum\":"); Serial.print(hum,1);
  Serial.print(",\"motion\":"); Serial.print(motion ? "true" : "false");
  Serial.print(",\"sound\":"); Serial.print(sound);
  Serial.println("}"); 
  
  delay(1000);
}

