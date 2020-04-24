import toid

print("please input ip (ex. 127.0.0.1):")
ip = input()
print(ip)
connect_address = "ws://{}:3012".format(ip)

player = toid.WebSocketPlayer(connect_address)
portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
portaudio_outputter.run()

player.send_num_lang("12345 643 2 1", 0.0, "main",)
