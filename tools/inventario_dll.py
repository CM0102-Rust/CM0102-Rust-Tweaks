"""Inventário de uma DLL de terceiros (NickDLL, a do Davdav...) contra os trechos que o Tweaks usa: procura, nas seções da
DLL, constantes de 32 bits que caiam nos trechos de cada ajuste (alvos de desvio, endereços que ela lê ou escreve). Não
roda nada e não copia código: só aponta onde olhar. Uma constante pode ser só um dado; a confirmação é o retrato em
memória (`tweaks.log`, linhas `runtime`).
Uso: python tools/inventario_dll.py NickDLL.dll [outra.dll ...]"""
import struct
import sys

import pefile

# (ajuste, início, fim) em VA do exe; os trechos que cada parte confere ou troca (src/lib.rs, src/trocas.rs)
TRECHOS = [
    ('save_over_2gb (zopen/zseek/ztell)', 0x9326e0, 0x9326e0 + 0x1180),
    ('ages (sorteio da idade)', 0x7ac58e, 0x7ac6fa),
    ('ages (corte da disputa, 0x7ace69)', 0x7ace65, 0x7ace6b),
    ('ages_out_of_game', 0x7abea0, 0x7abef1),
    ('rule = new (disputa 0x7ad0e0)', 0x7ad0e0, 0x7ad220),
    ('rule = new (chamada na 0x7abac0)', 0x7ace9b, 0x7aceb5),
    ('rule = new (estrangeiro 0x53d980)', 0x53d980, 0x53d9a0),
    ('rule = new (time B 0x540a50)', 0x540a50, 0x540a70),
    ('rule = new (elenco 0x53c590)', 0x53c590, 0x53c5b0),
    ('rule = new (pode chegar 0x8ce990)', 0x8ce990, 0x8ce9b0),
    ('rule = new / idle (sorteio 0x90cfc0)', 0x90cfc0, 0x90cfe0),
    ('retire_idle_players (0x7a8050)', 0x7a8050, 0x7a8193),
    ('retire_idle_players (observados 0x842080)', 0x842080, 0x8420a0),
    ('unattached (0x7ace6a..0x7ace9b)', 0x7ace6a, 0x7ace9b),
    ('regen (a 0x7abac0 inteira)', 0x7abac0, 0x7ad0e0),
    ('msgbox (0x5e8290)', 0x5e8290, 0x5e8299),
    ('trava das 30 linhas (Area::init 0x402ac0)', 0x402ac0, 0x402ac6),
    ('mutex do jogo (0x9f183c)', 0x9f183c, 0x9f1861),
]


def trocas():
    """Os trechos de no_cd e resolution, de src/trocas.rs."""
    import os
    import re
    s = open(os.path.join(os.path.dirname(__file__), '..', 'src', 'trocas.rs'), encoding='utf-8').read()
    out = []
    for nome in ('CD', 'RESOLUCAO'):
        bloco = s.split('pub const %s' % nome)[1].split('];')[0]
        for m in re.finditer(r'\((0x[0-9a-f]+), &\[([^\]]*)\]', bloco):
            va, n = int(m.group(1), 16), len(m.group(2).split(','))
            out.append(('no_cd' if nome == 'CD' else 'resolution', va, va + n))
    return out


def main():
    alvos = TRECHOS + trocas()
    for caminho in sys.argv[1:]:
        pe = pefile.PE(caminho, fast_load=True)
        print('==', caminho, 'compilada em %#x' % pe.FILE_HEADER.TimeDateStamp)
        achados = {}
        for sec in pe.sections:
            d = sec.get_data()
            for i in range(0, len(d) - 3):
                v = struct.unpack_from('<I', d, i)[0]
                if not 0x401000 <= v < 0xb00000:
                    continue
                for nome, a, b in alvos:
                    if a <= v < b:
                        achados.setdefault(nome, set()).add(v)
        for nome, _, _ in TRECHOS + [('no_cd', 0, 0), ('resolution', 0, 0)]:
            if nome in achados:
                vs = sorted(achados[nome])
                print('  %-45s %3d endereço(s): %s' % (nome, len(vs), ' '.join('%#x' % v for v in vs[:12]) + (' ...' if len(vs) > 12 else '')))


if __name__ == '__main__':
    main()
