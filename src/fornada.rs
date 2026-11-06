//! A fornada de regens (ligada no tweaks.cfg distribuído desde a 1.3; o desenho em docs/PESSOAS_NOVAS.md, "O desenho final"). Regen é regen: a fornada
//! muda a data (os regens só nascem no dia da fornada) e a quantidade (`intake_per_club` por clube com vaga); o resto é
//! a aba Regens.
//!
//! 1. **A reserva de pessoas:** o corte do banco (`database_trim_staff_tables` 0x524d70) já cria pessoas a mais na
//!    carreira nova (`extra = ftol(mantidas × 1% ou 2%)`, 0x525f8c); o `call` do `ftol` vai para um trecho que soma
//!    `RESERVA`, dentro da folga das tabelas (o `realloc` do corte só pode encolher). Antes de `game_run_to_start_date`
//!    0x5cc850 (chamada em 0x829fd9) elas ficam aposentadas (bits 0x30 = 0x20 do +0xb do registro de 79 bytes), sem
//!    clube, fora da fila: aposentado fora da fila é a reserva (sem arquivo ao lado do save).
//! 2. **A fila guardada:** a fila do dia (`process_regen_queue` 0x7a65d0, chamada em 0x5cda7a) roda com a contagem da
//!    fila zerada; o laço dos nascimentos não anda e a reposição dos não jogadores segue normal.
//! 3. **O dia da fornada** (`intake_day`; no padrão `season_end`, cada nação-chave no fim da própria temporada, pelo laço
//!    de `end_season_for_nation` 0x59cdd0 em 0x5d5ea2, e as outras no dia 181), em `soltar`:
//!    a. a cota de cada clube de competição das nações da garantia: `intake_per_club`, limitada às vagas (50);
//!    b. a fila das nações do dia nasce inteira, pela chamada da fila (0x7a664f: a 0x7abac0 e o `regen_origins`), com a
//!       disputa (chamada em 0x7acea2) segurada para as nações da garantia; juntam-se os regens livres de até 21 anos;
//!    c. os que faltam nascem de pessoas da reserva, com a ficha misturada de dois jogadores da nação na mesma posição;
//!    d. a distribuição: do maior PA para o menor, cada um sorteado pela curva da aba Regens entre os clubes da nação
//!       com cota (`crate::peso_na_fornada`, sem `season_cap` e sem `squad_limit`), assinado pela 0x7a6f10; as sobras do
//!       dia vão à disputa normal da nação entre os clubes com vaga, ou ficam sem clube.
//! 4. **O dia das aposentadorias** (`retire_day`): `add_retirement` 0x7a8510, a porta única das aposentadorias, é
//!    desviada na entrada; a data da aposentadoria (+0x10 dia, +0x12 ano do registro de 26 bytes) vira o dia escolhido
//!    (o jogo aposenta quando hoje passa da data, então grava o dia anterior). Vale sem a fornada também.
//! 5. **A mensagem** (`intake_news = 1`): o clube de técnico humano recebe uma notícia do próprio jogo com os jovens que
//!    chegaram e o parecer de cada um (faixas; os números com `intake_news_numbers = 1`), ou o aviso de falta de vaga. É
//!    a mensagem entre técnicos (tipo 0x1388): sem a DLL, o jogo mostra o texto curto das vagas dela (a partir da 3); com
//!    a DLL, `NewsItem::write_text` 0x77bc80 (desviado na entrada) escreve o texto completo na língua do jogo, pelas
//!    frases do intake_news.txt; `get_action_button` 0x782b70 tira o botão Responder.

use crate::{d, registro, sorteio, w};

pub struct Fornada {
    /// `intake = 1` (ou o `intake_stock` > 0 da 1.2): a fornada ligada
    pub ligada: bool,
    pub por_clube: i32,
    pub dia: i32,
    /// `intake_day = season_end` (o padrão): cada nação-chave no fim da própria temporada
    pub por_temporada: bool,
    pub so_jogaveis: bool,
    /// `intake_news`: a mensagem na caixa de entrada do técnico humano
    pub mensagem: bool,
    /// `intake_news_numbers`: o CA e o PA na mensagem, além das faixas
    pub numeros: bool,
    /// `retire_day`: o dia do ano das aposentadorias (−1 = o do jogo)
    pub dia_aposentar: i32,
}

/// A reserva de pessoas pedida na criação da carreira (oculta). O jogo dá o que couber na folga do corte do banco.
/// ponytail: número fixo nosso, pelos testes (a ideia é não acabar nem numa carreira de 100 anos); com todas as ligas a
/// folga das fichas encolhe, e aí o caminho é aumentar a alocação em 0x529bfe/0x529b39.
const RESERVA: i32 = 50_000;

