//! Reputação e público (EXPERIMENTAL, branch `estudos`): o sucesso pesar mais. Três chaves, todas desligadas no padrão
//! (o valor que reproduz o original); cada uma troca só o `call` de uma rotina por um embrulho que chama a original e
//! depois ajusta o que ela gravou. Ver docs/REPUTACAO_E_PUBLICO.md.
//! - `player_rep_speed` (1 = original): a mudança das três reputações do jogador por partida (0x6d26b0, chamada em
//!   0x6d2112) vezes isso, para cima e para baixo; o teto da subida fica o da habilidade (CA × 50 + 2500; a mundial
//!   CA × 50 + 1250), sem o teto da liga (4000 + 300 × nível), que trava as ligas de reputação baixa.
//! - `club_rep_speed` (1 = original): a subida da reputação do clube por partida (0x6d3de0, chamada em 0x6d2684) vezes isso;
//!   a queda fica a original.
//! - `attendance_pull` (0 = original): no fim da temporada de cada liga (0x5d6280, chamada em 0x5d5cfb), o público
//!   médio anda essa fração do caminho até a capacidade do estádio × min(1, reputação / `attendance_full_rep`), só para
//!   cima; e o recorde (+0x7b, o teto do público de cada jogo) sobe até min(capacidade, média × `attendance_peak`).
//!   `attendance_full_rep = league`: a maior reputação da própria liga, para os pequenos de cada liga também lotarem.
//!   No original o recorde só sobe com título ou acesso, então o clube que vai bem sem ganhar nada fica preso.
//! - `loan_rule = squad` (original = a regra do jogo): o clube da IA também empresta o jogador de estatuto 4 a 7 no elenco
//!   (reserva, promessa, jovem, dispensável) quando a reputação do clube que pede é pelo menos `loan_rep_ratio` × a
//!   reputação atual do jogador. No original (0x84af00, 7 chamadas) só empresta o listado para empréstimo ou o
//!   dispensável em má fase; salário e período não entram.
//! ponytail: confere só o `call` (E8 para a original) antes de trocar, sem as assinaturas das partes; entra nelas se virar chave.

use super::*;

#[derive(Clone, Copy)]
pub(crate) struct Sucesso {
    pub(crate) rep_jogador: f64,
    pub(crate) rep_clube: f64,
    pub(crate) puxar: f64,
    pub(crate) rep_cheio: f64,
    pub(crate) pico: f64,
    pub(crate) emprestimo: bool,
    pub(crate) emprestimo_rep: f64,
}

const PADRAO: Sucesso = Sucesso { rep_jogador: 1.0, rep_clube: 1.0, puxar: 0.0, rep_cheio: 7000.0, pico: 1.25, emprestimo: false, emprestimo_rep: 0.75 };
static mut S: Sucesso = PADRAO;

fn ler() -> Sucesso {
    let mut s = PADRAO;
    let caminho = std::env::current_exe().map(|p| p.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return s };
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        if k.trim() == "attendance_full_rep" && v.trim() == "league" {
            s.rep_cheio = 0.0; // relativo: a maior reputação da própria liga lota
            continue;
        }
        if k.trim() == "loan_rule" {
            s.emprestimo = v.trim() == "squad";
            continue;
        }
        let Ok(x) = v.trim().parse::<f64>() else { continue };
        match k.trim() {
            "player_rep_speed" => s.rep_jogador = x.clamp(0.0, 20.0),
            "club_rep_speed" => s.rep_clube = x.clamp(0.0, 20.0),
            "attendance_pull" => s.puxar = x.clamp(0.0, 1.0),
            "attendance_full_rep" => s.rep_cheio = x.clamp(500.0, 10000.0),
            "attendance_peak" => s.pico = x.clamp(1.0, 3.0),
            "loan_rep_ratio" => s.emprestimo_rep = x.clamp(0.0, 5.0),
            _ => {}
        }
    }
    s
}

