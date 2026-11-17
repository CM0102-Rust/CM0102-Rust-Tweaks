//! `grey_signing`: efetivar um jogador cinzento do clube do técnico humano (desenho em docs/EFETIVAR_CINZENTOS.md).
//! 1. No perfil de um cinzento do clube humano, o botão Action com o item "Efetivar" (o jogo não mostra o Action para
//!    cinzentos): um trecho em 0x874a58, logo depois do menu, na montagem do perfil 0x870700.
//! 2. O clique (`staff_screens_handle_profile_action` 0x888260, desviada na entrada): o id 0x50 efetiva; o resto vai
//!    para a original.
//! 3. A efetivação: uma pessoa real que sobra (uma adormecida da fornada ou, sem ela, a cabeça da fila de regens com
//!    uma ficha da reserva) vira regen pela rotina do próprio jogo (a chamada da fila, 0x7abac0), com a ficha do
//!    cinzento e sem a disputa dos clubes; depois recebe a identidade do cinzento (nomes, nascimento, nações e a ficha),
//!    a personalidade sorteada e a marca de regen, e assina com o clube humano (0x7a6f10): o salário padrão do jogo e
//!    um contrato de `grey_signing_months` meses (12). A negociação pela tela do jogo (16/11) foi tirada: nenhum
//!    efetivado aceitava a proposta.

use crate::{d, registro, sorteio, w};

static mut LIGADO: bool = false;
static mut MESES: i32 = 12;
/// grey_signing_wage: o salário do efetivado em % da mediana dos salários dos reais do elenco (100).
static mut SALARIO: i32 = 100;
/// grey_players_names = nickname: os cinzentos (de todos os clubes) ganham um apelido da nação (`nomes::apelidar`).
static mut APELIDO: bool = false;
/// Durante a efetivação, a disputa e a lista da nação (as duas chamadas de clube da 0x7abac0) devolvem 0.
static mut EFETIVANDO: bool = false;
static mut DISPUTA_ANTES: usize = 0;
static mut LISTA_ANTES: usize = 0;
static mut T_CLIQUE: usize = 0;
/// O id do item no menu Action (livre no perfil e fora da faixa da barra de menu, ≥ 0x3e8).
const ITEM: u16 = 0x50;

fn ler() {
    let caminho = std::env::current_exe().map(|x| x.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return };
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        unsafe {
            match k {
                "grey_signing" => LIGADO = v == "1" || v == "on",
                "grey_signing_months" => MESES = v.parse().unwrap_or(12).clamp(1, 60),
                "grey_players_names" => APELIDO = v == "nickname",
                "grey_signing_wage" => SALARIO = v.parse().unwrap_or(100).clamp(10, 300),
                _ => {}
            }
        }
    }
}

/// O técnico humano da vez (`[0xb74c26 + 0x300 × canal]`, canal = `[0xb77c7a]`): o clube dele (+0x39) e a seleção (+0x24).
unsafe fn times_humanos() -> [usize; 2] {
    let humano = d(0xb74c26 + 0x300 * w(0xb77c7a) as usize);
    if humano == 0 { [0, 0] } else { [d(humano + 0x39), d(humano + 0x24)] }
}

/// O cinzento `p` está no elenco (+0xd7, 50 vagas) do clube ou da seleção do técnico humano? Devolve a vaga e o time.
unsafe fn vaga_no_humano(p: usize) -> Option<(usize, usize)> {
    if p == 0 || d(p) < d(0xae2364) {
        return None;
    }
    times_humanos().into_iter().filter(|&c| c != 0).find_map(|c| (0..50).map(|k| c + 0xd7 + 4 * k).find(|&a| d(a) == p).map(|a| (a, c)))
}

fn em_portugues() -> bool {
    matches!(unsafe { d(0x9eb2dc) } as i32, 9 | 17)
}

