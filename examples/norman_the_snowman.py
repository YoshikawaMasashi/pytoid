import time

import toid
import toid.high_layer_trial as hlt

player = toid.local_play()

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

ph5 = hlt.parse_num_lang('12356' * 30, 3, -4)
ph6 = hlt.shuffle_start(ph5)
ph7 = hlt.delay(ph6, toid.toid.data.Beat(0.75))
ph8 = ph6 * ph7
ph9 = hlt.delay(ph6, toid.toid.data.Beat(1/2 + 1/8))
ph10 = hlt.invert_pitch(ph9, toid.toid.data.Pitch(60 - 4 + 2 + 12 * 3))
ph11 = ph8 * ph10
player['i'] = ph11

ph12 = hlt.invert_pitch(ph3, toid.toid.data.Pitch(60 - 4 + 2))
player['j'] = ph12

sp = player.sample_player
sp['kick'] = 'x x x x '
sp['hat'] = '- - - - '
sp['snare'] = '  o   o '

time.sleep(30)
