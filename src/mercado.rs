//! Mercado (1.5): os empréstimos e a verba. As chaves do `tweaks.cfg` (docs/EMPRESTIMOS.md §5): `loans` (on/off) e as
//! `loan_*`, `budget_caps`. Sem tweaks.cfg nada muda; com ele, a chave ausente é a nossa regra (08/11, Lucas).
//! Nas carreiras de medição (`TWEAKS_ESTUDO=1`), `TWEAKS_MERCADO` no ambiente ("chave=valor;...", os nomes de estudo
//! abaixo) passa por cima do cfg. Ver docs/MERCADO.md e o mapa do mercado do projeto CM0102 (MAPA_MERCADO.md).
//! - `jovem_aceita=1` (item 1): o dono da IA empresta o jovem sem espaço (idade ≤ `jovem_idade`, PA − CA ≥ `jovem_gap`,
//!   estatuto 4 a 7) a quem tem reputação ≥ `jovem_rep` × a do jogador (a 0x84af00 recusava: "not_listed").
//! - `jovem_treino=n` (item 1): o dono só empresta esse jovem a um clube com treino (+0x7f) ≥ o dele + n (recusa os outros,
//!   mesmo listados).
//! - `jovem_lista=1` (item 1): depois de cada avaliação do elenco (0x83cb40, chamada em 0x83a763), o mesmo jovem entra na
//!   lista de empréstimos (bit 2 de contrato+0x4e); a avaliação apaga as marcas a cada vez, então é refeito a cada vez.
//! - `destaque=F` (item 2): a vontade (0x846920) de ir para um clube com reputação ≥ a do atual + `destaque_dif`, de quem
//!   é titular (estatuto 1 a 3) e não tem queixa, vezes F (no original o bônus de reputação só vem com queixa).
//! - `lance=1` (item 3a): o sorteio do lance (0x84a69e) contra reputação + 1000, não 2 × reputação + 1000.
//! - `teto_meio=1` (item 3b): Alemanha, França, Escócia e Holanda: 50% da verba acima de 17,5 milhões, não 25%.
//! - `chave_preco=F` (item 4): o preço pedido (0x4d1600) do titular (estatuto 1 e 2) de clube da IA com reputação ≥
//!   `chave_rep`, vezes F.
//! - `negocio=k` (item 13): a verba (0x598800) vezes 1 + (Business − 10,5) × k, quando positiva (Business 20 com k = 0,03:
//!   × 1,29; Business 1: × 0,72); sem presidente, nada muda. No original o Business só entra na reserva de folha.
//! - `fix_clausula=1` (correção 8): a multa acima de 2,5 milhões vira (e − 2,5M) × 0,21 + 2,5M (0x86165b).
//! - `fix_workrate=1` (correção 10): o empréstimo da lista de reservas compara a reputação mundial (+0xd), não o byte +0x44.
//! Empréstimos, a proposta do Lucas (12/11): o dono empresta quem sobra, o jovem vai para onde joga, a volta para casa.
//! - `emp_regra=1` (A): a decisão do dono da IA num empréstimo (0x84af00) vira: estatuto 1 (Key) nunca; o jogador precisa
//!   ficar entre os `vaga_jovem` (até `idade_rodagem` anos) ou `vaga_outros` primeiros da linha dele no clube que pede
//!   (minutos); o jovem vai se o treino do destino for ≥ o do dono − `treino_folga`; o titular (2) só com `cobertura_min`
//!   jogadores da mesma linha com CA ≥ `cobertura_ratio` × o dele; rotação e reserva (3 a 7) com `cobertura_reserva`; o
//!   jovem não titular sem cobertura. Volta para casa (o destino é da nação do jogador, o dono não): cobertura − 1, sem o treino.
//! - `lista_emp=1` (B): depois de cada avaliação do elenco, quem a regra A emprestaria entra na lista de empréstimos (sem
//!   olhar o destino), e assim vira candidato nas listas de procura dos outros clubes.
//! - `vontade_pedido=x` (B): o limiar de vontade do pedido de empréstimo da IA (0x8427f0, original 3,5).
//! - `pedido_div=n` (B): o sorteio do pedido de empréstimo (0x84aaf9) contra reputação / (2,5 × 2^n) (original n = 2: / 10).
//! - `fator_casa=F` (C): a vontade de ir emprestado a um clube da nação dele, estando fora dela, vezes F.
//! - `fator_minutos=F`: a vontade de ir emprestado a um clube onde ele fica entre os 2 primeiros da linha, vezes F.
//! - `insatisfacao=n` (D): numa decisão de verdade (não nas previsões em massa), o empréstimo recusado de um titular (1, 2)
//!   para um clube com reputação ≥ a do dono + `insat_dif`: felicidade do contrato (+0x3d) − n.
//! - `sem_teto_nacao=1`: a verba sem o freio por nação (0x599307: o passo 15 da 0x598800 é pulado para todas).
//! - `procura_jovem=F` (E): na escolha do alvo de uma vaga de empréstimo (0x83f3a0), o jovem (até `idade_rodagem`) que
//!   ficaria entre os `vaga_jovem` primeiros da linha no clube que procura, e que vai para casa ou para um treino ≥ o do
//!   dono − `treino_folga`, tem a nota final × F (o sorteio 92 + sorteio(17) em 0x840610 passa por aqui).
//! - `elenco_min=n`: com as regras A/B, o dono não empresta (nem lista) se tem n jogadores ou menos no clube.
//! Transferências (12/11, o feeling do Lucas: artilheiro, jovem que estourou, titular de seleção e nota média fazem crescer):
//! - `dest_rep=n` (T1): a cada partida (0x6d26b0, chamada em 0x6d2112), depois da original, quem se destaca na temporada
//!   ganha n × pontos × (1 − rep/10000) nas reputações atual e em casa (a mundial, a metade), até o teto da habilidade
//!   (CA × 50 + 2500; a mundial + 1250), sem o teto da liga. Os pontos (`destaque`), cada um de 0 a 2, com ao menos
//!   `dest_jogos` jogos: a nota média (7,5 = 1, 8 = 2), os gols de liga por jogo (0,6 = 1, 0,9 = 2), os jogos como
//!   titular pela seleção (3 = 1, 6 = 2) e, até 21 anos, o CA ganho na temporada (10 = 1, 20 = 2).
//! - `dest_procura=F` (T2): na escolha do alvo de uma vaga (0x83f3a0, o sorteio em 0x840610), o candidato com 1 ponto ou
//!   mais tem a nota × F (transferência e empréstimo).
//! ponytail: chaves de estudo por variável de ambiente; viram chaves do tweaks.cfg quando o Lucas decidir quais ficam.

use super::*;

#[derive(Clone, Copy)]
struct M {
    jovem_aceita: bool,
    jovem_lista: bool,
    jovem_idade: i32,
    jovem_gap: i32,
    jovem_rep: f64,
    destaque: f64,
    destaque_dif: i32,
    lance: bool,
    teto_meio: bool,
    chave_preco: f64,
    chave_rep: i32,
    fix_clausula: bool,
    fix_workrate: bool,
    negocio: f64,
    jovem_treino: i32,
    emp_regra: bool,
    lista_emp: bool,
    idade_rodagem: i32,
    vaga_jovem: i32,
    vaga_outros: i32,
    treino_folga: i32,
    cobertura_min: i32,
    cobertura_reserva: i32,
    cobertura_ratio: f64,
    vontade_pedido: f64,
    pedido_div: i32,
    fator_casa: f64,
    fator_minutos: f64,
    insatisfacao: i32,
    insat_dif: i32,
    sem_teto_nacao: bool,
    procura_jovem: f64,
    elenco_min: i32,
    dest_rep: f64,
    dest_procura: f64,
    dest_jogos: i32,
    dest_min: f64,
    porta: f64,
    cont_dev: f64,
    cont_europa: f64,
    lista_verba: f64,
    lista_teto: f64,
    estr_usa: i32,
    estr_bra: i32,
    estr_jogo: bool,
    estr_custo: f64,
    estr_banco: f64,
    estr_folga: i32,
    estr_emp: bool,
    /// loan_list = spare (B): só entra na lista quem sobra no próprio clube (rotação para baixo e fora dos SOBRA primeiros
    /// da linha); any = a regra B de antes (quem a regra A emprestaria).
    lista_sobra: bool,
    /// loan_level (A): o adulto só vai para um clube com reputação ≥ isso × a reputação em casa dele (0 = sem a regra).
    emp_nivel: f64,
}

static mut C: M = M { jovem_aceita: false, jovem_lista: false, jovem_idade: 21, jovem_gap: 30, jovem_rep: 0.75, destaque: 1.0,
    destaque_dif: 1500, lance: false, teto_meio: false, chave_preco: 1.0, chave_rep: 6500, fix_clausula: false, fix_workrate: false, negocio: 0.0, jovem_treino: -99,
    emp_regra: false, lista_emp: false, idade_rodagem: 21, vaga_jovem: 2, vaga_outros: 2, treino_folga: 3, cobertura_min: 2,
    cobertura_reserva: 1, cobertura_ratio: 0.9, vontade_pedido: 3.5, pedido_div: 2, fator_casa: 1.0, fator_minutos: 1.0,
    insatisfacao: 0, insat_dif: 1000, sem_teto_nacao: false, procura_jovem: 1.0, elenco_min: 0, dest_rep: 0.0, dest_procura: 1.0, dest_jogos: 8, dest_min: 1.0, porta: 3.15, cont_dev: 0.2, cont_europa: 0.4, lista_verba: 0.0, lista_teto: 0.0, estr_usa: 0, estr_bra: 0,
    estr_jogo: false, estr_custo: 0.25, estr_banco: 0.25, estr_folga: 1, estr_emp: false, lista_sobra: false, emp_nivel: 0.0 };
static PROCURADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static DESTACADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static BONUS_REP: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
/// O limiar de vontade do pedido de empréstimo (o fcomp em 0x8427f0 passa a ler daqui).
static mut LIMIAR_PEDIDO: f64 = 3.5;
/// T4: os dois fatores de continente da vontade (0x846920): nações de desenvolvimento diferente e de outro continente (×0,2
/// em 0x8465ba) e o europeu indo para fora da Europa com o mesmo desenvolvimento (×0,4 em 0x8465f1).
static mut FATOR_CONT_DEV: f64 = 0.2;
static mut FATOR_CONT_EUROPA: f64 = 0.4;
static EMP_ACEITOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static EMP_RECUSADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static INSATISFEITOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static mut T_VERBA: usize = 0;
static mut T_VONTADE: usize = 0;
static mut T_PRECO: usize = 0;
static LISTADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static ACEITOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

unsafe fn contrato(jogador: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(d(0xae1050), jogador, clube)
}

unsafe fn humano(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, 1, 0, 0) != 0
}

/// O jovem sem espaço do item 1, no contrato `k` com `clube`.
unsafe fn jovem(jogador: usize, clube: usize, k: usize) -> bool {
    let ficha = d(jogador + 0x61);
    k != 0 && ficha != 0 && d(jogador + 0x39) == clube && *((k + 0x35) as *const u8) & 0x3f == 2
        && (b(jogador + 0x18) as i32) <= C.jovem_idade && (w(ficha + 7) - w(ficha + 5)) as i32 >= C.jovem_gap
        && (4..=7).contains(&(*((k + 0x4f) as *const u8) >> 4))
}

