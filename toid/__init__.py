import os
import pathlib
import time

from toid import high_layer_trial  # NOQA

from . import toid

WebSocketPlayerServer = toid.players.WebSocketPlayerServer  # NOQA
PortAudioOutputter = toid.outputters.PortAudioOutputter  # NOQA
Phrase = toid.data.Phrase  # NOQA
Track = toid.data.Track  # NOQA

example_sf2_path = str(
    pathlib.Path(os.path.dirname(__file__)) / 'sample-resource' / 'sf2' / 'sf2.toml'
)
example_drums_path = str(
    pathlib.Path(os.path.dirname(__file__)) / 'sample-resource' / 'drums' / 'drums.toml'
)

portaudio_outputter_for_quick = None


def local_play():
    global portaudio_outputter_for_quick
    player = LocalPlayer()
    portaudio_outputter_for_quick = PortAudioOutputter(player.get_toid_player())
    portaudio_outputter_for_quick.run()
    return player


def websocket_play(connect_address):
    global portaudio_outputter_for_quick
    player = WebSocketPlayer(connect_address)
    portaudio_outputter_for_quick = PortAudioOutputter(player.get_toid_player())
    portaudio_outputter_for_quick.run()
    return player


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
        self.current_beat = toid.data.Beat(0)

    def set_sf2_name(self, name):
        self.player.set_sf2_name(name)

    def send_num_lang(self, melody_string, octave, key, name):
        self.player.send_num_lang(
            melody_string, float(octave), float(key), self.current_beat, name,
            self.default_sf2)

    def send_sample_lang(self, sample_string, name):
        self.player.send_sample_lang(
            sample_string, self.current_beat, name, self.default_sample)

    def resource_register(self, path):
        self.player.resource_register(path)

    def load_sf2(self, name):
        self.player.load_sf2(name)

    def get_toid_player(self):
        return self.player.get_toid_player()

    def make_track(
        self, phrase=toid.data.Phrase(), sf2_name=None, vol=1.0, pan=0.0
    ):
        if sf2_name is None:
            sf2_name = self.default_sf2
        return toid.data.Track(phrase, sf2_name, vol, pan)

    def new_section(self, beat):
        self.player.new_section(beat)

    def get_section_beats(self):
        return self.player.get_section_beats()

    def next_section(self):
        self.current_beat = self.player.get_next_beat(self.current_beat)

    def prev_section(self):
        self.current_beat = self.player.get_prev_beat(self.current_beat)

    def __setitem__(self, key, value):
        if isinstance(key, str):
            if isinstance(value, Phrase):
                self.player.send_phrase(
                    value, self.current_beat, key, self.default_sf2)
            elif isinstance(value, Track):
                self.player.send_track(value, self.current_beat, key)
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
            raise Exception("invalid key")

    def __getitem__(self, key):
        if isinstance(key, str):
            return self.player.get_track(key, self.current_beat)
        else:
            raise Exception("invalid key")


class WebSocketPlayer(object):
    def __init__(self, connect_address):
        self.player = toid.players.WebSocketPlayer(connect_address)
        time.sleep(0.5)
        self.player.resource_register(example_sf2_path)
        self.player.resource_register(example_drums_path)
        self.default_sf2 = "example_sf2"
        self.default_sample = "example_drums"
        self.sample_player = SamplePlayer(self)
        self.current_beat = toid.data.Beat(0)

    def set_sf2_name(self, name):
        self.player.set_sf2_name(name)

    def send_num_lang(self, melody_string, octave, key, name):
        self.player.send_num_lang(
            melody_string, float(octave), float(key), self.current_beat, name,
            self.default_sf2)

    def send_sample_lang(self, sample_string, name):
        self.player.send_sample_lang(
            sample_string, self.current_beat, name, self.default_sample)

    def resource_register(self, path):
        self.player.resource_register(path)

    def load_sf2(self, name):
        self.player.load_sf2(name)

    def get_toid_player(self):
        return self.player.get_toid_player()

    def make_track(
        self, phrase=toid.data.Phrase(), sf2_name=None, vol=1.0, pan=0.0
    ):
        if sf2_name is None:
            sf2_name = self.default_sf2
        return toid.data.Track(phrase, sf2_name, vol, pan)

    def new_section(self, beat):
        self.player.new_section(beat)

    def get_section_beats(self):
        return self.player.get_section_beats()

    def next_section(self):
        self.current_beat = self.player.get_next_beat(self.current_beat)

    def prev_section(self):
        self.current_beat = self.player.get_prev_beat(self.current_beat)

    def sync_start(self):
        self.player.sync_start()

    def __setitem__(self, key, value):
        if isinstance(key, str):
            if isinstance(value, Phrase):
                self.player.send_phrase(
                    value, self.current_beat, key, self.default_sf2)
            elif isinstance(value, Track):
                self.player.send_track(value, self.current_beat, key)
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
            raise Exception("invalid key")

    def __getitem__(self, key):
        if isinstance(key, str):
            return self.player.get_track(key, self.current_beat)
        else:
            raise Exception("invalid key")
