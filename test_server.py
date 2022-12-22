import socket

def test():
    s = socket.socket()
    server_address = ("127.0.0.1", 8081)
    s.connect(server_address)
    print("Connected to the server")
    request = "GET / HTTP/1.1\r\nHost:google.com\r\n\r\n"
    print("Sending request to server:\n", request)
    s.sendall(request.encode())
    print("Request sent successfullly")
    print("Now read response from server")
    response = b"";
    while True:
        data = s.recv(1024)
        if data:
            response += data
            # if data==b"": break
        else:
            break
    print("Response received from server completely:\n", response.decode())


if __name__ == "__main__":
    test()