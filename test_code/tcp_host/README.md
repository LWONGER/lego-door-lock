# TCP Host Prototype

This folder contains a simple Python TCP socket server for testing network communication in the lego-door-lock project.

The goal of this prototype is to test basic communication before adding harder logic once the AI camera is set up


## Files

### `server_template.py`

### `server_template.py`

Original TCP socket server from a lab submission for Dr Jennifer McManis.

I kept this file as a reference because it was the starting point for the adapted `server.py` version used in this project.

This version sends a date/time message to the client when it connects.

### `server.py`

Current laptop TCP host/server.

This server waits for a client connection and replies with ACK until the client sends "stop"

This is the file that the esp32-rust/rust_client_test will interact with.