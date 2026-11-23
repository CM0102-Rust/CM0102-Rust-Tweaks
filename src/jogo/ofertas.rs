//! `manager_offers` (EXPERIMENTAL, desligada; docs/OFERTAS_AO_TECNICO.md): o técnico humano como candidato de verdade
//! às vagas de técnico de clube. O jogo só manda proposta ao humano quando ele vence a vaga (`add_job_vacancy`
//! 0x69ba50) contra todos os técnicos da IA, e mistura duas perguntas:
//! - "ele aceitaria vir?" (o lado do técnico: o interesse 0x844780, zero no primeiro ano no cargo, e a exigência de
//!   subir em `is_staff_suitable_for_job` 0x69f810). Para o humano quem responde é o jogador, ao receber a proposta:
//!   com a chave, a adequação do humano é só o freio "um passo acima" (desempregado, ou um clube de reputação maior
//!   que a do atual);
//! - "o clube o quer?" (a nota `score_staff_for_job` 0x69d1b0). Para o humano o jogo pula a parte de atributos e
//!   personalidade que dá à IA (de 4.000 a 6.000 nos vencedores medidos); com a chave, ela vira uma nota por mérito,
//!   do que o próprio jogo registra: a confiança da diretoria do clube atual (o registro de 0x49 bytes do
//!   MANAGER_MANAGER, +6..+0xc), o aproveitamento e os títulos e taças do registro do técnico (`Staff_Records`
//!   [0xdd7f00], 0x86f5b0).
//! Os filtros de clube (estrangeiro, região, ligas em andamento) e as vagas de seleção ficam como no original.
//! ponytail: a escala (6.000) e os pesos (0,5 / 0,3 / 0,2) são do estudo de 23/11; viram chave se a medição pedir.

use super::*;

/// As chamadas de 0x69f810 nos dois laços de candidatos de 0x69ba50 e a de 0x69d1b0 no laço principal.
const ADEQUACAO: [usize; 2] = [0x69bdf7, 0x69bf77];
const NOTA: usize = 0x69be55;
const ESCALA: f64 = 6000.0;
static mut T_ADEQUACAO: [usize; 2] = [0; 2];
static mut T_NOTA: usize = 0;

pub(crate) unsafe fn humano(p: usize) -> bool {
    let n = d(0xae2364);
    p != 0 && d(p) < n && d(p) + 16 >= n && b(p + 0x5f) == 1
}

unsafe fn selecao(c: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b900usize);
    f(c) != 0
}

/// O freio: desempregado, ou a vaga num clube de reputação maior que a do clube atual.
unsafe fn passo_acima(p: usize, clube: usize) -> bool {
    let meu = d(p + 0x39);
    meu == 0 || (meu != clube && w(clube + 0x80) > w(meu + 0x80))
}

/// A nota por mérito do humano, de 0 a ESCALA: 0,5 da confiança da diretoria (a média das quatro notas, de 2.000 a
/// 10.000; desempregado conta 0,4), 0,3 do aproveitamento (de 20% a 60% de vitórias, com pelo menos 10 jogos; menos,
/// conta 0,4) e 0,2 dos títulos e taças (dois títulos de liga, ou um título e duas taças, enchem).
pub(crate) unsafe fn merito(mm: usize, p: usize) -> i32 {
    let meu = d(p + 0x39);
    let confianca = if mm != 0 && d(mm) != 0 && meu != 0 && !selecao(meu) {
        let r = d(mm) + d(meu) * 0x49;
        let media = (w(r + 6) as f64 + w(r + 8) as f64 + w(r + 0xa) as f64 + w(r + 0xc) as f64) / 4.0;
        ((media - 2000.0) / 8000.0).clamp(0.0, 1.0)
    } else {
        0.4
    };
    let sr = d(0xdd7f00);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let rec = if sr != 0 { registro(sr, p, 0) } else { 0 };
    let (aproveitamento, titulos) = if rec != 0 {
        let (jogos, vitorias) = (w(rec + 0x8e) as f64, w(rec + 0x90) as f64);
        let a = if jogos >= 10.0 { ((vitorias / jogos - 0.2) / 0.4).clamp(0.0, 1.0) } else { 0.4 };
        let t = ((2.0 * w(rec + 0x8c) as f64 + w(rec + 0x8a) as f64) / 4.0).clamp(0.0, 1.0);
        (a, t)
    } else {
        (0.4, 0.0)
    };
    (ESCALA * (0.5 * confianca + 0.3 * aproveitamento + 0.2 * titulos)) as i32
}

unsafe fn adequado(k: usize, this: usize, p: usize, clube: usize, minimo: f64) -> u8 {
    if humano(p) && clube != 0 && !selecao(clube) {
        return passo_acima(p, clube) as u8;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8 = core::mem::transmute(T_ADEQUACAO[k]);
    f(this, p, clube, minimo)
}
unsafe extern "thiscall" fn adequado0(this: usize, p: usize, clube: usize, minimo: f64) -> u8 {
    adequado(0, this, p, clube, minimo)
}
unsafe extern "thiscall" fn adequado1(this: usize, p: usize, clube: usize, minimo: f64) -> u8 {
    adequado(1, this, p, clube, minimo)
}

unsafe extern "thiscall" fn nota(this: usize, p: usize, clube: usize, visto: usize, cargo: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> i32 = core::mem::transmute(T_NOTA);
    let n = f(this, p, clube, visto, cargo);
    if n > -10000 && humano(p) && clube != 0 && !selecao(clube) {
        return n + merito(this, p);
    }
    n
}

/// O alvo do `call rel32` em `em`, se for um `call`.
unsafe fn alvo(em: usize) -> Option<usize> {
    (*(em as *const u8) == 0xe8).then(|| (core::ptr::read_unaligned((em + 1) as *const i32) as usize).wrapping_add(em + 5))
}

pub(crate) unsafe fn instalar() {
    if !assumir::chave("tweaks.cfg", "manager_offers") {
        return;
    }
    let (Some(a0), Some(a1), Some(n)) = (alvo(ADEQUACAO[0]), alvo(ADEQUACAO[1]), alvo(NOTA)) else {
        return registro::linha("manager offers: the vacancy loop's calls (0x69bdf7, 0x69bf77, 0x69be55) differ from 3.9.68; off");
    };
    T_ADEQUACAO = [a0, a1];
    T_NOTA = n;
    chamar(ADEQUACAO[0], adequado0 as *const () as usize);
    chamar(ADEQUACAO[1], adequado1 as *const () as usize);
    chamar(NOTA, nota as *const () as usize);
    registro::linha("manager offers: on (a human manager is a candidate for any club vacancy a step up, scored by merit: board confidence, win rate, titles)");
}
