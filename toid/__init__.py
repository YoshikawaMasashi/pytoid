import os
import pathlib

from .toid import PortAudioOutputter, WebSocketPlayer, WebSocketPlayerServer  # NOQA

from . import toid

sample_sf2_path = str(
    pathlib.Path(os.path.dirname(__file__)) / 'sample-resource' / 'sf2' / 'sf2.toml'
)


class LocalPlayer(object):
    def __init__(self):
        self.player = toid.LocalPlayer()
        self.player.resource_register(sample_sf2_path)
        self.player.load_sf2("sf2.sample")
        self.player.set_sf2_name("sf2.sample")

    def set_sf2_name(self, name):
        self.player.set_sf2_name(name)

    def send_num_lang(self, melody_string, octave, name):
        self.player.send_num_lang(melody_string, octave, name)

    def resource_register(self, path):
        self.player.resource_register(path)
    
    def load_sf2(self, name):
        self.player.load_sf2(name)

    def get_toid_player(self):
        return self.player.get_toid_player()

    def __setitem__(self, key, value):
        if isinstance(key, str):
            if isinstance(value, tuple):
                if len(value) == 2:
                    self.send_num_lang(value[0], value[1], key)
                else:
                    raise Exception("invalid value")
            elif isinstance(value, str):
                self.send_num_lang(value, 0.0, key)
            else:
                raise Exception("invalid value")
        else:
            raise Exception("invalid value")
