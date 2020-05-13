import os
import pathlib
import time

from . import toid
WebSocketPlayerServer = toid.players.WebSocketPlayerServer  # NOQA
PortAudioOutputter = toid.outputters.PortAudioOutputter  # NOQA
Phrase = toid.data.Phrase  # NOQA
Track = toid.data.Track  # NOQA
parse_num_lang = toid.high_layer_trial.parse_num_lang  # NOQA

example_sf2_path = str(
    pathlib.Path(os.path.dirname(__file__)) / 'sample-resource' / 'sf2' / 'sf2.toml'
)
example_drums_path = str(
    pathlib.Path(os.path.dirname(__file__)) / 'sample-resource' / 'drums' / 'drums.toml'
)


class SamplePlayer(object):
    def __init__(self, player):
        self.player = player

    def __setitem__(self, key, value):
        if isinstance(key, str):
            if isinstance(value, str):
                self.player.send_sample_lang(value, key)
            else:
                raise Exception("invalid value")
        else:
            raise Exception("invalid value")


class LocalPlayer(object):
    def __init__(self):
        self.player = toid.players.LocalPlayer()
        self.player.resource_register(example_sf2_path)
        self.player.resource_register(example_drums_path)
        self.default_sf2 = "example_sf2"
        self.default_sample = "example_drums"
        self.sample_player = SamplePlayer(self)

    def set_sf2_name(self, name):
        self.player.set_sf2_name(name)

    def send_num_lang(self, melody_string, octave, key, name):
        self.player.send_num_lang(
            melody_string, float(octave), float(key), name, self.default_sf2)

    def send_sample_lang(self, sample_string, name):
        self.player.send_sample_lang(
            sample_string, name, self.default_sample)

    def resource_register(self, path):
        self.player.resource_register(path)
    
    def load_sf2(self, name):
        self.player.load_sf2(name)

    def get_toid_player(self):
        return self.player.get_toid_player()

    def __setitem__(self, key, value):
        if isinstance(key, str):
            if isinstance(value, Phrase):
                self.player.send_phrase(value, key, self.default_sf2)
            elif isinstance(value, Track):
                self.player.send_track(value, key)
            elif isinstance(value, tuple):
                if len(value) == 3:
                    self.send_num_lang(value[0], value[1], value[2], key)
                elif len(value) == 2:
                    self.send_num_lang(value[0], value[1], 0.0, key)
                else:
                    raise Exception("invalid value")
            elif isinstance(value, str):
                self.send_num_lang(value, 0.0, 0.0, key)
            else:
                raise Exception("invalid value")
        else:
            raise Exception("invalid value")


class WebSocketPlayer(object):
    def __init__(self, connect_address):
        self.player = toid.players.WebSocketPlayer(connect_address)
        time.sleep(0.5)
        self.player.resource_register(example_sf2_path)
        self.player.resource_register(example_drums_path)
        self.default_sf2 = "example_sf2"
        self.default_sample = "example_drums"
        self.sample_player = SamplePlayer(self)

    def set_sf2_name(self, name):
        self.player.set_sf2_name(name)

    def send_num_lang(self, melody_string, octave, key, name):
        self.player.send_num_lang(
            melody_string, float(octave), float(key), name, self.default_sf2)

    def send_sample_lang(self, sample_string, name):
        self.player.send_sample_lang(
            sample_string, name, self.default_sample)

    def resource_register(self, path):
        self.player.resource_register(path)

    def load_sf2(self, name):
        self.player.load_sf2(name)

    def get_toid_player(self):
        return self.player.get_toid_player()

    def sync_start(self):
        self.player.sync_start()

    def __setitem__(self, key, value):
        if isinstance(key, str):
            if isinstance(value, Phrase):
                self.player.send_phrase(value, key, self.default_sf2)
            elif isinstance(value, Track):
                self.player.send_track(value, key)
            elif isinstance(value, tuple):
                if len(value) == 3:
                    self.send_num_lang(value[0], value[1], value[2], key)
                elif len(value) == 2:
                    self.send_num_lang(value[0], value[1], 0.0, key)
                else:
                    raise Exception("invalid value")
            elif isinstance(value, str):
                self.send_num_lang(value, 0.0, 0.0, key)
            else:
                raise Exception("invalid value")
        else:
            raise Exception("invalid value")
