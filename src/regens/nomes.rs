//! Os nomes das pessoas novas (regens, comissão, árbitros): `database_pick_first_name_for_nation` 0x539c60 e
//! `database_pick_second_name_for_nation` 0x53a1b0 (Database.cpp; as versões em Rust são `primeiro_nome_da_nacao` e
//! `segundo_nome_da_nacao` do `motor.rs` do projeto CM0102). A original só usa os nomes da própria nação quando ela tem
//! mais de 100; com 100 ou menos, empresta de outra nação do grupo de língua (tabela fixa no exe) ou sorteia na tabela do
//! mundo inteiro. No grupo de língua 0, o primeiro nome ainda sai da tabela dos SEGUNDOS nomes (defeito do original).
//!
//! `regen_names = own`: a nação usa os próprios nomes a partir de `regen_names_min` (padrão 20), do mesmo jeito que a
//! original usa acima de 100 (20 sorteios na faixa da nação, fica o de maior peso +0x3b; no primeiro nome, só os aceitos).
//! Abaixo do mínimo, segue a original, menos o defeito do grupo 0, que passa a sortear na tabela dos primeiros nomes.
//! Os nomes vêm do banco (first_names.dat, second_names.dat); nada é acrescentado às tabelas nem ao save.

pub static mut MINIMO: i32 = 20;
static mut PRIMEIRO_ORIGINAL: usize = 0;
static mut SEGUNDO_ORIGINAL: usize = 0;

/// rand() da CRT escalado para 0..n (o `sorteio_escalado` da original).
unsafe fn sorteio(n: i32) -> i32 {
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    ((rand() as i64 * n as i64) / 0x8000) as i32
}

/// O índice de nomes da nação (id em +0): [0xae2338] + id × 36; +0/+4 a primeira linha dos primeiros/segundos nomes,
/// +0x18/+0x1c quantos (montado por `database_build_nation_name_index` 0x539750).
unsafe fn indice(nacao: usize) -> usize {
    (*(0xae2338 as *const usize)).wrapping_add((*(nacao as *const i32) as usize).wrapping_mul(36))
}

/// Nome aceito como primeiro nome: vazio sim; uma letra só ou com ponto não (como a original).
unsafe fn aceito(nome: usize) -> bool {
    if *(nome as *const u8) == 0 {
        return true;
    }
    let s = core::ffi::CStr::from_ptr(nome as *const core::ffi::c_char).to_bytes();
    s.len() != 1 && !s.contains(&b'.')
}

/// 20 sorteios na faixa da nação; fica o de maior peso (+0x3b, com sinal), como a original acima de 100.
unsafe fn dos_proprios(tabela: usize, inicio: i32, quantos: i32, filtrar: bool) -> usize {
    let linha = |i: i32| (*(tabela as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    let (mut melhor, mut escolhido) = (-1i8, inicio);
    for _ in 0..20 {
        let i = sorteio(quantos).wrapping_add(inicio);
        let nome = linha(i);
        let peso = *((nome + 0x3b) as *const i8);
        if peso > melhor && (!filtrar || aceito(nome)) {
            melhor = peso;
            escolhido = i;
        }
    }
    linha(escolhido)
}

unsafe extern "C" fn primeiro_nome(nacao: usize) -> usize {
    if nacao != 0 {
        let e = indice(nacao);
        let quantos = *((e + 0x18) as *const i32);
        if quantos >= MINIMO && quantos <= 100 {
            return dos_proprios(0xae23f0, *(e as *const i32), quantos, true);
        }
        // o defeito do grupo de língua 0: a original devolve uma linha dos segundos nomes
        if quantos <= 100 && *((nacao + 0x77) as *const u8) == 0 {
            let total = (*(0xae2398 as *const i32)).min(0x7fff);
            return (*(0xae23f0 as *const usize)).wrapping_add((sorteio(total) as usize).wrapping_mul(60));
        }
    }
    let original: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(PRIMEIRO_ORIGINAL);
    original(nacao)
}

unsafe extern "C" fn segundo_nome(nacao: usize) -> usize {
    if nacao != 0 {
        let e = indice(nacao);
        let quantos = *((e + 0x1c) as *const i32);
        if quantos >= MINIMO && quantos <= 100 {
            return dos_proprios(0xae23f4, *((e + 4) as *const i32), quantos, false);
        }
    }
    let original: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(SEGUNDO_ORIGINAL);
    original(nacao)
}

/// Desvia as duas rotinas (os prólogos de 5 bytes vão para trampolins que voltam ao original).
pub unsafe fn instalar() -> bool {
    // cada desvio só é feito depois de o trampolim dele existir: um que falhe não deixa o outro sem volta
    let Some(a) = crate::origem::trampolim(0x539c60, &[0x83, 0xec, 0x14, 0x53, 0x55], primeiro_nome as *const () as usize) else { return false };
    PRIMEIRO_ORIGINAL = a;
    let Some(b) = crate::origem::trampolim(0x53a1b0, &[0x51, 0x53, 0x55, 0x56, 0x57], segundo_nome as *const () as usize) else { return false };
    SEGUNDO_ORIGINAL = b;
    true
}
