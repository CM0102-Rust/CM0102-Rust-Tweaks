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
//! 5. **Só a fornada** (`intake_only = 1`, o padrão): a fila do dia roda com a contagem da fila zerada, então o laço dos
//!    nascimentos (0x7a65f0..0x7a66b7) não anda e a outra metade da 0x7a65d0 (a reposição dos não jogadores, +0x1c/+0x20)
//!    segue normal. Os jogadores aposentados que esperam na fila, com a ficha ainda na reserva, viram estoque da fornada
//!    (usados antes do estoque criado): saem da fila e a ficha sai da reserva, para ninguém mais herdar a mesma ficha.
//! 6. **A mensagem** (`intake_news = 1`): o clube de técnico humano recebe na caixa de entrada uma notícia do próprio jogo
//!    com os jovens que chegaram e o parecer de cada um (o nível de agora comparado com o elenco e o potencial, em faixas;
//!    os números só com `intake_news_numbers = 1`). É uma mensagem entre técnicos (tipo 0x1388, a do jogo em rede): sem a
//!    DLL, o jogo mostra o texto curto guardado nas vagas dela (a partir da vaga 3); com a DLL, o escritor das notícias
//!    (`NewsItem::write_text` 0x77bc80, desviado na entrada) escreve o texto completo na língua do jogo (português,
//!    espanhol, senão inglês). Os dados ficam nas vagas da notícia, que o save já guarda (news.dat).

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
    /// `intake_only = 1`: os regens jogadores só nascem na fornada (a fila do jogo para de criá-los)
    pub so_fornada: bool,
    /// `intake_news`: a mensagem na caixa de entrada do técnico humano
    pub mensagem: bool,
    /// `intake_news_numbers`: o CA e o PA na mensagem, além das faixas
    pub numeros: bool,
}

pub static mut F: Fornada = Fornada { estoque: 0, por_clube: 3, dia: 181, por_temporada: false, so_jogaveis: true, pa_min: 60, pa_max: 170, pa_espalhar: 30, elenco_max: 50, so_fornada: true, mensagem: true, numeros: false };

unsafe fn cfg() -> &'static Fornada {
    &*core::ptr::addr_of!(F)
}