const LINHAS: [&[usize]; 4] = [&[0xf], &[0x10, 0x11, 0x16], &[0x12, 0x13, 0x14], &[0x15]];

/// A melhor nota da ficha numa linha (goleiro, defesa, meio, ataque).
unsafe fn nota_linha(ficha: usize, l: usize) -> i32 {
    LINHAS[l].iter().map(|&o| b(ficha + o) as i32).max().unwrap_or(0)
}

/// A linha do jogador: a de nota mais alta.
unsafe fn linha(ficha: usize) -> usize {
    (0..4).max_by_key(|&l| (nota_linha(ficha, l), l)).unwrap_or(2)
}

/// Os CA dos jogadores do elenco de `clube` (só os que estão nele agora) que jogam na linha `l` (nota ≥ 15), menos `fora`.
unsafe fn da_linha(clube: usize, l: usize, fora: usize) -> Vec<i32> {
    (0..50).map(|j| d(clube + 0xd7 + 4 * j))
        .filter(|&q| q != fora && crate::elenco::pessoa_valida(q) && d(q + 0x39) == clube && d(q + 0x61) != 0)
        .filter(|&q| nota_linha(d(q + 0x61), l) >= 15)
        .map(|q| w(d(q + 0x61) + 5) as i32).collect()
}

/// Quantos do elenco do dono cobrem o jogador: mesma linha, CA ≥ `cobertura_ratio` × o dele.
unsafe fn cobertura(clube: usize, p: usize) -> i32 {
    let f = d(p + 0x61);
    let ca = w(f + 5) as f64;
    da_linha(clube, linha(f), p).iter().filter(|&&c| c as f64 >= C.cobertura_ratio * ca).count() as i32
}

/// O posto que o jogador teria na linha dele no `clube` (1 = o melhor).
unsafe fn posto(clube: usize, p: usize) -> i32 {
    let f = d(p + 0x61);
    let ca = w(f + 5) as i32;
    1 + da_linha(clube, linha(f), p).iter().filter(|&&c| c > ca).count() as i32
}

/// O id da nação (o campo pode ser ponteiro para o registro, cujo +0 é o id).
unsafe fn nacao(x: usize) -> usize {
    if x < 0x10000 { x } else { d(x) }
}

/// Volta para casa: o destino é da nação do jogador e o dono não.
unsafe fn volta_para_casa(p: usize, dono: usize, destino: usize) -> bool {
    let n = nacao(d(p + 0x1a));
    n != 0 && nacao(d(destino + 0x53)) == n && nacao(d(dono + 0x53)) != n
}

/// Os primeiros da linha que o dono nunca põe na lista (goleiro, defesa, meio, ataque): os titulares e um a mais.
const SOBRA: [i32; 4] = [2, 5, 5, 3];

/// loan_list = spare: o jogador sobra no próprio clube? Rotação ou abaixo (estatuto 3 a 7) e fora dos SOBRA primeiros da
/// linha dele no dono.
unsafe fn sobra(dono: usize, p: usize, k: usize) -> bool {
    let st = (*((k + 0x4f) as *const u8) >> 4) as i32;
    st >= 3 && posto(dono, p) > SOBRA[linha(d(p + 0x61))]
}

/// A regra A sem o destino: o dono da IA aceitaria emprestar este jogador (contrato `k`)? `casa` relaxa a cobertura.
unsafe fn emprestavel(dono: usize, p: usize, k: usize, casa: bool) -> bool {
    let st = (*((k + 0x4f) as *const u8) >> 4) as i32;
    if st <= 1 {
        return false;
    }
    // elenco_min: o dono não empresta se ficar com menos jogadores que isso (os que estão no clube agora)
    if C.elenco_min > 0 && (0..50).map(|j| d(dono + 0xd7 + 4 * j))
        .filter(|&q| crate::elenco::pessoa_valida(q) && d(q + 0x39) == dono && d(q + 0x61) != 0).count() as i32 <= C.elenco_min {
        return false;
    }
    let jovem = (b(p + 0x18) as i32) <= C.idade_rodagem;
    let mut precisa = if st == 2 { C.cobertura_min } else if jovem { 0 } else { C.cobertura_reserva };
    if casa {
        precisa = (precisa - 1).max(0);
    }
    precisa == 0 || cobertura(dono, p) >= precisa
}

/// A regra A inteira, para o empréstimo de `p` de `dono` (IA) para `destino`: Some(0) aceita, Some(2) recusa, None = não se aplica.
unsafe fn regra_a(dono: usize, p: usize, destino: usize) -> Option<u8> {
    let k = contrato(p, dono);
    if k == 0 || d(p + 0x61) == 0 || d(p + 0x39) != dono || *((k + 0x35) as *const u8) & 0x3f != 2 || *((k + 0x49) as *const u8) != 0
        || contrato(p, destino) != 0 {
        return None;
    }
    let casa = volta_para_casa(p, dono, destino);
    let jovem = (b(p + 0x18) as i32) <= C.idade_rodagem;
    if !emprestavel(dono, p, k, casa) {
        return Some(2);
    }
    if posto(destino, p) > if jovem { C.vaga_jovem } else { C.vaga_outros } {
        return Some(2);
    }
    if jovem && !casa && (b(destino + 0x7f) as i32) < b(dono + 0x7f) as i32 - C.treino_folga {
        return Some(2);
    }
    // loan_level: o adulto não desce para um clube muito menor que ele (a reputação do clube contra a dele em casa)
    if !jovem && C.emp_nivel > 0.0 && (w(destino + 0x80) as f64) < C.emp_nivel * w(d(p + 0x61) + 0xb) as f64 {
        return Some(2);
    }
    Some(0)
}

static mut T_DECISAO: usize = 0;
static mut QUEM_DECIDE: usize = 0;

/// A 0x84af00 (stdcall, ret 0x20) fora das carreiras de medição: a original e, num empréstimo, a regra do dono.
unsafe extern "stdcall" fn decisao(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let original: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8 = core::mem::transmute(T_DECISAO);
    let quem = QUEM_DECIDE.wrapping_sub(5);
    let r = original(vendedor, jogador, oferta, comprador, pedido, emprestimo);
    prever_emprestimo(vendedor, jogador, comprador, emprestimo, r, quem)
}

#[unsafe(naked)]
unsafe extern "C" fn entrada_decisao() {
    core::arch::naked_asm!("mov eax, [esp]", "mov [{q}], eax", "jmp {f}", q = sym QUEM_DECIDE, f = sym decisao);
}

/// As estatísticas da temporada de uma ficha numa categoria do PlayerStats ([0xb74350], `get_category_record` 0x7c3920:
/// 2 liga, 5 seleção, 6 geral): (jogos, entrando do banco, gols, soma das notas).
unsafe fn temporada(ficha: usize, cat: u32) -> (i32, i32, i32, i32) {
    let ps = d(0xb74350);
    if ps == 0 || ficha == 0 || d(ficha) >= d(0xae236c) {
        return (0, 0, 0, 0);
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    let r = f(ps, ficha, cat);
    if r == 0 || *((r + 0xe) as *const u16) == 0xffff {
        return (0, 0, 0, 0);
    }
    (b(r) as i32, b(r + 1) as i32, b(r + 2) as i32, *((r + 0xe) as *const i16) as i32)
}

/// Os destaques vistos nas partidas: id da pessoa -> (temporada, pontos, dados), lidos pela medição mensal.
static DESTAQUES: std::sync::Mutex<Option<std::collections::HashMap<usize, (i32, f64, [f64; 6])>>> = std::sync::Mutex::new(None);

/// Os destaques da temporada atual (para a medição), uma cópia.
pub(crate) fn destaques_da_temporada(temp: i32) -> Vec<(usize, f64, [f64; 6])> {
    DESTAQUES.lock().ok().and_then(|m| m.as_ref().map(|m| m.iter().filter(|e| e.1 .0 == temp).map(|(&k, v)| (k, v.1, v.2)).collect())).unwrap_or_default()
}

/// O CA de cada jogador no começo da temporada (id da pessoa -> (temporada, CA)), para o jovem que estourou.
static mut CA_INICIO: Option<std::collections::HashMap<usize, (i32, i32)>> = None;

/// Os pontos de destaque da temporada de uma pessoa e os dados (jogos, média, jogos de liga, gols de liga, titular na
/// seleção, CA ganho). `salto` = olhar o CA ganho (só na linha de execução do jogo: guarda a foto do começo).
pub(crate) unsafe fn destaque(p: usize, salto: bool) -> (f64, [f64; 6]) {
    let f = d(p + 0x61);
    if f == 0 {
        return (0.0, [0.0; 6]);
    }
    let (j, _, _, notas) = temporada(f, 6);
    let (jl, _, gl, _) = temporada(f, 2);
    let (js, rs, _, _) = temporada(f, 5);
    let media = if j > 0 { notas as f64 / j as f64 } else { 0.0 };
    let gpj = if jl > 0 { gl as f64 / jl as f64 } else { 0.0 };
    let sel = (js - rs).max(0) as f64;
    let mut ganho = 0.0;
    let mut pts = 0.0;
    if j >= C.dest_jogos {
        pts += ((media - 7.0) / 0.5).clamp(0.0, 2.0);
    }
    if jl >= C.dest_jogos {
        pts += ((gpj - 0.3) / 0.3).clamp(0.0, 2.0);
    }
    pts += (sel / 3.0).clamp(0.0, 2.0);
    if salto && (b(p + 0x18) as i32) <= 21 {
        let temp = w(0xae2c92) as i32 - (w(0xae2c90) < 182) as i32;
        let ca = w(f + 5) as i32;
        let mapa = (*core::ptr::addr_of_mut!(CA_INICIO)).get_or_insert_with(Default::default);
        let e = mapa.entry(d(p)).or_insert((temp, ca));
        if e.0 != temp {
            *e = (temp, ca);
        }
        ganho = (ca - e.1) as f64;
        pts += (ganho / 10.0).clamp(0.0, 2.0);
    }
    (pts, [j as f64, media, jl as f64, gl as f64, sel, ganho])
}

/// T1: { 0x6d26b0 } (cdecl; a chamada em 0x6d2112, o fechamento da partida): a original e o bônus de destaque nas três
/// reputações (em jm +0x47/+0x49/+0x4b; sem jm, na ficha), até o teto da habilidade, sem o teto da liga.
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn rep_partida(jm: usize, pessoa: usize, clube: usize, rep: usize, comp: usize, nota: f32, a: u32, b2: u32) {
    let original: unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32) = core::mem::transmute(0x6d26b0usize);
    original(jm, pessoa, clube, rep, comp, nota, a, b2);
    let ficha = if pessoa != 0 { d(pessoa + 0x61) } else { 0 };
    if ficha == 0 {
        return;
    }
    let (pts, x) = destaque(pessoa, true);
    // para a medição (a batida mensal é outra linha de execução: não lê o PlayerStats, que o jogo realoca)
    if crate::estudo::ligado() {
        if let Ok(mut m) = DESTAQUES.lock() {
            let temp = w(0xae2c92) as i32 - (w(0xae2c90) < 182) as i32;
            let m = m.get_or_insert_with(Default::default);
            if pts >= 1.0 { m.insert(d(pessoa), (temp, pts, x)); } else { m.remove(&d(pessoa)); }
        }
    }
    if pts < C.dest_min || C.dest_rep <= 0.0 {
        return;
    }
    let onde = if jm != 0 { [jm + 0x47, jm + 0x49, jm + 0x4b] } else { [ficha + 0xb, ficha + 9, ficha + 0xd] };
    let ca = w(ficha + 5) as f64;
    let teto = [ca * 50.0 + 2500.0, ca * 50.0 + 2500.0, ca * 50.0 + 1250.0];
    for i in 0..3 {
        let atual = w(onde[i]) as f64;
        let bonus = C.dest_rep * pts * (1.0 - atual / 10000.0).max(0.0) * if i == 2 { 0.5 } else { 1.0 };
        let novo = (atual + bonus).min(teto[i].min(10000.0)).max(atual);
        core::ptr::write_unaligned(onde[i] as *mut i16, novo.round() as i16);
    }
    BONUS_REP.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
}

