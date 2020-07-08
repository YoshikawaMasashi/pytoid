import time

import toid

if __name__ == "__main__":
    player = toid.LocalPlayer()
    portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
    portaudio_outputter.run()

    player['main'] = '12345 643 2 1   ', 1, -3
    player['sub'] = '1   4   5   1   ', 0, -3
    player['main'] = player['main'].add_effect(toid.Effect.reverb(1.0, 0.5))
    player['sub'] = player['sub'].add_effect(toid.Effect.reverb(1.0, 0.5))

    time.sleep(12)
    portaudio_outputter.stop()