/// O trecho em 0x874a58 chama com o bloco do `pushad` (esp, de cima para baixo: edi, esi, ebp, esp, ebx, edx, ecx, eax);
/// o quadro da montagem fica logo acima (+0x20): a pessoa em +0x20, as cores em +0x14 e +0x18.
unsafe extern "C" fn menu(regs: usize) {
    let quadro = regs + 32;
    let (ebp, staff) = (d(regs + 8) as i32, d(quadro + 0x20));
    if ebp != -1 || vaga_no_humano(staff).is_none() {
        return;
    }
    let sprintf: unsafe extern "C" fn(*mut u8, *const u8) -> usize = core::mem::transmute(0x66f4e0usize);
    let botao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, *const u8, usize, usize) -> usize =
        core::mem::transmute(0x804030usize);
    let area: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(0x55fba0usize);
    #[allow(clippy::type_complexity)]
    let item: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(0x55f990usize);
    let mut rotulo = [0u8; 64];
    sprintf(rotulo.as_mut_ptr(), 0xa8560c as *const u8); // "Action", pela tradução do jogo
    let b = botao(0xb74450, 0x294, 4, 0x311, 0x18, 0x30, d(quadro + 0x18), 0xc, 1, d(quadro + 0x14), rotulo.as_ptr(), 0, 0);
    let display = d(0xb74c4c + 0x300 * w(0xb77c7a) as usize);
    let a = area(display, 0, 0, 0, 0, 0, 0, 0, 0, 0x130, w(0xae31aa) as u16 as usize, b & 0xffff);
    if a as i32 == -1 {
        return;
    }
    let texto: &[u8] = if em_portugues() { b"Efetivar\0" } else { b"Make permanent\0" };
    sprintf(0xde1f64 as *mut u8, texto.as_ptr());
    item(display, 2, 0, 0, 0, 0, 0, 0, 0x10, w(0xae31aa) as u16 as usize, w(0xaebdec) as u16 as usize, 1, 1, w(0xaebdea) as u16 as usize,
        0xde1f64, 0, ITEM as usize, 0, a & 0xffff);
}

#[unsafe(naked)]
unsafe extern "C" fn trecho_do_menu() {
    // pushad; push esp (o bloco); call menu; add esp, 4; popad; as 3 instruções trocadas; volta em 0x874a5d
    core::arch::naked_asm!("pushad", "push esp", "call {m}", "add esp, 4", "popad", "xor esi, esi", "push esi", "push 4",
        "push 0x874a5d", "ret", m = sym menu);
}

/// { 0x888260 } (cdecl, short): o clique num item do perfil. O id do item: `Display + 0xba966 + idx × 0x18c` (ou
/// `[0xdd6bde]` com idx −1 ou id 0).
unsafe extern "C" fn clique(idx: i32) -> i32 {
    let idx16 = idx as i16;
    let display = d(0xb74c4c + 0x300 * w(0xb77c7a) as usize);
    let mut id = if idx16 == -1 { 0 } else { w(display + 0xba966 + idx16 as usize * 0x18c) as u16 };
    if idx16 == -1 || id == 0 {
        id = w(0xdd6bde) as u16;
    }
    if id == ITEM {
        let parametro: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7ff010usize);
        let cinzento = parametro(0xb74450, 0);
        efetivar(cinzento);
        return -4;
    }
    let original: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(T_CLIQUE);
    original(idx)
}

/// As duas chamadas de clube da 0x7abac0 (a disputa em 0x7acea2, a lista da nação em 0x7acf4f; thiscall, ret 4): 0
/// durante a efetivação, para o efetivado nascer sem clube.
unsafe extern "thiscall" fn disputa(this: usize, p: usize) -> usize {
    if EFETIVANDO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(DISPUTA_ANTES);
    f(this, p)
}
unsafe extern "thiscall" fn lista(this: usize, p: usize) -> usize {
    if EFETIVANDO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(LISTA_ANTES);
    f(this, p)
}

