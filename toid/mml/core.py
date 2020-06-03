'''
MMLパーサーのコア部分
（このファイルの中身は外部ライブラリに依存させたくない）

TODO: MML は規格が乱立気味なので，どこのものに対応していく予定かをある程度決める．
    https://docs.google.com/spreadsheets/d/1OVSLeYhjTLqdEHegvdAA3aFgFRQWiPIJ66e0hu8u6m8/edit#gid=0
    にまとめてる
'''

def find_corresponding_parenthesis(s, offs):
    '''
    Args
        s (str):
        offs (int): s[offs] must be a parenthesis
    Returns
        (int): offset of corresponding paernthesis
    '''
    parens = '()[]{}<>'
    c = s[offs]
    idx = parens.find(c)
    assert idx > 0
    direction = -1 if idx % 2 else +1
    c2 = parens[idx ^ 1]
    cnt = 0
    while 0 <= offs < len(s):
        if s[offs] == c:
            cnt += 1
        elif s[offs] == c2:
            cnt -= 1
        if cnt == 0:
            return offs
        offs += direction
    return offs

def parse_symbol(s, c):
    '''
    Args
        s (str):
        c (str): should be a character
    Returns
        offs (int): number of c read from s
    '''
    offs = 0
    for offs in range(len(s)):
        if s[offs] != c:
            return offs
    offs = len(s)
    return offs


def parse_int(s):
    '''
    Args
        s (str)
    Returns
        val (int): the number read from s
        offs (int): number of letters read from s
    '''
    val, offs = 0, 0
    for offs in range(len(s)):
        try:
            val = int(s[:1+offs])
        except:
            if offs > 0 or s[0] not in '+-':
                return val, offs
    offs = len(s)
    return val, offs


macro_dict = {}


def generate_chord_macro():
    chromatic_scale = ['c', 'c+', 'd', 'd+', 'e',
                       'f', 'f+', 'g', 'g+', 'a', 'a+', 'b']
    major_pitch = [0, 2, 4, 5, 7, 9, 11]
    for i, base in enumerate('CDEFGAB'):
        for j, sign in enumerate(['-', '', '+']):
            base_pitch = major_pitch[i] + j + 11

            chord_info = [
                ('', [0, 4, 7]),  # Major
                ('O', [0, 7, 16]),  # Major (open-voicing)
                ('8O', [0, 7, 12, 16]),  # Major (open-voicing)
                ('m', [0, 3, 7]),  # minor
                ('mO', [0, 7, 15]),  # minor (open-voicing)
                ('m8O', [0, 7, 12, 15]),  # minor (open-voicing)
                ('3', [0, 4]),
                ('m3', [0, 3]),
                ('5', [0, 7]),
                ('6', [0, 4, 7, 9]),
                ('m6', [0, 3, 7, 9]),
                ('7', [0, 4, 7, 10]),
                ('37', [0, 4, 10]),
                ('57', [0, 7, 10]),
                ('7O', [0, 7, 10, 16]),
                ('M7', [0, 4, 7, 11]),
                ('M37', [0, 4, 11]),
                ('M57', [0, 7, 11]),
                ('M7O', [0, 7, 11, 16]),
                ('m7', [0, 3, 7, 10]),
                ('m37', [0, 3, 10]),
                ('m57', [0, 7, 10]),
                ('m7O', [0, 7, 10, 15]),
                ('dim', [0, 3, 6]),
                ('dim7', [0, 3, 6, 9]),
                ('aug', [0, 4, 8]),
                ('augO', [0, 8, 16]),
                ('sus4', [0, 5, 7]),
                ('sus4O', [0, 7, 17]),
            ]
            for chord_symbol, chord_pitch in chord_info:
                for l in range(len(chord_pitch)):  # number of rotation
                    name = base + sign + chord_symbol + '^' * l
                    chord_string = '"' + ''.join([
                        chromatic_scale[(base_pitch + p) % 12] for p in chord_pitch[l:]+chord_pitch[:l]
                    ]) + '"'
                    macro_dict[name] = chord_string


generate_chord_macro()


def quote_chord(mml):
    return mml
    """
    ret = ''
    offs = 0
    while offs < len(mml):
        c = mml[offs]
        if 'A' <= c <= 'G':
            # pass
            pass
        offs += 1
    return mml
    """


