import time

import toid

if __name__ == "__main__":
    player = toid.LocalPlayer()
    portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
    portaudio_outputter.run()

    player.load_state("state.json")

    time.sleep(4)
    portaudio_outputter.stop()