/// E: o sorteio da nota final do candidato (0x840610, `push 0x11; call 0x90cfc0`) com a pessoa (esi), a vaga ([ebp+8]) e o
/// clube que procura ([ebp+0xc]) do quadro da 0x83f3a0.
#[unsafe(naked)]
unsafe extern "C" fn sorteio_nota() {
    // o preço pedido do candidato (0x4d1600, guardado em [esp + 0x34] do quadro em 0x84030d) é o 5º argumento
    core::arch::naked_asm!("push dword ptr [esp + 0x3c]", "push dword ptr [ebp + 0xc]", "push dword ptr [ebp + 8]", "push esi",
        "push dword ptr [esp + 0x14]", "call {f}", "add esp, 20", "ret", f = sym nota_sorteio);
}

pub(crate) static VERBA_CORTES: [core::sync::atomic::AtomicU32; 2] = [core::sync::atomic::AtomicU32::new(0), core::sync::atomic::AtomicU32::new(0)];

/// T7 (`lista_verba=F`): a vaga de transferência (não de empréstimo) não fica com quem pede mais que F x a verba do clube
/// que procura (a mesma consulta do build_bid em 0x8e6329: 0x5a0590(clube) e 0x598800(fin, pessoa, 0, 0, 1, 0)).
/// T7b (`lista_teto=F`): idem com o teto da própria vaga ([vaga + 7], a fatia da verba dada a ela em 0x83b9c4..0x83ba93;
/// -1 = sem teto; o build_bid usa no mínimo 5.000 e recusa a oferta acima dele em 0x8e7549).
/// T5 (estudo): o limite de estrangeiros (+0x13 do objeto de regra da nação, TRANSFER_MANAGER [0xde1f54] + 0x8ac + 4i; EUA
/// +0x910 = 4, Brasil +0x8bc = 3) trocado por `estr_usa`/`estr_bra`. As regras são refeitas a cada jogo novo e a cada carga
/// (0x8d26b0), então reaplica aqui, antes de cada candidato. ponytail: só o limite da contratação, não o da escalação.
/// Estudo (`TWEAKS_FERIAS=1`, para o CLZ 6.1, cuja carreira automática não cria): a receita do teste de save da CLZ.DLL.
/// Na entrada de 0x8387e0 (os humanos restaurados depois de um -load) cada técnico humano (os últimos 16 da tabela com +0x5f
/// = 1) vai de férias (HUMAN_MANAGER::set_on_holiday 0x5fd2f0) com o carimbo zerado (0x5fdd30): sem isso o jogo carregado
/// espera o Continuar. E o jogo não tira o técnico das férias (na virada do ano ele pararia esperando o Continuar).
static mut F_RESTAURAR: usize = 0;
static mut F_FERIAS: usize = 0;

unsafe extern "C" fn ferias_restaurar() -> i32 {
    let hm = d(0xb63c98);
    if hm != 0 {
        let de_ferias: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(F_FERIAS);
        let carimbo: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fdd30usize);
        let (base, n) = (d(0xae23bc), d(0xae2364));
        for i in 0..16usize.min(n) {
            let t = base + (n - 1 - i) * 0x6e;
            if b(t + 0x5f) == 1 {
                de_ferias(hm, 1, t);
                carimbo(hm, 0, t);
                registro::linha("market study: a human manager is on holiday after the load (TWEAKS_FERIAS)");
            }
        }
    }
    let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(F_RESTAURAR);
    f()
}

unsafe extern "thiscall" fn ferias_sempre(this: usize, ligar: u32, tecnico: usize) {
    if ligar != 0 {
        let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(F_FERIAS);
        f(this, ligar, tecnico);
    }
}

unsafe fn instalar_ferias() {
    match (crate::origem::trampolim(0x5fd2f0, &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00], ferias_sempre as *const () as usize),
        crate::origem::trampolim(0x8387e0, &[0x51, 0x53, 0x55, 0x33, 0xed], ferias_restaurar as *const () as usize)) {
        (Some(a), Some(r)) => {
            F_FERIAS = a;
            F_RESTAURAR = r;
            registro::linha("market study: holiday after load on (TWEAKS_FERIAS)");
        }
        _ => registro::linha("market study: 0x5fd2f0/0x8387e0 differ; TWEAKS_FERIAS off"),
    }
}

unsafe fn aplicar_estr() {
    let tm = d(0xde1f54);
    if tm < 0x10000 {
        return;
    }
    for (off, n) in [(0x910usize, C.estr_usa), (0x8bc, C.estr_bra)] {
        let r = d(tm + off);
        if n > 0 && r >= 0x10000 {
            *((r + 0x13) as *mut u8) = n.min(255) as u8;
        }
    }
}

unsafe fn verba_cobre(p: usize, vaga: usize, clube: usize, preco: i32) -> bool {
    if C.estr_usa > 0 || C.estr_bra > 0 {
        aplicar_estr();
    }
    if preco <= 0 || *((vaga + 0x24) as *const u8) & 1 != 0 {
        return true;
    }
    if C.lista_teto > 0.0 {
        let teto = core::ptr::read_unaligned((vaga + 7) as *const i32);
        if teto != -1 {
            let ok = preco as f64 <= teto.max(5000) as f64 * C.lista_teto;
            VERBA_CORTES[ok as usize].fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            if !ok {
                return false;
            }
        }
    }
    if C.lista_verba <= 0.0 {
        return true;
    }
    let fin_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let fin = fin_de(d(0xae2a38), clube);
    if fin < 0x10000 {
        return true;
    }
    let verba: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let ok = preco as f64 <= verba(fin, p, 0, 0, 1, 0) as f64 * C.lista_verba;
    VERBA_CORTES[ok as usize].fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    ok
}

/// T13 (`estr_jogo=1`): o limite de estrangeiros por jogo entra na contratação como análise, não como veto. O jogo tem dois
/// limites por nação: o do elenco (+0x13/+0x14 da regra, a única coisa que a contratação vê, `can_sign_player`) e o da
/// relação do jogo (a função 7 da regra, por competição; o Brasil: 2 no original, 5 no CLZ, sem limite de elenco), que só
/// a partida confere (0x8cf880, o técnico tira quem passa). O limite de jogo de cada clube é o da última partida que tinha
/// limite (a 0x8cf880 é embrulhada e chama a 0x8cf7e0, a mesma consulta dela, sobre a partida).
/// Na nota do candidato estrangeiro (para a nação do clube; tipo 1 = não comunitário) com o clube já com `L` ou mais:
/// × 1 / (1 + `estr_custo` × excesso), excesso = os estrangeiros que ficariam fora da relação depois da compra (a
/// infelicidade de quem não joga), e × `estr_banco` se nem ele entraria entre os L melhores (CA). Depois de cada avaliação
/// do elenco, o clube da IA com mais de L + `estr_folga` estrangeiros põe na lista de transferência os mais fracos que
/// sobram (transfer_list_player 0x842320, como a própria avaliação faz).
static LIMITE_JOGO: std::sync::Mutex<Option<std::collections::HashMap<usize, (u8, u8)>>> = std::sync::Mutex::new(None);
static mut T_RELACAO: usize = 0;
static ESTR_PESO: [core::sync::atomic::AtomicU32; 3] = [core::sync::atomic::AtomicU32::new(0), core::sync::atomic::AtomicU32::new(0), core::sync::atomic::AtomicU32::new(0)];
static ESTR_LISTADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// { 0x8cf880 } a conferência dos estrangeiros na relação do jogo (thiscall, TRANSFER_MANAGER, clube, partida, 4 mais, ret 0x18).
unsafe extern "thiscall" fn relacao(tm: usize, clube: usize, partida: usize, a: usize, b2: usize, c: usize, e: usize) -> u32 {
    if clube >= 0x10000 && partida >= 0x10000 {
        let mut lim = [0u8; 8];
        let consulta: unsafe extern "thiscall" fn(usize, usize, *mut u8) = core::mem::transmute(0x8cf7e0usize);
        consulta(tm, partida, lim.as_mut_ptr());
        // só a partida entre dois clubes da mesma nação (a liga e as copas de casa): a continental tem outra regra
        let (casa, fora) = (d(partida + 0x1c), d(partida + 0x20));
        let nac = d(clube + 0x53);
        if (lim[0] == 1 || lim[0] == 2) && lim[5] != 0xff && casa >= 0x10000 && fora >= 0x10000 && nac >= 0x10000
            && d(casa + 0x53) == nac && d(fora + 0x53) == nac {
            let mut m = LIMITE_JOGO.lock().unwrap();
            let m = m.get_or_insert_with(Default::default);
            m.insert(clube, (lim[0], lim[5]));
            // a nação (chave = o ponteiro da nação | 1, que não é endereço de clube): a reserva de quem ainda não jogou
            m.insert(nac | 1, (lim[0], lim[5]));
        }
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(T_RELACAO);
    f(tm, clube, partida, a, b2, c, e)
}

/// Estrangeiro para o limite de jogo do clube (o critério da 0x8cf880: 0x53d980; tipo 1 também não comunitário, 0x53d740).
unsafe fn estrangeiro(q: usize, clube: usize, tipo: u8) -> bool {
    let nac = d(clube + 0x53);
    let de_fora: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let comunitario: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    nac != 0 && de_fora(q, nac, 1) != 0 && (tipo != 1 || comunitario(q, nac) == 0)
}

/// Os estrangeiros do elenco (no clube agora), menos `fora`: (pessoa, CA).
unsafe fn estrangeiros(clube: usize, tipo: u8, fora: usize) -> Vec<(usize, i32)> {
    (0..50).map(|j| d(clube + 0xd7 + 4 * j))
        .filter(|&q| q != fora && crate::elenco::pessoa_valida(q) && d(q + 0x39) == clube && d(q + 0x61) != 0 && estrangeiro(q, clube, tipo))
        .map(|q| (q, w(d(q + 0x61) + 5) as i32)).collect()
}

unsafe fn limite_jogo(clube: usize) -> Option<(u8, u8)> {
    LIMITE_JOGO.lock().unwrap().as_ref().and_then(|m| m.get(&clube).or_else(|| m.get(&(d(clube + 0x53) | 1))).copied())
}

unsafe fn peso_estrangeiro(p: usize, clube: usize) -> f64 {
    use core::sync::atomic::Ordering::Relaxed;
    let Some((tipo, lim)) = limite_jogo(clube) else { return 1.0 };
    if !estrangeiro(p, clube, tipo) {
        return 1.0;
    }
    let ja = estrangeiros(clube, tipo, p);
    let excesso = ja.len() as i32 + 1 - lim as i32;
    if excesso <= 0 {
        ESTR_PESO[0].fetch_add(1, Relaxed);
        return 1.0;
    }
    let ca = w(d(p + 0x61) + 5) as i32;
    let posto = 1 + ja.iter().filter(|&&(_, c)| c > ca).count() as i32;
    let f = 1.0 / (1.0 + C.estr_custo * excesso as f64);
    if posto > lim as i32 {
        ESTR_PESO[2].fetch_add(1, Relaxed);
        f * C.estr_banco
    } else {
        ESTR_PESO[1].fetch_add(1, Relaxed);
        f
    }
}

static ESTR_EMP: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// Depois da avaliação do elenco: o clube com estrangeiros demais lista os mais fracos que sobram.
unsafe fn vender_sobra(t: usize, clube: usize) {
    let Some((tipo, lim)) = limite_jogo(clube) else { return };
    let mut ja = estrangeiros(clube, tipo, 0);
    let sobra = ja.len() as i32 - lim as i32 - C.estr_folga;
    if sobra <= 0 {
        return;
    }
    ja.sort_by_key(|&(_, c)| c);
    let listar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842320usize);
    for &(q, _) in ja.iter().take(sobra as usize) {
        let k = contrato(q, clube);
        if k == 0 || *((k + 0x4e) as *const u8) & 1 != 0 || *((k + 0x35) as *const u8) & 0x3f != 2 || *((k + 0x49) as *const u8) != 0 {
            continue;
        }
        listar(t, q, 1);
        ESTR_LISTADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        // estr_emp: também na lista de empréstimos (a saída sem preço: quem não paga ainda o leva)
        if C.estr_emp && *((k + 0x4e) as *const u8) & 2 == 0 {
            *((k + 0x4e) as *mut u8) |= 2;
            ESTR_EMP.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        }
    }
}

