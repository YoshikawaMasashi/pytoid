import toid

player = toid.LocalPlayer()
portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
portaudio_outputter.run()

player['a'] = '53' * 16, 2.0, -4.0
player['b'] = '97' * 16 + '86' * 16, 1.0, -4.0
player['c'] = '3121' * 8, 1.0, -4.0
player['d'] = '1     5 3       ', 3.0, -4.0
player['e'] = '3' * 16 + '5' * 16 + '4' * 32, -2.0, -4.0
player['f'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', -1.0, -4.0
player['g'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 0.0, -4.0
player['h'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 1.0, -4.0
