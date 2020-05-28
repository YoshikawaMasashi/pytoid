import time

import toid

if __name__ == "__main__":
    player = toid.LocalPlayer()
    portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
    portaudio_outputter.run()

    player['main'] = '12345 643 2 1   ', 1, -3
    player['sub'] = '1   4   5   1   ', 0, -3

    time.sleep(12)
    portaudio_outputter.stop()
