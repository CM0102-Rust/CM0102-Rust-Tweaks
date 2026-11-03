"""Gera src/trocas.rs: as trocas de bytes que `no_cd = 1` e `resolution = 1080` aplicam em memória, tiradas da diferença
entre os exes do projeto CM0102 (que gera os dois exes com patch_portatil.py e resolucao.py; o código de lá não vem):
- CD: original → portátil, só as 33 checagens do CD (cada uma um `jne` → `nop nop` e um `test eax,eax` → `xor eax,eax`);
  as outras trocas do portátil (a pasta do registro e o caminho do game.cfg) ficam de fora;
- resolução: portátil → 1080, todas.
Uso: python tools/gerar_trocas.py cm0102.exe cm0102-portatil.exe cm0102-1080.exe"""
import hashlib
import os
import sys

MD5_ORIGINAL = 'ed0cb3bc9bcc8ac3964dad52f6007cb8'
BASE = 0x400000


def va_do_arquivo(pe, off):
    """VA de um deslocamento no arquivo, pelas seções do PE."""
    nt = int.from_bytes(pe[0x3c:0x40], 'little')
    n = int.from_bytes(pe[nt + 6:nt + 8], 'little')
    tam_opc = int.from_bytes(pe[nt + 20:nt + 22], 'little')
    sec = nt + 24 + tam_opc
    for i in range(n):
        s = sec + 40 * i
        va, tam_bruto, ptr = (int.from_bytes(pe[s + k:s + k + 4], 'little') for k in (12, 16, 20))
        if ptr <= off < ptr + tam_bruto:
            return BASE + va + off - ptr
    raise ValueError('deslocamento %#x fora das seções' % off)


def trechos(a, b):
    """Os trechos contíguos em que a e b diferem: (deslocamento, bytes de a, bytes de b)."""
    assert len(a) == len(b), 'os exes têm tamanhos diferentes'
    out, i = [], 0
    while i < len(a):
        if a[i] != b[i]:
            j = i
            while j < len(a) and a[j] != b[j]:
                j += 1
            out.append((i, a[i:j], b[i:j]))
            i = j
        else:
            i += 1
    return out


def rust(nome, doc, lista, pe):
    linhas = ['/// %s' % doc, 'pub const %s: [(usize, &[u8], &[u8]); %d] = [' % (nome, len(lista))]
    for off, x, y in lista:
        linhas.append('    (%#x, &[%s], &[%s]),' % (va_do_arquivo(pe, off), ', '.join('%#04x' % c for c in x), ', '.join('%#04x' % c for c in y)))
    return '\n'.join(linhas + ['];', ''])


def main():
    orig, port, r1080 = (open(f, 'rb').read() for f in sys.argv[1:4])
    if hashlib.md5(orig).hexdigest() != MD5_ORIGINAL:
        sys.exit('o primeiro exe não é o cm0102.exe 3.9.68 intacto')
    todos = trechos(orig, port)
    cd = [t for t in todos if (t[1][:1] == b'\x75' and t[2] == b'\x90\x90') or (t[1] == b'\x85' and t[2] == b'\x31')]
    if len(cd) != 66:
        sys.exit('esperava 66 trocas do CD (33 checagens), achei %d' % len(cd))
    print('portátil: %d trechos, %d do CD; de fora: %s' % (len(todos), len(cd), ', '.join('%#x' % va_do_arquivo(orig, t[0]) for t in todos if t not in cd)))
    res = trechos(port, r1080)
    print('resolução: %d trechos, %d bytes' % (len(res), sum(len(t[1]) for t in res)))
    saida = os.path.join(os.path.dirname(__file__), '..', 'src', 'trocas.rs')
    with open(saida, 'w', encoding='utf-8', newline='\n') as f:
        f.write('//! GERADO por `python tools/gerar_trocas.py` (não editar): as trocas de `no_cd` e `resolution`, (VA, antes, depois),\n'
                '//! tiradas dos exes portátil e 1080 do projeto CM0102.\n\n')
        f.write(rust('CD', 'no_cd: as 33 checagens do CD (o `jne` da unidade vira `nop nop`; o `test eax,eax` do _findfirst, `xor eax,eax`).', cd, orig))
        f.write('\n')
        f.write(rust('RESOLUCAO', 'resolution = 1080: do exe portátil ao de 1920×1080.', res, orig))


if __name__ == '__main__':
    main()
