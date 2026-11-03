"""Confere o regen_origins.csv contra um save da mesma carreira: cada regen do CSV (nascido até a data do save) tem de
ser regen no save (bit 0x40 do +0xb do registro de 79 bytes) com o mesmo nome, a mesma nação, o mesmo PA (a ficha que
ele herdou) e, em geral, o mesmo clube (um regen pode mudar de clube depois de nascer: isso sai como aviso, não erro).
Também conta os donos desconhecidos e confere que o PA do regen é o PA da ficha registrada (o dono da ficha).
Só leitura. Uso: python tools/conferir_origens.py regen_origins.csv SAVE.sav [carreira]
(carreira: só as linhas com esse nome na coluna career; sem ele, todas)"""
import csv
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sav


def nomes(dados):
    out = {}
    for k in range(len(dados) // 60):
        o = k * 60
        out[struct.unpack_from('<i', dados, o + 0x33)[0]] = dados[o:o + 51].split(b'\0')[0].decode('latin-1')
    return out


def main():
    csv_, save = sys.argv[1], sys.argv[2]
    carreira = sys.argv[3] if len(sys.argv) > 3 else None
    f = sav.itens(save, ('staff.dat', 'general.dat', 'club.dat', 'player.dat', 'nation.dat', 'first_names.dat',
                         'second_names.dat', 'common_names.dat'))
    S, G, C, P, N = f['staff.dat'], f['general.dat'], f['club.dat'], f['player.dat'], f['nation.dat']
    pn, sn, cn = nomes(f['first_names.dat']), nomes(f['second_names.dat']), nomes(f['common_names.dat'])
    n = len(S) // 110
    T = G[0xfa7:0xfa7 + 79 * n]
    clube = {struct.unpack_from('<i', C, k * 581)[0]: C[k * 581 + 4:k * 581 + 55].split(b'\0')[0].decode('latin-1')
             for k in range(len(C) // 581)}
    nacao = {struct.unpack_from('<i', N, k * 290)[0]: N[k * 290 + 4:k * 290 + 55].split(b'\0')[0].decode('latin-1')
             for k in range(len(N) // 290)}
    pa = {struct.unpack_from('<i', P, k * 70)[0]: struct.unpack_from('<h', P, k * 70 + 7)[0] for k in range(len(P) // 70)}
    linhas = [l for l in csv.reader(open(csv_, encoding='utf-8-sig'), delimiter=';')][1:]
    # só o que nasceu antes da gravação deste save: as linhas até o "saved as" dele (os regens do mesmo dia, depois da
    # gravação, não estão no save)
    nome_save = os.path.basename(save).lower()
    fim = next((k for k, l in enumerate(linhas) if len(l) >= 4 and l[2].lower() == nome_save and l[3].startswith('saved as')), len(linhas))
    regens = [l for l in linhas[:fim] if len(l) >= 22 and l[3] == 'regen' and (carreira is None or l[2] == carreira)]
    erros, avisos, desconhecidos = [], [], 0
    for l in regens:
        rid, nome, pais, pa_csv, clube_csv = int(l[4]), l[5], l[7], int(l[9]), l[11]
        if l[17].startswith('(unknown'):
            desconhecidos += 1
        elif l[20] and int(l[20]) != pa_csv:
            erros.append('%s: PA %d, mas a ficha registrada (%s) tinha PA %s' % (nome, pa_csv, l[17], l[20]))
        if rid >= n:
            erros.append('%s: id %d fora do save' % (nome, rid))
            continue
        o = rid * 110
        if not T[79 * rid + 0xb] & 0x40:
            erros.append('%s (id %d): não é regen no save' % (nome, rid))
            continue
        comum = cn.get(struct.unpack_from('<i', S, o + 0xc)[0], '').strip()
        nome_save = comum or ('%s %s' % (pn.get(struct.unpack_from('<i', S, o + 4)[0], ''),
                                         sn.get(struct.unpack_from('<i', S, o + 8)[0], ''))).strip()
        if nome_save != nome:
            erros.append('%s (id %d): no save o nome é %s' % (nome, rid, nome_save))
        pais_save = nacao.get(struct.unpack_from('<i', S, o + 0x1a)[0], '?')
        if pais_save != pais:
            erros.append('%s: nação %s no CSV, %s no save' % (nome, pais, pais_save))
        pa_save = pa.get(struct.unpack_from('<i', S, o + 0x61)[0])
        if pa_save != pa_csv:
            erros.append('%s: PA %d no CSV, %s no save' % (nome, pa_csv, pa_save))
        c = struct.unpack_from('<i', S, o + 0x39)[0]
        clube_save = clube.get(c, '(no club)')
        if clube_save != clube_csv:
            avisos.append('%s: nasceu no %s, no save está no %s' % (nome, clube_csv, clube_save))
    print('%d regens conferidos; %d donos desconhecidos; %d erros; %d mudaram de clube depois' % (
        len(regens), desconhecidos, len(erros), len(avisos)))
    for e in erros[:30]:
        print('  ERRO', e)
    for a in avisos[:10]:
        print('  aviso', a)
    sys.exit(1 if erros else 0)


if __name__ == '__main__':
    main()
