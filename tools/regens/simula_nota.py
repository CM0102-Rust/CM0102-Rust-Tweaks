"""Monte Carlo da 0x7ad220 + 0x7ad0e0 (a nota do clube para o jovem e o maior sorteio). Fórmula transcrita de
rust/src/motor.rs (nota_do_clube_para_o_jovem). Clubes e jovem ilustrativos."""
import random, collections

def chance(ca, jp):  # 0x544410
    x = (ca + 2 * jp) * 0.05 + 10
    return int(max(1, min(20, (x + 0.1 * x * x) / 3 + 0.5)))

def nota(j, c, R=random.randrange):
    rep = c['rep']
    s = 5000 + rep - abs(j['rep'] - rep) - abs(j['pa'] * 50 - rep) - abs(j['ca'] * 50 - rep)
    tr = c['tr']
    if j['pa'] > R(20) + 100:
        if tr == 20:
            s += rep // 2 + 7000
        elif R(tr) > R(20):
            s += rep // 2 + (tr + 100) * 50
    if j['pa'] > 50 and c['ativa']:
        if c['humano'] and j['pa'] > 120:
            s += 250  # estado +4 do clube = 0 (suposto)
        if rep > 5750:
            s += 1000 if c['elenco'] < 40 else 750 if c['elenco'] < 45 else 250
        elif rep > 4250:
            s += 750 if c['elenco'] < 35 else 500 if c['elenco'] < 40 else 250
        else:
            s += 250
    m = c.get('tec')
    if m and not c['humano']:
        s += (m['jovens'] - 10) * 30
        v = chance(m['ca'], m['jp'])
        if j['pa'] > R(75) + 100:
            s += (v - 10) * 60
        if v + m['jovens'] + tr > 40 and rep > 4250 and j['pa'] > 80:
            s = rep if s < rep else s + 250
    elif c['humano']:
        s += 200
        if j['pa'] > 120:
            if tr == 20 and s < rep:
                s = rep
            s += tr * 25
    s += 1000  # mesma nação
    return R(s) if s > 0 else 0

def corrida(j, clubes, n=100000):
    v = collections.Counter()
    for _ in range(n):
        v[max(clubes, key=lambda c: nota(j, c))['nome']] += 1
    return {k: v[k] / n for k in v}

bom = dict(jovens=16, ca=150, jp=16)
medio = dict(jovens=10, ca=100, jp=10)
clubes = [
    dict(nome='A grande IA (9000, tr 20)', rep=9000, tr=20, elenco=35, ativa=1, humano=0, tec=bom),
    dict(nome='B médio IA (6500, tr 14)', rep=6500, tr=14, elenco=30, ativa=1, humano=0, tec=medio),
    dict(nome='C humano (6500, tr 20)', rep=6500, tr=20, elenco=30, ativa=1, humano=1, tec=bom),
    dict(nome='D 2a divisão IA (4000, tr 10)', rep=4000, tr=10, elenco=28, ativa=1, humano=0, tec=medio),
] + [dict(nome='E pequenos (1500, tr 3) x40', rep=1500, tr=3, elenco=25, ativa=1, humano=0, tec=medio) for _ in range(40)]

if __name__ == '__main__':
    random.seed(1)
    for j in (dict(pa=170, ca=80, rep=2250), dict(pa=130, ca=65, rep=1950), dict(pa=90, ca=45, rep=1550)):
        print(j)
        for k, p in sorted(corrida(j, clubes, 20000).items(), key=lambda x: -x[1]):
            print('   %-32s %5.1f%%' % (k, 100 * p))
    # sem o clube humano: a mesma corrida só com a IA
    print('sem o humano, PA 170:', corrida(dict(pa=170, ca=80, rep=2250), [c for c in clubes if not c['humano']], 20000))