/// O id da nação (ponteiro da tabela de nações, [0xae23a8], 0x122 bytes cada).
unsafe fn id_da_nacao(n: usize) -> usize {
    if n == 0 { 0 } else { (n - d(0xae23a8)) / 0x122 }
}

/// Efetiva o cinzento `g` do clube humano; devolve a pessoa real.
unsafe fn efetivar(g: usize) -> Option<usize> {
    let (vaga, clube) = vaga_no_humano(g)?;
    // o time do cinzento: o bit 0 do +0x5e (1 principal, 0 reservas), como o `get_squad_placement` 0x4cbb60 lê
    let no_principal = *((g + 0x5e) as *const u8) & 1 == 1;
    let regen = d(0xb7434c);
    let ficha_g = d(g + 0x61);
    if regen == 0 || ficha_g == 0 {
        return None;
    }
    let (pessoas, fichas) = (d(0xae23bc), d(0xae23c4));
    // a pessoa: uma adormecida da fornada (com a própria ficha) ou a cabeça da fila de regens (com a ficha da frente da
    // reserva, o que custa um regen)
    let adormecidas = crate::fornada::adormecidos(regen);
    let (p, da_fila) = match adormecidas.first() {
        Some(&p) => (p, false),
        None if d(regen + 0x14) > 0 && d(regen + 0x40) > 0 => (pessoas + 110 * d(d(regen + 0x10)), true),
        None => {
            registro::linha(&format!("grey signing\t{}\tno spare person: no dormant person and the regen queue or record pool is empty", crate::data_do_jogo()));
            return None;
        }
    };
    let ficha = if da_fila { fichas + 0x46 * d(d(regen + 0x3c)) } else { d(p + 0x61) };
    let id_ficha = d(ficha);
    core::ptr::copy_nonoverlapping(ficha_g as *const u8, ficha as *mut u8, 70);
    *(ficha as *mut usize) = id_ficha;
    if !da_fila {
        crate::fornada::por_na_frente_da_reserva(regen, id_ficha, id_da_nacao(d(g + 0x1a)));
    }
    // o regen, pela chamada da fila (a 0x7abac0, ou o regen_origins por cima dela), sem a disputa
    let alvo = (core::ptr::read_unaligned(0x7a6650 as *const i32) as usize).wrapping_add(0x7a6654);
    let criar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(alvo);
    let antes = d(regen + 0x40);
    EFETIVANDO = true;
    criar(regen, p);
    EFETIVANDO = false;
    if da_fila {
        crate::fornada::tirar(regen, 0x10, 12, d(p));
    }
    if d(regen + 0x40) >= antes || d(p + 0x39) != 0 || d(p + 0x61) != ficha {
        registro::linha(&format!("grey signing\t{}\tthe regen routine didn't take person {} (club {:#x}); nothing signed", crate::data_do_jogo(), d(p), d(p + 0x39)));
        return None;
    }
    // a identidade do cinzento: nomes, nascimento, idade, nações (+4..+0x22) e a ficha inteira (a 0x7abac0 mexe no CA)
    core::ptr::copy_nonoverlapping((g + 4) as *const u8, (p + 4) as *mut u8, 0x1e);
    core::ptr::copy_nonoverlapping(ficha_g as *const u8, ficha as *mut u8, 70);
    *(ficha as *mut usize) = id_ficha;
    if APELIDO {
        crate::nomes::marcar_apelido(p);
    }
    nivelar(p, ficha, clube);
    // a personalidade sorteada (+0x56..+0x5d; o cinzento tem 10 em tudo) e a marca de regen
    for o in 0x56..=0x5d {
        *((p + o) as *mut u8) = (5 + sorteio(16)) as u8;
    }
    *crate::fornada::registro79(d(p)) |= 0x40; // o +0xb do registro de 79 bytes
    if clube == times_humanos()[1] {
        return convocar(p, g, vaga, clube, ficha, da_fila);
    }
    // o clube: a vaga do cinzento fica livre e a assinatura do regen põe o efetivado no elenco
    core::ptr::write_unaligned(vaga as *mut usize, 0);
    let assinar: unsafe extern "stdcall" fn(usize, usize) -> u32 = core::mem::transmute(0x7a6f10usize);
    if assinar(p, clube) & 0xff == 0 {
        core::ptr::write_unaligned(vaga as *mut usize, g);
        registro::linha(&format!("grey signing\t{}\tperson {} could not join the club (squad full)", crate::data_do_jogo(), d(p)));
        return None;
    }
    // o time: a assinatura do regen põe o efetivado nas reservas do clube humano; quem era do principal sobe
    // (`CONTRACT_MANAGER::move_to_first_team` 0x4cc0b0). O gerente dos contratos é [0xae1050] (o vetor, [[0xae1050]]).
    let cm = d(0xae1050);
    let mediana = mediana_dos_salarios(cm, clube, p);
    if no_principal {
        let subir: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cc0b0usize);
        subir(cm, p, clube);
    }
    // o contrato: o fim em hoje + grey_signing_months
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contrato(cm, p);
    if k != 0 {
        let (mut dia, mut ano) = (w(0xae2c90) as i32 + MESES * 365 / 12, w(0xae2c92) as i32);
        while dia > 364 {
            dia -= 365;
            ano += 1;
        }
        core::ptr::write_unaligned((k + 0x2d) as *mut u16, dia as u16);
        core::ptr::write_unaligned((k + 0x2f) as *mut u16, ano as u16);
        // o tipo do contrato (os 6 bits baixos do +0x35): a assinatura do regen põe 1 (mensal: "Expira: A curto prazo",
        // preço de transferência 0) no clube humano; 2 é o contrato normal, com a data de fim. O 0x40 (reservas) e o 0x80 ficam.
        *((k + 0x35) as *mut u8) = (*((k + 0x35) as *const u8) & 0xc0) | 2;
        // o salário: a assinatura do regen dá o padrão do jogo (250 no Friburguense, onde o elenco ganha 100); passa a ser
        // a mediana do elenco × grey_signing_wage %, com o mínimo do jogo (55)
        if let Some(m) = mediana {
            core::ptr::write_unaligned((k + 0xc) as *mut i32, ((m as i64 * SALARIO as i64 / 100) as i32).max(55));
        }
    }
    registro::linha(&format!("grey signing\t{}\tperson {} made permanent at {} (CA {}, PA {}; from {})", crate::data_do_jogo(), d(p),
        crate::nome(clube), w(ficha + 5), w(ficha + 7), if da_fila { "the regen queue" } else { "a dormant person" }));
    Some(p)
}