def extend_macro(mml):
    ret = ''
    offs = 0
    while offs < len(mml):
        c = mml[offs]
        if 'A' <= c <= 'Z':
            accepted_offs = offs+1
            for offs2 in range(offs+1, len(mml)+1):
                if mml[offs2-1] == ' ':
                    break  # name of macro must not contain space.
                if mml[offs:offs2] in macro_dict:
                    c = macro_dict[mml[offs:offs2]]
                    accepted_offs = offs2
            offs = accepted_offs
            ret += c
            continue
        ret += c
        offs += 1
    return ret


def parse_mml(mml, parser_state=None, verbose=False):
    """
    ----
    Args:
        mml (str):
            MML string. 対応コマンドは下記参照
        parser_state (dict or None):
            パーサーの変数の状態．None の場合，以下が設定されます．
            {
                'l': 8,  # length (l8)
                'o': 4,  # octave (o4)
                'k': 0,  # key-transpose (k0)
            }
    Returns:
        {
            "pitch" (list of (int or tuple-of-int)):
                各時刻に発音するノートナンバーのリスト．
                単音ならノートナンバーは単一の int として表現され，
                和音ならノートナンバーは int の tuple として表現される．
                ノートナンバーは C4 が 60.
                休符については不定
            "duration" (list of float):
                音符長または休符長のリスト．長さの単位は拍数．
                正の値は音符，負の値は休符．
            "parser_state" (dict):
                パーサーの変数の状態．
        }
    Examples:
        parse_mml("l4cderefg")
            => {
                "pitch": [60, 62, 64, 0, 64, 65, 67],
                "duration": [1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0],
                "parse_state": {...}
            }
        parse_mml("o5l1 'egb'")
            => {
                "pitch": [(76, 79, 83)],
                "duration": [4.0],
                "parse_state": {...}
            }
    MML の例:
        - (かえるのうた) "l4 cdefedcr efgagfer crcrcrcr l8 ccddeefferdrcrrr"
        - (ふるさと) "l4 ccc d.e8d eefg2. fga e.f8e dd<b>c2."
    MML 対応コマンド:
        cdefgabr (音符・休符)
        -+ (フラット・シャープ: 後置)
        . (付点: 後置)
        _^ (一時的なオクターブ指定: 前置)
        ~ (音価の延長: 後置)
        >< (オクターブ上下)
        l (音価指定)
        o (オクターブ指定)
        k (キートランスポーズ)
        "' (和音)
        [] (連符)
        上記に加えてコードマクロ（独自定義）が利用可能です．
           C => "ceg"
           C^ => "egc" (第一転回)
           C^^ => "gce" (第二転回)
           CO => "cge" (Open-voicing)
           Csus4 => "cfg"
           他（全ての定義済みマクロは generate_chord_macro の定義を参照ください）
        最初にマクロが展開された後，MML 文字列として解釈されます．

    実装について
        入れ子になりうるコマンド（括弧）のみ再帰的に処理して
        それ以外は非再帰的に処理している
        （が，全て再帰的にした方が簡潔に記述できるかもしれない．）
    """
    mml = extend_macro(mml)
    if verbose:
        print('macro extended: {}'.format(mml))

    if parser_state is None:
        state = {
            'l': 8,  # length (l8)
            'o': 4,  # octave (o4)
            'k': 0,  # key-transpose (k0)
        }
    else:
        state = parser_state

    val_temp_o = 0  # _, ^（ノート・和音を抜けると解除される）
    chord_val_o = None  # 和音内での o（和音を抜けると解除される）

    pitch_list, dur_list = [], []
    offs = 0
    val_futen = 0  # .
    val_hold = 0  # ~
    chord = []
    in_chord = False
    while offs < len(mml):
        c = mml[offs]
        n = 'cdefgabr'.find(c)
        if n >= 0:  # new note!
            # process postfix
            if val_futen > 0:
                dur_list[-1] *= 2 - 0.5**val_futen
                val_futen = 0  # clear futen
            if val_hold > 0:
                dur_list[-1] *= 1 + val_hold
                val_hold = 0

            # compute note number
            chroma = [0, 2, 4, 5, 7, 9, 11, 0][n]
            if in_chord:
                chord.append(chroma + (chord_val_o + 1) * 12 + state['k'])
                if len(chord) > 1 and chord[-2] >= chord[-1]:
                    chord[-1] = 12 - (chord[-2] - chord[-1]) % 12 + chord[-2]
            else:  # single note
                pitch_list.append(chroma + (state['o'] + val_temp_o + 1) * 12 + state['k'])
                dur_list.append(4 / state['l'] if n < 7 else -4 / state['l'])
                val_temp_o = 0
            offs += 1
            continue
        if c in ['"', "'"]:
            if in_chord:  # end of chord
                pitch_list.append(tuple(chord))
                in_chord = False
                chord = []
                dur_list.append(4 / state['l'])
                val_temp_o = 0
                offs += 1
                continue
            else:  # start of chord
                in_chord = True
                chord_val_o = state['o'] + val_temp_o
                offs += 1
                continue
        if '0' <= c <= '9' and not in_chord:  # temporary l-setting
            l, delta_offs = parse_int(mml[offs:])
            dur_list[-1] = -4 / l if dur_list[-1] <= 0 else 4 / l
            offs += delta_offs
            continue
        # other commands
        if c == 'l' and not in_chord:  # length
            offs += 1
            state['l'], delta_offs = parse_int(mml[offs:])
            offs += delta_offs
            continue
        elif c == 'o':  # octave
            offs += 1
            val, delta_offs = parse_int(mml[offs:])
            if in_chord:
                chord_val_o = val
            else:
                state['o'] = val
            offs += delta_offs
            continue
        elif c == 'k' and not in_chord:  # key-transpose
            offs += 1
            state['k'], delta_offs = parse_int(mml[offs:])
            offs += delta_offs
            continue
        elif c == '+':
            if in_chord:
                chord[-1] += 1
            else:
                pitch_list[-1] += 1
        elif c == '-':
            if in_chord:
                chord[-1] -= 1
            else:
                pitch_list[-1] -= 1
        elif c == '>':
            if in_chord:
                chord_val_o += 1
            else:
                state['o'] += 1
        elif c == '<':
            if in_chord:
                chord_val_o -= 1
            else:
                state['o'] -= 1
        elif c == '.':
            val_futen += 1
        elif c == '^':
            val_temp_o += 1
        elif c == '_':
            val_temp_o -= 1
        elif c == '~':  # 直前のノートの長さを (~ 記号の数) 倍だけ伸ばす
            val_hold += 1
        elif c == '[':  # 連符 再帰的に処理する
            idx = find_corresponding_parenthesis(mml, offs)
            # 中身を再帰的に処理
            res = parse_mml(mml[offs + 1:idx], parser_state=state)
            offs = idx + 1
            # 長さに関する postfix を処理（NOTE: 通常のノートの処理とまとめられないか？）
            l, delta_offs = parse_int(mml[offs:])
            par_dur = 4 / (l if l > 0 else state['l'])
            offs += delta_offs
            par_futen = parse_symbol(mml[offs:], '.')
            offs += par_futen
            par_hold = parse_symbol(mml[offs:], '~')
            offs += par_hold
            par_dur *= (2 - 2**(-par_futen)) * (1 + par_hold)
            dur_ratio = par_dur / sum([abs(_) for _ in res["duration"]])
            # 中身を展開
            pitch_list += res["pitch"]
            dur_list += [_ * dur_ratio for _ in res["duration"]]
            continue
        offs += 1
    # end of mml

    # process last postfix
    if val_futen > 0:
        dur_list[-1] *= 2 - 0.5**val_futen
        val_futen = 0
    if val_hold > 0:
        dur_list[-1] *= 1 + val_hold
        val_hold = 0

    return {
        "pitch": pitch_list,
        "duration": dur_list,
        "parser_state": state
    }

