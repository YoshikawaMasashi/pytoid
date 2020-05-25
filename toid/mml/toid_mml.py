import toid

try:
    from core import parse_mml
except:
    from .core import parse_mml


def mml_to_phrase(mml_string):
    """
    ----
    Args:
        mml_strings (str): MML string
    Returns:
        (Phrase)
    """
    res = parse_mml(mml_string)
    pitch_list, dur_list = res["pitch"], res["duration"]
    ph = toid.Phrase()
    now = 0
    for pitch, dur in zip(pitch_list, dur_list):
        if dur >= 0:
            if not hasattr(pitch, "__iter__"):
                pitch = [pitch]
            for p in pitch:
                ph = ph.add_note(toid.Pitch(p), toid.Beat(now), toid.Beat(dur))
        now += abs(dur)
    ph = ph.set_length(toid.Beat(now))
    return ph


if __name__ == '__main__':
    ph = mml_to_phrase("o4l4 ccggaagr ffeeddcr")
    player = toid.local_play()
    player['a'] = ph

    import time
    time.sleep(10)