const PADRAO: Fornada = Fornada { ligada: false, por_clube: 3, dia: 181, por_temporada: true, so_jogaveis: true, mensagem: true, numeros: false, dia_aposentar: -1 };

pub static mut F: Fornada = PADRAO;

unsafe fn cfg() -> &'static Fornada {
    &*core::ptr::addr_of!(F)
}

/// As chaves `intake*` e `retire_day` do tweaks.cfg.
pub fn ler() -> Fornada {
    let mut f = PADRAO;
    let caminho = std::env::current_exe().map(|p| p.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return f };
    let (mut intake, mut estoque_12) = (None, false);
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        match (k, v) {
            ("intake_clubs", _) => f.so_jogaveis = v != "selected", // playable | selected
            ("retire_day", "original") => f.dia_aposentar = -1,
            ("intake_day", "season_end") => f.por_temporada = true,
            _ => {
                let Ok(x) = v.parse::<i32>() else { continue };
                match k {
                    "intake" => intake = Some(x != 0),
                    "intake_stock" => estoque_12 = x > 0, // o cfg da 1.2, sem a linha intake
                    "intake_per_club" => f.por_clube = x.clamp(0, 20),
                    "intake_day" => {
                        f.dia = x.clamp(0, 365);
                        f.por_temporada = false;
                    }
                    "intake_news" => f.mensagem = x != 0,
                    "intake_news_numbers" => f.numeros = x != 0,
                    "retire_day" => f.dia_aposentar = x.clamp(0, 365),
                    _ => {}
                }
            }
        }
    }
    f.ligada = intake.unwrap_or(estoque_12);
    f
}

/// O estoque criado nesta sessão (carreira nova): ids [inicio, fim).
static mut CRIADOS: (usize, usize) = (0, 0);
/// Durante a soltura: a disputa não escolhe clube para os regens das nações da garantia (a distribuição vem depois).
static mut SEGURAR: bool = false;
/// O ano da última soltura nesta sessão.
static mut ULTIMO_ANO: i32 = -1;
/// A disputa que estava no `call` de 0x7acea2 (a original ou a do Tweaks).
static mut DISPUTA: usize = 0x7ad0e0;
/// Os ids das nações cuja temporada acabou e ainda não receberam (modo `season_end`).
static mut PENDENTES: Vec<usize> = Vec::new();

/// Os ids das nações da tabela das nações-chave (35 entradas de 72 bytes em 0xb63d60, +0 a nação; +0 da nação, o id).
unsafe fn nacoes_chave() -> Vec<usize> {
    (0..0x23).map(|i| d(0xb63d60 + 72 * i)).filter(|&n| n != 0).map(|n| d(n)).collect()
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
    let nac = if d(entrada) != 0 { d(d(entrada)) } else { 0 }; // o id da nação (+0 da nação)
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
    let k = RESERVA.min(folga);
    let total = extra + k;
    let fim = (pessoas + total - 16) as usize; // as extras ficam em N−17−i; as K primeiras (i < K) são o estoque
    CRIADOS = (fim - k as usize, fim);
    registro::linha(&format!("intake: reserve of {} people created with the career (asked {}, room {}; the game's own extra {}; ids {}..{})",
        k, RESERVA, folga, extra, fim - k as usize, fim));
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
    if SEGURAR && na_garantia(d(jogador + 0x1a)) {
        return 0;
    }
    let original: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(DISPUTA);
    original(this, jogador)
}

/// No lugar da chamada de `process_regen_queue` em 0x5cda7a (thiscall): a fila do dia e, no dia da fornada, a soltura.
unsafe extern "thiscall" fn fila_do_dia(this: usize) {
    let original: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7a65d0usize);
    // a contagem da fila zerada só durante a chamada: o laço dos nascimentos não anda, o resto da rotina sim
    let n = d(this + 0x14);
    *((this + 0x14) as *mut usize) = 0;
    original(this);
    *((this + 0x14) as *mut usize) = n;
    let (dia, ano) = (w(0xae2c90) as i32, w(0xae2c92) as i32);
    if cfg().por_temporada {
        let pend = core::mem::take(&mut *core::ptr::addr_of_mut!(PENDENTES));
        if !pend.is_empty() {
            soltar(this, &|id| pend.contains(&id));
        }
        // as nações fora da tabela das nações-chave: no dia 181
        if dia == 181 && ano != ULTIMO_ANO {
            ULTIMO_ANO = ano;
            let chave = nacoes_chave();
            soltar(this, &|id| !chave.contains(&id));
            gravar_teste(ano);
        }
    } else if dia == cfg().dia && ano != ULTIMO_ANO {
        ULTIMO_ANO = ano;
        soltar(this, &|_| true);
        gravar_teste(ano);
    }
}