unsafe extern "C" fn nota_sorteio(n: u32, p: usize, vaga: usize, clube: usize, preco: i32) -> i32 {
    let sorteio: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let r = sorteio(n);
    if crate::estudo::ligado() {
        funil(1, p, vaga, clube);
    }
    if p < 0x10000 || vaga < 0x10000 || clube < 0x10000 || d(p + 0x61) == 0 {
        return r;
    }
    if !verba_cobre(p, vaga, clube, preco) {
        return -92; // nota 0: não passa da melhor (0x84063c)
    }
    let mut f = 1.0;
    // E: o jovem que jogaria ali, numa vaga de empréstimo
    let dono = d(p + 0x39);
    if C.procura_jovem != 1.0 && *((vaga + 0x24) as *const u8) & 1 != 0 && (b(p + 0x18) as i32) <= C.idade_rodagem && dono >= 0x10000
        && dono != clube && posto(clube, p) <= C.vaga_jovem
        && (volta_para_casa(p, dono, clube) || (b(clube + 0x7f) as i32) >= b(dono + 0x7f) as i32 - C.treino_folga) {
        PROCURADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        f *= C.procura_jovem;
    }
    // T13: o estrangeiro acima do limite de jogo, numa vaga de transferência
    if C.estr_jogo && *((vaga + 0x24) as *const u8) & 1 == 0 {
        f *= peso_estrangeiro(p, clube);
    }
    // T2: quem se destacou na temporada
    if C.dest_procura != 1.0 && destaque(p, true).0 >= C.dest_min {
        DESTACADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        f *= C.dest_procura;
    }
    if f == 1.0 { r } else { ((92 + r) as f64 * f) as i32 - 92 }
}

/// Chamada pelo embrulho do estudo na 0x84af00: `quem` é o endereço da chamada (as previsões em massa das listas de procura
/// são 0x8435c3, 0x84049c e 0x8405a2; as outras são decisões de verdade).
pub(crate) unsafe fn prever_emprestimo(vendedor: usize, jogador: usize, comprador: usize, emprestimo: u32, r: u8, quem: usize) -> u8 {
    use core::sync::atomic::Ordering::Relaxed;
    let r = prever(vendedor, jogador, comprador, emprestimo, r);
    if !C.emp_regra || emprestimo as u8 == 0 || vendedor < 0x10000 || comprador < 0x10000 || vendedor == comprador
        || d(vendedor + 0xcf) == jogador || humano(vendedor) {
        return r;
    }
    let novo = regra_a(vendedor, jogador, comprador).unwrap_or(r);
    if !matches!(quem, 0x8435c3 | 0x84049c | 0x8405a2) {
        if novo == 0 { EMP_ACEITOS.fetch_add(1, Relaxed); } else { EMP_RECUSADOS.fetch_add(1, Relaxed); }
        // D: o titular que queria ir para um clube maior e o dono não deixou
        if novo == 2 && C.insatisfacao > 0 && w(comprador + 0x80) as i32 >= w(vendedor + 0x80) as i32 + C.insat_dif {
            let k = contrato(jogador, vendedor);
            if k != 0 && (1..=2).contains(&(*((k + 0x4f) as *const u8) >> 4)) {
                let h = (*((k + 0x3d) as *const i8) as i32 - C.insatisfacao).max(-100);
                *((k + 0x3d) as *mut i8) = h as i8;
                INSATISFEITOS.fetch_add(1, Relaxed);
            }
        }
    }
    novo
}

/// Chamada pelo embrulho do estudo na 0x84af00, logo depois da original (0 aceita, 1 negocia, 2 recusa).
pub(crate) unsafe fn prever(vendedor: usize, jogador: usize, comprador: usize, emprestimo: u32, r: u8) -> u8 {
    if !(C.jovem_aceita || C.jovem_treino > -99) || emprestimo as u8 == 0 || vendedor == 0 || comprador < 0x10000 || vendedor == comprador
        || d(vendedor + 0xcf) == jogador || humano(vendedor) {
        return r;
    }
    let k = contrato(jogador, vendedor);
    if !jovem(jogador, vendedor, k) {
        return r;
    }
    // jovem_treino: o jovem só vai para um clube com treino (+0x7f) ≥ o do dono + a folga (pode ser negativa)
    if C.jovem_treino > -99 && (b(comprador + 0x7f) as i32) < b(vendedor + 0x7f) as i32 + C.jovem_treino {
        return 2;
    }
    if C.jovem_aceita && r == 2 && contrato(jogador, comprador) == 0
        && w(comprador + 0x80) as f64 >= C.jovem_rep * w(d(jogador + 0x61) + 0xb) as f64 {
        ACEITOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return 0;
    }
    r
}

/// { 0x83cb40 } `assess_club_squad(this, entrada, clube, daily, reset)`, thiscall, ret 0x10, embrulhada na chamada de 0x83a763.
unsafe extern "thiscall" fn avaliar(t: usize, entrada: usize, clube: usize, diario: u32, reset: u32) {
    let original: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x83cb40usize);
    original(t, entrada, clube, diario, reset);
    if clube < 0x10000 || humano(clube) {
        return;
    }
    for j in 0..50 {
        let p = d(clube + 0xd7 + 4 * j);
        if !crate::elenco::pessoa_valida(p) {
            continue;
        }
        let k = contrato(p, clube);
        if k == 0 || *((k + 0x4e) as *const u8) & 2 != 0 {
            continue;
        }
        let lista = (C.jovem_lista && jovem(p, clube, k)) || (C.lista_emp && d(p + 0x61) != 0 && d(p + 0x39) == clube
            && *((k + 0x35) as *const u8) & 0x3f == 2 && *((k + 0x49) as *const u8) == 0 && emprestavel(clube, p, k, false)
            && (!C.lista_sobra || sobra(clube, p, k)));
        if lista {
            *((k + 0x4e) as *mut u8) |= 2;
            LISTADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        }
    }
    if C.estr_jogo {
        vender_sobra(t, clube);
    }
}

/// { 0x846920 } `staff_interest_in_joining_club(this, pessoa, clube, salário, empréstimo, estatuto, &motivos)`, thiscall, st0.
unsafe extern "thiscall" fn vontade(t: usize, pessoa: usize, clube: usize, salario: u32, emp: u32, st: u32, motivos: usize) -> f64 {
    let original: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(T_VONTADE);
    let mut v = original(t, pessoa, clube, salario, emp, st, motivos);
    if v <= 0.0 || pessoa == 0 || clube < 0x10000 || d(pessoa + 0x61) == 0 {
        return v;
    }
    if emp as u8 != 0 {
        let atual = d(pessoa + 0x39);
        if C.fator_casa != 1.0 && atual >= 0x10000 && volta_para_casa(pessoa, atual, clube) {
            v *= C.fator_casa;
        }
        if C.fator_minutos != 1.0 && posto(clube, pessoa) <= 2 {
            v *= C.fator_minutos;
        }
        return v;
    }
    if C.destaque == 1.0 {
        return v;
    }
    let atual = d(pessoa + 0x39);
    if atual < 0x10000 || atual == clube || (w(clube + 0x80) as i32) < w(atual + 0x80) as i32 + C.destaque_dif {
        return v;
    }
    let k = contrato(pessoa, atual);
    if k == 0 || !(1..=3).contains(&(*((k + 0x4f) as *const u8) >> 4)) || *((k + 0x45) as *const u8) & 0x10 != 0 {
        return v;
    }
    v * C.destaque
}

/// { 0x4d1600 } `calc_asking_price(this, jogador, vendedor, comprador, &motivos, ajuste, piso)`, thiscall, ret 0x18, i32.
unsafe extern "thiscall" fn preco(t: usize, jogador: usize, vendedor: usize, comprador: usize, motivos: usize, ajuste: u32, piso: u32) -> i32 {
    let original: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(T_PRECO);
    let p = original(t, jogador, vendedor, comprador, motivos, ajuste, piso);
    if p <= 0 || vendedor < 0x10000 || (w(vendedor + 0x80) as i32) < C.chave_rep || humano(vendedor) {
        return p;
    }
    let k = contrato(jogador, vendedor);
    if k != 0 && (1..=2).contains(&(*((k + 0x4f) as *const u8) >> 4)) {
        return (p as f64 * C.chave_preco).min(2.0e9) as i32;
    }
    p
}

/// { 0x598800 } `get_transfer_budget(fin, jogador, somar_listados, &ajustada, conhecimento, pendentes)`, thiscall, ret 0x14,
/// i64; o clube é o id em fin+8.
unsafe extern "thiscall" fn verba(fin: usize, jogador: usize, somar: u32, aj: usize, conh: u32, pend: u32) -> i64 {
    let original: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32) -> i64 = core::mem::transmute(T_VERBA);
    let v = original(fin, jogador, somar, aj, conh, pend);
    let id = d(fin + 8);
    if v <= 0 || id >= d(0xae235c) {
        return v;
    }
    let pres = d(d(0xae23b4) + 0x245 * id + 0xbf);
    let np = if pres != 0 { d(pres + 0x69) } else { 0 };
    if np == 0 {
        return v;
    }
    (v as f64 * (1.0 + (b(np + 0xf) as f64 - 10.5) * C.negocio).max(0.1)) as i64
}

/// Os contadores do item 1 para a linha `tick` (zera a cada leitura).
/// As regras de empréstimo estão valendo (para a linha `tick` do tweaks.log mostrar os contadores).
pub(crate) fn ligado() -> bool {
    unsafe { C.emp_regra }
}

