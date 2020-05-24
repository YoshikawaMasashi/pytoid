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
ph4 = hlt.change_pitch_in_key(ph3, toid.PitchInOctave(-4), 4)
player['h'] = ph4
ph5 = hlt.invert_pitch(ph3, toid.Pitch(60 - 4 + 2))
player['i'] = ph5

player.new_section(32)
player.next_section()

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
ph4 = hlt.change_pitch_in_key(ph3, toid.PitchInOctave(-4), 4)
player['h'] = ph4
ph5 = hlt.invert_pitch(ph3, toid.Pitch(60 - 4 + 2))
player['i'] = ph5

for key, pan in zip(['jr', 'jl'], [1.0, -1.0]):
    ph6 = hlt.parse_num_lang('12356' * 30, 3, -4)
    ph7 = hlt.shuffle_start(ph6)
    ph8 = hlt.delay(ph7, toid.Beat(0.75))
    ph9 = ph7 * ph8
    ph10 = hlt.delay(ph7, toid.Beat(1/2 + 1/8))
    ph11 = hlt.invert_pitch(ph10, toid.Pitch(60 - 4 + 2 + 12 * 3))
    ph12 = ph9 * ph11
    player[key] = player.make_track(ph12, pan=pan, vol=0.3)

sp = player.sample_player
sp['kick'] = 'x x x x '
sp['hat'] = '- - - - '
sp['snare'] = '  o   o '

time.sleep(33)
