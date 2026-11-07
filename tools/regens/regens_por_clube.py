"""Regens (bit 0x40 do +0xb do registro de 79 bytes) por clube: reputação (+0x80) e instalações (Training, +0x7f).
So leitura. Uso: python regens_por_clube.py SAVE"""
import collections, os, struct, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sav

f = sav.itens(sys.argv[1], ('staff.dat', 'general.dat', 'club.dat', 'player.dat'))
S, G, C, P = f['staff.dat'], f['general.dat'], f['club.dat'], f['player.dat']
N = len(S) // 110
T = G[0xfa7:0xfa7 + 79 * N]
clubes = {}
for k in range(len(C) // 581):
    o = k * 581
    cid = struct.unpack_from('<i', C, o)[0]
    nome = C[o + 4:o + 55].split(b'\0')[0].decode('latin1')
    rep = struct.unpack_from('<H', C, o + 0x80)[0]
    clubes[cid] = dict(nome=nome, rep=rep, tr=C[o + 0x7f], nacao=struct.unpack_from('<i', C, o + 0x53)[0],
                       elenco=sum(struct.unpack_from('<i', C, o + 0xd7 + 4 * j)[0] not in (-1, 0) for j in range(50)))
pa = {}
for k in range(len(P) // 70):
    pa[struct.unpack_from('<i', P, k * 70)[0]] = struct.unpack_from('<h', P, k * 70 + 7)[0]
reg = []
for p in range(N):
    if T[79 * p + 0xb] & 0x40:
        o = p * 110
        reg.append((struct.unpack_from('<i', S, o + 0x39)[0], pa.get(struct.unpack_from('<i', S, o + 0x61)[0], -1)))
com = [r for r in reg if r[0] in clubes]
print('regens', len(reg), 'com clube', len(com), 'clubes', len(clubes))
faixa = lambda r: min(r // 1000, 9)
por = collections.Counter(faixa(clubes[c]['rep']) for c, _ in com)
nfx = collections.Counter(faixa(v['rep']) for v in clubes.values())
alto = collections.Counter(faixa(clubes[c]['rep']) for c, a in com if a >= 150)
print('reputacao  clubes  regens  por_clube  PA>=150')
for b in range(10):
    print('%d000-%d999  %5d  %6d  %8.2f  %6d' % (b, b, nfx[b], por[b], por[b] / max(nfx[b], 1), alto[b]))
tr = collections.Counter(clubes[c]['tr'] for c, _ in com)
ntr = collections.Counter(v['tr'] for v in clubes.values())
print('training  clubes  regens  por_clube')
for x in sorted(ntr):
    print('%3d  %5d  %5d  %6.2f' % (x, ntr[x], tr[x], tr[x] / ntr[x]))
top = collections.Counter(c for c, _ in com)
print('clubes com mais regens:')
for c, n in top.most_common(20):
    v = clubes[c]
    print('  %-28s rep %5d tr %2d elenco %2d regens %d (PA>=150: %d)' % (v['nome'], v['rep'], v['tr'], v['elenco'], n,
          sum(1 for cc, a in com if cc == c and a >= 150)))
print('PA dos regens:', sorted(collections.Counter(min(a // 20 * 20, 180) for _, a in reg).items()))