/// Na seleção (o técnico humano de uma seleção, `manage_any_team`): o efetivado fica sem clube e convocado, no lugar do
/// cinzento na lista (+0xd7). O vínculo com a seleção (+0x24 a seleção, +0x28 o cargo, 0x0b para jogador, e as datas até
/// +0x38) vem de um convocado real, ou, sem nenhum, do próprio cinzento, que o jogo cria convocado. Sem clube e convocado,
/// a aposentadoria de quem fica um ano sem clube e sem seleção (0x7a8050) não o pega; qualquer clube pode contratá-lo.
unsafe fn convocar(p: usize, g: usize, vaga: usize, selecao: usize, ficha: usize, da_fila: bool) -> Option<usize> {
    let reais = d(0xae2364);
    let modelo = (0..50).map(|k| d(selecao + 0xd7 + 4 * k)).find(|&q| q != 0 && d(q) < reais && d(q + 0x24) == selecao).unwrap_or(g);
    core::ptr::copy_nonoverlapping((modelo + 0x24) as *const u8, (p + 0x24) as *mut u8, 0x15);
    *((p + 0x5e) as *mut u8) |= 1;
    core::ptr::write_unaligned(vaga as *mut usize, p);
    registro::linha(&format!("grey signing\t{}\tperson {} made permanent and called up by {} (no club; CA {}, PA {}; from {})", crate::data_do_jogo(), d(p),
        crate::nome(selecao), w(ficha + 5), w(ficha + 7), if da_fila { "the regen queue" } else { "a dormant person" }));
    Some(p)
}