/// Teste na bancada (TWEAKS_SAVE_ON_INTAKE=ANO): no dia da fornada desse ano, depois dela, o jogo grava a carreira em
/// fornada_ANO.sav ao lado do exe, pela própria `save_game_data` 0x82e430 (cdecl: caminho, tipo 0x16 = .sav).
unsafe fn gravar_teste(ano: i32) {
    let Ok(alvo) = std::env::var("TWEAKS_SAVE_ON_INTAKE") else { return };
    if alvo.trim().parse::<i32>().ok() != Some(ano) {
        return;
    }
    let caminho = std::env::current_exe().map(|p| p.with_file_name(format!("fornada_{}.sav", ano))).unwrap_or_default();
    let mut c: Vec<u8> = caminho.to_string_lossy().bytes().collect();
    c.push(0);
    let gravar: unsafe extern "C" fn(*const u8, u32) -> u32 = core::mem::transmute(0x82e430usize);
    let r = gravar(c.as_ptr(), 0x16);
    registro::linha(&format!("intake-save-test	{}	{}	{}", crate::data_do_jogo(), caminho.display(), r));
}

/// O estoque: aposentados (bits 0x30 = 0x20), sem clube, com ficha, fora da fila e abaixo dos humanos. Os aposentados
/// da fila não entram: eles nascem pela disputa, com a curva (`nascer_da_fila`).
unsafe fn adormecidos(this: usize) -> Vec<usize> {
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    let na_fila: std::collections::HashSet<usize> = (0..d(this + 0x14)).map(|k| d(d(this + 0x10) + 12 * k)).collect();
    (0..n.saturating_sub(16))
        .filter(|&id| (*registro79(id) & 0x30) == 0x20 && !na_fila.contains(&id))
        .map(|id| pessoas + 110 * id)
        .filter(|&p| d(p) == (p - pessoas) / 110 && d(p + 0x39) == 0 && d(p + 0x61) != 0)
        .collect()
}

/// Põe na frente da reserva de fichas (+0x3c, 16 bytes, +4 o id da nação) as fichas das nações escolhidas, na mesma
/// ordem; devolve quantas são.
unsafe fn fichas_na_frente(this: usize, da_nacao: &dyn Fn(usize) -> bool) -> usize {
    let (v, q) = (d(this + 0x3c), d(this + 0x40));
    let mut entradas: Vec<[u8; 16]> = (0..q).map(|k| *((v + 16 * k) as *const [u8; 16])).collect();
    entradas.sort_by_key(|e| !da_nacao(u32::from_le_bytes([e[4], e[5], e[6], e[7]]) as usize)); // estável: as escolhidas primeiro
    let n = entradas.iter().take_while(|e| da_nacao(u32::from_le_bytes([e[4], e[5], e[6], e[7]]) as usize)).count();
    for (k, e) in entradas.iter().enumerate() {
        *((v + 16 * k) as *mut [u8; 16]) = *e;
    }
    n
}