/// As chaves `intake_*` do tweaks.cfg.
pub fn ler() -> Fornada {
    let mut f = Fornada { estoque: 0, por_clube: 3, dia: 181, por_temporada: false, so_jogaveis: true, pa_min: 60, pa_max: 170, pa_espalhar: 30, elenco_max: 50, so_fornada: true, mensagem: true, numeros: false };
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
            "intake_only" => f.so_fornada = x != 0,
            "intake_news" => f.mensagem = x != 0,
            "intake_news_numbers" => f.numeros = x != 0,
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
    // a mesma conferência do começo da 0x59cdd0 (0x456770: a data de início, 8 bytes por valor, depois de hoje?): na
    // preparação da carreira o laço passa por todas as nações, e o original não fecha temporada nenhuma
    let depois_de_hoje: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x456770usize);
    if depois_de_hoje(0xae2c90, d(0xdd6ec8), d(0xdd6ecc)) != 0 {
        return;
    }
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
    if cfg().so_fornada {
        // a contagem da fila zerada só durante a chamada: o laço dos nascimentos não anda, o resto da rotina sim
        let n = d(this + 0x14);
        *((this + 0x14) as *mut usize) = 0;
        original(this);
        *((this + 0x14) as *mut usize) = n;
    } else {
        original(this);
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

/// Os adormecidos: aposentados (bits 0x30 = 0x20), sem clube, com ficha, fora da fila e abaixo dos humanos. Com
/// `intake_only`, no fim (os primeiros a sair) os jogadores aposentados da fila cuja ficha ainda está na reserva.
unsafe fn adormecidos(this: usize) -> Vec<usize> {
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    let fila: Vec<usize> = (0..d(this + 0x14)).map(|k| d(d(this + 0x10) + 12 * k)).collect();
    let na_fila: std::collections::HashSet<usize> = fila.iter().copied().collect();
    let livre = |id: usize| d(pessoas + 110 * id) == id && d(pessoas + 110 * id + 0x39) == 0 && d(pessoas + 110 * id + 0x61) != 0;
    let mut v: Vec<usize> = (0..n.saturating_sub(16))
        .filter(|&id| (*registro79(id) & 0x30) == 0x20 && !na_fila.contains(&id) && livre(id))
        .map(|id| pessoas + 110 * id)
        .collect();
    if cfg().so_fornada {
        let reserva: std::collections::HashSet<usize> = (0..d(this + 0x40)).map(|k| d(d(this + 0x3c) + 16 * k)).collect();
        // a fila vai da cabeça para o fim; o pop tira do fim do vetor, então a cabeça fica por último
        v.extend(fila.iter().rev().filter(|&&id| id < n.saturating_sub(16) && (*registro79(id) & 0x30) != 0 && livre(id)
            && reserva.contains(&d(d(pessoas + 110 * id + 0x61)))).map(|&id| pessoas + 110 * id));
    }
    v
}

/// Tira a entrada `chave` (o dword +0) de um vetor do jogo: base em `this+ptr`, quantidade em `this+ptr+4`, `tam` bytes
/// por entrada; a última vaga fica com −1 e zeros, como a 0x7a65d0 deixa a fila.
unsafe fn tirar(this: usize, ptr: usize, tam: usize, chave: usize) -> bool {
    let (v, q) = (d(this + ptr), d(this + ptr + 4));
    let Some(k) = (0..q).find(|&k| d(v + tam * k) == chave) else { return false };
    core::ptr::copy((v + tam * (k + 1)) as *const u8, (v + tam * k) as *mut u8, tam * (q - k - 1));
    core::ptr::write_bytes((v + tam * (q - 1)) as *mut u8, 0, tam);
    *((v + tam * (q - 1)) as *mut i32) = -1;
    *((this + ptr + 4) as *mut usize) = q - 1;
    true
}

/// Quantas vagas ocupadas no elenco (+0xd7, 50).
unsafe fn elenco(clube: usize) -> usize {
    (0..50).filter(|&k| d(clube + 0xd7 + 4 * k) != 0).count()
}

unsafe fn soltar(this: usize, da_nacao: &dyn Fn(usize) -> bool) {
    TESTE_FEITO = false;
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
    let (mut dados, mut cheios, mut sem_molde, mut falhas, mut da_fila) = (0, 0, 0, 0, 0);
    for &c in &clubes {
        let nivel = nivel_do_elenco(c);
        let mut chegaram = Vec::new();
        let mut acabou = false;
        for _ in 0..cfg().por_clube {
            if elenco(c) >= cfg().elenco_max as usize {
                cheios += 1;
                break;
            }
            let Some(p) = estoque.pop() else {
                acabou = true;
                break;
            };
            let nac = d(c + 0x53);
            let Some(lista) = moldes.get(&nac).filter(|l| !l.is_empty()) else {
                sem_molde += 1;
                estoque.push(p);
                break;
            };
            let molde = d(lista[sorteio(lista.len() as i32) as usize] + 0x61);
            let ficha = d(p + 0x61);
            let id_ficha = d(ficha);
            // um aposentado da fila (só com intake_only): sai da fila e a ficha dele sai da reserva
            if cfg().so_fornada && tirar(this, 0x10, 12, d(p)) {
                tirar(this, 0x3c, 16, id_ficha);
                da_fila += 1;
            }
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
                chegaram.push(p);
            } else {
                falhas += 1;
            }
            // intake-regen  data  idade  PA  CA  goleiro/defesa/meia/atacante (+0xf, +0x11, +0x13, +0x15)  clube (vazio: sem)
            let f = d(p + 0x61);
            registro::linha(&format!("intake-regen	{}	{}	{}	{}	{}/{}/{}/{}	{}", crate::data_do_jogo(), crate::b(p + 0x18), w(f + 7), w(f + 5),
                crate::b(f + 0xf), crate::b(f + 0x11), crate::b(f + 0x13), crate::b(f + 0x15), if d(p + 0x39) != 0 { crate::nome(d(p + 0x39)) } else { String::new() }));
        }
        if cfg().mensagem && !chegaram.is_empty() {
            avisar(c, nivel, &chegaram);
        }
        if acabou {
            break;
        }
    }
    if clubes.is_empty() {
        return;
    }
    registro::linha(&format!("intake\t{}\tclubs {}\tgiven {}\tfull {}\tno template {}\tleft without the club {}\tstock left {}\tfrom the queue {}",
        crate::data_do_jogo(), clubes.len(), dados, cheios, sem_molde, falhas, estoque.len(), da_fila));
}