pub(crate) fn contadores() -> String {
    use core::sync::atomic::Ordering::Relaxed;
    format!("loan_listed {} loan_accepted {} loan_real_yes {} loan_real_no {} unhappy {} young_sought {} standout_sought {} standout_rep {} loan_end_noclub {}", LISTADOS.swap(0, Relaxed), ACEITOS.swap(0, Relaxed),
        EMP_ACEITOS.swap(0, Relaxed), EMP_RECUSADOS.swap(0, Relaxed), INSATISFEITOS.swap(0, Relaxed), PROCURADOS.swap(0, Relaxed),
        DESTACADOS.swap(0, Relaxed), BONUS_REP.swap(0, Relaxed), SEM_CLUBE.swap(0, Relaxed))
}

static SEM_CLUBE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static mut T_FIM: usize = 0;

/// `CONTRACT_MANAGER::end_loan` (0x4d07e0): sem clube (+0x39 vazio), faz o que a passagem diária do próprio jogo faz
/// (0x4c80ed): só tira o contrato de empréstimo (`remove_loan_contract`, 0x4d0bd0), sem a caixa 5929. A transferência
/// (0x8c9d5d) chamava sem conferir.
unsafe extern "thiscall" fn fim_do_emprestimo(cm: usize, pessoa: usize) {
    if pessoa != 0 && d(pessoa + 0x39) == 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d0bd0usize);
        let id = d(pessoa) as usize;
        let linha = d(0xae2d0c) + 79 * id;
        let (ip, ie) = (d(linha) as i32, d(linha + 4) as i32);
        let data = |i: i32| if i < 0 { String::from("-") } else { let k = d(cm) + 0x50 * i as usize; format!("{}-{:03}", *((k + 0x2f) as *const u16), *((k + 0x2d) as *const u16)) };
        registro::linha(&format!("loan_end_noclub\t{}\tperson {}\tcontract {} ends {}\tloan {} ends {}", data_do_jogo(), id, ip, data(ip), ie, data(ie)));
        tirar(cm, pessoa);
        SEM_CLUBE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(T_FIM);
    f(cm, pessoa)
}

unsafe fn instalar_fim_do_emprestimo() {
    match crate::origem::trampolim(0x4d07e0, &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00], fim_do_emprestimo as *const () as usize) {
        Some(t) => T_FIM = t,
        None => registro::linha("loans: 0x4d07e0 differs from 3.9.68; a loan of a person without a club still shows the box contract_manager..cpp 5929"),
    }
}

/// As nossas regras de empréstimo (a configuração EJ3m das carreiras de 12/11), o padrão de cada chave ausente.
fn nosso(mut c: M) -> M {
    (c.emp_regra, c.lista_emp, c.idade_rodagem, c.vaga_jovem, c.vaga_outros, c.treino_folga, c.cobertura_min, c.cobertura_reserva,
        c.cobertura_ratio, c.elenco_min, c.vontade_pedido, c.pedido_div, c.fator_casa, c.fator_minutos, c.procura_jovem, c.insatisfacao, c.insat_dif) =
        (true, true, 21, 2, 2, 3, 2, 1, 0.9, 20, 2.75, 3, 2.0, 1.3, 3.0, 30, 1000);
    c
}

/// O `tweaks.cfg` ao lado do exe: sem ele, o jogo original (`c` como veio).
fn ler_cfg(original: M) -> M {
    let caminho = std::env::current_exe().map(|x| x.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return original };
    let mut c = nosso(original);
    c.sem_teto_nacao = true;
    let mut ligado = true;
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        let n = v.parse::<f64>().ok();
        let num = |atual: f64, lo: f64, hi: f64| n.unwrap_or(atual).clamp(lo, hi);
        match k {
            "loans" => ligado = v != "off",
            "budget_caps" => c.sem_teto_nacao = v == "off",
            "loan_youth_age" => c.idade_rodagem = num(c.idade_rodagem as f64, 15.0, 30.0) as i32,
            "loan_youth_top" => c.vaga_jovem = num(c.vaga_jovem as f64, 1.0, 11.0) as i32,
            "loan_others_top" => c.vaga_outros = num(c.vaga_outros as f64, 1.0, 11.0) as i32,
            "loan_training_margin" => c.treino_folga = num(c.treino_folga as f64, -20.0, 20.0) as i32,
            "loan_cover_regular" => c.cobertura_min = num(c.cobertura_min as f64, 0.0, 10.0) as i32,
            "loan_cover_backup" => c.cobertura_reserva = num(c.cobertura_reserva as f64, 0.0, 10.0) as i32,
            // em %: 90 = quem cobre precisa de CA ≥ 90% do dele
            "loan_cover_ratio" => c.cobertura_ratio = num(c.cobertura_ratio * 100.0, 0.0, 200.0) / 100.0,
            "loan_squad_min" => c.elenco_min = num(c.elenco_min as f64, 0.0, 50.0) as i32,
            "loan_request_willingness" => c.vontade_pedido = num(c.vontade_pedido, 0.0, 10.0),
            // vezes o original, em potências de 2 (1, 2, 4, 8...): o sorteio contra reputação / (2,5 × 2^n), n = 2 + log2
            "loan_requests" => c.pedido_div = 2 + num(2f64.powi(c.pedido_div - 2), 1.0, 64.0).log2().round() as i32,
            "loan_home" => c.fator_casa = num(c.fator_casa, 0.1, 10.0),
            "loan_playing_time" => c.fator_minutos = num(c.fator_minutos, 0.1, 10.0),
            "loan_youth_demand" => c.procura_jovem = num(c.procura_jovem, 0.1, 10.0),
            // felicidade do contrato (−100 a 100) que o titular perde quando o dono recusa o empréstimo para um clube maior
            "loan_unhappiness" => c.insatisfacao = num(c.insatisfacao as f64, 0.0, 200.0) as i32,
            "loan_list" => c.lista_sobra = v == "spare",
            "loan_level" => c.emp_nivel = num(c.emp_nivel, 0.0, 2.0),
            "loan_unhappiness_gap" => c.insat_dif = num(c.insat_dif as f64, 0.0, 10000.0) as i32,
            _ => {}
        }
    }
    if !ligado {
        let o = original;
        (c.emp_regra, c.lista_emp, c.vontade_pedido, c.pedido_div, c.fator_casa, c.fator_minutos, c.procura_jovem, c.elenco_min) =
            (o.emp_regra, o.lista_emp, o.vontade_pedido, o.pedido_div, o.fator_casa, o.fator_minutos, o.procura_jovem, o.elenco_min);
        c.insatisfacao = o.insatisfacao;
    }
    c
}

