"""Compara dois tweaks.log gravados com log = 1 (um com a regra nova, outro com rule = original, que só registra a escolha
da regra do jogo): os jovens da disputa por faixa de reputação do clube, os de PA ≥ 140, as idades, os que ficaram sem
clube, as surpresas e as aposentadorias pela regra dos inativos.
Uso: python tools/logs/comparar_logs.py ORIGINAL.log NOVA.log"""
import collections, sys


def ler(caminho):
    regens, idle, nascidos = [], [], 0
    for l in open(caminho, encoding='utf-8', errors='replace'):
        c = l.rstrip('\n').split('\t')
        if c[0] == 'regen' and len(c) >= 10:
            regens.append(dict(regra=c[1], data=c[2], idade=int(c[3]), pa=int(c[4]), ca=int(c[5]), rep=int(c[6]), tr=int(c[7]),
                               elenco=int(c[8]), clube=c[9], surpresa=len(c) > 12 and c[12] == 'surprise'))
        elif c[0] == 'birth':
            nascidos += 1
        elif c[0] == 'idle' and len(c) >= 5:
            idle.append(dict(data=c[1], idade=int(c[2]), rep=int(c[3]), dias=int(c[4])))
    return regens, idle, nascidos


def faixa(rep):
    if rep < 0:
        return 'sem clube'
    for limite, nome in ((2000, 'abaixo de 2000'), (4000, '2000-3999'), (6000, '4000-5999'), (8000, '6000-7999')):
        if rep < limite:
            return nome
    return '8000 ou mais'


FAIXAS = ['sem clube', 'abaixo de 2000', '2000-3999', '4000-5999', '6000-7999', '8000 ou mais']


def resumo(nome, regens, idle, nascidos):
    datas = sorted(r['data'] for r in regens)
    print('%s: %d jovens na disputa (%s a %s), %d surpresas, %d aposentadorias pela regra dos inativos, %s regens nascidos' % (
        nome, len(regens), datas[0] if datas else '-', datas[-1] if datas else '-', sum(r['surpresa'] for r in regens), len(idle),
        nascidos if nascidos else '- (sem a tabela de idades não há registro)'))
    if idle:
        print('  idades das aposentadorias pelos inativos:', sorted(collections.Counter(i['idade'] for i in idle).items()))
    print('  idades dos jovens:', sorted(collections.Counter(r['idade'] for r in regens).items()))


def main():
    o, n = ler(sys.argv[1]), ler(sys.argv[2])
    resumo('original', *o)
    resumo('nova', *n)
    print('\n%-16s %10s %10s %14s %14s' % ('clube', 'original', 'nova', 'PA>=140 orig.', 'PA>=140 nova'))
    for f in FAIXAS:
        co = sum(1 for r in o[0] if faixa(r['rep']) == f)
        cn = sum(1 for r in n[0] if faixa(r['rep']) == f)
        po = sum(1 for r in o[0] if faixa(r['rep']) == f and r['pa'] >= 140)
        pn = sum(1 for r in n[0] if faixa(r['rep']) == f and r['pa'] >= 140)
        pct = lambda x, t: '%5.1f%%' % (100 * x / t) if t else '   - '
        print('%-16s %4d %s %4d %s %6d %s %6d %s' % (f, co, pct(co, len(o[0])), cn, pct(cn, len(n[0])),
              po, pct(po, sum(r['pa'] >= 140 for r in o[0])), pn, pct(pn, sum(r['pa'] >= 140 for r in n[0]))))
    for nome, (regens, _, _) in (('original', o), ('nova', n)):
        top = collections.Counter(r['clube'] for r in regens if r['rep'] >= 0).most_common(10)
        print('\n%s: os 10 clubes que mais receberam' % nome)
        for clube, q in top:
            exemplo = next(r for r in regens if r['clube'] == clube)
            print('  %-30s rep %5d tr %2d  %d' % (clube[:30], exemplo['rep'], exemplo['tr'], q))


if __name__ == '__main__':
    main()