// ---- A mensagem na caixa de entrada ----

/// Vaga 2 da notícia (a resposta citada na mensagem entre técnicos; nenhuma notícia tem esse id) e vaga 20: a marca.
const MARCA: u32 = 0x4e524f46; // "FORN"
const MARCA2: u32 = 0x4b544e49; // "INTK"
/// Vagas da notícia: 3..19 o texto curto (sem a DLL), 20 a marca, 21 quantos, 22.. dois por jovem (id e o parecer).
const MAX_NA_MENSAGEM: usize = 14;

/// O nível do elenco: a média do CA dos 11 melhores (ponteiros em +0xd7, 50 vagas; a ficha em +0x61, CA em +5).
unsafe fn nivel_do_elenco(clube: usize) -> i32 {
    let mut cas: Vec<i32> = (0..50).map(|k| d(clube + 0xd7 + 4 * k)).filter(|&p| p != 0 && d(p + 0x61) != 0)
        .map(|p| w(d(p + 0x61) + 5) as i16 as i32).collect();
    cas.sort_unstable_by(|a, b| b.cmp(a));
    cas.truncate(11);
    if cas.is_empty() { 0 } else { cas.iter().sum::<i32>() / cas.len() as i32 }
}

/// Faixa do nível de agora (0 pronto .. 3 cru), pela distância à média do elenco.
fn faixa_ca(ca: i32, nivel: i32) -> u32 {
    match ca - nivel {
        x if x >= -5 => 0,
        x if x >= -20 => 1,
        x if x >= -40 => 2,
        _ => 3,
    }
}

/// Faixa do potencial (0 dos melhores do mundo .. 4 divisões de baixo).
fn faixa_pa(pa: i32) -> u32 {
    match pa {
        x if x >= 170 => 0,
        x if x >= 150 => 1,
        x if x >= 130 => 2,
        x if x >= 100 => 3,
        _ => 4,
    }
}

/// 0 inglês, 1 português, 2 espanhol: a língua do jogo (`langlib`, [0x9eb2dc]; os códigos em 0x9eb244).
unsafe fn lingua() -> usize {
    let i = d(0x9eb2dc) as i32;
    if !(0..19).contains(&i) {
        return 0;
    }
    let p = d(0x9eb244 + 4 * i as usize);
    let c: Vec<u8> = (0..8).map(|k| *((p + k) as *const u8)).take_while(|&c| c != 0).collect();
    match &c[..] {
        b"POR" | b"POR_BRA" => 1,
        b"ESP" | b"ESP_MEX" => 2,
        _ => 0,
    }
}

/// Texto em Latin-1 (o do jogo).
fn latin1(s: &str) -> Vec<u8> {
    s.chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect()
}

/// O nome da pessoa como o jogo escreve (`database_format_staff_name` 0x53dba0, estilo 1).
unsafe fn nome_da_pessoa(p: usize) -> Vec<u8> {
    let mut buf = [0u8; 128];
    let f: unsafe extern "C" fn(usize, *mut u8, i16, i8) = core::mem::transmute(0x53dba0usize);
    f(p, buf.as_mut_ptr(), 120, 1);
    buf.iter().copied().take_while(|&c| c != 0).collect()
}

/// A posição pela ficha (goleiro +0xf, defesa +0x11, meia +0x13, atacante +0x15; a maior).
unsafe fn posicao(p: usize) -> usize {
    let f = d(p + 0x61);
    [0xf, 0x11, 0x13, 0x15].iter().enumerate().max_by_key(|&(_, &o)| crate::b(f + o)).map(|(i, _)| i).unwrap_or(2)
}

