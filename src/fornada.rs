//! A fornada de regens (EXPERIMENTAL, branch `fornada`; ver docs/PESSOAS_NOVAS.md, caminho 1, e docs/FORNADA_DE_REGENS.md).
//!
//! 1. **Criar:** o corte do banco (`database_trim_staff_tables` 0x524d70) já cria pessoas a mais na carreira nova
//!    (`extra = ftol(mantidas × 1% ou 2%)`, 0x525f8c), com ficha e preferências, antes dos 16 humanos. O `call` do `ftol`
//!    vai para um trecho que soma `intake_stock`, dentro da folga das tabelas (o `realloc` do corte só pode encolher).
//! 2. **Adormecer:** antes de `game_run_to_start_date` 0x5cc850 (chamada em 0x829fd9), as pessoas do estoque ficam
//!    aposentadas (bits 0x30 = 0x20 do +0xb do registro de 79 bytes), sem clube, fora da fila. Nos saves, todo aposentado
//!    está na fila; aposentado fora da fila é, então, o estoque (sem arquivo ao lado do save).
//! 3. **Soltar:** no dia `intake_day`, depois da fila do dia (`process_regen_queue` 0x7a65d0, chamada em 0x5cda7a), cada
//!    clube recebe `intake_per_club` jovens: a ficha da pessoa vira a cópia de um jogador real da nação do clube, com o PA
//!    da fornada, vai para a frente da reserva de fichas, e `regenerate_as_youth_player` 0x7abac0 faz o regen. A disputa
//!    (chamada em 0x7acea2) devolve o clube da fornada; se o regen sair sem clube, `player_regen_sign_for_club` 0x7a6f10.
//! 4. **No fim da temporada** (`intake_day = season_end`): o laço das 35 nações-chave (tabela 0xb63d60, 72 bytes, +0 a
//!    nação, +0x15 o dia do fim) chama `end_season_for_nation` 0x59cdd0 em 0x5d5ea2; a nação fica pendente e os clubes
//!    dela recebem na fila do dia seguinte. Clube de nação fora da tabela recebe no dia 181.

use crate::{d, registro, sorteio, w};

pub struct Fornada {
    pub estoque: i32,
    pub por_clube: i32,
    pub dia: i32,
    /// `intake_day = season_end`: cada nação-chave no fim da própria temporada
    pub por_temporada: bool,
    pub so_jogaveis: bool,
    pub pa_min: i32,
    pub pa_max: i32,
    pub pa_espalhar: i32,
    /// clube com este tanto no elenco fica de fora da fornada (o elenco tem 50 vagas)
    pub elenco_max: i32,
}

pub static mut F: Fornada = Fornada { estoque: 0, por_clube: 3, dia: 181, por_temporada: false, so_jogaveis: true, pa_min: 60, pa_max: 170, pa_espalhar: 30, elenco_max: 50 };

unsafe fn cfg() -> &'static Fornada {
    &*core::ptr::addr_of!(F)
}

/// As chaves `intake_*` do tweaks.cfg.
pub fn ler() -> Fornada {
    let mut f = Fornada { estoque: 0, por_clube: 3, dia: 181, por_temporada: false, so_jogaveis: true, pa_min: 60, pa_max: 170, pa_espalhar: 30, elenco_max: 50 };
    let caminho = std::env::current_exe().map(|p| p.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return f };
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        if k == "intake_clubs" {
            f.so_jogaveis = v != "selected"; // playable | selected
            continue;
        }
        if k == "intake_day" && v == "season_end" {
            f.por_temporada = true;
            continue;
        }
        let Ok(x) = v.parse::<i32>() else { continue };
        match k {
            "intake_stock" => f.estoque = x.clamp(0, 100_000),
            "intake_per_club" => f.por_clube = x.clamp(0, 20),
            "intake_day" => f.dia = x.clamp(0, 365),
            "intake_pa_min" => f.pa_min = x.clamp(1, 200),
            "intake_pa_max" => f.pa_max = x.clamp(1, 200),
            "intake_pa_spread" => f.pa_espalhar = x.clamp(0, 200),
            "intake_squad_max" => f.elenco_max = x.clamp(1, 50),
            _ => {}
        }
    }
    f
}

/// O estoque criado nesta sessão (carreira nova): ids [inicio, fim).
static mut CRIADOS: (usize, usize) = (0, 0);
/// O clube que a disputa devolve durante a soltura (0: a disputa normal).
static mut FORCADO: usize = 0;
/// O ano da última soltura nesta sessão.
static mut ULTIMO_ANO: i32 = -1;
/// A disputa que estava no `call` de 0x7acea2 (a original ou a do Tweaks).
static mut DISPUTA: usize = 0x7ad0e0;
/// As nações cuja temporada acabou e ainda não receberam (modo `season_end`).
static mut PENDENTES: Vec<usize> = Vec::new();

