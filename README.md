# Lego Door Lock

*This project is centered in learning and applying new skill to a hands-on project. Using Git and Obsidian for documentation, Rust and interfacing with a new device (OPENMV AE3)*

This project is a Lego figure recognition door-lock prototype.

The system will use:
- OpenMV camera for Lego figure recognition
- Rust for the ESP32 control code
- Python/MicroPython for camera and AI control

## Modes

### Training mode
A train button is used to train an allowed Lego figure.

### Access mode
An access button is used to test the current Lego figure.
If the figure is recognised, the pass LED turns on.
If the figure is unknown, the fail LED turns on.

## Current prototype

The current prototype tests:
- TRAIN button
- ACCESS button
- PASS LED
- FAIL LED
- basic trained/not-trained logic
- simple debounce
