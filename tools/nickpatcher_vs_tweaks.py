"""Quais opções do NickPatcher (e quais patches do MiscPatches.zip) escrevem nos trechos que cada ajuste do Tweaks confere
ou troca. Lê os dados do NickPatcher como dados (o dicionário `patches` do Patcher.cs e os .patch do zip), sem rodar nada
e sem copiar código; crédito a Nick+Co e aos autores dos patches.
Uso: python tools/nickpatcher_vs_tweaks.py CAMINHO/CM0102Patcher"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from retrato_exe import ler_catalogo  # noqa: E402

BASE = 0x400000
RAIZ = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')


def trechos_dos_ajustes():
    """(ajuste, início, fim) em VA: as bordas e o corpo de cada parte, as trocas da tela e do CD e os desvios avulsos."""
    s = open(os.path.join(RAIZ, 'src', 'partes.rs'), encoding='utf-8').read()
    out = []
    for m in re.finditer(r'pub const (\w+): Parte = Parte \{\s*nome: "([^"]+)",\s*bordas: &\[(.*?)\],.*?corpo: &\[(.*?)\]', s, re.S):
        for grupo in (m.group(3), m.group(4)):
            for a, n in re.findall(r'\((0x[0-9a-f]+),\s*([0-9a-fx +\-]+?)\)', grupo):
                try:
                    tam = eval(n)  # noqa: S307 (expressões simples como 0x7ac6f6 - 0x7ac58e)
                except Exception:
                    continue
                out.append((m.group(2), int(a, 16), int(a, 16) + tam))
    t = open(os.path.join(RAIZ, 'src', 'trocas.rs'), encoding='utf-8').read()
    for nome, chave in (('CD', 'no_cd'), ('RESOLUCAO', 'resolution')):
        bloco = t.split('pub const %s' % nome)[1].split('];')[0]
        for a, b in re.findall(r'\((0x[0-9a-f]+), &\[([^\]]*)\]', bloco):
            out.append((chave, int(a, 16), int(a, 16) + len(b.split(','))))
    out += [('regen_origins', 0x7a664f, 0x7a6654), ('regen_origins', 0x7a6335, 0x7a633a), ('regen_origins', 0x7a64c9, 0x7a64ce),
            ('regen_origins', 0x7a64e2, 0x7a64e7), ('match report (30 rows)', 0x402ac0, 0x402ac6), ('log (message boxes)', 0x5e8290, 0x5e8299),
            ('ages (contest age)', 0x7ace69, 0x7ace6a)]
    return out


def opcoes_do_patcher(pasta):
    """[(nome, [(deslocamento, n)])] do dicionário `patches` do Patcher.cs."""
    s = open(os.path.join(pasta, 'Patcher.cs'), encoding='utf-8', errors='replace').read()
    ini = s.index('public Dictionary<string, List<HexPatch>> patches')
    fim = s.index('public Dictionary<string, List<HexPatch>> ReversePatches')
    bloco = s[ini:fim]
    out = []
    for m in re.finditer(r'\{\s*"(\w+)",\s*new List<HexPatch>\s*\{(.*?)\}\s*\}', bloco, re.S):
        bytes_ = []
        for off, hexa in re.findall(r'new HexPatch\((0x[0-9A-Fa-f]+|\d+),\s*"([0-9A-Fa-f]+)"\)', m.group(2)):
            bytes_.append((int(off, 0), len(hexa) // 2))
        if bytes_:
            out.append((m.group(1), bytes_))
    return out


def main():
    pasta = sys.argv[1]
    ajustes = trechos_dos_ajustes()
    opcoes = opcoes_do_patcher(pasta)
    for nome, desc, bytes_ in ler_catalogo(os.path.join(pasta, 'MiscPatches.zip')):
        opcoes.append(('MiscPatches: ' + nome, [(o, 1) for o, _, _ in bytes_]))
    print('| Opção do NickPatcher | Ajustes do Tweaks cujos trechos ela escreve |')
    print('|---|---|')
    for nome, bytes_ in opcoes:
        tocados = {}
        for off, n in bytes_:
            va0, va1 = off + BASE, off + BASE + n
            for aj, a, b in ajustes:
                if va0 < b and a < va1:
                    tocados[aj] = tocados.get(aj, 0) + min(b, va1) - max(a, va0)
        if tocados:
            print('| %s | %s |' % (nome, ', '.join('%s (%d bytes)' % kv for kv in sorted(tocados.items()))))


if __name__ == '__main__':
    main()
