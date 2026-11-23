//! Os jogadores cinzentos (`VIRTUAL_STAFF`, virtual_staff.cpp): o jogo completa o elenco por posição antes de cada partida
//! com pessoas virtuais (id ≥ [0xAE2364]); ver docs/JOGADORES_CINZENTOS.md do projeto CM0102. A original dá a eles um CA
//! pela reputação do clube que despenca abaixo de 5000 (R / 25 − 100: negativo abaixo de 2500), salta em 5000 nas seleções
//! e tira mais 30 a 70 do "jovem" (liga ativa) e de novo do reserva. Aqui o CA é o nível do próprio elenco.

use crate::{d, data_do_jogo, nome, registro, w};

/// grey_players_spread: o CA sai numa faixa dessa largura centrada no nível (10: do nível − 5 ao nível + 5).
pub static mut ESPALHAR: i32 = 10;
pub static mut REGISTRAR: bool = false;
/// grey_players_surprise: (chance %, bônus mínimo, bônus máximo). A surpresa sai do mesmo `rand` semeado por clube e
/// grupo, então o cinzento-surpresa é o mesmo a cada partida enquanto a vaga existir.
pub static mut SURPRESA: (i32, i32, i32) = (5, 15, 35);
/// grey_players_template = level: o molde dos atributos pelo nível (`molde_do_nivel`), chamado depois de
/// `create_virtual_player` pelo desvio de efetivar.rs.
pub static mut MOLDE: bool = false;

/// A `create_virtual_player` (0x90e0a0) sorteia como molde qualquer natural da posição do banco (as primeiras 30.000
/// fichas), de qualquer nível, copia os 70 bytes e troca só o id, o número, o CA, o PA e as reputações (+0..+0xe). Com o
/// CA no nível do clube, os atributos que não dependem do CA continuam os do molde: o cinzento de quinta divisão joga
/// como um de divisão superior (relato do fórum de 23/11). Aqui o molde é trocado por um natural da mesma posição (o
/// mesmo critério da original) com CA a até 8, 16 ou 32 do do cinzento; os primeiros 15 bytes ficam. O sorteio é
/// determinístico (o clube e o molde da original), então a mesma vaga recebe o mesmo molde de uma partida para a outra.
pub(crate) unsafe fn molde_do_nivel(clube: usize, grupo: u8, ficha: usize) {
    let alvo = w(ficha + 5) as i32;
    let n = (d(0xae236c) as usize).min(30000);
    let fichas = d(0xae23c4);
    if ficha == 0 || alvo <= 0 || n == 0 || fichas == 0 {
        return;
    }
    let b = |f: usize, o: usize| *((f + o) as *const i8);
    let natural = |f: usize| -> bool {
        let c = |o: usize| b(f, o) > 14;
        match grupo {
            0 => b(f, 0xf) == 20,
            1 => c(0x11) && c(0x18),
            2 => c(0x11) && c(0x17),
            3 => c(0x11) && c(0x19),
            4 => c(0x13) && c(0x18),
            5 => c(0x13) && c(0x19),
            6 => c(0x13) && c(0x17),
            7 => c(0x15),
            _ => false,
        }
    };
    let mut x: u32 = (d(clube) as u32).wrapping_mul(2654435761) ^ 0x9e3779b9;
    for o in 0xf..0x20 {
        x = (x ^ *((ficha + o) as *const u8) as u32).wrapping_mul(16777619);
    }
    for faixa in [8, 16, 32] {
        for _ in 0..3000 {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            let f = fichas + 0x46 * (x as usize % n);
            if f != ficha && natural(f) && (w(f + 5) as i32 - alvo).abs() <= faixa {
                core::ptr::copy_nonoverlapping((f + 0xf) as *const u8, (ficha + 0xf) as *mut u8, 0x46 - 0xf);
                if REGISTRAR && crate::estudo::ligado() {
                    registro::linha(&format!("grey template\t{}\t{}\t{}\tCA {}\ttemplate CA {}", data_do_jogo(), nome(clube), grupo, alvo, w(f + 5)));
                }
                return;
            }
        }
    }
}