/// As nações da tabela das nações-chave (35 entradas de 72 bytes em 0xb63d60, +0 a nação).
unsafe fn nacoes_chave() -> Vec<usize> {
    (0..0x23).map(|i| d(0xb63d60 + 72 * i)).filter(|&n| n != 0).collect()
}

/// No lugar da chamada de `end_season_for_nation` 0x59cdd0 em 0x5d5ea2 (thiscall, ret 4; o argumento é a entrada da
/// nação-chave): a nação fica pendente para a fila do dia.
unsafe extern "thiscall" fn fim_da_temporada(this: usize, entrada: usize) {
    let original: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x59cdd0usize);
    original(this, entrada);
    let nac = d(entrada);
    let pend = &mut *core::ptr::addr_of_mut!(PENDENTES);
    if nac != 0 && !pend.contains(&nac) {
        pend.push(nac);
    }
}

/// Depois do `ftol` do corte (cdecl): o extra do jogo, as pessoas, fichas e preferências mantidas. Devolve o extra novo,
/// dentro da folga: pessoas `mantidas + extra + 0x52d ≤ N_antes` (o teto da 0x525f9b, que com defeito soma em vez de
/// subtrair), fichas `mantidas + extra ≤ F_antes`, preferências `mantidas + extra ≤ P_antes`.
unsafe extern "C" fn extra_do_corte(extra: i32, pessoas: i32, fichas: i32, prefs: i32) -> i32 {
    let (n, f, p) = (d(0xae2364) as i32, d(0xae236c) as i32, d(0xae2370) as i32);
    let folga = (n - pessoas - 0x52d - extra).min(f - fichas - extra).min(p - prefs - extra).max(0);
    let k = cfg().estoque.min(folga);
    let total = extra + k;
    let fim = (pessoas + total - 16) as usize; // as extras ficam em N−17−i; as K primeiras (i < K) são o estoque
    CRIADOS = (fim - k as usize, fim);
    registro::linha(&format!("intake: stock {} people created with the career (asked {}, room {}; the game's own extra {}; ids {}..{})",
        k, cfg().estoque, folga, extra, fim - k as usize, fim));
    total
}

unsafe fn registro79(id: usize) -> *mut u8 {
    (d(0xae2d0c) + 79 * id + 0xb) as *mut u8
}

/// No lugar da chamada de `game_run_to_start_date` (0x829fd9): adormece o estoque, depois roda o original.
unsafe extern "C" fn antes_de_rodar() -> u32 {
    let (ini, fim) = CRIADOS;
    let (pessoas, mut n) = (d(0xae23bc), 0);
    for id in ini..fim {
        let p = pessoas + 110 * id;
        if d(p) == id && d(p + 0x39) == 0 && d(p + 0x61) != 0 {
            let r = registro79(id);
            *r = (*r & 0xcf) | 0x20;
            n += 1;
        }
    }
    if fim > ini {
        registro::linha(&format!("intake: {} of {} stock people put to sleep (retired, outside the regen queue)", n, fim - ini));
    }
    let original: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x5cc850usize);
    original()
}

/// A disputa no `call` de 0x7acea2 (thiscall, ret 4).
unsafe extern "thiscall" fn disputa(this: usize, jogador: usize) -> usize {
    if FORCADO != 0 {
        return FORCADO;
    }
    let original: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(DISPUTA);
    original(this, jogador)
}

/// No lugar da chamada de `process_regen_queue` em 0x5cda7a (thiscall): a fila do dia e, no dia da fornada, a soltura.
unsafe extern "thiscall" fn fila_do_dia(this: usize) {
    let original: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7a65d0usize);
    original(this);
    if cfg().por_clube <= 0 {
        return;
    }
    let (dia, ano) = (w(0xae2c90) as i32, w(0xae2c92) as i32);
    if cfg().por_temporada {
        let pend = core::mem::take(&mut *core::ptr::addr_of_mut!(PENDENTES));
        if !pend.is_empty() {
            soltar(this, &|n| pend.contains(&n));
        }
        // as nações fora da tabela das nações-chave: no dia 181
        if dia == 181 && ano != ULTIMO_ANO {
            ULTIMO_ANO = ano;
            let chave = nacoes_chave();
            soltar(this, &|n| !chave.contains(&n));
        }
    } else if dia == cfg().dia && ano != ULTIMO_ANO {
        ULTIMO_ANO = ano;
        soltar(this, &|_| true);
    }
}

/// Os adormecidos: aposentados (bits 0x30 = 0x20), sem clube, com ficha, fora da fila e abaixo dos humanos.
unsafe fn adormecidos(this: usize) -> Vec<usize> {
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    let fila: std::collections::HashSet<usize> = (0..d(this + 0x14)).map(|k| d(d(this + 0x10) + 12 * k)).collect();
    (0..n.saturating_sub(16))
        .filter(|&id| (*registro79(id) & 0x30) == 0x20 && !fila.contains(&id))
        .map(|id| pessoas + 110 * id)
        .filter(|&p| d(p + 0x39) == 0 && d(p + 0x61) != 0)
        .collect()
}

