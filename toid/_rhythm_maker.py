from toid import high_layer_trial

class RhythmMaker(object):
    def __init__(self, sound):
        self.sound = sound

    def __call__(self, *args):
        if len(args) == 1:
            return high_layer_trial.encode_rhythm_array(self.sound, args[0], 0.5)
        elif len(args) == 2:
            return high_layer_trial.encode_rhythm_array(self.sound, args[0], args[1])
        else:
            raise Exception("invalid value")

class RhythmMakerProcy(object):
    def __init__(self):
        for i in range(ord("A"), ord("z") + 1):
            self.__setattr__(chr(i), RhythmMaker(chr(i)))

rhythm_maker = RhythmMakerProcy()
