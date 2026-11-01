"""Calibração da regra dos regens: os clubes reais de um país (lidos de um save) disputam uma temporada de jovens pela
regra original (a nota da 0x7ad220 e o maior sorteio da 0x7ad0e0) e pela regra nova (sorteio proporcional, os pesos de
`regens.cfg`). Só leitura do save; nada roda no jogo.

Uso: python tools/calibrar.py SAVE PAIS [--cfg regens.cfg] [--temporadas 20] [--humano "Nome do clube"]
Ex.: python tools/calibrar.py TesteTelas.sav Inglaterra --humano Fulham

Simplificações (ponytail: bastam para comparar as duas regras; o jogo real é a prova):
- os jovens são os do ramo da disputa (3 em 4 dos regens de nação no jogo), com a idade pela tabela original (regra
  original) ou pela do .cfg (regra nova, age_NN); os que passam de contest_max_age ficam de fora; o PA vem dos jogadores
  de 31 anos ou mais do país (os próximos a se aposentar), sorteado com reposição; quantos por temporada (antes do corte
  da idade): --jovens, ou 1 para cada 120 jogadores do país (no Valencia.sav, a Inglaterra teve ~88 regens por temporada,
  e 3 em 4 vão para o ramo da disputa: ~66 para 7.900 jogadores);
- CA = 0,25·PA + sorteio(0,6·PA) com o teto da idade, sem a posição;
- competição ativa = clube com divisão; time B = nome terminado em " B" ou " II";
- o bônus de +250/+500 do humano pelo estado do clube (ainda não decifrado) fica de fora."""
import argparse, collections, math, os, random, struct, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sav


# ---------- o save ----------

