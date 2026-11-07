"""Dá nome às linhas do retrato no tweaks.log (exe-fn, exe-data, runtime-fn): a função do 3.9.68, o arquivo de código e os
patches do catálogo do NickPatcher que mexem nela.

Uso: python tools/exe/ler_retrato.py tweaks.log [--exe O_EXE.exe] [--catalogo MiscPatches.zip] [--nomes PASTA_DO_MAPA]

Com --exe, só lista os patches do catálogo que estão mesmo aplicados nele; sem, os que tocam a função (candidatos).
Os nomes vêm do mapa da comunidade (maxevans60, CC BY 4.0) e o catálogo é o que acompanha o NickPatcher (Nick+Co e
os autores de cada patch), lidos como dado."""
import argparse, bisect, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from retrato_exe import BASE, ler_catalogo, ler_mapa, ler_nomes


def main():
    a = argparse.ArgumentParser()
    a.add_argument('log'); a.add_argument('--exe'); a.add_argument('--catalogo'); a.add_argument('--nomes')
    o = a.parse_args()
    ini, info = ler_mapa()
    nomes = ler_nomes(o.nomes)
    catalogo = ler_catalogo(o.catalogo)
    exe = open(o.exe, 'rb').read() if o.exe else None
    for linha in open(o.log, encoding='utf-8', errors='replace'):
        c = linha.rstrip('\n').split('\t')
        if c[0] not in ('exe-fn', 'exe-data', 'runtime-fn'):
            if c[0] in ('exe', 'runtime'):
                print(linha.rstrip())
            continue
        va, n = int(c[1], 16), int(c[2])
        k = bisect.bisect_right(ini, va) - 1
        f = ini[k] if k >= 0 else 0
        tam, arq = info[k] if k >= 0 else (0, '')
        nome = nomes.get(f, '') if c[0] != 'exe-data' else ''
        patches = []
        for p, desc, bs in catalogo:
            dentro = [(d, x, y) for d, x, y in bs if va <= d + BASE < va + n]
            if dentro and (exe is None or all(d < len(exe) and exe[d] == y for d, _, y in dentro)):
                patches.append(p)
        onde = '%08x %s (%s)' % (f, nome or '?', arq or '?') if c[0] != 'exe-data' else 'data block'
        print('%-10s %08x %6d  %s%s' % (c[0], va, n, onde, ('  <- ' + ', '.join(sorted(set(patches))[:6])) if patches else ''))


if __name__ == '__main__':
    main()
