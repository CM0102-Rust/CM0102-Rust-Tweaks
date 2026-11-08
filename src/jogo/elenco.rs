//! `squad_guard` (nosso padrão: ligado com tweaks.cfg; `squad_guard = off` desliga): a mesma pessoa nunca duas vezes no
//! elenco de um clube (as 50 vagas de clube+0xd7). As duas rotinas do jogo que põem alguém num elenco,
//! `database_add_staff_to_club` (0x53bd10, cdecl de 6) e `player_regen_sign_for_club` (0x7a6f10, stdcall de 2), só
//! procuram a primeira vaga vazia; a que tira (0x53c190) apaga só a primeira cópia. Com um caminho que põe de novo quem já
//! está (a troca entre o time principal e o B com jogador emprestado, 0x4cbd20, chamada pela avaliação de elenco
//! 0x83cb40 a cada 1 a 3 meses), as cópias se acumulam até a escalação automática (0x6dbc50) pôr a mesma pessoa em duas
//! posições: a caixa "match_man.cpp 1225". A proteção:
//! - depois de cada uma das duas rotinas, apaga as cópias repetidas da pessoa naquele clube (a rotina faz todo o resto);
//! - antes de cada escalação automática, limpa o elenco do clube;
//! - ao carregar um save, limpa todos os elencos (os saves que já têm cópias).
//! Cada limpeza vai para o tweaks.log com o clube, a pessoa e quem chamou.
//! ponytail: a vaga apagada fica vazia no meio do elenco, como a que o jogo deixa ao dispensar alguém.

use super::*;

static mut LIGADO: bool = false;
static mut T_ADD: usize = 0;
static mut T_REGEN: usize = 0;
/// O endereço de retorno da última chamada interceptada (quem pôs a pessoa no elenco).
static mut QUEM: usize = 0;

pub(crate) fn ligado() -> bool {
    unsafe { LIGADO }
}

fn ler() -> bool {
    let caminho = std::env::current_exe().map(|x| x.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return false };
    let mut ligado = true; // o nosso padrão
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        if let Some((k, v)) = linha.split_once('=') {
            if k.trim() == "squad_guard" {
                ligado = v.trim() != "off" && v.trim() != "0";
            }
        }
    }
    ligado
}

/// Um ponteiro de pessoa de verdade: dentro da tabela das pessoas ([0xae23bc], 110 bytes cada, [0xae2364] pessoas), no
/// começo de um registro (durante a carga do banco, os elencos ainda têm ids e −1).
pub(crate) unsafe fn pessoa_valida(p: usize) -> bool {
    let (base, n) = (d(0xae23bc), d(0xae2364));
    base != 0 && n < 2_000_000 && p >= base && p < base + 110 * n && (p - base) % 110 == 0
}

/// Apaga as cópias repetidas de `pessoa` no elenco de `clube` (fica a primeira); quantas apagou.
unsafe fn limpar(clube: usize, pessoa: usize, onde: &str) -> usize {
    if clube < 0x10000 || !pessoa_valida(pessoa) {
        return 0;
    }
    let mut achou = false;
    let mut apagou = 0;
    for j in 0..50 {
        let a = clube + 0xd7 + 4 * j;
        if d(a) == pessoa {
            if achou {
                core::ptr::write_unaligned(a as *mut usize, 0);
                apagou += 1;
            }
            achou = true;
        }
    }
    if apagou > 0 {
        registro::linha(&format!("squad guard: removed {} repeated cop{} of person {} from club {} {} ({}, called from {:#x}, {})",
            apagou, if apagou == 1 { "y" } else { "ies" }, d(pessoa), d(clube), nome(clube), onde, QUEM.wrapping_sub(5), data_do_jogo()));
    }
    apagou
}

/// Limpa o elenco inteiro de um clube.
pub(crate) unsafe fn limpar_clube(clube: usize, onde: &str) {
    if !LIGADO || clube < 0x10000 {
        return;
    }
    for j in 0..50 {
        let p = d(clube + 0xd7 + 4 * j);
        if p != 0 {
            limpar(clube, p, onde);
        }
    }
}

/// Depois da carga de um save: todos os elencos.
pub(crate) unsafe fn carregou() {
    if !LIGADO {
        return;
    }
    let (base, n) = (d(0xae23b4), d(0xae235c));
    if base == 0 || n > 100_000 {
        return;
    }
    for k in 0..n {
        limpar_clube(base + 0x245 * k, "save loaded");
    }
}

unsafe extern "C" fn add_protegida(clube: usize, pessoa: usize, cargo: u32, dados: usize, gravar: u32, reduzir: u32) -> u32 {
    let original: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(T_ADD);
    let r = original(clube, pessoa, cargo, dados, gravar, reduzir);
    limpar(clube, pessoa, "after 0x53bd10");
    r
}

unsafe extern "stdcall" fn regen_protegida(pessoa: usize, clube: usize) -> u8 {
    let original: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(T_REGEN);
    let r = original(pessoa, clube);
    limpar(clube, pessoa, "after 0x7a6f10");
    r
}

#[unsafe(naked)]
unsafe extern "C" fn entrada_add() {
    // guarda quem chamou e segue para a versão protegida com a pilha intacta
    core::arch::naked_asm!("mov eax, [esp]", "mov [{q}], eax", "jmp {f}", q = sym QUEM, f = sym add_protegida);
}

#[unsafe(naked)]
unsafe extern "C" fn entrada_regen() {
    core::arch::naked_asm!("mov eax, [esp]", "mov [{q}], eax", "jmp {f}", q = sym QUEM, f = sym regen_protegida);
}

/// A escalação automática (0x6dbc50, thiscall(comissão, outro, modo)) com o elenco do clube limpo antes.
unsafe extern "thiscall" fn escalar(t: usize, outro: usize, modo: u32) {
    let original: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6dbc50usize);
    QUEM = 0x6dbc55;
    limpar_clube(d(t + 0x2f), "before the team selection");
    original(t, outro, modo)
}

pub(crate) unsafe fn instalar() {
    if !ler() {
        return;
    }
    let add = crate::origem::trampolim(0x53bd10, &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00], entrada_add as *const () as usize);
    let regen = crate::origem::trampolim(0x7a6f10, &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00], entrada_regen as *const () as usize);
    let (Some(a), Some(r)) = (add, regen) else {
        registro::linha("squad guard: this exe differs from 3.9.68 where people enter a squad; nothing was changed");
        return;
    };
    T_ADD = a;
    T_REGEN = r;
    LIGADO = true;
    // a escalação: as 5 chamadas, se ainda vão para a original (o estudo, quando ligado, chama limpar_clube ele mesmo)
    let sitios = [0x6db8fe, 0x6db91b, 0x6db973, 0x6db98d, 0x6dba73];
    let chama = |de: usize| *(de as *const u8) == 0xe8 && (de + 5).wrapping_add(core::ptr::read_unaligned((de + 1) as *const i32) as usize) == 0x6dbc50;
    if !crate::estudo::ligado() && sitios.iter().all(|&de| chama(de)) {
        for de in sitios {
            chamar(de, escalar as *const () as usize);
        }
    }
    if !crate::origem::ganchos_do_save() {
        registro::linha("squad guard: the save loading routine differs from 3.9.68; saves are not cleaned when loaded");
    }
    registro::linha("squad guard: on (a person is never kept twice in a club's squad; checked after every squad entry, before every team selection and when a save is loaded)");
}