/// A mediana dos salários (+0xc do contrato) dos jogadores reais do elenco do clube que não são regens, sem `fora`;
/// nada sem nenhum (aí fica o salário padrão do jogo).
unsafe fn mediana_dos_salarios(cm: usize, clube: usize, fora: usize) -> Option<i32> {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let reais = d(0xae2364);
    let mut s: Vec<i32> = (0..50)
        .map(|k| d(clube + 0xd7 + 4 * k))
        // sem os regens (os efetivados têm a marca e ganhavam 250: puxariam a mediana para cima)
        .filter(|&q| q != 0 && q != fora && d(q) < reais && d(q + 0x39) == clube && *crate::fornada::registro79(d(q)) & 0x40 == 0)
        .filter_map(|q| {
            let c = contrato(cm, q);
            (c != 0).then(|| core::ptr::read_unaligned((c + 0xc) as *const i32)).filter(|&x| x > 0)
        })
        .collect();
    s.sort_unstable();
    (!s.is_empty()).then(|| s[s.len() / 2])
}

/// O CA e o PA do efetivado. Com `grey_players = original`, o cinzento de clube pequeno nasce com CA negativo (a conta
/// do jogo pela reputação): o CA passa a ser ao menos o nível do clube, a mesma conta de `grey_players = squad` (a
/// média dos 16 melhores reais, ou a reputação: R × 0,016 + 20), ± 5. O PA deixa espaço pela idade (até 18: +30 a +50;
/// 19–21: +15 a +35; 22–24: +5 a +20; 25 ou mais: até +8). As reputações do jogador (+9 atual, +0xb em casa, +0xd
/// mundial) saem do CA como a 0x7abac0 faz com o regen.
unsafe fn nivelar(p: usize, ficha: usize, clube: usize) {
    let rep = w(clube + 0x80) as i32;
    let base = crate::cinzentos::nivel(clube).unwrap_or(0).max(rep * 16 / 1000 + 20);
    let ca = (w(ficha + 5) as i32).max(base + sorteio(11) - 5).clamp(1, 200);
    let nasc = w(p + 0x18) as i32;
    let idade = if nasc > 1900 { w(0xae2c92) as i32 - nasc } else { *((p + 0x18) as *const u8) as i32 };
    let folga = match idade {
        ..=18 => 30 + sorteio(21),
        19..=21 => 15 + sorteio(21),
        22..=24 => 5 + sorteio(16),
        _ => sorteio(9),
    };
    let pa = (w(ficha + 7) as i32).max(ca + folga).clamp(ca, 200);
    core::ptr::write_unaligned((ficha + 5) as *mut i16, ca as i16);
    core::ptr::write_unaligned((ficha + 7) as *mut i16, pa as i16);
    let atual = 0x7d + sorteio(0x2ee) + ca * 15;
    core::ptr::write_unaligned((ficha + 9) as *mut i16, atual.min(10000) as i16);
    core::ptr::write_unaligned((ficha + 0xb) as *mut i16, (atual + sorteio(0x32)).min(10000) as i16);
    core::ptr::write_unaligned((ficha + 0xd) as *mut i16, 1);
}

static mut T_CINZENTO: usize = 0;