pub(crate) unsafe fn instalar() {
    let mut c = ler_cfg(C);
    let estudo = crate::estudo::ligado();
    let txt = if estudo { std::env::var("TWEAKS_MERCADO").unwrap_or_default() } else { String::new() };
    for par in txt.split(';') {
        let Some((k, v)) = par.split_once('=') else { continue };
        let x: f64 = v.trim().parse().unwrap_or(0.0);
        match k.trim() {
            "jovem_aceita" => c.jovem_aceita = x != 0.0,
            "jovem_lista" => c.jovem_lista = x != 0.0,
            "jovem_idade" => c.jovem_idade = x as i32,
            "jovem_gap" => c.jovem_gap = x as i32,
            "jovem_rep" => c.jovem_rep = x,
            "destaque" => c.destaque = x,
            "destaque_dif" => c.destaque_dif = x as i32,
            "lance" => c.lance = x != 0.0,
            "teto_meio" => c.teto_meio = x != 0.0,
            "chave_preco" => c.chave_preco = x,
            "chave_rep" => c.chave_rep = x as i32,
            "fix_clausula" => c.fix_clausula = x != 0.0,
            "fix_workrate" => c.fix_workrate = x != 0.0,
            "negocio" => c.negocio = x,
            "jovem_treino" => c.jovem_treino = x as i32,
            "emp_regra" => c.emp_regra = x != 0.0,
            "lista_emp" => c.lista_emp = x != 0.0,
            "idade_rodagem" => c.idade_rodagem = x as i32,
            "vaga_jovem" => c.vaga_jovem = x as i32,
            "vaga_outros" => c.vaga_outros = x as i32,
            "treino_folga" => c.treino_folga = x as i32,
            "cobertura_min" => c.cobertura_min = x as i32,
            "cobertura_reserva" => c.cobertura_reserva = x as i32,
            "cobertura_ratio" => c.cobertura_ratio = x,
            "vontade_pedido" => c.vontade_pedido = x,
            "pedido_div" => c.pedido_div = x as i32,
            "fator_casa" => c.fator_casa = x,
            "fator_minutos" => c.fator_minutos = x,
            "insatisfacao" => c.insatisfacao = x as i32,
            "insat_dif" => c.insat_dif = x as i32,
            "sem_teto_nacao" => c.sem_teto_nacao = x != 0.0,
            "procura_jovem" => c.procura_jovem = x,
            "elenco_min" => c.elenco_min = x as i32,
            "dest_rep" => c.dest_rep = x,
            "dest_procura" => c.dest_procura = x,
            "dest_jogos" => c.dest_jogos = x as i32,
            "dest_min" => c.dest_min = x,
            "porta" => c.porta = x,
            "lista_verba" => c.lista_verba = x,
            "lista_teto" => c.lista_teto = x,
            "estr_usa" => c.estr_usa = x as i32,
            "estr_bra" => c.estr_bra = x as i32,
            "estr_jogo" => c.estr_jogo = x != 0.0,
            "estr_custo" => c.estr_custo = x,
            "estr_banco" => c.estr_banco = x,
            "estr_folga" => c.estr_folga = x as i32,
            "estr_emp" => c.estr_emp = x != 0.0,
            "cont_dev" => c.cont_dev = x,
            "cont_europa" => c.cont_europa = x,
            outra => registro::linha(&format!("market study: unknown key {}", outra)),
        }
    }
    C = c;
    if !txt.is_empty() {
        registro::linha(&format!("market study: {}", txt));
    }
    if c.emp_regra {
        registro::linha(&format!("loans: on (youth up to {} among the top {} of his line, others top {}, cover {}/{} at {}%, owner keeps {}+, \
            request willingness {}, requests x{}, home x{}, playing time x{}, youth demand x{}, unhappiness {} at +{} reputation)", c.idade_rodagem, c.vaga_jovem, c.vaga_outros,
            c.cobertura_min, c.cobertura_reserva, (c.cobertura_ratio * 100.0).round(), c.elenco_min, c.vontade_pedido, 1 << (c.pedido_div - 2).clamp(0, 30),
            c.fator_casa, c.fator_minutos, c.procura_jovem, c.insatisfacao, c.insat_dif));
        instalar_fim_do_emprestimo();
        // a decisão do dono (0x84af00): nas carreiras de medição o embrulho do estudo já chama a regra
        if !estudo {
            match crate::origem::trampolim(0x84af00, &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00], entrada_decisao as *const () as usize) {
                Some(t) => T_DECISAO = t,
                None => {
                    registro::linha("loans: 0x84af00 differs from 3.9.68; the owner's rule is off");
                    C.emp_regra = false;
                }
            }
        }
    } else {
        registro::linha("loans: off (the original game's rules)");
    }
    if c.sem_teto_nacao {
        registro::linha("budget_caps: off (no nation's budget is compressed)");
    }
    // no estudo o limite de jogo de cada clube é sempre lido (para a medição da carreira base)
    if c.estr_jogo || estudo {
        match crate::origem::trampolim(0x8cf880, &[0x83, 0xec, 0x0c, 0x55, 0x8b, 0x6c, 0x24, 0x18], relacao as *const () as usize) {
            Some(t) => {
                T_RELACAO = t;
                registro::linha(&format!("market study: match-day foreign limit weighs on signings (cost {}, bench x{}, spare {})", c.estr_custo, c.estr_banco, c.estr_folga));
            }
            None => {
                registro::linha("market study: 0x8cf880 differs; estr_jogo off");
                C.estr_jogo = false;
            }
        }
    }
    if c.jovem_lista || c.lista_emp || C.estr_jogo {
        let ok = *(0x83a763 as *const u8) == 0xe8 && (0x83a768usize).wrapping_add(core::ptr::read_unaligned(0x83a764 as *const i32) as usize) == 0x83cb40;
        if ok {
            chamar(0x83a763, avaliar as *const () as usize);
        } else {
            registro::linha("market study: 0x83a763 differs; young loan listing off");
        }
    }
    if c.destaque != 1.0 || c.fator_casa != 1.0 || c.fator_minutos != 1.0 {
        match crate::origem::trampolim(0x846920, &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8], vontade as *const () as usize) {
            Some(t) => T_VONTADE = t,
            None => registro::linha("market study: 0x846920 differs; destaque off"),
        }
    }
    if c.chave_preco != 1.0 {
        match crate::origem::trampolim(0x4d1600, &[0x83, 0xec, 0x7c, 0x53, 0x55], preco as *const () as usize) {
            Some(t) => T_PRECO = t,
            None => registro::linha("market study: 0x4d1600 differs; chave_preco off"),
        }
    }
    if c.negocio != 0.0 {
        match crate::origem::trampolim(0x598800, &[0x83, 0xec, 0x38, 0x8b, 0x41, 0x08], verba as *const () as usize) {
            Some(t) => T_VERBA = t,
            None => registro::linha("market study: 0x598800 differs; negocio off"),
        }
    }
    if c.dest_rep > 0.0 || crate::estudo::ligado() {
        let ok = *(0x6d2112 as *const u8) == 0xe8 && (0x6d2117usize).wrapping_add(core::ptr::read_unaligned(0x6d2113 as *const i32) as usize) == 0x6d26b0;
        if ok {
            chamar(0x6d2112, rep_partida as *const () as usize);
            registro::linha(&format!("market study: standout reputation on ({} a point per match)", c.dest_rep));
        } else {
            registro::linha("market study: 0x6d2112 differs (player_rep_speed?); dest_rep off");
        }
    }
    if crate::estudo::ligado() && std::env::var("TWEAKS_FERIAS").is_ok() {
        instalar_ferias();
    }
    if crate::estudo::ligado() {
        PORTA_VONTADE = c.porta;
        instalar_funil();
        if c.porta != 3.15 {
            registro::linha(&format!("market study: willingness gate {}", c.porta));
        }
    }
    if c.procura_jovem != 1.0 || c.dest_procura != 1.0 || crate::estudo::ligado() {
        let ok = *(0x840610 as *const u8) == 0xe8 && (0x840615usize).wrapping_add(core::ptr::read_unaligned(0x840611 as *const i32) as usize) == 0x90cfc0;
        if ok {
            chamar(0x840610, sorteio_nota as *const () as usize);
            registro::linha("market study: young loan demand on");
        } else {
            registro::linha("market study: 0x840610 differs; procura_jovem off");
        }
    }
    if c.sem_teto_nacao {
        // je 0x5994a8 (sem nação) -> jmp 0x5994a8: o teto por nação nunca entra
        aplicar_trocas("market study: budget without the per-nation caps", &[(0x599307, &[0x0f, 0x84, 0x9b, 0x01, 0x00, 0x00], &[0xe9, 0x9c, 0x01, 0x00, 0x00, 0x90])], true);
    }
    if c.cont_dev != 0.2 {
        FATOR_CONT_DEV = c.cont_dev;
        let a = (core::ptr::addr_of!(FATOR_CONT_DEV) as usize as u32).to_le_bytes();
        aplicar_trocas("market study: continent factor (development)", &[(0x8465ba, &[0xdc, 0x0d, 0x18, 0x89, 0x96, 0x00], &[0xdc, 0x0d, a[0], a[1], a[2], a[3]])], true);
    }
    if c.cont_europa != 0.4 {
        FATOR_CONT_EUROPA = c.cont_europa;
        let a = (core::ptr::addr_of!(FATOR_CONT_EUROPA) as usize as u32).to_le_bytes();
        aplicar_trocas("market study: continent factor (leaving Europe)", &[(0x8465f1, &[0xdc, 0x0d, 0x00, 0x8e, 0x96, 0x00], &[0xdc, 0x0d, a[0], a[1], a[2], a[3]])], true);
    }
    if c.vontade_pedido != 3.5 {
        LIMIAR_PEDIDO = c.vontade_pedido;
        let a = (core::ptr::addr_of!(LIMIAR_PEDIDO) as usize as u32).to_le_bytes();
        aplicar_trocas("market study: loan request willingness", &[(0x8427f0, &[0xdc, 0x1d, 0x38, 0x8e, 0x96, 0x00], &[0xdc, 0x1d, a[0], a[1], a[2], a[3]])], true);
    }
    if c.pedido_div != 2 {
        aplicar_trocas("market study: loan request draw", &[(0x84aaf9, &[0xc1, 0xfa, 0x02], &[0xc1, 0xfa, c.pedido_div.clamp(0, 8) as u8])], true);
    }
    if c.lance {
        // lea edx, [ecx + ecx + 0x3e8] -> lea edx, [ecx + 0x3e8]; nop
        aplicar_trocas("market study: bid draw", &[(0x84a69e, &[0x8d, 0x94, 0x09, 0xe8, 0x03, 0x00, 0x00], &[0x8d, 0x91, 0xe8, 0x03, 0x00, 0x00, 0x90])], true);
    }
    if c.teto_meio {
        // fmul [0x9689e0] (0,25) -> fmul [0x968968] (0,5)
        aplicar_trocas("market study: budget cap GER/FRA/SCO/NED", &[(0x599422, &[0xdc, 0x0d, 0xe0, 0x89, 0x96, 0x00], &[0xdc, 0x0d, 0x68, 0x89, 0x96, 0x00])], true);
    }
    if c.fix_clausula {
        // fmul 0,21; fsub 2,5M -> fsub 2,5M; fmul 0,21 (o fadd 2,5M fica)
        aplicar_trocas("market study: clause penalty", &[(0x86165b, &[0xdc, 0x0d, 0x98, 0xd3, 0x96, 0x00, 0xdc, 0x25, 0x50, 0xa6, 0x96, 0x00],
            &[0xdc, 0x25, 0x50, 0xa6, 0x96, 0x00, 0xdc, 0x0d, 0x98, 0xd3, 0x96, 0x00])], true);
    }
    if c.fix_workrate {
        // movsx edx, byte [eax+0x44] -> movsx edx, word [eax+0xd]
        aplicar_trocas("market study: reserve-list loan check", &[(0x83e482, &[0x0f, 0xbe, 0x50, 0x44], &[0x0f, 0xbf, 0x50, 0x0d])], true);
    }
}

// ---- o funil da escolha do alvo (0x83f3a0), só no estudo: onde morre o candidato de outro continente ----
// etapas: 0 chega ao filtro de conhecimento (0x83fed8), 1 conhecido e com preço (o sorteio da nota, 0x840610),
// 2 a vontade calculada (0x8407a2), 3 passou a vontade e o salário (0x84093f), 4 escolhido para a lista (0x840ad3).
// categorias: 0 mesmo continente, 1 outro continente, 2 sem clube, 3 outro continente com CA 120+.
static mut FUNIL: [[[u64; 4]; 8]; 3] = [[[0; 4]; 8]; 3]; // etapa 7: o build_bid recusou (sem lance)
// etapas 5 (lance feito em make_club_bids, 0x84a70e) e 6 (o vendedor aceita, accept_transfer_bid 0x8c0080): vaga = 1
static mut RET_LANCE: usize = 0;
// etapas 4 (na lista) e 5 (lance) pelo comprador: [temporada][etapa 4/5][grupo][mesmo/outro]; grupos 0..7 continente,
// 8..10 LEAGUE_SELECTED (+0x11c) 0/1/2, 11..14 reputação < 2500, < 4750, <= 6250, > 6250
static mut FUNIL_COMPRADOR: [[[[u64; 2]; 15]; 2]; 3] = [[[[0; 2]; 15]; 2]; 3];
// no aceite do vendedor: taxa / valor do jogador, por faixa (< 0,8, 0,8..1, 1..1,5, 1,5..2,5, >= 2,5)
static mut FUNIL_PRECO: [[[u64; 5]; 4]; 3] = [[[0; 5]; 4]; 3];
// a vontade na etapa 2 por faixa: < 1, 1..2,75, 2,75..3,15, >= 3,15
static mut FUNIL_VONTADE: [[[u64; 4]; 4]; 3] = [[[0; 4]; 4]; 3];
// os bits de motivo da vontade abaixo de 3,15, por temporada e categoria (0..3)
static mut FUNIL_MOTIVOS: [[[u64; 32]; 4]; 3] = [[[0; 32]; 4]; 3];
static mut MOTIVOS: u32 = 0;
/// T4c: a porta da vontade na escolha do alvo (0x8407a6, original 3,15); abaixo dela o salário pedido (3,5/vontade) decide.
static mut PORTA_VONTADE: f64 = 3.15;

unsafe fn funil_cat(p: usize, clube: usize) -> Option<(usize, usize)> {
    if p < 0x10000 || clube < 0x10000 || d(p + 0x61) == 0 {
        return None;
    }
    let t = (w(0xae2c92) as i32 - (w(0xae2c90) < 182) as i32 - 2001).clamp(0, 2) as usize;
    let cont = |c: usize| {
        let n = d(c + 0x53);
        if n >= 0x10000 { d(n + 0x71) } else { 0 }
    };
    let dono = d(p + 0x39);
    if dono < 0x10000 {
        return Some((t, 2));
    }
    Some((t, if cont(dono) == cont(clube) { 0 } else { 1 }))
}

// a recusa do build_bid pelo teto de preço da vaga ([vaga + 7], comparado em 0x8e7549): [temporada][categoria][faixa]
// faixa 0 = recusado por outro ramo; 1..5 = pelo teto, oferta/teto < 1,25, < 1,5, < 2, < 4, >= 4
static mut FUNIL_TETO: [[[u64; 6]; 2]; 3] = [[[0; 6]; 2]; 3];
static mut BB_TETO: f64 = 0.0;
// as saídas com 0 do build_bid: cada salto (rel32) vai para um trampolim que grava o número da saída (1..) em BB_SAIDA
const BB_SAIDAS: [(usize, &[u8]); 17] = [(0x8e63cd, &[0x0f, 0x84]), (0x8e6430, &[0x0f, 0x84]), (0x8e643f, &[0xe9]), (0x8e649e, &[0x0f, 0x84]),
    (0x8e6d04, &[0x0f, 0x84]), (0x8e6d0d, &[0x0f, 0x85]), (0x8e6d2c, &[0x0f, 0x84]), (0x8e70ea, &[0xe9]), (0x8e7145, &[0x0f, 0x84]),
    (0x8e7156, &[0x0f, 0x85]), (0x8e7160, &[0x0f, 0x8d]), (0x8e7275, &[0xe9]), (0x8e747e, &[0x0f, 0x8f]), (0x8e74b3, &[0x0f, 0x8c]),
    (0x8e74bb, &[0x0f, 0x82]), (0x8e7681, &[0x0f, 0x84]), (0x8e769c, &[0xe9])];
