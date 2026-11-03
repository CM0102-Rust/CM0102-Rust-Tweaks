"""O retrato de um exe do CM 01/02 contra o 3.9.68 original: o que ele muda, onde e, quando possível, com que nome.

Uso: python tools/retrato_exe.py ORIGINAL.exe EXE [--catalogo MiscPatches.zip] [--nomes PASTA_DO_MAPA] [--saida relatorio.md]

- Compara os bytes de cada seção do original (.text, .rdata, .data, .rsrc) com os do exe; as seções novas (como a .nick do
  NickPatcher) são listadas à parte.
- Agrupa os bytes mudados em trechos e põe cada trecho do código na sua função (tools/mapa_funcoes.csv, as fronteiras das
  funções do 3.9.68), com o nome do mapa da comunidade (maxevans60, CC BY 4.0) quando a pasta dele é dada.
- Nos trechos de código, procura desvios (jmp/call) para fora das seções originais: é assim que um patcher pendura código
  novo numa seção própria.
- Dá nome aos trechos pelo catálogo de patches da comunidade que acompanha o NickPatcher (MiscPatches.zip: um .patch com
  "deslocamento: original novo" por byte e um .txt com a descrição), lido como dado; crédito a Nick+Co e aos autores.
  O que não casa com nenhum patch aparece como mudança desconhecida.
Só leitura."""
import argparse, bisect, csv, glob, io, os, re, zipfile

import capstone
import pefile

AQUI = os.path.dirname(os.path.abspath(__file__))
BASE = 0x400000


def secoes(pe):
    return {s.Name.rstrip(b'\0').decode('latin1'): s for s in pe.sections}


def trechos(posicoes, folga=8):
    """Agrupa posições em trechos [ini, fim) juntando as que ficam a até `folga` bytes."""
    out = []
    for p in posicoes:
        if out and p - out[-1][1] <= folga:
            out[-1][1] = p + 1
        else:
            out.append([p, p + 1])
    return out


def ler_mapa():
    ini, info = [], []
    with open(os.path.join(AQUI, 'mapa_funcoes.csv'), encoding='utf-8') as f:
        for r in csv.DictReader(f):
            ini.append(int(r['endereco'], 16))
            info.append((int(r['tamanho']), r['arquivo']))
    return ini, info


def ler_nomes(pasta):
    nomes = {}
    if not pasta or not os.path.isdir(pasta):
        return nomes
    for arq in glob.glob(os.path.join(pasta, '*.md')):
        for linha in open(arq, encoding='utf-8', errors='replace'):
            c = linha.split('|')
            if len(c) > 4 and re.match(r'\s*0x[0-9a-f]+\s*$', c[1]):
                m = re.search(r'`([^`]+)`', c[3])
                if m:
                    nomes[int(c[1], 16)] = m.group(1)
    return nomes


def ler_catalogo(caminho):
    """[(nome, descrição, [(deslocamento no arquivo, original, novo)])]."""
    out = []
    if not caminho or not os.path.exists(caminho):
        return out
    with zipfile.ZipFile(caminho) as z:
        textos = {n[:-4]: n for n in z.namelist() if n.lower().endswith('.txt')}
        for n in z.namelist():
            if not n.lower().endswith('.patch'):
                continue
            bytes_ = []
            for linha in io.TextIOWrapper(z.open(n), encoding='latin1'):
                # dois formatos no catálogo: "003A5806: 80 E9" e "0x3abeab 0xf 0xe9" (ou "0x124DED E1 66")
                m = re.match(r'\s*(?:0x)?([0-9A-Fa-f]+)\s*:?\s+(?:0x)?([0-9A-Fa-f]{1,2})\s+(?:0x)?([0-9A-Fa-f]{1,2})\s*$', linha)
                if m:
                    bytes_.append((int(m.group(1), 16), int(m.group(2), 16), int(m.group(3), 16)))
            desc = ''
            if n[:-6] in textos:
                desc = io.TextIOWrapper(z.open(textos[n[:-6]]), encoding='latin1').read().strip().split('\n')[0][:160]
            if bytes_:
                out.append((n.rsplit('/', 1)[-1][:-6], desc, bytes_))
    return out


