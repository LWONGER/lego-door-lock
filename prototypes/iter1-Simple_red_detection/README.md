
## Function:

OpenMV AE3 camera detects whether red is visible and prints it to the server host through USB.

The server reads the latest camera message *RED/NO_RED* then over WIFI tells the ESP if the camera detected red or not.

If when the ACCESS button is pressed the Green or Red LED will light, depending on if the camera detected Red.


## Notes

change  the follow values to your own 
``` rust
const WIFI_SSID: &str = "CHANGE_ME";
const WIFI_PASSWORD: &str = "CHANGE_ME";
const LAPTOP_IP: &str = "CHANGE_ME";
```

