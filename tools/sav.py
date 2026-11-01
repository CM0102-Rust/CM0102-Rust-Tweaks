"""Leitura dos itens de um save do CM 01/02 (.sav): o índice de 1000 entradas e o conteúdo de cada item, com o RLE
dos saves comprimidos (tipo 4). Só leitura."""
import struct


def indice(dados):
    """(tipo, id, [(nome, posicao, tamanho)])."""
    tipo, ident, n = struct.unpack_from('<3I', dados)
    itens = []
    for i in range(min(n, 1000)):
        e = 12 + i * 0x10c
        pos, tam = struct.unpack_from('<2I', dados, e)
        itens.append((dados[e + 8:e + 0x10c].split(b'\0')[0].decode('latin-1'), pos, tam))
    return tipo, ident, itens


def conteudo(dados, tipo, pos, tam):
    """O item descomprimido. RLE do tipo 4: b <= 0x80 literal; b > 0x80 = b - 0x80 cópias do byte seguinte."""
    if tipo != 4:
        return dados[pos:pos + tam]
    saida, j = bytearray(), pos
    while len(saida) < tam and j < len(dados):
        b = dados[j]
        if b > 0x80:
            saida += dados[j + 1:j + 2] * (b - 0x80)
            j += 2
        else:
            saida.append(b)
            j += 1
    return bytes(saida)


def itens(caminho, nomes):
    """{nome: conteúdo} dos itens pedidos (a primeira ocorrência de cada um)."""
    d = open(caminho, 'rb').read()
    t, _, it = indice(d)
    out = {}
    for n, p, tam in it:
        if n.lower() in nomes and n.lower() not in out:
            out[n.lower()] = conteudo(d, t, p, tam)
    return out