/// { 0x90e0a0 } `create_virtual_player(clube, _, id da pessoa, id da ficha, ...)` (thiscall com 7 na pilha, ret 0x1c): o jogo sorteia
/// os nomes do cinzento com o `rand` semeado por clube e grupo, então o cinzento que repõe a vaga de um efetivado nasce
/// com o mesmo nome dele. Depois da original: se um jogador real do elenco tem o mesmo nome e sobrenome, novos nomes da
/// nação do clube (+0x53; as mesmas rotinas da original, 0x539c60 e 0x53a1b0), até 10 tentativas.
unsafe extern "thiscall" fn cinzento(this: usize, clube: usize, a2: usize, id: usize, a4: usize, a5: usize, a6: usize, a7: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize) = core::mem::transmute(T_CINZENTO);
    f(this, clube, a2, id, a4, a5, a6, a7);
    let nacao = d(clube + 0x53);
    if clube < 0x10000 || nacao == 0 {
        return;
    }
    let p = d(0xae23bc) + 110 * id;
    if APELIDO {
        crate::nomes::apelidar(p, nacao);
    }
    let reais = d(0xae2364);
    let repetido = |p: usize| (0..50).map(|k| d(clube + 0xd7 + 4 * k)).any(|q| q != 0 && q != p && d(q) < reais && d(q + 4) == d(p + 4) && d(q + 8) == d(p + 8));
    let nome: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    let sobrenome: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a1b0usize);
    let mut n = 0;
    while repetido(p) && n < 10 {
        *((p + 4) as *mut usize) = nome(nacao);
        *((p + 8) as *mut usize) = sobrenome(nacao);
        if APELIDO {
            crate::nomes::apelidar(p, nacao);
        }
        n += 1;
    }
}

/// O `call rel32` em `em`: o alvo atual, se for um `call`.
unsafe fn alvo_do_call(em: usize) -> Option<usize> {
    (*(em as *const u8) == 0xe8).then(|| (core::ptr::read_unaligned((em + 1) as *const i32) as usize).wrapping_add(em + 5))
}

/// Depois da fornada (que pode já ter trocado as duas chamadas de clube: encadeia nelas).
pub(crate) unsafe fn instalar() {
    ler();
    // o desvio da criação do cinzento serve às duas chaves: o nome que não repete o do efetivado e o apelido
    if LIGADO || APELIDO {
        match crate::origem::trampolim(0x90e0a0, &[0x6a, 0xff, 0x68, 0xa0, 0x55, 0x96, 0x00], cinzento as *const () as usize) {
            Some(t) => T_CINZENTO = t,
            None => {
                APELIDO = false;
                registro::linha("grey players: create_virtual_player at 0x90e0a0 differs from 3.9.68; no nicknames, and a new grey player may repeat a signed one's name");
            }
        }
    }
    if APELIDO {
        registro::linha("grey players: nicknames (a one-word nickname of the club's nation that no real person uses, and one-word names)");
    }
    if !LIGADO {
        return;
    }
    let menu_ok = core::slice::from_raw_parts(0x874a58 as *const u8, 5) == [0x33, 0xf6, 0x56, 0x6a, 0x04];
    let (Some(da), Some(li)) = (alvo_do_call(0x7acea2), alvo_do_call(0x7acf4f)) else {
        return registro::linha("grey signing: the regen routine's club calls (0x7acea2, 0x7acf4f) differ from 3.9.68; off");
    };
    if !menu_ok {
        return registro::linha("grey signing: the profile build at 0x874a58 differs from 3.9.68; off");
    }
    let Some(t) = crate::origem::trampolim(0x888260, &[0x6a, 0xff, 0x68, 0x9b, 0x2f, 0x96, 0x00], clique as *const () as usize) else {
        return registro::linha("grey signing: the profile click handler at 0x888260 differs from 3.9.68; off");
    };
    T_CLIQUE = t;
    DISPUTA_ANTES = da;
    LISTA_ANTES = li;
    crate::fornada::trocar_call(0x7acea2, disputa as *const () as usize);
    crate::fornada::trocar_call(0x7acf4f, lista as *const () as usize);
    crate::desviar(0x874a58, trecho_do_menu as *const () as usize);
    registro::linha(&format!("grey signing: on (the human club's grey players get a Make permanent item: a real player on a {}-month contract at {}% of the squad's median wage)", { MESES }, { SALARIO }));
}
