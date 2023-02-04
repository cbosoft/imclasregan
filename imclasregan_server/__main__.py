from http.server import HTTPServer
import time

from .server import ImClasRegAnServer

hostname = "localhost"
serverport = 8080

if __name__ == "__main__":        
    webServer = HTTPServer((hostname, serverport), ImClasRegAnServer)
    print(f"Server started http://{hostname}:{serverport}")

    try:
        webServer.serve_forever()
    except KeyboardInterrupt:
        pass

    webServer.server_close()
    print("Server stopped.")