/// Quantas vagas ocupadas no elenco (+0xd7, 50).
unsafe fn elenco(clube: usize) -> usize {
    (0..50).filter(|&k| d(clube + 0xd7 + 4 * k) != 0).count()
}

unsafe fn soltar(this: usize, da_nacao: &dyn Fn(usize) -> bool) {
    let mut estoque = adormecidos(this);
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    // os moldes: jogadores reais (com ficha, não aposentados, não humanos) por nação
    let mut moldes: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    for id in 0..n.saturating_sub(16) {
        let p = pessoas + 110 * id;
        if d(p + 0x61) != 0 && d(p + 0x1a) != 0 && (*registro79(id) & 0x30) == 0 {
            moldes.entry(d(p + 0x1a)).or_default().push(p);
        }
    }
    let total = (d(0xae235c) as i32 - 2 * d(0xae2350) as i32).max(0) as usize;
    let base = d(0xae23b4);
    let clubes: Vec<usize> = (0..total)
        .map(|i| base + 0x245 * i)
        .filter(|&c| {
            let nac = d(c + 0x53);
            nac != 0 && da_nacao(nac) && crate::em_competicao(c) && if cfg().so_jogaveis { *((nac + 0x11c) as *const u8) == 2 } else { *((nac + 0x11c) as *const u8) != 0 }
        })
        .collect();
    let (mut dados, mut cheios, mut sem_molde, mut falhas) = (0, 0, 0, 0);
    'clubes: for &c in &clubes {
        for _ in 0..cfg().por_clube {
            if elenco(c) >= cfg().elenco_max as usize {
                cheios += 1;
                continue 'clubes;
            }
            let Some(p) = estoque.pop() else { break 'clubes };
            let nac = d(c + 0x53);
            let Some(lista) = moldes.get(&nac).filter(|l| !l.is_empty()) else {
                sem_molde += 1;
                estoque.push(p);
                continue 'clubes;
            };
            let molde = d(lista[sorteio(lista.len() as i32) as usize] + 0x61);
            let ficha = d(p + 0x61);
            let id_ficha = d(ficha);
            core::ptr::copy_nonoverlapping(molde as *const u8, ficha as *mut u8, 70);
            *(ficha as *mut usize) = id_ficha;
            *((ficha + 4) as *mut u8) = 0;
            *((ficha + 0x45) as *mut u8) = 10;
            let rep = w(c + 0x80) as i32;
            let meio = cfg().pa_min + (cfg().pa_max - cfg().pa_min) * rep.clamp(0, 10000) / 10000;
            let pa = if cfg().pa_espalhar > 0 { meio + sorteio(cfg().pa_espalhar + 1) - cfg().pa_espalhar / 2 } else { meio };
            // intake_pa_max é o teto de verdade: o espalhamento não passa dele (na 1ª versão ia até 200)
            let (lo, hi) = (cfg().pa_min.min(cfg().pa_max), cfg().pa_min.max(cfg().pa_max));
            *((ficha + 7) as *mut i16) = pa.clamp(lo, hi) as i16;
            por_na_frente_da_reserva(this, id_ficha, d(nac));
            FORCADO = c;
            let regen: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7abac0usize);
            regen(this, p);
            FORCADO = 0;
            if d(p + 0x39) == 0 {
                let assinar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x7a6f10usize);
                assinar(this, p, c);
            }
            if d(p + 0x39) == c {
                dados += 1;
            } else {
                falhas += 1;
            }
            // intake-regen  data  idade  PA  CA  goleiro/defesa/meia/atacante (+0xf, +0x11, +0x13, +0x15)  clube (vazio: sem)
            let f = d(p + 0x61);
            registro::linha(&format!("intake-regen	{}	{}	{}	{}	{}/{}/{}/{}	{}", crate::data_do_jogo(), crate::b(p + 0x18), w(f + 7), w(f + 5),
                crate::b(f + 0xf), crate::b(f + 0x11), crate::b(f + 0x13), crate::b(f + 0x15), if d(p + 0x39) != 0 { crate::nome(d(p + 0x39)) } else { String::new() }));
        }
    }
    if clubes.is_empty() {
        return;
    }
    registro::linha(&format!("intake\t{}\tclubs {}\tgiven {}\tfull {}\tno template {}\tleft without the club {}\tstock left {}",
        crate::data_do_jogo(), clubes.len(), dados, cheios, sem_molde, falhas, estoque.len()));
}

