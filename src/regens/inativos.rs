//! A aposentadoria dos inativos (`retire_idle_players`).

use super::*;

/// Está na lista de observados de algum clube (0x842080 do gerente de listas em [[0xde1f54] + 0x856])?
pub(crate) unsafe fn observado(pessoa: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
    f(d(d(0xde1f54) + 0x856), pessoa, 0) != 0
}

/// { 0x7a8050 } (stdcall, ret 8): `player_regen_check_unemployed_retirement(pessoa, saida)`, com `retire_idle_players`.
/// A original (a mesma cópia em Rust do projeto CM0102, comparada sem divergência): o observado não se aposenta; o
/// técnico (+0x69) sem clube e sem seleção, com +0xa < 50, em 1 de 5; o jogador (+0x61) sem clube e sem seleção,
/// desempregado há mais de 365 dias, com sorteio(reputação atual +0xb) < 750; sem nenhum dos dois registros, sempre.
/// O que muda, só no jogador:
/// - abaixo de `proteger_jovens` anos (+0x18), não se aposenta por esta regra;
/// - com `inativo_idade` anos ou mais e desempregado há mais de `inativo_dias` dias, aposenta com sorteio(reputação) <
///   `inativo_rep` + `inativo_rep_por_ano` × (idade − `inativo_idade`); os outros seguem a regra original.
/// Aposenta: a saída (26 bytes) recebe a pessoa, o motivo 0x10, o clube (−1 sem) e a data de hoje, e devolve 1.
pub(crate) unsafe extern "stdcall" fn aposentadoria_do_desempregado(pessoa: usize, saida: usize) -> u8 {
    let r = REGRAS;
    if observado(pessoa) {
        return 0;
    }
    let tecnico = d(pessoa + 0x69);
    let sem_registro = tecnico == 0 && d(pessoa + 0x61) == 0;
    let aposenta = if tecnico != 0 {
        if d(pessoa + 0x39) != 0 || d(pessoa + 0x24) != 0 {
            return 0;
        }
        w(tecnico + 0xa) < 50 && sorteio(5) == 0 // o técnico fraco, como na original
    } else if d(pessoa + 0x61) == 0 {
        false
    } else if d(pessoa + 0x39) != 0 || d(pessoa + 0x24) != 0 {
        false
    } else {
        let idade = b(pessoa + 0x18) as i32;
        let anos = (w(0xae2c92) as u16 as i32) - (w(pessoa + 0x40) as u16 as i32);
        let dias = anos * 365 - w(pessoa + 0x3e) as i32 + w(0xae2c90) as i32;
        let rep = w(d(pessoa + 0x61) + 0xb) as i32;
        if r.proteger_jovens > 0 && idade < r.proteger_jovens {
            false
        } else if idade >= r.inativo_idade && dias > r.inativo_dias {
            let limiar = r.inativo_rep + r.inativo_rep_por_ano * (idade - r.inativo_idade) as f64;
            let sai = rep <= 0 || (sorteio(rep) as f64) < limiar;
            if sai && r.registrar {
                APOSENTADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                registro::linha(&format!("idle\t{}\t{}\t{}\t{}", data_do_jogo(), idade, rep, dias));
            }
            sai
        } else {
            dias > 365 && sorteio(rep) < 750 // a regra original
        }
    };
    if !aposenta && !sem_registro {
        return 0;
    }
    core::ptr::write_unaligned(saida as *mut usize, d(pessoa));
    *((saida + 0x18) as *mut u8) = 0x10; // o motivo: desempregado
    *((saida + 0x19) as *mut u8) = 0;
    let clube = d(pessoa + 0x39);
    core::ptr::write_unaligned((saida + 4) as *mut u32, if clube != 0 { d(clube) as u32 } else { u32::MAX });
    for o in [8usize, 0x10] {
        core::ptr::write_unaligned((saida + o) as *mut u16, w(0xae2c90) as u16);
        core::ptr::write_unaligned((saida + o + 2) as *mut u16, w(0xae2c92) as u16);
    }
    1
}
