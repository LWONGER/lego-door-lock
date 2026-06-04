// ESP32 Lego Door Lock Button + LED Test
// Pull-down resistor button circuit:
// Not pressed = LOW
// Pressed     = HIGH

const int TRAIN_BUTTON_PIN = 41;
const int ACCESS_BUTTON_PIN = 42;

const int PASS_LED_PIN = 2;
const int FAIL_LED_PIN = 1;

bool trained = false;

// Used to stop one button press being detected many times
bool lastTrainButtonState = LOW;
bool lastAccessButtonState = LOW;

void setup() {
  Serial.begin(115200);

  pinMode(TRAIN_BUTTON_PIN, INPUT);
  pinMode(ACCESS_BUTTON_PIN, INPUT);

  pinMode(PASS_LED_PIN, OUTPUT);
  pinMode(FAIL_LED_PIN, OUTPUT);

  digitalWrite(PASS_LED_PIN, LOW);
  digitalWrite(FAIL_LED_PIN, LOW);

  Serial.println("ESP32 Lego Door Lock Test");
  Serial.println("-------------------------");
  Serial.println("Press TRAIN button to simulate training.");
  Serial.println("Press ACCESS button to test access.");
}

void loop() {
  bool trainButtonState = digitalRead(TRAIN_BUTTON_PIN);
  bool accessButtonState = digitalRead(ACCESS_BUTTON_PIN);

  // TRAIN button pressed
  if (trainButtonState == HIGH && lastTrainButtonState == LOW) {
    Serial.println();
    Serial.println("TRAIN button pressed.");
    Serial.println("Simulating training...");

    trained = true;

    blinkLed(PASS_LED_PIN, 3, 200);

    Serial.println("Training complete.");
    Serial.println("A figure is now trained.");
  }

  // ACCESS button pressed
  if (accessButtonState == HIGH && lastAccessButtonState == LOW) {
    Serial.println();
    Serial.println("ACCESS button pressed.");
    Serial.println("Checking access...");

    if (trained == true) {
      Serial.println("Access granted.");
      Serial.println("PASS LED on.");

      digitalWrite(PASS_LED_PIN, HIGH);
      digitalWrite(FAIL_LED_PIN, LOW);
      delay(1000);
      digitalWrite(PASS_LED_PIN, LOW);
    } else {
      Serial.println("Access denied.");
      Serial.println("No figure trained yet.");
      Serial.println("FAIL LED on.");

      digitalWrite(PASS_LED_PIN, LOW);
      digitalWrite(FAIL_LED_PIN, HIGH);
      delay(1000);
      digitalWrite(FAIL_LED_PIN, LOW);
    }
  }

  lastTrainButtonState = trainButtonState;
  lastAccessButtonState = accessButtonState;

  delay(50); // simple debounce delay
}

void blinkLed(int ledPin, int numberOfBlinks, int blinkDelay) {
  for (int i = 0; i < numberOfBlinks; i++) {
    digitalWrite(ledPin, HIGH);
    delay(blinkDelay);
    digitalWrite(ledPin, LOW);
    delay(blinkDelay);
  }
}