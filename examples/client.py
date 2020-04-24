import toid

print("please input ip (ex. 127.0.0.1:3012):")
connect_address = input()
if len(connect_address) == 0:
    connect_address = "127.0.0.1:3012"
print("please input user:")
user = input()
print("please input password:")
password = input()
connect_address = "ws://{}:{}@{}".format(user, password, connect_address)

player = toid.WebSocketPlayer(connect_address)
portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
portaudio_outputter.run()

player['main'] = '12345 643 2 1'