const POSICOES: [[&str; 4]; 3] = [["goalkeeper", "defender", "midfielder", "forward"], ["goleiro", "zagueiro", "meio-campista", "atacante"], ["portero", "defensa", "centrocampista", "delantero"]];
const FAIXAS_CA: [[&str; 4]; 3] = [
    ["is already ready for the first team", "could get chances in the first team soon", "needs a year or two in the youth team", "is still very raw and will take time"],
    ["já está pronto para o time principal", "pode ganhar chances no time principal em breve", "precisa de um ou dois anos na base", "ainda está muito cru, vai levar tempo"],
    ["ya está listo para el primer equipo", "puede tener oportunidades en el primer equipo pronto", "necesita uno o dos años en las inferiores", "todavía está muy verde, llevará tiempo"],
];
const FAIXAS_PA: [[&str; 5]; 3] = [
    ["could become one of the best in the world", "has the potential to play for his country", "could be a regular at a big club", "should make a career in the top flight", "is unlikely to go beyond the lower divisions"],
    ["pode virar um dos melhores do mundo", "tem potencial para a seleção", "pode ser titular de um clube grande", "deve fazer carreira na primeira divisão", "dificilmente passará das divisões de baixo"],
    ["puede llegar a ser uno de los mejores del mundo", "tiene potencial para la selección", "puede ser titular en un club grande", "debería hacer carrera en primera división", "difícilmente pasará de las divisiones inferiores"],
];

/// O texto curto, guardado na notícia na língua do dia (o que o jogo mostra sem a DLL; até 67 bytes).
fn texto_curto(l: usize, n: usize) -> String {
    match l {
        1 => format!("Fornada: {} jovens chegaram da base.", n),
        2 => format!("Hornada: {} juveniles llegaron de la cantera.", n),
        _ => format!("Youth intake: {} youngsters joined from the youth team.", n),
    }
}

/// Posta a notícia: `NewsItem::NewsItem` 0x77bbe0 (tipo 0x1388, importante), as vagas, e
/// `News::send_to_club_manager` 0x7861e0, que só entrega a técnico humano.
unsafe fn avisar(clube: usize, nivel: i32, jovens: &[usize]) {
    let mut tecnico = d(clube + 0xcf);
    let humano = |t: usize| t != 0 && d(t) >= d(0xae2364).saturating_sub(16);
    // teste na bancada (TWEAKS_INTAKE_NEWS_TEST=1, carreira automática sem clube): a 1ª mensagem de cada fornada vai
    // para o técnico humano da tela ([0xb74c26]) e o texto escrito pelo desvio vai para o registro
    // só com o técnico humano sem clube: com clube, ele já recebe a mensagem do clube dele (eram duas na caixa)
    let teste = !humano(tecnico) && std::env::var("TWEAKS_INTAKE_NEWS_TEST").is_ok() && !TESTE_FEITO && humano(d(0xb74c26)) && d(d(0xb74c26) + 0x39) == 0;
    if teste {
        tecnico = d(0xb74c26);
        TESTE_FEITO = true;
    }
    if !humano(tecnico) || d(0xb7431c) == 0 {
        return;
    }
    let mut item = [0u8; 0xe0];
    let item_p = item.as_mut_ptr() as usize;
    let novo: unsafe extern "thiscall" fn(usize, i32, i32) -> usize = core::mem::transmute(0x77bbe0usize);
    novo(item_p, 0x1388, 1);
    let vaga = |k: usize, v: u32| core::ptr::write_unaligned((item_p + 5 + 4 * k) as *mut u32, v);
    vaga(0, d(tecnico) as u32); // quem manda: o próprio técnico (o título sem a DLL)
    vaga(1, 1); // particular
    vaga(2, MARCA);
    let mut curto = latin1(&texto_curto(lingua(), jovens.len()));
    curto.truncate(67);
    core::ptr::copy_nonoverlapping(curto.as_ptr(), (item_p + 5 + 4 * 3) as *mut u8, curto.len());
    vaga(20, MARCA2);
    let n = jovens.len().min(MAX_NA_MENSAGEM);
    vaga(21, n as u32);
    for (i, &p) in jovens.iter().take(n).enumerate() {
        let f = d(p + 0x61);
        let (ca, pa) = (w(f + 5) as i16 as i32, w(f + 7) as i16 as i32);
        vaga(22 + 2 * i, d(p) as u32);
        vaga(23 + 2 * i, (ca.clamp(0, 255) as u32) | ((pa.clamp(0, 255) as u32) << 8) | (faixa_ca(ca, nivel) << 16) | (faixa_pa(pa) << 24));
    }
    if teste {
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize, i32) -> i32 = core::mem::transmute(0x785d40usize);
        let r = entregar(d(0xb7431c), item_p, tecnico, 0);
        for completo in [0, 1] {
            let mut t = [0usize; 8];
            let texto: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4ae590usize);
            texto(t.as_mut_ptr() as usize, 0);
            let escrever_texto: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32) -> i32 = core::mem::transmute(0x77bc80usize);
            escrever_texto(item_p, tecnico, t.as_mut_ptr() as usize, completo, 1);
            let c = t[0];
            let lido: String = if c == 0 { String::new() } else { (0..4000).map(|k| *((c + k) as *const u8)).take_while(|&x| x != 0).map(|x| if x == b'\n' { '|' } else { x as char }).collect() };
            registro::linha(&format!("intake-news-test	delivered {}	{}	{}", r, if completo == 0 { "title" } else { "body" }, lido));
            let limpar: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x4afa40usize);
            limpar(t.as_mut_ptr() as usize);
        }
    } else {
        let enviar: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x7861e0usize);
        enviar(d(0xb7431c), item_p, clube);
    }
    registro::linha(&format!("intake-news	{}	{}	{} youngsters", crate::data_do_jogo(), crate::nome(clube), jovens.len()));
}

