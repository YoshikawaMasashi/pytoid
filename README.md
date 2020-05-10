# pytoid
[![PyPI version](https://badge.fury.io/py/toid.svg)](https://badge.fury.io/py/toid)

## install
```
$ pip install toid
```

## example
```python
import toid

player = toid.LocalPlayer()
portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
portaudio_outputter.run()

player['a'] = '53' * 16, 2, -4
player['b'] = '97' * 16 + '86' * 16, 1, -4
player['c'] = '3121' * 8, 1, -4
player['d'] = '1     5 3       ', 3, -4
player['e'] = '3' * 16 + '5' * 16 + '4' * 32, -2, -4
player['f'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', -1, -4
player['g'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 0, -4
player['h'] = '2  1          1 5           5 432  1          1 3       4 3 2 1 ', 1, -4
```
