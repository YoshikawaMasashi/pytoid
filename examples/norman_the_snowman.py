import toid
import toid.high_layer_trial as hlt

player = toid.LocalPlayer()
portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
portaudio_outputter.run()

ph1 = hlt.parse_num_lang('53' * 32, 2, -4)
ph2 = hlt.parse_num_lang('97' * 16 + '86' * 16, 1, -4)
player['a'] = ph1 * ph2
player['b'] = '3121' * 8, 1, -4
player['c'] = '1     5 3       ', 3, -4
player['d'] = '3' * 16 + '5' * 16 + '4' * 32, -2, -4
player['e'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', -1, -4
player['f'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 0, -4
player['g'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 1, -4
ph3 = hlt.parse_num_lang('2  1          1 5           5 432  1          1 3       4 3 2 1 ', 0, -4)
ph4 = hlt.change_pitch_in_key(ph3, -4, 4)
player['h'] = ph4

sp = player.sample_player
sp['kick'] = 'x x x x '
sp['hat'] = '- - - - '
sp['snare'] = '   ooo  '