/// O teste da mensagem já rodou nesta fornada.
static mut TESTE_FEITO: bool = false;

/// O começo original de `NewsItem::write_text` (o trampolim).
static mut ESCREVER: usize = 0;
/// O começo original de `NewsItem::get_action_button` (o trampolim).
static mut BOTAO: usize = 0;

/// A notícia é a nossa (tipo 0x1388 com as duas marcas)?
unsafe fn e_nossa(item: usize) -> bool {
    let vaga = |k: usize| core::ptr::read_unaligned((item + 5 + 4 * k) as *const u32);
    item != 0 && d(item) == 0x1388 && vaga(2) == MARCA && vaga(20) == MARCA2
}

/// { 0x782b70 } (thiscall, ret 0x10): `get_action_button(item, técnico, rótulo, tamanho, tela)`, 2 responder, 1 ver,
/// 0 sem botão. A nossa notícia fica sem botão: o "Responder" da mensagem entre técnicos não tem a quem responder.
unsafe extern "thiscall" fn botao(item: usize, tecnico: usize, rotulo: *mut u8, tamanho: i32, tela: *mut usize) -> u8 {
    if e_nossa(item) {
        if !rotulo.is_null() {
            *rotulo = 0;
        }
        if !tela.is_null() {
            *tela = 0;
        }
        return 0;
    }
    let original: unsafe extern "thiscall" fn(usize, usize, *mut u8, i32, *mut usize) -> u8 = core::mem::transmute(BOTAO);
    original(item, tecnico, rotulo, tamanho, tela)
}

