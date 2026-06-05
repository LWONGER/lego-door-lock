from socket import *

HOST = "127.0.0.1"
PORT = 12345

clientSocket = socket(AF_INET, SOCK_STREAM)
clientSocket.connect((HOST, PORT))

print("Connected to server")
print("Type messages. Type stop to close the connection.")

while True:
    message = input("Send -> ")

    clientSocket.send(message.encode())

    reply = clientSocket.recv(1024).decode()
    print("From Server:", reply)

    if message.lower() == "stop":
        break

clientSocket.close()
print("Client stopped")