static mut BB_SAIDA: u32 = 0;
// a saída 17 é a can_sign_player (0x8ced40, as regras da nação do comprador): o motivo (o short em [esp + 0x4a])
static mut BB_MOTIVO: u32 = 0;
static mut FUNIL_CS: [[[u64; 64]; 2]; 3] = [[[0; 64]; 2]; 3];
// recusas da can_sign_player por nação do comprador (ponteiro da nação, mesmo, outro), as 2 temporadas juntas
static mut CS_NACOES: [(usize, u64, u64); 300] = [(0, 0, 0); 300];

#[unsafe(naked)]
unsafe extern "C" fn bb_motivo() {
    // no lugar de `cmp word ptr [esp + 0x4a], 9` (6 bytes) em 0x8e7677; eax está livre ali
    core::arch::naked_asm!("movzx eax, word ptr [esp + 0x4e]", "mov dword ptr [{m}], eax", "cmp word ptr [esp + 0x4e], 9", "ret",
        m = sym BB_MOTIVO);
}
// [temporada][categoria][saída 0 = nenhuma marcada, 1..17, 18 = o teto da vaga]
static mut FUNIL_SAIDA: [[[u64; 19]; 2]; 3] = [[[0; 19]; 2]; 3];

unsafe fn instalar_saidas() {
    let buf = crate::win::VirtualAlloc(core::ptr::null_mut(), 4096, 0x3000, 0x40) as usize;
    if buf == 0 {
        return;
    }
    for (i, &(va, op)) in BB_SAIDAS.iter().enumerate() {
        if core::slice::from_raw_parts(va as *const u8, op.len()) != op {
            registro::linha(&format!("market study: build_bid exit {:#x} differs", va));
            continue;
        }
        let fim = va + op.len() + 4;
        let alvo = fim.wrapping_add(core::ptr::read_unaligned((va + op.len()) as *const i32) as usize);
        let st = buf + i * 16;
        // mov dword ptr [BB_SAIDA], i + 1; jmp alvo
        *(st as *mut u8) = 0xc7;
        *((st + 1) as *mut u8) = 0x05;
        core::ptr::write_unaligned((st + 2) as *mut u32, core::ptr::addr_of!(BB_SAIDA) as usize as u32);
        core::ptr::write_unaligned((st + 6) as *mut u32, i as u32 + 1);
        *((st + 10) as *mut u8) = 0xe9;
        core::ptr::write_unaligned((st + 11) as *mut i32, alvo.wrapping_sub(st + 15) as i32);
        let mut antiga = 0u32;
        crate::win::VirtualProtect((va + op.len()) as *mut core::ffi::c_void, 4, 0x40, &mut antiga);
        core::ptr::write_unaligned((va + op.len()) as *mut i32, st.wrapping_sub(fim) as i32);
        crate::win::VirtualProtect((va + op.len()) as *mut core::ffi::c_void, 4, antiga, &mut antiga);
    }
} // oferta/teto da última recusa pelo teto (0 = não foi pelo teto)

unsafe extern "C" fn bb_teto_r(oferta: i32, teto: i32) {
    BB_TETO = if teto > 0 { (oferta as f64 / teto as f64).max(1.0) } else { 99.0 };
}

#[unsafe(naked)]
unsafe extern "C" fn bb_teto() {
    // no lugar de `cmp [esi + 0x1b], eax; jle 0x8e7577` (5 bytes) em 0x8e7549: eax = o teto, esi = os termos
    core::arch::naked_asm!("cmp dword ptr [esi + 0x1b], eax", "jle 2f", "pushad", "push eax", "push dword ptr [esi + 0x1b]",
        "call {f}", "add esp, 8", "popad", "mov dword ptr [esp], 0x8e754e", "ret", "2:", "mov dword ptr [esp], 0x8e7577", "ret",
        f = sym bb_teto_r);
}

unsafe extern "C" fn funil(etapa: u32, p: usize, vaga: usize, clube: usize) {
    if vaga != 1 && (vaga < 0x10000 || *((vaga + 0x24) as *const u8) & 1 != 0) {
        return; // vaga de empréstimo: fora
    }
    let teto = BB_TETO;
    let saida = BB_SAIDA as usize;
    if etapa == 5 || etapa == 7 {
        BB_TETO = 0.0;
        BB_SAIDA = 0;
    }
    if let Some((t, k)) = funil_cat(p, clube) {
        let f = &mut *core::ptr::addr_of_mut!(FUNIL);
        f[t][etapa as usize][k] += 1;
        if etapa == 7 && k < 2 {
            let faixa = if teto == 0.0 { 0 } else if teto < 1.25 { 1 } else if teto < 1.5 { 2 } else if teto < 2.0 { 3 } else if teto < 4.0 { 4 } else { 5 };
            (*core::ptr::addr_of_mut!(FUNIL_TETO))[t][k][faixa] += 1;
            (*core::ptr::addr_of_mut!(FUNIL_SAIDA))[t][k][if teto != 0.0 { 18 } else { saida.min(17) }] += 1;
            if saida == 17 && teto == 0.0 {
                (*core::ptr::addr_of_mut!(FUNIL_CS))[t][k][(BB_MOTIVO as usize).min(63)] += 1;
                let n = d(clube + 0x53);
                let tab = &mut *core::ptr::addr_of_mut!(CS_NACOES);
                if let Some(e) = tab.iter_mut().find(|e| e.0 == n || e.0 == 0) {
                    e.0 = n;
                    if k == 0 { e.1 += 1 } else { e.2 += 1 }
                }
            }
        }
        if (etapa == 4 || etapa == 5) && k < 2 {
            let g = &mut (*core::ptr::addr_of_mut!(FUNIL_COMPRADOR))[t][etapa as usize - 4];
            let n = d(clube + 0x53);
            if n >= 0x10000 {
                let c = d(n + 0x71);
                let ci = if c >= 0x10000 { (d(c) as usize).min(7) } else { 7 };
                g[ci][k] += 1;
                g[8 + (*((n + 0x11c) as *const u8)).min(2) as usize][k] += 1;
            }
            let r = w(clube + 0x80);
            g[if r < 2500 { 11 } else if r < 4750 { 12 } else if r <= 6250 { 13 } else { 14 }][k] += 1;
        }
        if k == 1 && w(d(p + 0x61) + 5) >= 120 {
            f[t][etapa as usize][3] += 1;
        }
    }
}

unsafe extern "C" fn funil_vontade(p: usize, vaga: usize, clube: usize, v: f64) {
    funil(2, p, vaga, clube);
    if vaga < 0x10000 || *((vaga + 0x24) as *const u8) & 1 != 0 {
        return;
    }
    if let Some((t, k)) = funil_cat(p, clube) {
        let faixa = if v < 1.0 { 0 } else if v < 2.75 { 1 } else if v < 3.15 { 2 } else { 3 };
        (*core::ptr::addr_of_mut!(FUNIL_VONTADE))[t][k][faixa] += 1;
        if v < 3.15 {
            let m = MOTIVOS;
            let cats: &[usize] = if k == 1 && w(d(p + 0x61) + 5) >= 120 { &[1, 3] } else { core::slice::from_ref(&k) };
            for &kk in cats {
                for b in 0..32 {
                    if m & (1 << b) != 0 {
                        (*core::ptr::addr_of_mut!(FUNIL_MOTIVOS))[t][kk][b] += 1;
                    }
                }
            }
        }
    }
    MOTIVOS = 0;
}

#[unsafe(naked)]
unsafe extern "C" fn funil_lance() {
    // no lugar de `call 0x8e5d60` (build_bid) em 0x84a70e: jogador em esi, comprador em ebx
    core::arch::naked_asm!("pop dword ptr [{r}]", "mov dword ptr [{s}], 0", "mov eax, 0x8e5d60", "call eax", "pushad", "push ebx", "push 1", "push esi",
        "test eax, eax", "mov eax, 5", "mov ecx, 7", "cmovz eax, ecx", "push eax", "call {f}", "add esp, 16", "popad", "push dword ptr [{r}]", "ret", r = sym RET_LANCE, f = sym funil, s = sym BB_SAIDA);
}

unsafe extern "C" fn funil_aceite_r(slot: usize) {
    if slot < 0x10000 {
        return;
    }
    let o = d(slot);
    if o < 0x10000 || *((o + 0x1f) as *const u8) == 1 {
        return; // empréstimo: fora
    }
    let p = d(0xae23bc).wrapping_add(d(o + 0xc).wrapping_mul(0x6e));
    let c = d(0xae23b4).wrapping_add(d(o + 8).wrapping_mul(0x245));
    funil(6, p, 1, c);
    if let Some((t, k)) = funil_cat(p, c) {
        let v = d(p + 0x52) as i32;
        if v > 0 {
            let r = d(o + 0x1b) as i32 as f64 / v as f64;
            let f = if r < 0.8 { 0 } else if r < 1.0 { 1 } else if r < 1.5 { 2 } else if r < 2.5 { 3 } else { 4 };
            (*core::ptr::addr_of_mut!(FUNIL_PRECO))[t][k][f] += 1;
        }
    }
}

#[unsafe(naked)]
unsafe extern "C" fn funil_aceite() {
    // no lugar de `push -1; push 0x963b48` (7 bytes) na entrada de accept_transfer_bid 0x8c0080 (thiscall, slot em [esp+4])
    core::arch::naked_asm!("pushad", "push dword ptr [esp + 36]", "call {g}", "add esp, 4", "popad", "push -1", "push 0x963b48",
        "push 0x8c0087", "ret", g = sym funil_aceite_r);
}

#[unsafe(naked)]
unsafe extern "C" fn funil_vontade_chamada() {
    // a chamada da 0x846920 em 0x840668: o &motivos (o 6º argumento, nulo) vira a nossa variável
    core::arch::naked_asm!("mov dword ptr [{m}], 0", "lea eax, [{m}]", "mov dword ptr [esp + 0x18], eax", "push 0x846920", "ret", m = sym MOTIVOS);
}

#[unsafe(naked)]
unsafe extern "C" fn funil_p0() {
    // no lugar de `mov eax, [esp + 0x84]` (7 bytes) em 0x83fed8
    core::arch::naked_asm!("pushad", "push dword ptr [ebp + 0xc]", "push dword ptr [ebp + 8]", "push esi", "push 0", "call {f}", "add esp, 16",
        "popad", "mov eax, dword ptr [esp + 0x88]", "ret", f = sym funil);
}

#[unsafe(naked)]
unsafe extern "C" fn funil_p2() {
    // no lugar de `fld qword [esp + 0x34]; fcomp qword [0x968e58]` (10 bytes) em 0x8407a2
    core::arch::naked_asm!("pushad", "sub esp, 8", "fld qword ptr [esp + 0x60]", "fstp qword ptr [esp]", "push dword ptr [ebp + 0xc]",
        "push dword ptr [ebp + 8]", "push esi", "call {f}", "add esp, 20", "popad", "fld qword ptr [esp + 0x38]",
        "fcomp qword ptr [{g}]", "ret", f = sym funil_vontade, g = sym PORTA_VONTADE);
}