/// { 0x77bc80 } (thiscall, ret 0x10): `write_text(item, técnico, saída, texto_completo, marca)`. A nossa notícia (tipo
/// 0x1388 com as duas marcas) ganha o texto completo; as outras seguem para a original.
unsafe extern "thiscall" fn escrever(item: usize, tecnico: usize, saida: usize, completo: i32, marca: i32) -> i32 {
    let vaga = |k: usize| core::ptr::read_unaligned((item + 5 + 4 * k) as *const u32);
    if saida == 0 || !e_nossa(item) {
        let original: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32) -> i32 = core::mem::transmute(ESCREVER);
        return original(item, tecnico, saida, completo, marca);
    }
    let l = lingua();
    let n = (vaga(21) as usize).min(MAX_NA_MENSAGEM);
    let (pessoas, total) = (d(0xae23bc), d(0xae2364));
    let mut s: Vec<u8> = Vec::new();
    let mut links: Vec<(Vec<u8>, usize)> = Vec::new();
    if completo == 0 {
        s = latin1(&match l {
            1 => format!("Fornada da base: {} jovens chegaram", n),
            2 => format!("Hornada de juveniles: llegaron {} jóvenes", n),
            _ => format!("Youth intake: {} youngsters have arrived", n),
        });
    } else {
        s.extend(latin1(match l {
            1 => "O treinador da base apresentou os jovens que sobem para o elenco nesta fornada:",
            2 => "El entrenador de las inferiores presentó a los jóvenes que suben a la plantilla en esta hornada:",
            _ => "The youth coach has presented the youngsters joining the squad in this intake:",
        }));
        for i in 0..n {
            let id = vaga(22 + 2 * i) as usize;
            let parecer = vaga(23 + 2 * i);
            if id >= total {
                continue;
            }
            let p = pessoas + 110 * id;
            let nome = nome_da_pessoa(p);
            s.extend(b"\n\n");
            s.extend(&nome);
            let idade = crate::b(p + 0x18);
            s.extend(latin1(&match l {
                1 => format!(" ({} anos, {}): ", idade, POSICOES[1][posicao(p)]),
                2 => format!(" ({} años, {}): ", idade, POSICOES[2][posicao(p)]),
                _ => format!(" ({}, {}): ", idade, POSICOES[0][posicao(p)]),
            }));
            let (fca, fpa) = (((parecer >> 16) & 0xff).min(3) as usize, ((parecer >> 24) & 0xff).min(4) as usize);
            let junta = match l { 1 => " e ", 2 => " y ", _ => " and " };
            s.extend(latin1(FAIXAS_CA[l][fca]));
            s.extend(latin1(junta));
            s.extend(latin1(FAIXAS_PA[l][fpa]));
            s.push(b'.');
            if cfg().numeros {
                s.extend(latin1(&format!(" (CA {}, PA {})", parecer & 0xff, (parecer >> 8) & 0xff)));
            }
            if !nome.is_empty() {
                links.push((nome, p));
            }
        }
    }
    s.push(0);
    let limpar: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x4afa40usize);
    let por_texto: unsafe extern "thiscall" fn(usize, *const u8) -> i32 = core::mem::transmute(0x4ae660usize);
    let ligar: unsafe extern "thiscall" fn(usize, *const u8, i32, usize) -> i32 = core::mem::transmute(0x4ae8a0usize);
    limpar(saida);
    por_texto(saida, s.as_ptr());
    for (mut nome, p) in links {
        nome.push(0);
        ligar(saida, nome.as_ptr(), 0x7d8, p); // 0x7d8: o link de pessoa, como nas notícias de contrato
    }
    1
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
    // ligada só com estoque e jovens por clube: sem isso, intake_only pararia os regens do jogo sem fornada nenhuma
    if cfg().estoque <= 0 || cfg().por_clube <= 0 {
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
        // a mensagem: o escritor das notícias desviado na entrada (push -1; mov eax, fs:[0])
        if cfg().mensagem {
            match crate::origem::trampolim(0x77bc80, &[0x6a, 0xff, 0x64, 0xa1, 0, 0, 0, 0], escrever as *const () as usize) {
                Some(t) => {
                    ESCREVER = t;
                    // o botão: mov eax, fs:[0]
                    match crate::origem::trampolim(0x782b70, &[0x64, 0xa1, 0, 0, 0, 0], botao as *const () as usize) {
                        Some(t) => BOTAO = t,
                        None => registro::linha("intake: the news button routine at 0x782b70 isn't the 3.9.68 one; the message keeps the Reply button"),
                    }
                }
                None => {
                    F.mensagem = false;
                    registro::linha("intake: the news writer at 0x77bc80 isn't the 3.9.68 one; no inbox message");
                }
            }
        }
    }
    if ok {
        registro::linha(&format!("intake (EXPERIMENTAL): on (stock {}, {} per club on {}, {} clubs, PA {}..{} by reputation, spread {}, squads up to {}{})",
            cfg().estoque, cfg().por_clube, if cfg().por_temporada { "each nation's season end".to_string() } else { format!("day {}", cfg().dia) }, if cfg().so_jogaveis { "playable-league" } else { "selected-league" }, cfg().pa_min, cfg().pa_max, cfg().pa_espalhar, cfg().elenco_max,
            if cfg().so_fornada { "; the game's own youth regens off: retired players in the queue feed the intake" } else { "; the game's own regens too" }));
    }
    ok
}
