"""Gera src/exe/referencia.rs: o retrato do cm0102.exe 3.9.68 que a DLL usa para dizer o que outro exe mudou.

Uso: python tools/exe/gerar_referencia.py ORIGINAL.exe

- uma linha por função do .text (tools/exe/mapa_funcoes.csv): endereço, tamanho e o FNV-1a de 32 bits dos bytes;
- blocos de 4 KB das outras seções, como estão no arquivo (.rdata, .data, .rsrc);
- o ponto de entrada e as seções originais.
Só hashes e endereços: nada do código do jogo vai para o repositório."""
import csv, os, sys

import pefile

AQUI = os.path.dirname(os.path.abspath(__file__))
BLOCO = 0x1000


def fnv32(b):
    h = 0x811c9dc5
    for x in b:
        h = ((h ^ x) * 0x01000193) & 0xffffffff
    return h


def main():
    pe = pefile.PE(sys.argv[1], fast_load=True)
    d = open(sys.argv[1], 'rb').read()
    secoes = [(s.Name.rstrip(b'\0').decode(), 0x400000 + s.VirtualAddress, s.Misc_VirtualSize, s.PointerToRawData, s.SizeOfRawData) for s in pe.sections]
    texto = next(s for s in secoes if s[0] == '.text')
    arquivo = lambda va: va - texto[1] + texto[3]
    funcoes = []
    with open(os.path.join(AQUI, 'mapa_funcoes.csv'), encoding='utf-8') as f:
        for r in csv.DictReader(f):
            va, n = int(r['endereco'], 16), int(r['tamanho'])
            fim = min(va + n, texto[1] + texto[2])
            if va < fim:
                funcoes.append((va, fim - va, fnv32(d[arquivo(va):arquivo(fim)])))
    blocos = []
    for nome, va, _, cru, tam in secoes:
        if nome == '.text':
            continue
        for k in range(0, tam, BLOCO):
            n = min(BLOCO, tam - k)
            blocos.append((va + k, n, fnv32(d[cru + k:cru + k + n])))
    linhas = ['//! Gerado por tools/exe/gerar_referencia.py a partir do cm0102.exe 3.9.68: só endereços, tamanhos e FNV-1a de 32 bits.',
              '//! As fronteiras das funções vêm de tools/exe/mapa_funcoes.csv (projeto CM0102).', '',
              'pub const ENTRADA: u32 = 0x%x;' % (0x400000 + pe.OPTIONAL_HEADER.AddressOfEntryPoint),
              '/// (nome, endereço, tamanho na memória)',
              'pub const SECOES: [(&str, u32, u32); %d] = [%s];' % (len(secoes), ', '.join('("%s", 0x%x, 0x%x)' % (n, va, vs) for n, va, vs, _, _ in secoes)),
              '/// (endereço, tamanho, hash) de cada função do .text',
              'pub static FUNCOES: [(u32, u32, u32); %d] = [' % len(funcoes)]
    linhas += ['    %s' % ' '.join('(0x%x, %d, 0x%08x),' % f for f in funcoes[i:i + 6]) for i in range(0, len(funcoes), 6)]
    linhas += ['];', '/// (endereço, tamanho, hash) dos blocos de %d bytes das outras seções, como no arquivo' % BLOCO,
               'pub static BLOCOS: [(u32, u32, u32); %d] = [' % len(blocos)]
    linhas += ['    %s' % ' '.join('(0x%x, %d, 0x%08x),' % b for b in blocos[i:i + 6]) for i in range(0, len(blocos), 6)]
    linhas += ['];', '']
    open(os.path.join(AQUI, '..', '..', 'src', 'exe', 'referencia.rs'), 'w', encoding='utf-8', newline='\n').write('\n'.join(linhas))
    print('%d funções, %d blocos' % (len(funcoes), len(blocos)))


if __name__ == '__main__':
    main()
