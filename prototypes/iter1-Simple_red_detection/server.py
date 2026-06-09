# laptop bridge server for lego door project
# reads camera flag over usb serial
# replies to esp32 over tcp

import socket
import serial
import time

# openmv camera serial port check on device manager for example COM3
CAMERA_PORT = "CHANGEME"
CAMERA_BAUD = 115200

# tcp server settings
HOST = "0.0.0.0"
PORT = 12345

latest_camera_flag = "NO_RED"


def read_camera_flag(camera):
    global latest_camera_flag

    # read all waiting camera lines
    while camera.in_waiting > 0:
        line = camera.readline().decode("utf-8", errors="ignore").strip()

        if line != "":
            print("raw camera:", line)

        if line == "RED" or line == "NO_RED":
            latest_camera_flag = line
            print("camera:", latest_camera_flag)


def handle_esp_message(message):
    print("esp32:", message)

    if message == "Ping":
        return "ACK"

    if message == "ACCESS":
        if latest_camera_flag == "RED":
            return "OPEN"
        else:
            return "REJECT"

    if message == "stop":
        return "stopping"

    return "UNKNOWN"


print("starting camera serial...")

camera = serial.Serial(CAMERA_PORT, CAMERA_BAUD, timeout=0.1)
time.sleep(2)
camera.reset_input_buffer()

print("starting tcp server...")

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# allows the server to restart more easily
server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

server_socket.bind((HOST, PORT))
server_socket.listen(1)

print("server waiting for esp32...")

conn = None

try:
    conn, addr = server_socket.accept()
    print("esp32 connected:", addr)

    while True:
        read_camera_flag(camera)

        conn.settimeout(0.1)

        try:
            data = conn.recv(1024)

            if data:
                message = data.decode("utf-8", errors="ignore").strip()
                reply = handle_esp_message(message)

                conn.sendall((reply + "\n").encode("utf-8"))
                print("sent:", reply)

                if message == "stop":
                    break

        except socket.timeout:
            pass

except KeyboardInterrupt:
    print("server stopped by user")

finally:
    if conn is not None:
        conn.close()

    server_socket.close()
    camera.close()

    print("server closed")