/// O nível do elenco: a média do CA dos 16 melhores jogadores reais (+0xd7, 50 vagas; real = id < [0xAE2364], com ficha em
/// +0x61). Nada com menos de 5 reais.
pub(crate) unsafe fn nivel(clube: usize) -> Option<i32> {
    let pessoas = d(0xae2364);
    let mut cas: Vec<i32> = (0..50)
        .filter_map(|k| {
            let p = d(clube + 0xd7 + 4 * k);
            if p == 0 || d(p) >= pessoas || d(p + 0x61) == 0 {
                return None;
            }
            Some(w(d(p + 0x61) + 5) as i32).filter(|&ca| ca > 0) // CA negativo (cinzento efetivado da conta original) não conta
        })
        .collect();
    if cas.len() < 5 {
        return None;
    }
    cas.sort_unstable_by(|a, b| b.cmp(a));
    cas.truncate(16);
    Some(cas.iter().sum::<i32>() / cas.len() as i32)
}

/// No lugar do CA da original (o trecho de `instalar` chama com o clube, as marcas de jovem e de reserva e o CA dela).
/// O nível é o maior entre o do elenco e o da reputação, pela fórmula de cima da original (reputação acima de 5000) para
/// qualquer reputação: R × 0,016 + 20 no clube, R / 100 + 40 na seleção, sem o degrau e sem negativo. A reputação segura
/// o grande clube de liga não carregada, que no banco mínimo tem poucos reais (o Corinthians com média 95). O `rand` é o
/// da CRT, que a original semeia por clube e grupo antes de cada cinzento: o mesmo clube continua recebendo os mesmos.
unsafe extern "C" fn ca_do_cinzento(clube: usize, _jovem: i32, _reserva: i32, original: i32) -> i32 {
    let selecao: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x53b900usize);
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    let rep = w(clube + 0x80) as i32;
    let pela_rep = if selecao(clube) != 0 { rep / 100 + 40 } else { rep * 16 / 1000 + 20 };
    let (base, fonte) = match nivel(clube) {
        Some(n) if n >= pela_rep => (n, "squad"),
        _ => (pela_rep, "reputation"),
    };
    let mut ca = (base + if ESPALHAR > 0 { rand() % (ESPALHAR + 1) - ESPALHAR / 2 } else { 0 }).clamp(1, 200);
    let (chance, min, max) = SURPRESA;
    let surpresa = chance > 0 && rand() % 100 < chance;
    if surpresa {
        ca = (base + min + rand() % (max - min + 1)).clamp(1, 200);
    }
    if REGISTRAR {
        registro::linha(&(format!("grey\t{}\t{}\t{}\t{}\t{}\t{}\t{}", data_do_jogo(), nome(clube), rep, fonte, base, original as i16, ca) + if surpresa { "\tsurprise" } else { "" }));
    }
    ca
}

/// Troca as duas leituras de 0x90e6d4 (`mov edi, [esp+0x50]; mov eax, [esp+0x54]`, logo antes de gravar o CA em +5 e +7
/// da ficha) por um trecho que chama `ca_do_cinzento(ebx, [esp+0x58], [esp+0x5c], esi)`, põe o resultado no `esi`, refaz
/// as duas leituras e volta em 0x90e6dc.
pub unsafe fn instalar() -> bool {
    let t = crate::VirtualAlloc(core::ptr::null_mut(), 64, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return false;
    }
    let f = ca_do_cinzento as *const () as u32;
    // push esi; push [esp+0x60] (reserva); push [esp+0x60] (jovem); push ebx; mov eax, f
    let mut c: Vec<u8> = vec![0x56, 0xff, 0x74, 0x24, 0x60, 0xff, 0x74, 0x24, 0x60, 0x53, 0xb8];
    c.extend_from_slice(&f.to_le_bytes());
    // call eax; add esp, 0x10; mov esi, eax; mov edi, [esp+0x50]; mov eax, [esp+0x54]; jmp
    c.extend_from_slice(&[0xff, 0xd0, 0x83, 0xc4, 0x10, 0x8b, 0xf0, 0x8b, 0x7c, 0x24, 0x50, 0x8b, 0x44, 0x24, 0x54, 0xe9]);
    let fim = t as usize + c.len() + 4;
    c.extend_from_slice(&(0x90e6dcusize.wrapping_sub(fim) as i32).to_le_bytes());
    core::ptr::copy_nonoverlapping(c.as_ptr(), t, c.len());
    crate::desviar(0x90e6d4, t as usize);
    true
}