/// O passo 3a: os regens do jogo guardados na fila nascem agora, `n` fichas da frente da reserva, cada um pela chamada da
/// fila (0x7a664f: a 0x7abac0 ou o registro do `regen_origins` por cima dela) e pela disputa, com a curva. Como a
/// 0x7a65d0: a vaga da cabeça da fila, a ficha da frente da reserva, e a cabeça sai da fila. Sem olhar a data da fila:
/// quem se aposentou até hoje nasce nesta fornada. Devolve os jovens com clube, por clube, e os que ficaram sem clube.
unsafe fn nascer_da_fila(this: usize, n: usize, chegaram: &mut std::collections::HashMap<usize, Vec<usize>>) -> (usize, Vec<usize>) {
    let (pessoas, total) = (d(0xae23bc), d(0xae2364));
    let alvo = (core::ptr::read_unaligned(0x7a6650 as *const i32) as usize).wrapping_add(0x7a6654);
    let criar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(alvo);
    let meta = d(this + 0x40).saturating_sub(n);
    let (mut nascidos, mut sem_clube) = (0, Vec::new());
    while d(this + 0x14) > 0 && d(this + 0x40) > meta {
        let id = d(d(this + 0x10));
        if id < total {
            let p = pessoas + 110 * id;
            let antes = d(this + 0x40);
            criar(this, p);
            if d(this + 0x40) < antes {
                nascidos += 1;
                match d(p + 0x39) {
                    0 => sem_clube.push(p),
                    c => chegaram.entry(c).or_default().push(p),
                }
            }
        }
        tirar(this, 0x10, 12, id);
    }
    (nascidos, sem_clube)
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

/// A nação (ponteiro) está na garantia: liga jogável (+0x11c = 2) ou, com `intake_clubs = selected`, também de fundo.
unsafe fn na_garantia(nac: usize) -> bool {
    nac != 0 && if cfg().so_jogaveis { crate::b(nac + 0x11c) == 2 } else { crate::b(nac + 0x11c) != 0 }
}

/// O jovem: pessoa com ficha, sem clube.
unsafe fn livre(p: usize) -> bool {
    d(p + 0x39) == 0 && d(p + 0x61) != 0
}

/// Um regen novo da nação `nac` (ponteiro): a pessoa `p` da reserva e a ficha dela virando a mistura de duas fichas da
/// mesma posição, `a` e uma de `mesma_posicao`: as posições e os pés de `a`, cada atributo de uma das duas, o PA sorteado
/// entre os dois; a ficha vai para a frente da reserva de fichas e a chamada da fila (0x7a664f: a 0x7abac0, e o
/// `regen_origins` por cima) faz o regen, como qualquer outro.
unsafe fn criar_regen(this: usize, p: usize, nac: usize, a: usize, mesma_posicao: &[usize]) -> bool {
    let b = d(if mesma_posicao.is_empty() { a } else { mesma_posicao[sorteio(mesma_posicao.len() as i32) as usize] } + 0x61);
    let a = d(a + 0x61);
    let ficha = d(p + 0x61);
    let id_ficha = d(ficha);
    core::ptr::copy_nonoverlapping(a as *const u8, ficha as *mut u8, 70);
    *(ficha as *mut usize) = id_ficha;
    *((ficha + 4) as *mut u8) = 0; // número
    *((ficha + 0x45) as *mut u8) = 10; // moral
    for o in 0x1b..=0x44usize {
        // os pés (+0x30 esquerdo, +0x3b direito) ficam os da primeira, como as posições
        if o != 0x30 && o != 0x3b && sorteio(2) == 1 {
            *((ficha + o) as *mut u8) = *((b + o) as *const u8);
        }
    }
    let (pa_a, pa_b) = ((w(a + 7) as i16 as i32).clamp(1, 200), (w(b + 7) as i16 as i32).clamp(1, 200));
    let (lo, hi) = (pa_a.min(pa_b), pa_a.max(pa_b));
    *((ficha + 7) as *mut i16) = (lo + sorteio(hi - lo + 1)) as i16;
    por_na_frente_da_reserva(this, id_ficha, d(nac));
    let alvo = (core::ptr::read_unaligned(0x7a6650 as *const i32) as usize).wrapping_add(0x7a6654);
    let criar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(alvo);
    let antes = d(this + 0x40);
    criar(this, p);
    d(this + 0x40) < antes
}

/// Sorteia um clube pelo peso da curva (`crate::peso_na_fornada`); nenhum peso positivo entre os elegíveis: um deles
/// por igual. `None`: nenhum pode recebê-lo.
unsafe fn sortear_clube(p: usize, clubes: &[usize]) -> Option<usize> {
    let surpresa = crate::sorteia_surpresa();
    let mut pesos: Vec<(usize, f64)> = clubes.iter().filter_map(|&c| crate::peso_na_fornada(p, c, surpresa).map(|x| (c, x))).collect();
    if pesos.is_empty() {
        return None;
    }
    let soma: f64 = pesos.iter().map(|x| x.1).sum();
    if soma <= 0.0 {
        return Some(pesos[sorteio(pesos.len() as i32) as usize].0);
    }
    let mut x = crate::uniforme() * soma;
    let ultimo = pesos.pop().map(|x| x.0);
    for (c, peso) in pesos {
        x -= peso;
        if x < 0.0 {
            return Some(c);
        }
    }
    ultimo
}

/// A fornada das nações escolhidas (`da_nacao`, pelo id da nação). Ver docs/PESSOAS_NOVAS.md, "O desenho final".
unsafe fn soltar(this: usize, da_nacao: &dyn Fn(usize) -> bool) {
    use std::collections::{HashMap, HashSet};
    TESTE_FEITO = false;
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    let total = (d(0xae235c) as i32 - 2 * d(0xae2350) as i32).max(0) as usize;
    let base = d(0xae23b4);
    let todos: Vec<usize> = (0..total).map(|i| base + 0x245 * i).collect();
    // 1. a cota: os clubes de competição das nações da garantia, menos os times B (a curva não manda regen para eles:
    // com cota, ficavam faltando todo ano); intake_per_club, limitado às vagas (50)
    let clubes: Vec<usize> = todos
        .iter()
        .copied()
        .filter(|&c| {
            let nac = d(c + 0x53);
            nac != 0 && da_nacao(d(nac)) && na_garantia(nac) && crate::em_competicao(c) && !crate::time_b(c)
        })
        .collect();
    let nivel: HashMap<usize, i32> = clubes.iter().map(|&c| (c, nivel_do_elenco(c))).collect();
    let mut cota: HashMap<usize, i32> = clubes.iter().map(|&c| (c, cfg().por_clube.min(50 - elenco(c) as i32).max(0))).collect();
    let sem_vaga: Vec<usize> = clubes.iter().copied().filter(|c| cota[c] == 0 && cfg().por_clube > 0).collect();
    let mut por_nacao: HashMap<usize, Vec<usize>> = HashMap::new();
    for &c in &clubes {
        por_nacao.entry(d(c + 0x53)).or_default().push(c);
    }
    // 2. os regens do dia: a fila das nações do dia nasce inteira; a disputa não escolhe clube para os da garantia
    let mut chegaram: HashMap<usize, Vec<usize>> = HashMap::new();
    SEGURAR = true;
    let k = fichas_na_frente(this, da_nacao);
    let (da_fila, nascidos) = nascer_da_fila(this, k, &mut chegaram);
    let do_dia: HashSet<usize> = nascidos.iter().copied().collect();
    // o grupo de cada nação: os do dia e os regens livres de até 21 anos que sobraram de fornadas anteriores
    let mut grupo: HashMap<usize, Vec<usize>> = HashMap::new();
    for &p in &nascidos {
        if livre(p) && por_nacao.contains_key(&d(p + 0x1a)) {
            grupo.entry(d(p + 0x1a)).or_default().push(p);
        }
    }
    let mut livres_antes = 0;
    for id in 0..n.saturating_sub(16) {
        let p = pessoas + 110 * id;
        // regen (bit 0x40), não aposentado (bits 0x30)
        if !do_dia.contains(&p) && livre(p) && crate::b(p + 0x18) <= 21 && (*registro79(id) & 0x70) == 0x40 && por_nacao.contains_key(&d(p + 0x1a)) {
            grupo.entry(d(p + 0x1a)).or_default().push(p);
            livres_antes += 1;
        }
    }
    // 3. os que faltam: pessoas da reserva com a ficha misturada de duas fichas da mesma posição. Os pais são os regens
    // que acabaram de nascer de aposentados (a ficha deles é a do aposentado: o PA, as posições, o perfil), para os
    // criados seguirem a distribuição de quem se aposenta: os da própria nação; com menos de `MIN_PAIS`, também os das
    // outras nações do dia (a nação do regen continua a do clube); com menos ainda, os jogadores de 30 anos ou mais da
    // nação. Poucos pais dariam quase clones; jogadores em atividade como pais dariam PA alto demais (no teste, mediana
    // 112 a 118 contra 72 a 80 dos aposentados). O primeiro pai é sorteado do grupo inteiro: as posições saem na
    // proporção dele.
    const MIN_PAIS: usize = 50;
    let mut reserva = adormecidos(this);
    let todos_do_dia: Vec<usize> = nascidos.iter().copied().filter(|&p| d(p + 0x61) != 0).collect();
    let mut pais: HashMap<usize, Vec<usize>> = HashMap::new();
    for &p in &todos_do_dia {
        if por_nacao.contains_key(&d(p + 0x1a)) {
            pais.entry(d(p + 0x1a)).or_default().push(p);
        }
    }
    let mut curtas: HashSet<usize> = HashSet::new();
    for &nac in por_nacao.keys() {
        let proprios = pais.entry(nac).or_default();
        if proprios.len() < MIN_PAIS {
            let ja: HashSet<usize> = proprios.iter().copied().collect();
            proprios.extend(todos_do_dia.iter().copied().filter(|p| !ja.contains(p)));
            if proprios.len() < MIN_PAIS {
                curtas.insert(nac);
            }
        }
    }
    if !curtas.is_empty() {
        for id in 0..n.saturating_sub(16) {
            let p = pessoas + 110 * id;
            let nac = d(p + 0x1a);
            if curtas.contains(&nac) && d(p + 0x61) != 0 && (*registro79(id) & 0x30) == 0 && crate::b(p + 0x18) >= 30 && !do_dia.contains(&p) {
                pais.entry(nac).or_default().push(p);
            }
        }
    }
    let mut por_posicao: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (&nac, l) in &pais {
        for &p in l {
            por_posicao.entry((nac, posicao(p))).or_default().push(p);
        }
    }
    let (mut criados, mut sem_pais, mut acabou) = (0, 0, false);
    for (&nac, cs) in &por_nacao {
        let falta = cs.iter().map(|c| cota[c]).sum::<i32>() - grupo.get(&nac).map_or(0, |g| g.len()) as i32;
        let Some(lista) = pais.get(&nac).filter(|x| !x.is_empty()) else {
            sem_pais += falta.max(0);
            continue;
        };
        for _ in 0..falta.max(0) {
            let Some(p) = reserva.pop() else {
                acabou = true;
                break;
            };
            let a = lista[sorteio(lista.len() as i32) as usize];
            let mesma = por_posicao.get(&(nac, posicao(a))).map(|v| v.as_slice()).unwrap_or(&[]);
            if criar_regen(this, p, nac, a, mesma) && livre(p) {
                criados += 1;
                if let Some(g) = grupo.get_mut(&d(p + 0x1a)) {
                    g.push(p);
                } else if por_nacao.contains_key(&d(p + 0x1a)) {
                    grupo.insert(d(p + 0x1a), vec![p]);
                }
            }
        }
    }
    SEGURAR = false;
    // 4. a distribuição: do maior PA para o menor, cada um pela curva entre os clubes da nação que ainda têm cota
    let assinar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x7a6f10usize);
    let (mut dados, mut sobras_com_clube, mut sobras_sem_clube) = (0, 0, 0);
    for (nac, mut g) in grupo {
        g.sort_by_key(|&p| core::cmp::Reverse(w(d(p + 0x61) + 7) as i16));
        let Some(cs) = por_nacao.get(&nac) else { continue };
        for p in g {
            let abertos: Vec<usize> = cs.iter().copied().filter(|c| cota[c] > 0).collect();
            if let Some(c) = if abertos.is_empty() { None } else { sortear_clube(p, &abertos) } {
                assinar(this, p, c);
                if d(p + 0x39) == c {
                    crate::marcar_fornada(d(p), d(c), false);
                    *cota.get_mut(&c).unwrap() -= 1;
                    chegaram.entry(c).or_default().push(p);
                    dados += 1;
                    continue;
                }
            }
            // 5. as sobras do dia: a disputa normal da nação, entre os clubes com vaga (qualquer divisão)
            if do_dia.contains(&p) {
                let da_nac: Vec<usize> = todos.iter().copied().filter(|&c| d(c + 0x53) == nac).collect();
                if let Some(c) = sortear_clube(p, &da_nac) {
                    assinar(this, p, c);
                    if d(p + 0x39) == c {
                        crate::marcar_fornada(d(p), d(c), true);
                    }
                }
                if d(p + 0x39) != 0 {
                    sobras_com_clube += 1
                } else {
                    sobras_sem_clube += 1
                }
            }
        }
    }
    // 6. a mensagem: quem chegou, e o aviso de falta de vaga
    if cfg().mensagem {
        for &c in &clubes {
            if let Some(v) = chegaram.get(&c).filter(|v| !v.is_empty()) {
                avisar(c, nivel.get(&c).copied().unwrap_or(0), v);
            }
        }
        for &c in &sem_vaga {
            avisar(c, 0, &[]);
        }
    }
    if clubes.is_empty() && da_fila == 0 {
        return;
    }
    let completos = clubes.iter().filter(|c| cota[c] == 0).count();
    let faltou: i32 = cota.values().sum();
    registro::linha(&format!(
        "intake\t{}\tclubs {}\tborn from the queue {}\tfree regens up to 21 {}\tcreated {}\tgiven to clubs {}\tclubs with their quota {}\tmissing {}\tno room {}\tleftovers signed {}\tleftovers without a club {}\tno parents {}\treserve left {}{}",
        crate::data_do_jogo(), clubes.len(), da_fila, livres_antes, criados, dados, completos, faltou, sem_vaga.len(), sobras_com_clube, sobras_sem_clube, sem_pais, reserva.len(),
        if acabou { "\treserve ran out" } else { "" }
    ));
    // ponytail: só os 20 primeiros, para o diagnóstico
    let faltando: Vec<String> = clubes.iter().filter(|c| cota[c] > 0).take(20).map(|&c| format!("{} ({})", crate::nome(c), cota[&c])).collect();
    if !faltando.is_empty() {
        registro::linha(&format!("intake-missing\t{}\t{}", crate::data_do_jogo(), faltando.join(", ")));
    }
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

/// O código da língua do jogo (`langlib`, [0x9eb2dc]; os códigos em 0x9eb244: ENG_UK, ENG_US, FRA, ..., POR_BRA, ESP_MEX).
unsafe fn codigo_da_lingua() -> String {
    let i = d(0x9eb2dc) as i32;
    if !(0..19).contains(&i) {
        return "ENG_UK".into();
    }
    let p = d(0x9eb244 + 4 * i as usize);
    (0..8).map(|k| *((p + k) as *const u8)).take_while(|&c| c != 0).map(|c| c as char).collect()
}

/// Texto em Latin-1 (o do jogo).
fn latin1(s: &str) -> Vec<u8> {
    s.chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect()
}

/// As frases embutidas (as do intake_news.txt para inglês, português e espanhol), quando o arquivo falta ou não tem a
/// frase. `chave` como no arquivo.
fn embutida(codigo: &str, chave: &str) -> &'static str {
    let l = match codigo {
        "POR" | "POR_BRA" => 1,
        "ESP" | "ESP_MEX" => 2,
        _ => 0,
    };
    let t: [&str; 3] = match chave {
        "title" => ["Youth intake: {n} youngsters have arrived", "Fornada da base: {n} jovens chegaram", "Hornada de juveniles: llegaron {n} jóvenes"],
        "intro" => ["The youth coach has presented the youngsters joining the squad in this intake:", "O treinador da base apresentou os jovens que sobem para o elenco nesta fornada:", "El entrenador de las inferiores presentó a los jóvenes que suben a la plantilla en esta hornada:"],
        "player" => ["{name} ({age}, {position}): {now} and {potential}.", "{name} ({age} anos, {position}): {now} e {potential}.", "{name} ({age} años, {position}): {now} y {potential}."],
        "numbers" => ["(CA {ca}, PA {pa})", "(CA {ca}, PA {pa})", "(CA {ca}, PA {pa})"],
        "short" => ["Youth intake: {n} youngsters joined from the youth team.", "Fornada: {n} jovens chegaram da base.", "Hornada: {n} juveniles llegaron de la cantera."],
        "goalkeeper" => ["goalkeeper", "goleiro", "portero"],
        "defender" => ["defender", "zagueiro", "defensa"],
        "midfielder" => ["midfielder", "meio-campista", "centrocampista"],
        "forward" => ["forward", "atacante", "delantero"],
        "now_0" => ["is already ready for the first team", "já está pronto para o time principal", "ya está listo para el primer equipo"],
        "now_1" => ["could get chances in the first team soon", "pode ganhar chances no time principal em breve", "puede tener oportunidades en el primer equipo pronto"],
        "now_2" => ["needs a year or two in the youth team", "precisa de um ou dois anos na base", "necesita uno o dos años en las inferiores"],
        "now_3" => ["is still very raw and will take time", "ainda está muito cru, vai levar tempo", "todavía está muy verde, llevará tiempo"],
        "potential_0" => ["could become one of the best in the world", "pode virar um dos melhores do mundo", "puede llegar a ser uno de los mejores del mundo"],
        "potential_1" => ["has the potential to play for his country", "tem potencial para a seleção", "tiene potencial para la selección"],
        "potential_2" => ["could be a regular at a big club", "pode ser titular de um clube grande", "puede ser titular en un club grande"],
        "potential_3" => ["should make a career in the top flight", "deve fazer carreira na primeira divisão", "debería hacer carrera en primera división"],
        "no_room_title" => ["Youth intake: no youngsters this year", "Fornada da base: nenhum jovem este ano", "Hornada de juveniles: ningún juvenil este año"],
        "no_room" => ["The youth coach had youngsters ready to step up, but the squad is full (50 players), so none of them could join in this intake. Free some places before the next one.", "O treinador da base tinha jovens prontos para subir, mas o elenco está cheio (50 jogadores), e nenhum pôde chegar nesta fornada. Libere vagas antes da próxima.", "El entrenador de las inferiores tenía juveniles listos para subir, pero la plantilla está llena (50 jugadores) y ninguno pudo llegar en esta hornada. Libera plazas antes de la próxima."],
        "potential_4" => ["is unlikely to go beyond the lower divisions", "dificilmente passará das divisões de baixo", "difícilmente pasará de las divisiones inferiores"],
        _ => ["", "", ""],
    };
    t[l]
}

