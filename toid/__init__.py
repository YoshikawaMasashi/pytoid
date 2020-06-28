import os
import pathlib
import time

import numpy

from toid import high_layer_trial  # NOQA
from toid._rhythm_maker import rhythm_maker # NOQA

from . import toid
from . import mml as mml_mod

WebSocketPlayerServer = toid.players.WebSocketPlayerServer  # NOQA
PortAudioOutputter = toid.outputters.PortAudioOutputter  # NOQA
WaveFileOutputter = toid.outputters.WaveFileOutputter  # NOQA
Phrase = toid.data.Phrase  # NOQA
Track = toid.data.Track  # NOQA
Pitch = toid.data.Pitch  # NOQA
Beat = toid.data.Beat  # NOQA
PitchInterval = toid.data.PitchInterval  # NOQA
PitchInOctave = toid.data.PitchInOctave  # NOQA
Instrument = toid.data.Instrument  # NOQA
Chord = toid.data.Chord  # NOQA
ChordProgression = toid.data.ChordProgression  # NOQA
Scale = toid.data.Scale  # NOQA
Wave = toid.data.Wave  # NOQA

example_sf2_path = str(
    pathlib.Path(os.path.dirname(__file__)) / 'sample-resource' / 'sf2' / 'sf2.toml'
)
example_samples_path = str(
    pathlib.Path(os.path.dirname(__file__)) / 'sample-resource' / 'samples' / 'samples.toml'
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
            if isinstance(value, Phrase):
                self.player.send_sample_phrase(value, key)
            elif isinstance(value, Track):
                self.player.player.send_track(value, self.player.current_beat, key)
            elif isinstance(value, str):
                self.player.send_sample_lang(value, key)
            elif isinstance(value, tuple):
                if len(value) == 2:
                    ph = high_layer_trial.encode_rhythm_array(key, value[0], value[1])
                    self.player.send_sample_phrase(ph, key)
                elif len(value) == 3:
                    ph = high_layer_trial.encode_rhythm_array(value[0], value[1], value[2])
                    self.player.send_sample_phrase(ph, key)
                else:
                    raise Exception("invalid value")
            elif isinstance(value, numpy.ndarray):
                ph = high_layer_trial.encode_rhythm_array(key, value, 0.5)
                self.player.send_sample_phrase(ph, key)
            else:
                raise Exception("invalid value")
        else:
            raise Exception("invalid value")

    def __getitem__(self, key):
        if isinstance(key, str):
            return self.player.player.get_sample_track(key, self.player.current_beat)
        else:
            raise Exception("invalid key")


class Player(object):
    def __init__(self):
        self.default_sf2 = "example_sf2"
        self.default_sample = "example_samples"
        self.preset_idx = 0
        self.sample_player = SamplePlayer(self)
        self.current_beat = toid.data.Beat(0)
        self.parse_mode = "num"

    def change_parse_mode(self, mode):
        self.parse_mode = mode

    def send_num_lang(self, melody_string, octave, key, name):
        inst = Instrument.sf2(self.default_sf2, self.preset_idx)
        self.player.send_num_lang(
            melody_string, float(octave), float(key), self.current_beat, name,
            inst)

    def send_mml(self, mml_string, name):
        phrase = mml_mod.mml_to_phrase(mml_string)
        inst = Instrument.sf2(self.default_sf2, self.preset_idx)
        self.player.send_phrase(
            phrase, self.current_beat, name, inst)

    def send_sample_lang(self, sample_string, name):
        self.player.send_sample_lang(
            sample_string, self.current_beat, name, self.default_sample)

    def resource_register(self, path):
        self.player.resource_register(path)

    def get_toid_player(self):
        return self.player.get_toid_player()

    def make_track(
        self, phrase=toid.data.Phrase(), sf2_name=None, vol=1.0, pan=0.0
    ):
        if sf2_name is None:
            sf2_name = self.default_sf2
        inst = Instrument.sf2(sf2_name, self.preset_idx)
        return toid.data.Track(phrase, inst, vol, pan)

    def new_section(self, beat):
        self.player.new_section(beat)

    def get_section_beats(self):
        return self.player.get_section_beats()

    def next_section(self):
        self.current_beat = self.player.get_next_beat(self.current_beat)

    def prev_section(self):
        self.current_beat = self.player.get_prev_beat(self.current_beat)

    def change_bpm(self, bpm):
        self.player.change_bpm(bpm)

    def print_preset_names(self):
        self.player.print_preset_names()

    def clear_states(self):
        self.player.clear_states()

    def clear_sections(self):
        self.player.clear_sections()

    def get_pitch_track_names(self):
        return self.player.get_pitch_track_names(self.current_beat)

    def get_sample_track_names(self):
        return self.player.get_sample_track_names(self.current_beat)

    def save_state(self, path):
        self.player.save_state(path)

    def load_state(self, path):
        self.player.load_state(path)

    def send_pitch_phrase(self, ph, name):
        inst = Instrument.sf2(self.default_sf2, self.preset_idx)
        self.player.send_phrase(
            ph, self.current_beat, name, inst
        )

    def send_sample_phrase(self, ph, name):
        inst = Instrument.sample(self.default_sample)
        self.player.send_phrase(
            ph, self.current_beat, name, inst
        )

    def __setitem__(self, key, value):
        if isinstance(key, str):
            if isinstance(value, Phrase):
                self.send_pitch_phrase(value, key)
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
                if self.parse_mode == "num":
                    self.send_num_lang(value, 0.0, 0.0, key)
                elif self.parse_mode == "mml":
                    self.send_mml(value, key)
                else:
                    raise Exception("invalid parse mode")
            else:
                raise Exception("invalid value")
        else:
            raise Exception("invalid key")

    def __getitem__(self, key):
        if isinstance(key, str):
            return self.player.get_pitch_track(key, self.current_beat)
        else:
            raise Exception("invalid key")


class LocalPlayer(Player):
    def __init__(self):
        self.player = toid.players.LocalPlayer()
        self.player.resource_register(example_sf2_path)
        self.player.resource_register(example_samples_path)
        super().__init__()


class WebSocketPlayer(Player):
    def __init__(self, connect_address):
        self.player = toid.players.WebSocketPlayer(connect_address)
        time.sleep(0.5)
        self.player.resource_register(example_sf2_path)
        self.player.resource_register(example_samples_path)
        super().__init__()

    def sync_start(self):
        self.player.sync_start()

    def sync_state(self):
        self.player.sync_state()