/// A entrada {ficha, nação, 0, −1} na frente da reserva de fichas (+0x3c ponteiro, +0x40 quantidade, +0x44 capacidade),
/// crescendo com o `realloc` do jogo (0x945667) como a 0x7a8350.
unsafe fn por_na_frente_da_reserva(this: usize, ficha: usize, nacao: usize) {
    let (mut v, q, mut cap) = (d(this + 0x3c), d(this + 0x40), d(this + 0x44));
    if q >= cap {
        cap += 250;
        let realloc: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
        v = realloc(v, cap * 16);
        *((this + 0x3c) as *mut usize) = v;
        *((this + 0x44) as *mut usize) = cap;
    }
    core::ptr::copy((v) as *const u8, (v + 16) as *mut u8, q * 16);
    let e = v as *mut i32;
    *e = ficha as i32;
    *e.add(1) = nacao as i32;
    *e.add(2) = 0;
    *e.add(3) = -1;
    *((this + 0x40) as *mut usize) = q + 1;
}

/// O `call rel32` (E8) em `em` aponta para `antes`?
unsafe fn e_call(em: usize, antes: usize) -> bool {
    let ok = *(em as *const u8) == 0xe8 && (core::ptr::read_unaligned((em + 1) as *const i32) as usize).wrapping_add(em + 5) == antes;
    if !ok {
        registro::linha(&format!("intake: the call at {:#x} isn't the 3.9.68 one; nothing was changed", em));
    }
    ok
}

/// Troca o alvo do `call rel32` em `em`.
unsafe fn trocar_call(em: usize, para: usize) {
    (*core::ptr::addr_of_mut!(crate::MARCAS)).push((em, 5));
    let mut antiga = 0u32;
    crate::VirtualProtect(em as *mut core::ffi::c_void, 5, 0x40, &mut antiga);
    core::ptr::write_unaligned((em + 1) as *mut i32, para.wrapping_sub(em + 5) as i32);
    crate::VirtualProtect(em as *mut core::ffi::c_void, 5, antiga, &mut antiga);
}

pub unsafe fn instalar() -> bool {
    F = ler();
    if cfg().estoque <= 0 && cfg().por_clube <= 0 {
        return false;
    }
    // o trecho do ftol: call ftol; push [esp+0x54] (preferências); push [esp+0x60] (fichas); push [esp+0x2c] (pessoas);
    // push eax; call extra_do_corte; add esp, 0x10; ret
    let t = crate::VirtualAlloc(core::ptr::null_mut(), 64, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return false;
    }
    let mut c: Vec<u8> = vec![0xe8, 0, 0, 0, 0];
    c.extend_from_slice(&[0xff, 0x74, 0x24, 0x54, 0xff, 0x74, 0x24, 0x60, 0xff, 0x74, 0x24, 0x2c, 0x50, 0xe8, 0, 0, 0, 0, 0x83, 0xc4, 0x10, 0xc3]);
    let base = t as usize;
    core::ptr::write_unaligned(c.as_mut_ptr().add(1) as *mut i32, 0x945640usize.wrapping_sub(base + 5) as i32);
    core::ptr::write_unaligned(c.as_mut_ptr().add(19) as *mut i32, (extra_do_corte as *const () as usize).wrapping_sub(base + 23) as i32);
    core::ptr::copy_nonoverlapping(c.as_ptr(), t, c.len());
    // a disputa: o call de 0x7acea2 pode já apontar para a 0x7ad0e0 desviada (rule = new); o alvo atual fica guardado
    let ok = e_call(0x525f8c, 0x945640) && e_call(0x829fd9, 0x5cc850) && e_call(0x5cda7a, 0x7a65d0) && e_call(0x5d5ea2, 0x59cdd0) && *(0x7acea2 as *const u8) == 0xe8;
    if ok {
        DISPUTA = (core::ptr::read_unaligned(0x7acea3 as *const i32) as usize).wrapping_add(0x7acea7);
        trocar_call(0x525f8c, base);
        trocar_call(0x829fd9, antes_de_rodar as *const () as usize);
        trocar_call(0x5cda7a, fila_do_dia as *const () as usize);
        trocar_call(0x7acea2, disputa as *const () as usize);
        if cfg().por_temporada {
            trocar_call(0x5d5ea2, fim_da_temporada as *const () as usize);
        }
    }
    if ok {
        registro::linha(&format!("intake (EXPERIMENTAL): on (stock {}, {} per club on {}, {} clubs, PA {}..{} by reputation, spread {}, squads up to {})",
            cfg().estoque, cfg().por_clube, if cfg().por_temporada { "each nation's season end".to_string() } else { format!("day {}", cfg().dia) }, if cfg().so_jogaveis { "playable-league" } else { "selected-league" }, cfg().pa_min, cfg().pa_max, cfg().pa_espalhar, cfg().elenco_max));
    }
    ok
}