/// O intake_news.txt (ao lado do exe), lido uma vez: seção `[CÓDIGO]` por língua do jogo e linhas `chave = frase`.
static mut FRASES: Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>> = None;

unsafe fn frases() -> &'static std::collections::HashMap<String, std::collections::HashMap<String, String>> {
    let f = &mut *core::ptr::addr_of_mut!(FRASES);
    f.get_or_insert_with(|| {
        let mut m: std::collections::HashMap<String, std::collections::HashMap<String, String>> = std::collections::HashMap::new();
        let caminho = std::env::current_exe().map(|p| p.with_file_name("intake_news.txt")).unwrap_or_else(|_| "intake_news.txt".into());
        let Ok(texto) = std::fs::read_to_string(&caminho) else { return m };
        let mut secao = String::new();
        for linha in texto.lines() {
            let l = linha.trim().trim_start_matches('\u{feff}');
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            if l.starts_with('[') && l.ends_with(']') {
                secao = l[1..l.len() - 1].trim().to_uppercase();
                continue;
            }
            if let Some((k, v)) = l.split_once('=') {
                m.entry(secao.clone()).or_default().insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        }
        registro::linha(&format!("intake: the message phrases from intake_news.txt ({} languages)", m.len()));
        m
    })
}

/// A frase `chave` na língua do jogo: a do intake_news.txt, senão a embutida.
unsafe fn frase(chave: &str) -> String {
    let c = codigo_da_lingua();
    frases().get(&c).and_then(|s| s.get(chave)).filter(|v| !v.is_empty()).cloned().unwrap_or_else(|| embutida(&c, chave).to_string())
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

/// O texto curto, guardado na notícia na língua do dia (o que o jogo mostra sem a DLL; até 67 bytes).
unsafe fn texto_curto(n: usize) -> String {
    frase("short").replace("{n}", &n.to_string())
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
    let mut curto = latin1(&if jovens.is_empty() { frase("no_room_title") } else { texto_curto(jovens.len()) });
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
    let n = (vaga(21) as usize).min(MAX_NA_MENSAGEM);
    let (pessoas, total) = (d(0xae23bc), d(0xae2364));
    let mut s: Vec<u8> = Vec::new();
    let mut links: Vec<(Vec<u8>, usize)> = Vec::new();
    if n == 0 {
        // o clube sem vaga no elenco: ninguém chegou
        s = latin1(&frase(if completo == 0 { "no_room_title" } else { "no_room" }));
    } else if completo == 0 {
        s = latin1(&frase("title").replace("{n}", &n.to_string()));
    } else {
        s.extend(latin1(&frase("intro")));
        const POSICOES: [&str; 4] = ["goalkeeper", "defender", "midfielder", "forward"];
        for i in 0..n {
            let id = vaga(22 + 2 * i) as usize;
            let parecer = vaga(23 + 2 * i);
            if id >= total {
                continue;
            }
            let p = pessoas + 110 * id;
            let nome = nome_da_pessoa(p);
            let (fca, fpa) = (((parecer >> 16) & 0xff).min(3), ((parecer >> 24) & 0xff).min(4));
            let mut linha = frase("player")
                .replace("{age}", &crate::b(p + 0x18).to_string())
                .replace("{position}", &frase(POSICOES[posicao(p)]))
                .replace("{now}", &frase(&format!("now_{}", fca)))
                .replace("{potential}", &frase(&format!("potential_{}", fpa)));
            if cfg().numeros {
                linha += " ";
                linha += &frase("numbers").replace("{ca}", &(parecer & 0xff).to_string()).replace("{pa}", &((parecer >> 8) & 0xff).to_string());
            }
            // o nome vem do jogo, já em Latin-1: entra no lugar de {name} sem passar pela conversão
            s.extend(b"\n\n");
            let mut partes = linha.split("{name}");
            s.extend(latin1(partes.next().unwrap_or("")));
            for resto in partes {
                s.extend(&nome);
                s.extend(latin1(resto));
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

// ---- O dia das aposentadorias ----

/// O começo original de `add_retirement` (o trampolim).
static mut APOSENTAR: usize = 0;

/// { 0x7a8510 } (thiscall, ret 4): `add_retirement(registro)`. A data da aposentadoria (+0x10 dia, +0x12 ano) vira o
/// `retire_day` deste ano, ou do próximo se o dia já passou. O jogo aposenta quando hoje passa da data (comparação
/// estrita, 0x7a5a57): para aposentar no dia N, grava N − 1.
unsafe extern "thiscall" fn aposentar(this: usize, registro: usize) {
    let n = cfg().dia_aposentar;
    if n >= 0 && registro != 0 {
        let (hoje, ano) = (w(0xae2c90) as i32, w(0xae2c92) as i32);
        let ano_alvo = if hoje < n { ano } else { ano + 1 };
        let (dia, ano_gravar) = if n == 0 { (364, ano_alvo - 1) } else { (n - 1, ano_alvo) };
        *((registro + 0x10) as *mut u16) = dia as u16;
        *((registro + 0x12) as *mut u16) = ano_gravar as u16;
    }
    let original: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(APOSENTAR);
    original(this, registro)
}

/// O dia das aposentadorias (`retire_day`), com ou sem a fornada.
/// Chamada depois de `instalar` (que lê o tweaks.cfg em `F`).
pub unsafe fn instalar_aposentadoria() {
    if cfg().dia_aposentar < 0 {
        return;
    }
    // sub esp, 0x200
    match crate::origem::trampolim(0x7a8510, &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00], aposentar as *const () as usize) {
        Some(t) => {
            APOSENTAR = t;
            registro::linha(&format!("retire day: every retirement on day {} of the year", cfg().dia_aposentar));
        }
        None => registro::linha("retire day: add_retirement at 0x7a8510 isn't the 3.9.68 one; retirements keep the game's dates"),
    }
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
    // ligada com regens por clube: sem isso a fila ficaria guardada sem fornada nenhuma
    if !cfg().ligada || cfg().por_clube <= 0 {
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
        registro::linha(&format!("intake: on ({} per club with squad room on {}, {} clubs; the regens are only born on intake day, retired players' and created ones alike, and the Regens tab's curve hands them out; season_cap and squad_limit don't apply)",
            cfg().por_clube, if cfg().por_temporada { "each nation's season end".to_string() } else { format!("day {}", cfg().dia) }, if cfg().so_jogaveis { "playable-league" } else { "selected-league" }));
    }
    ok
}