/// O `call` em `de` ainda chama `alvo` (o exe é o 3.9.68 ali)?
unsafe fn chama(de: usize, alvo: usize) -> bool {
    *(de as *const u8) == 0xe8 && (de + 5).wrapping_add(core::ptr::read_unaligned((de + 1) as *const i32) as usize) == alvo
}

unsafe fn trocar(nome: &str, de: usize, alvo: usize, para: usize, texto: String) {
    if chama(de, alvo) {
        chamar(de, para);
        registro::linha(&format!("{}: on ({})", nome, texto));
    } else {
        registro::linha(&format!("{}: this exe differs from 3.9.68 at {:#x}; nothing was changed", nome, de));
    }
}

pub(crate) unsafe fn instalar() {
    let s = ler();
    S = s;
    if s.rep_jogador != 1.0 {
        trocar("player reputation", 0x6d2112, 0x6d26b0, jogador as *const () as usize, format!("changes per match x {}", s.rep_jogador));
    }
    if s.rep_clube != 1.0 {
        trocar("club reputation", 0x6d2684, 0x6d3de0, clube as *const () as usize, format!("changes per match x {}", s.rep_clube));
    }
    if s.puxar > 0.0 {
        trocar("attendance", 0x5d5cfb, 0x5d6280, publico as *const () as usize,
            format!("season end: {} of the way to capacity x reputation / {}; record up to average x {}", s.puxar,
                if s.rep_cheio > 0.0 { s.rep_cheio.to_string() } else { "the league's best".into() }, s.pico));
    }
    if s.emprestimo {
        let sitios = [0x784b4d, 0x784e0b, 0x84049c, 0x8405a2, 0x8435c3, 0x8c6560, 0x8c6a1b];
        let todos = sitios.iter().all(|&de| chama(de, 0x84af00));
        if todos {
            for de in sitios {
                chamar(de, emprestimo as *const () as usize);
            }
            registro::linha(&format!("loans: squad (AI clubs also lend squad status 4 to 7 when the asking club's reputation is {} x the player's or more)", s.emprestimo_rep));
        } else {
            registro::linha("loans: this exe differs from 3.9.68 where the loan rule is called; nothing was changed");
        }
    }
}

/// { 0x84af00 } embrulhada: a resposta do vendedor (0 aceita, 1 negocia, 2 recusa).
unsafe extern "stdcall" fn emprestimo(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, e: u32) -> u32 {
    let original: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u32 = core::mem::transmute(0x84af00usize);
    let r = original(vendedor, jogador, oferta, comprador, pedido, e);
    if r as u8 != 2 || e as u8 == 0 || vendedor == 0 || comprador == 0 || d(vendedor + 0xcf) == jogador {
        return r;
    }
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let k = com_clube(d(0xae1050), jogador, vendedor);
    let ficha = d(jogador + 0x61);
    // só quem está no vendedor e sem contrato com quem pede: o emprestado fica nos dois elencos, e emprestá-lo de novo ao
    // clube onde já está o põe duas vezes na escalação (caixa match_man.cpp 1225, carreira D de 07/11, em 2005)
    if k == 0 || ficha == 0 || *((k + 0x35) as *const u8) & 0x3f != 2 || humano(vendedor, 0, 1, 0, 0) != 0
        || vendedor == comprador || d(jogador + 0x39) != vendedor || com_clube(d(0xae1050), jogador, comprador) != 0 {
        return r;
    }
    let estatuto = *((k + 0x4f) as *const u8) >> 4;
    if (4..=7).contains(&estatuto) && w(comprador + 0x80) as f64 >= S.emprestimo_rep * w(ficha + 0xb) as f64 {
        return (r & !0xff) as u32; // 0: aceita
    }
    r
}