def ler_save(caminho):
    f = sav.itens(caminho, ('club.dat', 'nation.dat', 'staff.dat', 'player.dat', 'nonplayer.dat'))
    i32 = lambda b, o: struct.unpack_from('<i', b, o)[0]
    nacoes = {i32(f['nation.dat'], k * 290): f['nation.dat'][k * 290 + 4:k * 290 + 55].split(b'\0')[0].decode('latin1')
              for k in range(len(f['nation.dat']) // 290)}
    P = f['player.dat']
    fichas = {i32(P, k * 70): (struct.unpack_from('<h', P, k * 70 + 7)[0]) for k in range(len(P) // 70)}
    NP = f['nonplayer.dat']
    tecnicos = {i32(NP, k * 68): dict(ca=struct.unpack_from('<h', NP, k * 68 + 4)[0], jp=NP[k * 68 + 0x18], jovens=NP[k * 68 + 0x22])
                for k in range(len(NP) // 68)}
    S = f['staff.dat']
    pessoas = {}
    for k in range(len(S) // 110):
        o = k * 110
        pessoas[i32(S, o)] = dict(idade=S[o + 0x18], nacao=i32(S, o + 0x1a), clube=i32(S, o + 0x39), ficha=i32(S, o + 0x61), np=i32(S, o + 0x69))
    C = f['club.dat']
    clubes = []
    for k in range(len(C) // 581):
        o = k * 581
        elenco = [i32(C, o + 0xd7 + 4 * j) for j in range(50)]
        comissao = [i32(C, o + 0xcf)] + [i32(C, o + 0x19f + 4 * j) for j in range(5)]
        clubes.append(dict(nome=C[o + 4:o + 55].split(b'\0')[0].decode('latin1'), nacao=i32(C, o + 0x53), divisao=i32(C, o + 0x57),
                           rep=struct.unpack_from('<H', C, o + 0x80)[0], tr=C[o + 0x7f],
                           elenco=sum(1 for e in elenco if e >= 0), comissao=[tecnicos.get(pessoas[s]['np']) if s in pessoas else None for s in comissao]))
    return nacoes, fichas, pessoas, clubes


# ---------- a regra original (0x7ad220, transcrita da reescrita em Rust do projeto CM0102) ----------

def chance_de_conhecer(ca, jp):  # 0x544410
    x = (ca + 2 * jp) * 0.05 + 10
    return int(max(1, min(20, (x + 0.1 * x * x) / 3 + 0.5)))


def nota_original(j, c, R=random.randrange):
    rep = c['rep']
    s = 5000 + rep - abs(j['rep'] - rep) - abs(j['pa'] * 50 - rep) - abs(j['ca'] * 50 - rep)
    tr = c['tr']
    if j['pa'] > R(20) + 100:
        if tr == 20:
            s += rep // 2 + 7000
        elif tr > 0 and R(tr) > R(20):
            s += rep // 2 + (tr + 100) * 50
    if j['pa'] > 50 and c['divisao'] >= 0:
        if rep > 5750:
            s += 1000 if c['elenco'] < 40 else 750 if c['elenco'] < 45 else 250
        elif rep > 4250:
            s += 750 if c['elenco'] < 35 else 500 if c['elenco'] < 40 else 250
        else:
            s += 250
    m = c['comissao'][0]
    if m and not c['humano']:
        s += (m['jovens'] - 10) * 30
        v = chance_de_conhecer(m['ca'], m['jp'])
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
    s += 1000  # mesmo país
    s = (s + 0x8000) % 0x10000 - 0x8000  # só a palavra baixa vai ao sorteio
    return R(s) if s > 0 else -1


def escolher_original(j, clubes):
    melhor, nota = None, -25000
    for c in clubes:
        if c['elenco'] < 50:
            n = nota_original(j, c)
            if n > nota:
                melhor, nota = c, n
    return melhor


# ---------- a regra nova (sorteio proporcional) ----------

PADRAO = dict(target_k=50.0, target_ca=0.0, sigma=1500.0, level_floor=0.002, facilities_base=0.5, facilities_k=0.05, staff_min=0.8, staff_max=1.2,
              squad_limit=50, squad_exp=0.5, season_cap=3, temperature=1.0)


# A distribuição original das idades (0x7ac58e), que é também o padrão do .cfg: idade: (peso %, teto do CA ou None).
IDADES = {15: (0.0025, 80), 16: (16.67, 80), 17: (29.11, 80), 18: (29.11, 100), 19: (12.44, 110), 20: (12.44, 110),
          21: (0.04, None), 22: (0.04, None), 23: (0.04, None), 24: (0.04, None), 25: (0.04, None), 26: (0.04, None)}
ORIGINAL = dict(idades=IDADES, contest_max_age=18, ca_cap_skip=0.5)


def ler_cfg(caminho):
    cfg = dict(PADRAO, **ORIGINAL)
    cfg['idades'] = dict(IDADES)
    if caminho and os.path.exists(caminho):
        for linha in open(caminho, encoding='utf-8'):
            linha = linha.split('#')[0].strip()
            if '=' not in linha:
                continue
            k, v = (x.strip() for x in linha.split('=', 1))
            if k.startswith('age_') and k[4:].isdigit():
                partes = [p.strip() for p in v.split(',')]
                cfg['idades'][int(k[4:])] = (float(partes[0]), int(partes[1]) if len(partes) > 1 and partes[1] else None)
            elif k == 'ages' and v == 'original':
                cfg['idades_originais'] = True
            elif k in cfg and k != 'idades':
                cfg[k] = float(v)
    if cfg.pop('idades_originais', False):
        cfg.update(ORIGINAL)
    return cfg


def peso_novo(j, c, cfg, recebidos):
    """O peso do clube: nível × instalações × comissão × elenco, à potência da temperatura (o mesmo que src/lib.rs)."""
    if c['elenco'] >= cfg['squad_limit'] or recebidos >= cfg['season_cap'] > 0:
        return 0.0
    alvo = cfg['target_k'] * ((1 - cfg['target_ca']) * j['pa'] + cfg['target_ca'] * j['ca'])
    nivel = cfg['level_floor'] + (1 - cfg['level_floor']) * math.exp(-0.5 * ((c['rep'] - alvo) / cfg['sigma']) ** 2)
    inst = cfg['facilities_base'] + cfg['facilities_k'] * c['tr']
    jovens = max((t['jovens'] for t in c['comissao'] if t), default=1)
    comissao = cfg['staff_min'] + (cfg['staff_max'] - cfg['staff_min']) * (jovens - 1) / 19
    elenco = ((cfg['squad_limit'] - c['elenco']) / cfg['squad_limit']) ** cfg['squad_exp']
    return (nivel * inst * comissao * elenco) ** cfg['temperature']


def escolher_novo(j, clubes, cfg, recebidos):
    pesos = [peso_novo(j, c, cfg, recebidos[c['nome']]) for c in clubes]
    total = sum(pesos)
    if total <= 0:
        return None
    x = random.random() * total
    for c, p in zip(clubes, pesos):
        x -= p
        if x < 0:
            return c
    return clubes[-1]


# ---------- a temporada ----------

def jovem(pa, idades):
    """Um jovem pela tabela de idades (a original ou a do .cfg); None se a idade passa do corte da disputa."""
    tabela = list(idades['idades'].items())
    idade, (_, teto) = random.choices(tabela, weights=[p for _, (p, _) in tabela])[0]
    if idade > idades['contest_max_age']:
        return None
    ca = int(0.25 * pa) + random.randrange(max(1, int(0.6 * pa)))
    if teto is not None and ca > teto and random.random() * 100 >= idades['ca_cap_skip']:
        ca = teto
    return dict(pa=pa, ca=ca, rep=250 + random.randrange(750) + 20 * ca + random.randrange(50))


def temporada(regra, clubes, pas, n, cfg):
    for c in clubes:
        c['elenco'] = c['elenco0']
    recebidos = collections.Counter()
    for _ in range(n):
        j = jovem(random.choice(pas), ORIGINAL if regra == 'original' else cfg)
        if j is None:
            continue
        c = escolher_original(j, clubes) if regra == 'original' else escolher_novo(j, clubes, cfg, recebidos)
        if c:
            c['elenco'] += 1
            recebidos[c['nome']] += 1
            yield c, j


def main():
    a = argparse.ArgumentParser()
    a.add_argument('save'); a.add_argument('pais')
    a.add_argument('--cfg', default=os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'regens.cfg'))
    a.add_argument('--temporadas', type=int, default=20); a.add_argument('--jovens', type=int)
    a.add_argument('--humano', default=''); a.add_argument('--semente', type=int, default=1)
    o = a.parse_args()
    random.seed(o.semente)
    nacoes, fichas, pessoas, clubes = ler_save(o.save)
    alvo = [i for i, n in nacoes.items() if n.lower() == o.pais.lower()] or [i for i, n in nacoes.items() if o.pais.lower() in n.lower()]
    if not alvo:
        sys.exit('país não achado: ' + o.pais)
    nid = alvo[0]
    cl = [c for c in clubes if c['nacao'] == nid and not c['nome'].endswith((' B', ' II'))]
    for c in cl:
        c['elenco0'] = c['elenco']
        c['humano'] = bool(o.humano) and o.humano.lower() in c['nome'].lower()
    jogadores = [p for p in pessoas.values() if p['nacao'] == nid and p['ficha'] in fichas]
    pas = [fichas[p['ficha']] for p in jogadores if p['idade'] >= 31 and fichas[p['ficha']] > 0]
    n = o.jovens or max(1, len(jogadores) // 120)
    cfg = ler_cfg(o.cfg)
    print('%s: %d clubes, %d jogadores, %d jovens por temporada no ramo da disputa (antes do corte da idade), %d temporadas' % (nacoes[nid], len(cl), len(jogadores), n, o.temporadas))
    faixa = lambda r: min(r // 1000, 9)
    nfx = collections.Counter(faixa(c['rep']) for c in cl)
    res = {}
    for regra in ('original', 'novo'):
        por, bons, clube = collections.Counter(), collections.Counter(), collections.Counter()
        for _ in range(o.temporadas):
            for c, j in temporada(regra, cl, pas, n, cfg):
                por[faixa(c['rep'])] += 1
                clube[c['nome']] += 1
                if j['pa'] >= 140:
                    bons[faixa(c['rep'])] += 1
        res[regra] = (por, bons, clube)
    print('\nreputação   clubes | por clube/temporada: original  novo | PA>=140 por clube/temporada: original  novo')
    for b in sorted(nfx):
        po, bo, _ = res['original']; pn, bn, _ = res['novo']
        k = nfx[b] * o.temporadas
        print('%d000-%d999  %5d  |  %8.2f  %6.2f  |  %8.3f  %6.3f' % (b, b, nfx[b], po[b] / k, pn[b] / k, bo[b] / k, bn[b] / k))
    for regra in ('original', 'novo'):
        print('\n%s: os 12 que mais recebem (por temporada)' % regra)
        for nome, q in res[regra][2].most_common(12):
            c = next(x for x in cl if x['nome'] == nome)
            print('  %-30s rep %5d tr %2d elenco %2d%s  %.2f' % (nome[:30], c['rep'], c['tr'], c['elenco0'], ' (humano)' if c['humano'] else '', q / o.temporadas))


if __name__ == '__main__':
    main()
