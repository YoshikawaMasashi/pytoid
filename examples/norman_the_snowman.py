import toid

player = toid.LocalPlayer()
portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
portaudio_outputter.run()

ph1 = toid.parse_num_lang('53' * 32, 2, -4)
ph2 = toid.parse_num_lang('97' * 16 + '86' * 16, 1, -4)
player['a'] = ph1 * ph2
player['b'] = '3121' * 8, 1, -4
player['c'] = '1     5 3       ', 3, -4
player['d'] = '3' * 16 + '5' * 16 + '4' * 32, -2, -4
player['e'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', -1, -4
player['f'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 0, -4
player['g'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 1, -4

sp = player.sample_player
sp['kick'] = 'x x x x '
sp['hat'] = '- - - - '
sp['snare'] = '   ooo  '