fn escalar(antes: i32, depois: i32, k: f64, teto: i32) -> i16 {
    let delta = depois - antes;
    if delta == 0 {
        return depois as i16;
    }
    let mut v = (antes as f64 + delta as f64 * k).round() as i32;
    if delta > 0 {
        v = v.min(teto.max(depois));
    }
    v.clamp(1, 10000) as i16
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn jogador(jm: usize, pessoa: usize, clube: usize, rep: usize, comp: usize, nota: f32, a: u32, b: u32) {
    let original: unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32) = core::mem::transmute(0x6d26b0usize);
    let ficha = d(pessoa + 0x61);
    if ficha == 0 {
        return original(jm, pessoa, clube, rep, comp, nota, a, b);
    }
    // a original parte da ficha (+0xb atual, +9 em casa, +0xd mundial) e grava em jm +0x47/+0x49/+0x4b (sem jm, na ficha)
    let antes = [w(ficha + 0xb) as i32, w(ficha + 9) as i32, w(ficha + 0xd) as i32];
    original(jm, pessoa, clube, rep, comp, nota, a, b);
    let onde = if jm != 0 { [jm + 0x47, jm + 0x49, jm + 0x4b] } else { [ficha + 0xb, ficha + 9, ficha + 0xd] };
    let ca = w(ficha + 5) as i32;
    let teto = [ca * 50 + 2500, ca * 50 + 2500, ca * 50 + 1250];
    for i in 0..3 {
        let v = escalar(antes[i], w(onde[i]) as i32, S.rep_jogador, teto[i]);
        core::ptr::write_unaligned(onde[i] as *mut i16, v);
    }
}

unsafe extern "C" fn clube(this: usize, lado: u32, valor: f32, alt: u32, bonus: u32) {
    let original: unsafe extern "C" fn(usize, u32, f32, u32, u32) = core::mem::transmute(0x6d3de0usize);
    let partida = d(this + 0x4792);
    let lado8 = lado as u8 as i8;
    let meu = if partida != 0 { d(partida + if lado8 != 0 { 0x20 } else { 0x1c }) } else { 0 };
    let antes = if meu != 0 { w(meu + 0x80) as i32 } else { -1 };
    original(this, lado, valor, alt, bonus);
    let onde = this + 0x1d6 + 2 * lado8 as isize as usize;
    // só a subida: a queda vezes k derrubava a média de todos (carreira B de 07/11: 249 clubes caíram 1000+, 164 no original)
    if antes > 0 && w(onde) as i32 > antes {
        let v = escalar(antes, w(onde) as i32, S.rep_clube, 10000);
        core::ptr::write_unaligned(onde as *mut i16, v);
    }
}

unsafe extern "C" fn publico(liga: usize) {
    let original: unsafe extern "C" fn(usize) = core::mem::transmute(0x5d6280usize);
    original(liga);
    let ordenada: unsafe extern "thiscall" fn(usize, u32, *mut i32, u32) -> usize = core::mem::transmute(0x4a4b20usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut n = 0i32;
    let t = ordenada(d(liga + 8), 1, &mut n, 1000);
    if t == 0 {
        return;
    }
    let s = S;
    let clube_i = |i: usize| d(0xae23b4) + 0x245 * d(t + 0x76 * i);
    let cheio = if s.rep_cheio > 0.0 { s.rep_cheio } else { (0..n.max(0) as usize).map(|i| w(clube_i(i) + 0x80) as f64).fold(1.0, f64::max) };
    for i in 0..n.max(0) as usize {
        let c = clube_i(i);
        let estadio = d(c + 0x69);
        if estadio == 0 {
            continue;
        }
        let cap = d(estadio + 0x3c) as i32;
        if cap <= 0 {
            continue;
        }
        let alvo = cap as f64 * (w(c + 0x80) as f64 / cheio).clamp(0.0, 1.0);
        let media = d(c + 0x73) as i32;
        let nova = if alvo > media as f64 { media + ((alvo - media as f64) * s.puxar) as i32 } else { media };
        let pico = cap.min((nova as f64 * s.pico) as i32);
        if pico > d(c + 0x7b) as i32 {
            core::ptr::write_unaligned((c + 0x7b) as *mut i32, pico);
        }
        core::ptr::write_unaligned((c + 0x73) as *mut i32, nova.min(d(c + 0x7b) as i32));
    }
    soltar(t);
}