def def_macro(key, value):
    if ' ' in key:
        raise ValueError('Macro keys must not contain spaces.')
    elif key == '':
        raise ValueError('Macro keys must not be "".')
    elif not 'A' <= key[0] <= 'Z':
        raise ValueError('The first letter of the macro key must be CAPITAL.')

    macro_dict[key] = value

# TODO: アルベジエータ？（コードの積み重ね記法は便利なのでそれに乗っかる形で）
def def_arp():
    pass
    '''
    コードの「演奏法」を指定できる．
    通常 'l2 "ceg"' は 和音として演奏されるが，
    これを 'l8 cgeg' のような演奏法に変更することができるイメージ．

    この置き換えで，'"gbe"' は 'g>e<b>e' の演奏法で演奏されてほしい．
    オクターブ変化位置が動的に変わるので，MML文字列への変換ではなく，
    「演奏法」として扱った方が良さそう．

    アルペジエータの名前と，演奏法を表す文字列 'l8 acbc' だけ与えれば良い？
    （ここでは A,a,b,c,d,e,f,g は引数を表す文字として用いることにする (Aはコード直弾き)）
    '''

# =================================================


if __name__ == '__main__':
    print(parse_mml('o4l4 ccggaagr ffeeddcr'))
    # print(macro_dict)