def main():
    a = argparse.ArgumentParser()
    a.add_argument('original'); a.add_argument('exe')
    a.add_argument('--catalogo'); a.add_argument('--nomes'); a.add_argument('--saida')
    o = a.parse_args()
    po, pe = pefile.PE(o.original, fast_load=True), pefile.PE(o.exe, fast_load=True)
    bo, be = open(o.original, 'rb').read(), open(o.exe, 'rb').read()
    so, se = secoes(po), secoes(pe)
    ini, info = ler_mapa()
    nomes = ler_nomes(o.nomes)
    catalogo = ler_catalogo(o.catalogo)
    linhas = ['# Retrato de `%s`' % os.path.basename(o.exe), '', 'Contra `%s` (o 3.9.68 original).' % os.path.basename(o.original), '']

    novas = [n for n in se if n not in so]
    linhas += ['## Seções', '']
    for n, s in se.items():
        marca = ' **(nova)**' if n in novas else ''
        linhas.append('- `%s` em 0x%x, %d KB%s' % (n, BASE + s.VirtualAddress, s.Misc_VirtualSize // 1024, marca))
    faixas_originais = [(BASE + s.VirtualAddress, BASE + s.VirtualAddress + s.Misc_VirtualSize) for s in so.values()]
    fora = lambda va: not any(i <= va < f for i, f in faixas_originais)

    # os bytes mudados, por seção (pelo deslocamento no arquivo; as seções originais têm o mesmo leiaute)
    mudados = {}
    for n, s in so.items():
        if n not in se:
            continue
        t = se[n]
        tam = min(s.SizeOfRawData, t.SizeOfRawData)
        for k in range(tam):
            if bo[s.PointerToRawData + k] != be[t.PointerToRawData + k]:
                mudados.setdefault(n, []).append(s.PointerToRawData + k)

    # o catálogo: um patch "está aplicado" quando todos os bytes dele (nas seções comparadas) têm o valor novo
    explicado, aplicados = set(), []
    todas = {p for v in mudados.values() for p in v}
    for nome, desc, bs in catalogo:
        dentro = [(d, x, y) for d, x, y in bs if d < len(bo) and d < len(be)]
        if not dentro:
            continue
        iguais = sum(1 for d, x, y in dentro if be[d] == y and bo[d] == x)
        if iguais == len(dentro):
            aplicados.append((nome, desc, len(dentro), min(d for d, _, _ in dentro)))
            explicado |= {d for d, _, _ in dentro}
        elif iguais >= max(3, 0.8 * len(dentro)):
            aplicados.append((nome + ' (em parte: %d de %d bytes)' % (iguais, len(dentro)), desc, iguais, min(d for d, _, _ in dentro)))
            explicado |= {d for d, x, y in dentro if be[d] == y}

    linhas += ['', '## Resumo', '', '- bytes mudados nas seções originais: %d (%s)' % (len(todas), ', '.join('%s %d' % (n, len(v)) for n, v in mudados.items())),
               '- patches do catálogo reconhecidos: %d; bytes explicados por eles: %d; bytes sem nome: %d' % (len(aplicados), len(explicado & todas), len(todas - explicado))]
    if catalogo:
        linhas += ['', '## Patches reconhecidos (catálogo do NickPatcher)', '']
        for nome, desc, n, d in sorted(aplicados, key=lambda x: x[3]):
            linhas.append('- **%s** (0x%x, %d bytes): %s' % (nome, d + BASE, n, desc or '-'))

    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
    linhas += ['', '## Mudanças sem nome no código (por função)', '']
    por_funcao = {}
    for d0, d1 in trechos(sorted(p for p in mudados.get('.text', []) if p not in explicado)):
        va = d0 + BASE
        k = bisect.bisect_right(ini, va) - 1
        f = ini[k] if k >= 0 else 0
        por_funcao.setdefault(f, []).append((d0, d1))
    for f, ts in sorted(por_funcao.items()):
        k = ini.index(f)
        tam, arq = info[k]
        nome = nomes.get(f, '')
        desvios = []
        for d0, d1 in ts:
            for i in md.disasm(be[max(d0 - 8, 0):d1 + 8], max(d0 - 8, 0) + BASE):
                if i.mnemonic in ('jmp', 'call') and i.op_str.startswith('0x'):
                    alvo = int(i.op_str, 16)
                    if fora(alvo):
                        desvios.append('%s 0x%x -> 0x%x' % (i.mnemonic, i.address, alvo))
        trechos_txt = ', '.join('0x%x..0x%x' % (d0 + BASE, d1 + BASE) for d0, d1 in ts[:6]) + (' ...' if len(ts) > 6 else '')
        linhas.append('- `0x%x` %s (%s, %d bytes): %d bytes mudados em %s%s' % (
            f, ('`%s`' % nome) if nome else '', arq, tam, sum(d1 - d0 for d0, d1 in ts), trechos_txt,
            ('; **desvio para fora:** ' + '; '.join(sorted(set(desvios)))) if desvios else ''))

    linhas += ['', '## Mudanças sem nome nos dados', '']
    for n in ('.rdata', '.data', '.rsrc'):
        ts = trechos(sorted(p for p in mudados.get(n, []) if p not in explicado), folga=16)
        if ts:
            linhas.append('- `%s`: %d trechos, %d bytes: %s%s' % (n, len(ts), sum(d1 - d0 for d0, d1 in ts),
                          ', '.join('0x%x..0x%x' % (d0 + BASE, d1 + BASE) for d0, d1 in ts[:10]), ' ...' if len(ts) > 10 else ''))
    texto = '\n'.join(linhas) + '\n'
    if o.saida:
        open(o.saida, 'w', encoding='utf-8').write(texto)
        print('relatório em', o.saida)
    else:
        print(texto)


if __name__ == '__main__':
    main()