#[unsafe(naked)]
unsafe extern "C" fn funil_p3() {
    // no lugar de `mov ecx, [esp + 0x2c]; mov [esp + 0x88], esi` (11 bytes) em 0x84093f
    core::arch::naked_asm!("pushad", "push dword ptr [ebp + 0xc]", "push dword ptr [ebp + 8]", "push esi", "push 3", "call {f}", "add esp, 16",
        "popad", "mov dword ptr [esp + 0x8c], esi", "mov ecx, dword ptr [esp + 0x30]", "ret", f = sym funil);
}

#[unsafe(naked)]
unsafe extern "C" fn funil_p4() {
    // a chamada de add_shortlist_entry (0x8423f0) em 0x840ad3: pessoa em ebx, clube em edi
    core::arch::naked_asm!("pushad", "push edi", "push dword ptr [ebp + 8]", "push ebx", "push 4", "call {f}", "add esp, 16",
        "popad", "push 0x8423f0", "ret", f = sym funil);
}

/// Instala o funil (só no estudo), conferindo os bytes de cada lugar.
pub(crate) unsafe fn instalar_funil() {
    let em = |va: usize, b: &[u8]| core::slice::from_raw_parts(va as *const u8, b.len()) == b;
    let ok = em(0x83fed8, &[0x8b, 0x84, 0x24, 0x84, 0x00, 0x00, 0x00])
        && em(0x8407a2, &[0xdd, 0x44, 0x24, 0x34, 0xdc, 0x1d, 0x58, 0x8e, 0x96, 0x00])
        && em(0x84093f, &[0x8b, 0x4c, 0x24, 0x2c, 0x89, 0xb4, 0x24, 0x88, 0x00, 0x00, 0x00])
        && em(0x840ad3, &[0xe8])
        && (0x840ad8usize).wrapping_add(core::ptr::read_unaligned(0x840ad4 as *const i32) as usize) == 0x8423f0;
    if !ok {
        registro::linha("market study: 0x83f3a0 differs; funnel off");
        return;
    }
    chamar(0x83fed8, funil_p0 as *const () as usize);
    nops(0x83fedd, 2);
    chamar(0x8407a2, funil_p2 as *const () as usize);
    nops(0x8407a7, 5);
    chamar(0x84093f, funil_p3 as *const () as usize);
    nops(0x840944, 6);
    chamar(0x840ad3, funil_p4 as *const () as usize);
    if *(0x84a70e as *const u8) == 0xe8 && (0x84a713usize).wrapping_add(core::ptr::read_unaligned(0x84a70f as *const i32) as usize) == 0x8e5d60 {
        chamar(0x84a70e, funil_lance as *const () as usize);
        if core::slice::from_raw_parts(0x8e7549 as *const u8, 5) == [0x39, 0x46, 0x1b, 0x7e, 0x29] {
            chamar(0x8e7549, bb_teto as *const () as usize);
        }
        instalar_saidas();
        if core::slice::from_raw_parts(0x8e7677 as *const u8, 6) == [0x66, 0x83, 0x7c, 0x24, 0x4a, 0x09] {
            chamar(0x8e7677, bb_motivo as *const () as usize);
            nops(0x8e767c, 1);
        }
    }
    if em(0x8c0080, &[0x6a, 0xff, 0x68, 0x48, 0x3b, 0x96, 0x00]) {
        desviar(0x8c0080, funil_aceite as *const () as usize);
        nops(0x8c0085, 2);
    }
    if *(0x840668 as *const u8) == 0xe8 && (0x84066dusize).wrapping_add(core::ptr::read_unaligned(0x840669 as *const i32) as usize) == 0x846920 {
        chamar(0x840668, funil_vontade_chamada as *const () as usize);
    }
    registro::linha("market study: funnel on");
}

/// O funil acumulado, por temporada (estudo_funil.tsv, reescrito a cada mês).
pub(crate) fn gravar_funil() {
    let caminho = std::env::current_exe().map(|p| p.with_file_name("estudo_funil.tsv")).unwrap_or_else(|_| "estudo_funil.tsv".into());
    let mut s = String::from("temporada\tetapa\tmesmo\toutro\tsem_clube\toutro_ca120\n");
    for &(n, a, b2) in unsafe { (*core::ptr::addr_of!(CS_NACOES)).iter() } {
        if n >= 0x10000 {
            let nome: String = unsafe { core::slice::from_raw_parts((n + 4) as *const u8, 51) }.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect();
            s += &format!("total\tcs_nacao {}\t{}\t{}\t0\t0\n", nome, a, b2);
        }
    }
    // as 28 regras de transferência (TRANSFER_MANAGER + 0x8ac): nação, escopo (+0x12), estrangeiros (+0x13), não comunitários (+0x14)
    unsafe {
        let tm = d(0xde1f54);
        // ponytail: só nações que algum clube usa (+0x53); as regras trocadas do CLZ podem ter outro leiaute em +0xa
        let (base, nc) = (d(0xae23b4), d(0xae235c));
        let nacoes: std::collections::HashSet<usize> = if base >= 0x10000 && nc < 100_000 { (0..nc).map(|k| d(base + 0x245 * k + 0x53)).collect() } else { Default::default() };
        if tm >= 0x10000 {
            // segue o próprio exe em memória: a NickDLL do CLZ troca `add ecx, 0x8ac` (0x8ced4d) por `mov ecx, [ecx + 0x8ac]`
            // (a lista vira um ponteiro para a dela) e o 28 do `cmp al, 0x1c` (0x8ced63) pelo número dela
            let lista = if *(0x8ced4d as *const u8) == 0x8b { d(tm + 0x8ac) } else { tm + 0x8ac };
            let quantas = if *(0x8ced63 as *const u8) == 0x3c { *(0x8ced64 as *const u8) as usize } else { 28 };
            s += &format!("total\tregras {} lista {:#x}\t0\t0\t0\t0\n", quantas, lista);
            for i in 0..quantas {
                let r = d(lista + 4 * i);
                #[link(name = "kernel32")]
                unsafe extern "system" {
                    fn IsBadReadPtr(p: *const u8, n: usize) -> i32;
                }
                // ponytail: só no estudo; logo depois da carga o gerenciador pode estar ainda por montar
                if r < 0x10000 || IsBadReadPtr(r as *const u8, 0x16) != 0 {
                    continue;
                }
                let n = d(r + 0xa);
                let nome: String = if n >= 0x10000 && nacoes.contains(&n) {
                    core::slice::from_raw_parts((n + 4) as *const u8, 51).iter().take_while(|&&c| c != 0).map(|&c| c as char).collect()
                } else {
                    "-".into()
                };
                s += &format!("regra\t{} {} vtab={:#x} escopo={}\t{}\t{}\t0\t0\n", i, nome, d(r), *((r + 0x12) as *const u8), *((r + 0x13) as *const u8), *((r + 0x14) as *const u8));
            }
        }
    }
    // T13: os pesos dados (sem excesso / joga / ficaria fora), os listados e, por nação e limite de jogo visto, os clubes,
    // os estrangeiros que eles têm agora, os que passam do limite e o maior elenco de estrangeiros
    {
        use core::sync::atomic::Ordering::Relaxed;
        s += &format!("total\testr_peso_livre_joga_fora\t{}\t{}\t{}\t0\n", ESTR_PESO[0].load(Relaxed), ESTR_PESO[1].load(Relaxed), ESTR_PESO[2].load(Relaxed));
        s += &format!("total\testr_listados\t{}\t0\t0\t0\n", ESTR_LISTADOS.load(Relaxed));
        s += &format!("total\testr_emprestimo\t{}\t0\t0\t0\n", ESTR_EMP.load(Relaxed));
        let mapa = LIMITE_JOGO.lock().unwrap().clone().unwrap_or_default();
        let mut por: std::collections::BTreeMap<(String, u8, u8), (u64, u64, u64, u64)> = Default::default();
        for (&clube, &(tipo, lim)) in mapa.iter() {
            if clube & 1 != 0 {
                continue;
            }
            let nac = unsafe { d(clube + 0x53) };
            if nac < 0x10000 {
                continue;
            }
            let nome: String = unsafe { core::slice::from_raw_parts((nac + 4) as *const u8, 51) }.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect();
            let n = unsafe { estrangeiros(clube, tipo, 0) }.len() as u64;
            let e = por.entry((nome, tipo, lim)).or_default();
            *e = (e.0 + 1, e.1 + n, e.2 + n.saturating_sub(lim as u64), e.3.max(n));
        }
        for ((nome, tipo, lim), (cl, n, sobra, mx)) in por {
            s += &format!("total\testr_jogo {} tipo{} lim{} clubes_estr_acima_max\t{}\t{}\t{}\t{}\n", nome, tipo, lim, cl, n, sobra, mx);
        }
    }
    s += &format!("total\tverba_cobre_nao_sim\t{}\t{}\t0\t0\n", VERBA_CORTES[0].load(core::sync::atomic::Ordering::Relaxed),
        VERBA_CORTES[1].load(core::sync::atomic::Ordering::Relaxed));
    unsafe {
        let f = &*core::ptr::addr_of!(FUNIL);
        let v = &*core::ptr::addr_of!(FUNIL_VONTADE);
        for t in 0..3 {
            for e in 0..8 {
                s += &format!("{}\t{}\t{}\t{}\t{}\t{}\n", 2001 + t, e, f[t][e][0], f[t][e][1], f[t][e][2], f[t][e][3]);
            }
            let ft = &*core::ptr::addr_of!(FUNIL_TETO);
            let fs = &*core::ptr::addr_of!(FUNIL_SAIDA);
            for x in 0..19 {
                s += &format!("{}\tsaida{}\t{}\t{}\t0\t0\n", 2001 + t, x, fs[t][0][x], fs[t][1][x]);
            }
            let cs = &*core::ptr::addr_of!(FUNIL_CS);
            for x in 0..64 {
                if cs[t][0][x] + cs[t][1][x] > 0 {
                    s += &format!("{}\tcs_motivo{}\t{}\t{}\t0\t0\n", 2001 + t, x, cs[t][0][x], cs[t][1][x]);
                }
            }
            for fx in 0..6 {
                s += &format!("{}\trecusa_teto{}\t{}\t{}\t0\t0\n", 2001 + t, fx, ft[t][0][fx], ft[t][1][fx]);
            }
            for k in 0..3 {
                s += &format!("{}\tvontade_cat{}\t{}\t{}\t{}\t{}\n", 2001 + t, k, v[t][k][0], v[t][k][1], v[t][k][2], v[t][k][3]);
            }
            let g = &*core::ptr::addr_of!(FUNIL_COMPRADOR);
            for x in 0..15 {
                s += &format!("{}	comprador_{}	{}	{}	{}	{}
", 2001 + t, x, g[t][0][x][0], g[t][0][x][1], g[t][1][x][0], g[t][1][x][1]);
            }
            let pr = &*core::ptr::addr_of!(FUNIL_PRECO);
            for k in 0..3 {
                s += &format!("{}	preco_cat{}	{}	{}	{}	{}	{}
", 2001 + t, k, pr[t][k][0], pr[t][k][1], pr[t][k][2], pr[t][k][3], pr[t][k][4]);
            }
            let m = &*core::ptr::addr_of!(FUNIL_MOTIVOS);
            for b in 0..32 {
                if (0..4).any(|k| m[t][k][b] > 0) {
                    s += &format!("{}\tmotivo_{:#x}\t{}\t{}\t{}\t{}\n", 2001 + t, 1u32 << b, m[t][0][b], m[t][1][b], m[t][2][b], m[t][3][b]);
                }
            }
        }
    }
    let _ = std::fs::write(caminho, s);
}
