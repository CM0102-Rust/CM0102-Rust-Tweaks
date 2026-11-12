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
}

static mut C: M = M { jovem_aceita: false, jovem_lista: false, jovem_idade: 21, jovem_gap: 30, jovem_rep: 0.75, destaque: 1.0,
    destaque_dif: 1500, lance: false, teto_meio: false, chave_preco: 1.0, chave_rep: 6500, fix_clausula: false, fix_workrate: false, negocio: 0.0, jovem_treino: -99,
    emp_regra: false, lista_emp: false, idade_rodagem: 21, vaga_jovem: 2, vaga_outros: 2, treino_folga: 3, cobertura_min: 2,
    cobertura_reserva: 1, cobertura_ratio: 0.9, vontade_pedido: 3.5, pedido_div: 2, fator_casa: 1.0, fator_minutos: 1.0,
    insatisfacao: 0, insat_dif: 1000, sem_teto_nacao: false, procura_jovem: 1.0, elenco_min: 0 };
static PROCURADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
/// O limiar de vontade do pedido de empréstimo (o fcomp em 0x8427f0 passa a ler daqui).
static mut LIMIAR_PEDIDO: f64 = 3.5;
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

/// E: o sorteio da nota final do candidato (0x840610, `push 0x11; call 0x90cfc0`) com a pessoa (esi), a vaga ([ebp+8]) e o
/// clube que procura ([ebp+0xc]) do quadro da 0x83f3a0.
#[unsafe(naked)]
unsafe extern "C" fn sorteio_nota() {
    core::arch::naked_asm!("push dword ptr [ebp + 0xc]", "push dword ptr [ebp + 8]", "push esi", "push dword ptr [esp + 0x10]",
        "call {f}", "add esp, 16", "ret", f = sym nota_sorteio);
}

unsafe extern "C" fn nota_sorteio(n: u32, p: usize, vaga: usize, clube: usize) -> i32 {
    let sorteio: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let r = sorteio(n);
    if p < 0x10000 || vaga < 0x10000 || clube < 0x10000 || *((vaga + 0x24) as *const u8) & 1 == 0 || d(p + 0x61) == 0
        || (b(p + 0x18) as i32) > C.idade_rodagem {
        return r;
    }
    let dono = d(p + 0x39);
    if dono < 0x10000 || dono == clube || posto(clube, p) > C.vaga_jovem
        || (!volta_para_casa(p, dono, clube) && (b(clube + 0x7f) as i32) < b(dono + 0x7f) as i32 - C.treino_folga) {
        return r;
    }
    PROCURADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    ((92 + r) as f64 * C.procura_jovem) as i32 - 92
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
            && *((k + 0x35) as *const u8) & 0x3f == 2 && *((k + 0x49) as *const u8) == 0 && emprestavel(clube, p, k, false));
        if lista {
            *((k + 0x4e) as *mut u8) |= 2;
            LISTADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        }
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
    format!("loan_listed {} loan_accepted {} loan_real_yes {} loan_real_no {} unhappy {} young_sought {}", LISTADOS.swap(0, Relaxed), ACEITOS.swap(0, Relaxed),
        EMP_ACEITOS.swap(0, Relaxed), EMP_RECUSADOS.swap(0, Relaxed), INSATISFEITOS.swap(0, Relaxed), PROCURADOS.swap(0, Relaxed))
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
    if c.jovem_lista || c.lista_emp {
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
    if c.procura_jovem != 1.0 {
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
