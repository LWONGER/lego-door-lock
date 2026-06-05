from socket import *

HOST = "0.0.0.0"
PORT = 12345

serverSocket = socket(AF_INET, SOCK_STREAM)
serverSocket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)

serverSocket.bind((HOST, PORT))
serverSocket.listen(1)

print("Laptop TCP server ready")
print("waiting for client...")

while True:
    connectionSocket, addr = serverSocket.accept()
    print("Client connected:", addr)

    while True:
        message = connectionSocket.recv(1024).decode()

        if message == "":
            print("Client disconnected")
            break

        print("Client message:", message)

        if message.lower() == "stop":
            print("Stopping connection")
            connectionSocket.send("Stopping connection".encode())
            print("Client sent stop")
            break

        connectionSocket.send("ACK".encode())

    connectionSocket.close()
    print("waiting for client...")