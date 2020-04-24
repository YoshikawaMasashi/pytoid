import time

import toid

if __name__ == '__main__':
    print("please input ip and port (ex. 127.0.0.1:3012):")
    connect_address = input()
    if len(connect_address) == 0:
        connect_address = "127.0.0.1:3012"
    connect_address = "{}".format(connect_address)
    print("connect_address: ws://{}".format(connect_address))

    server = toid.WebSocketPlayerServer(connect_address, "password", 3)

    while True:
        time.sleep(5)
