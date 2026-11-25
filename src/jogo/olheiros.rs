//! `scout_reputation` (0 a 100; 100 = o original): quanto da fama do jogador entra no julgamento dos olheiros do clube
//! humano (relato do fórum de 25/11: depois de algumas temporadas quase nenhum relatório de 5 estrelas, embora haja
//! regens de PA perto de 200).
//!
//! `match_man_judge_player_ability` (0x6e86d0, cdecl, 5 argumentos: quem julga, o jogador, o clube, o modo, a
//! reputação ou −1) estima o CA (modo 2) ou o PA do jogador como `((valor + ruído) × p + r × (20 − p)) / 20`, com `p`
//! = julgamento do olheiro / 2 + 10 (de 10 a 20) e `r` = a reputação do jogador vista por quem julga / 50 (no PA, menos
//! (idade − 23)², ou (idade − 26)² no goleiro). Um olheiro de julgamento 10 vê 75% de habilidade e 25% de fama: o
//! regen bom e desconhecido sai muito abaixo do que é. O relatório (`PLAYER_SEARCH::report_on_player` 0x7b62a0) dá as
//! estrelas por essa estimativa, pela nota de `match_man_score_player_for_role` 0x6e2f20, que chama o julgamento para o
//! CA e para o PA. Com a chave abaixo de 100, quando o jogo pede a reputação (o 5º argumento −1) e o clube é de um
//! técnico humano, a DLL passa no lugar dela a mistura `k × reputação + (1 − k) × valor × 50`: com 0 o termo da fama
//! vira o próprio valor, e sobra só o ruído, que já depende do julgamento do olheiro. A IA (mercado, listas de procura)
//! continua julgando como no original.

use super::*;

static mut FAMA: i32 = 100;
static mut T_JULGAR: usize = 0;

unsafe fn clube_humano(c: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    c != 0 && f(c, 0, 1, 0, 0) != 0
}

unsafe extern "C" fn julgar(quem: usize, jogador: usize, clube: usize, modo: u32, reputacao: i32) -> i32 {
    let original: unsafe extern "C" fn(usize, usize, usize, u32, i32) -> i32 = core::mem::transmute(T_JULGAR);
    let r16 = reputacao as i16;
    let ficha = if jogador != 0 { d(jogador + 0x61) } else { 0 };
    if r16 >= 0 || ficha == 0 || !clube_humano(clube) {
        return original(quem, jogador, clube, modo, reputacao);
    }
    // a reputação como a original a pede (0x5407e0: quem julga, ou o técnico do clube, como observador)
    let observador = if quem != 0 { quem } else { d(clube + 0xcf) };
    let vista: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5407e0usize);
    let r = vista(jogador, observador, 0) as i16 as i32;
    let valor = w(ficha + if modo as u8 == 2 { 5 } else { 7 }) as i32;
    let mistura = (FAMA * r + (100 - FAMA) * valor * 50) / 100;
    original(quem, jogador, clube, modo, (reputacao as u32 & 0xffff_0000) as i32 | (mistura.clamp(0, 10000) as u16 as i32))
}

pub(crate) unsafe fn instalar() {
    let caminho = std::env::current_exe().map(|p| p.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return };
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        if let Some((k, v)) = linha.split_once('=') {
            if k.trim() == "scout_reputation" {
                FAMA = v.trim().parse::<i32>().unwrap_or(100).clamp(0, 100);
            }
        }
    }
    if FAMA >= 100 {
        return;
    }
    match crate::origem::trampolim(0x6e86d0, &[0x51, 0x8b, 0x15, 0x20, 0x71, 0xdd, 0x00], julgar as *const () as usize) {
        Some(t) => {
            T_JULGAR = t;
            registro::linha(&format!("scouts: your scouts judge with {}% of a player's reputation (100 = the original), the rest by his ability", { FAMA }));
        }
        None => registro::linha("scouts: match_man_judge_player_ability at 0x6e86d0 differs from 3.9.68; off"),
    }
}
