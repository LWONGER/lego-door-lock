from socket import *
from datetime import datetime

HOST = '0.0.0.0'
PORT = 12345

serverSocket = socket(AF_INET, SOCK_STREAM) #configures socket to use IPV4 and TCP
serverSocket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)

serverSocket.bind((HOST, PORT))
serverSocket.listen(1)

print('The server is ready to receive')

while True:
    connectionSocket, addr = serverSocket.accept()

    currentTime = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    connectionSocket.send(currentTime.encode())

    connectionSocket.close()
