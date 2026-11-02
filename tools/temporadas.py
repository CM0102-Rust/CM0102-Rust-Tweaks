"""A carreira longa por temporada, de um ou mais tweaks.log (log = 1): as linhas `season` (a população em julho) e, para
cada temporada (de julho a junho), os jovens da disputa: quantos, quantos clubes, o máximo num clube, a parte dos
grandes (reputação ≥ 7000) e os sem clube.
Uso: python tools/temporadas.py A.log [B.log ...]"""
import collections, sys


def temporada(data):
    ano, dia = int(data[:4]), int(data[5:])
    return ano if dia >= 182 else ano - 1


def main():
    for caminho in sys.argv[1:]:
        print('==', caminho)
        por = collections.defaultdict(list)
        for l in open(caminho, encoding='utf-8', errors='replace'):
            c = l.rstrip('\n').split('\t')
            if c[0] == 'season':
                print('  ' + '  '.join(c[1:]))
            elif c[0] == 'regen' and len(c) >= 10:
                por[temporada(c[2])].append((int(c[6]), c[9], int(c[4])))
        print('  temporada  disputa  clubes  max/clube  grandes  sem_clube  PA>=150')
        for t in sorted(por):
            r = por[t]
            com = [x for x in r if x[0] >= 0]
            clubes = collections.Counter(x[1] for x in com)
            print('  %d/%02d  %7d  %6d  %9d  %6.0f%%  %9d  %7d' % (
                t, (t + 1) % 100, len(r), len(clubes), max(clubes.values(), default=0),
                100 * sum(x[0] >= 7000 for x in com) / max(len(r), 1), len(r) - len(com), sum(x[2] >= 150 for x in r)))


if __name__ == '__main__':
    main()
