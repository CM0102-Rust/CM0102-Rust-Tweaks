//! Público e estádio (EXPERIMENTAL, branch `estudos`; desenho em docs/REPUTACAO_E_PUBLICO.md §5). O público é a torcida ×
//! o momento × o jogo: cresce pelo que acontece em campo e esfria quando o desempenho volta ao esperado. Toda chave parte
//! do valor do jogo original; sem nenhuma, nada é desviado. Três `call` trocados, cada um chamando a original:
//! - o público de cada jogo (0x85cba0, chamada em 0x6b86bc): a original roda com o recorde do mandante (+0x7b) e a
//!   capacidade do estádio (+0x3c) folgados, e devolve a **demanda** (quanta gente quis vir); os tetos são aplicados aqui:
//!   o recorde no jogo comum (no clássico, recorde × 1,5, como a original, que multiplica depois de cortar), o estádio no
//!   mata-mata (`knockout_ceiling = stadium`); o visitante até 15% da capacidade. `attendance_league_caps = off` tira o corte
//!   das ligas de nível até 4 (acima de 1.500 × nível, só 20% do excesso). `attendance_record = real`: o recorde passa a ser
//!   o maior público que veio; `pent_up_demand`: a fração da demanda acima do recorde (até a capacidade) que vira recorde.
//! - a média do fim da temporada (0x5d6280, chamada em 0x5d5cfb): com `attendance_league_caps = off`, a média nova é a
//!   metade do caminho até o público real da temporada (o somado da estatística da liga), × (1 + 0,05 × (prevista −
//!   final)), sem os tetos de 2.000, 3.000 ou 5.000 × o nível; `tradition_weight`: quem foi melhor que o previsto anda
//!   essa fração × (prevista − final) / 10 do caminho até o recorde (o histórico do banco, a torcida que já existiu).
//! - o estádio no fim da temporada (0x59cfa0, chamada em 0x59cee4): `stadium_seating_growth = off` impede a conversão em
//!   lugares sentados de somar na capacidade; qualquer outra chave do estádio tira a ampliação da original (o teto fica
//!   igual à capacidade durante a chamada) e a decide aqui: apto com a ocupação acima de 92% na temporada e, com
//!   `stadium_full_seasons = 2`, a média de antes também acima (aproximação: a média do clube resume as temporadas
//!   passadas); obra do tamanho da demanda média dos jogos em casa (`stadium_size = demand`) ou da original; até o teto
//!   do banco (sem teto, `stadium_ceiling_zero = demand` usa a demanda), um terço dele e `stadium_max`; custo
//!   `stadium_seat_cost` por lugar, dentro do saldo acima da reserva de 5,25 milhões.
//! Com `log = 1`: `publico_jogos.tsv` (cada jogo em casa no estádio do mandante), `publico_temporada.tsv` (cada clube no
//! fim da temporada da liga) e `publico_estadio.tsv` (cada avaliação de obra).
//! ponytail: confere só o `call` (E8 para a original); a contagem da temporada por jogo fica na memória da DLL (perde-se
//! ao carregar um save no meio da temporada; a ocupação do estádio vem da estatística das competições, que vai no save);
//! a soma de um clube zera no fim da temporada da liga dele.

use super::*;
use std::collections::HashMap;
use std::io::Write;

#[derive(Clone, Copy)]
pub(crate) struct Publico {
    sentados_crescem: bool,
    gatilho_reputacao: bool,
    tetos_da_liga: bool,
    recorde_real: bool,
    reprimida: f64,
    tradicao: f64,
    mata_estadio: bool,
    temporadas_cheias: u32,
    ocupacao_tudo: bool,
    obra_demanda: bool,
    custo_lugar: f64,
    teto_demanda: bool,
    maximo: i32,
    disputa: bool,
}

/// O jogo original (sem tweaks.cfg nenhum, nada muda).
const ORIGINAL: Publico = Publico {
    sentados_crescem: true, gatilho_reputacao: true, tetos_da_liga: true, recorde_real: false, reprimida: 0.0, tradicao: 0.0,
    mata_estadio: false, temporadas_cheias: 1, ocupacao_tudo: false, obra_demanda: false, custo_lugar: 0.0, teto_demanda: false,
    maximo: 125_000, disputa: false,
};
/// As nossas regras, o padrão de cada chave ausente de um tweaks.cfg (08/11, Lucas: o padrão é sempre o nosso). O custo
/// por lugar −1 = `auto` (calibrado pelo caixa dos clubes do banco).
const NOSSO: Publico = Publico {
    sentados_crescem: false, gatilho_reputacao: false, tetos_da_liga: false, recorde_real: true, reprimida: 0.1, tradicao: 0.5,
    mata_estadio: true, temporadas_cheias: 2, ocupacao_tudo: true, obra_demanda: true, custo_lugar: -1.0, teto_demanda: true,
    maximo: 125_000, disputa: true,
};
static mut P: Publico = ORIGINAL;

fn ler() -> Publico {
    let caminho = std::env::current_exe().map(|x| x.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return ORIGINAL };
    let mut p = NOSSO;
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        let n = v.parse::<f64>().ok();
        match k {
            "stadium_seating_growth" => p.sentados_crescem = v == "on",
            "stadium_reputation_trigger" => p.gatilho_reputacao = v == "on",
            "attendance_league_caps" => p.tetos_da_liga = v == "on",
            "attendance_record" => p.recorde_real = v != "fixed",
            "pent_up_demand" => p.reprimida = n.unwrap_or(p.reprimida).clamp(0.0, 1.0),
            "tradition_weight" => p.tradicao = n.unwrap_or(p.tradicao).clamp(0.0, 2.0),
            "knockout_ceiling" => p.mata_estadio = v != "record",
            "stadium_full_seasons" => p.temporadas_cheias = n.unwrap_or(p.temporadas_cheias as f64).clamp(1.0, 2.0) as u32,
            "stadium_occupancy" => p.ocupacao_tudo = v != "league",
            "stadium_size" => p.obra_demanda = v != "original",
            "stadium_seat_cost" => p.custo_lugar = match v {
                "original" => 0.0,
                "auto" => -1.0,
                _ => n.map(|x| x.clamp(1.0, 1e7)).unwrap_or(p.custo_lugar),
            },
            "stadium_ceiling_zero" => p.teto_demanda = v != "original",
            "title_race" => p.disputa = v != "off",
            "stadium_max" => p.maximo = n.unwrap_or(p.maximo as f64).clamp(1_000.0, 1e7) as i32,
            _ => {}
        }
    }
    p
}

impl Publico {
    fn mexe_no_jogo(&self) -> bool {
        !self.tetos_da_liga || self.recorde_real || self.reprimida > 0.0 || self.mata_estadio || self.disputa
    }
    fn mexe_na_media(&self) -> bool {
        !self.tetos_da_liga || self.tradicao > 0.0
    }
    fn assume_obra(&self) -> bool {
        !self.gatilho_reputacao || self.temporadas_cheias != 1 || self.ocupacao_tudo || self.obra_demanda || self.custo_lugar != 0.0
            || self.teto_demanda || self.maximo != 125_000
    }
}

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
    let p = ler();
    P = p;
    // controle: com TWEAKS_ESTUDO=1 e log = 1, as três rotinas passam pelos embrulhos só para registrar (jogo igual ao original)
    let observar = crate::estudo::ligado() && REGRAS.registrar;
    if observar && !p.mexe_no_jogo() {
        trocar("match attendance", 0x6b86bc, 0x85cba0, jogo as *const () as usize, "observing only".into());
    }
    if observar && !p.mexe_na_media() {
        trocar("season attendance", 0x5d5cfb, 0x5d6280, media as *const () as usize, "observing only".into());
    }
    if observar && p.sentados_crescem && !p.assume_obra() {
        trocar("stadium works", 0x59cee4, 0x59cfa0, estadio as *const () as usize, "observing only".into());
    }
    if p.mexe_no_jogo() {
        trocar("match attendance", 0x6b86bc, 0x85cba0, jogo as *const () as usize, format!(
            "league caps {}, record {}, pent-up demand {}, knockout ceiling {}, title race {}", liga_txt(p.tetos_da_liga),
            if p.recorde_real { "real" } else { "fixed" }, p.reprimida, if p.mata_estadio { "stadium" } else { "record" }, on(p.disputa)));
    }
    if p.mexe_na_media() {
        trocar("season attendance", 0x5d5cfb, 0x5d6280, media as *const () as usize,
            format!("league caps {}, tradition {}", liga_txt(p.tetos_da_liga), p.tradicao));
    }
    if !p.sentados_crescem || p.assume_obra() {
        trocar("stadium works", 0x59cee4, 0x59cfa0, estadio as *const () as usize, format!(
            "seating growth {}, reputation trigger {}, full seasons {}, occupancy {}, size {}, seat cost {}, zero ceiling {}, max {}",
            on(p.sentados_crescem), on(p.gatilho_reputacao), p.temporadas_cheias, if p.ocupacao_tudo { "all" } else { "league" },
            if p.obra_demanda { "demand" } else { "original" }, if p.custo_lugar > 0.0 { p.custo_lugar.to_string() } else if p.custo_lugar < 0.0 { "auto".into() } else { "original".into() },
            if p.teto_demanda { "demand" } else { "original" }, p.maximo));
    }
}

fn on(x: bool) -> &'static str {
    if x { "on" } else { "off" }
}
fn liga_txt(x: bool) -> &'static str {
    on(x)
}

unsafe fn hoje() -> String {
    format!("{}-{:03}", w(0xae2c92), w(0xae2c90))
}

fn gravar(arquivo: &str, cabecalho: &str, linha: &str) {
    let caminho = std::env::current_exe().map(|p| p.with_file_name(arquivo)).unwrap_or_else(|_| arquivo.into());
    let novo = !caminho.exists();
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(caminho) {
        if novo {
            let _ = writeln!(f, "{}", cabecalho);
        }
        let _ = writeln!(f, "{}", linha);
    }
}

unsafe fn i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}
unsafe fn pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v)
}

/// O que a DLL somou dos jogos em casa de cada clube nesta temporada (só na memória).
#[derive(Default, Clone, Copy)]
struct Soma {
    jogos: i32,
    publico: i64,
    demanda: i64,
    capacidade: i64,
    cheios: i32,
    mata: i32,
    classicos: i32,
}
static mut SOMAS: Option<HashMap<usize, Soma>> = None;
/// A soma da temporada que acabou (a média do fim da temporada a passa para cá; o estádio pode vir depois).
static mut ULTIMA: Option<HashMap<usize, Soma>> = None;
/// A média de cada clube antes do fim da temporada da liga (para o estádio, que pode vir depois).
static mut MEDIA_ANTES: Option<HashMap<usize, i32>> = None;

unsafe fn somas() -> &'static mut HashMap<usize, Soma> {
    (*core::ptr::addr_of_mut!(SOMAS)).get_or_insert_with(HashMap::new)
}
unsafe fn ultima() -> &'static mut HashMap<usize, Soma> {
    (*core::ptr::addr_of_mut!(ULTIMA)).get_or_insert_with(HashMap::new)
}
unsafe fn medias_antes() -> &'static mut HashMap<usize, i32> {
    (*core::ptr::addr_of_mut!(MEDIA_ANTES)).get_or_insert_with(HashMap::new)
}

unsafe fn rival(a: usize, b: usize) -> bool {
    d(a + 0xb3) == b || d(a + 0xb7) == b || d(a + 0xbb) == b
}

/// A ficha do clube na estatística da competição (0x4a4850, thiscall(stats, clube)), ou 0.
unsafe fn ficha(comp_obj: usize, clube: usize) -> usize {
    if comp_obj == 0 || d(comp_obj + 8) == 0 {
        return 0;
    }
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    achar(d(comp_obj + 8), clube)
}

/// O objeto da competição do registro `comp` ([0xadadfc][id]), ou 0.
unsafe fn objeto(comp: usize) -> usize {
    if comp == 0 || d(0xadadfc) == 0 {
        return 0;
    }
    d(d(0xadadfc) + 4 * d(comp))
}

/// { 0x85cba0 } embrulhada: (this, partida, saída) -> total.
unsafe extern "thiscall" fn jogo(this: usize, partida: usize, saida: *mut i32) -> i32 {
    let original: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> i32 = core::mem::transmute(0x85cba0usize);
    if partida == 0 || saida.is_null() {
        return original(this, partida, saida);
    }
    let (casa, fora, est) = (d(partida + 0x1c), d(partida + 0x20), d(partida + 0x18));
    // só o jogo no estádio do mandante, fora de campo neutro; estádio abaixo de 1.500 a original reescreve
    if casa == 0 || fora == 0 || est == 0 || *((partida + 0x3c) as *const u8) != 0 || d(casa + 0x69) != est || i(est + 0x3c) < 1500 {
        return original(this, partida, saida);
    }
    let p = P;
    if !p.mexe_no_jogo() {
        // só observando: a original como está; a demanda não é medida (−1)
        let total = original(this, partida, saida);
        let codigo = w(partida + 0x30) as u16 as i32;
        let mata = eh_mata(partida, d(partida + 0x14) != 0 && d(casa + 0x57) == d(partida + 0x14), codigo);
        somar_jogo(casa, *saida.add(1), *saida.add(1), i(est + 0x3c), mata, rival(casa, fora));
        registro_jogo(partida, -1, -1, -1, *saida.add(1), *saida.add(2), i(casa + 0x7b), i(casa + 0x7b), 1.0);
        return total;
    }
    let cap = i(est + 0x3c);
    let recorde = i(casa + 0x7b);
    let dono = *((casa + 0x6d) as *const u8);
    let comp = d(partida + 0x14);
    let nivel = if comp != 0 { w(comp + 0x69) as i32 } else { 0 };
    let da_liga = comp != 0 && d(casa + 0x57) == comp;
    pi(casa + 0x7b, 50_000_000);
    pi(est + 0x3c, 50_000_000);
    if !p.tetos_da_liga && da_liga && nivel <= 4 {
        *((casa + 0x6d) as *mut u8) = 0; // o corte das ligas fracas só vale para o dono do estádio
    }
    let mut s = [0i32; 3];
    original(this, partida, s.as_mut_ptr());
    pi(casa + 0x7b, recorde);
    pi(est + 0x3c, cap);
    *((casa + 0x6d) as *mut u8) = dono;
    let (dem_casa, dem_fora) = (s[1].max(0), s[2].max(0));
    let codigo = w(partida + 0x30) as u16 as i32;
    let mata = eh_mata(partida, da_liga, codigo);
    let classico = rival(casa, fora);
    let disputa = if p.disputa && da_liga { fator_disputa(objeto(comp), casa, fora) } else { 1.0 };
    let dem_casa = (dem_casa as f64 * disputa) as i32;
    let teto = if mata && p.mata_estadio {
        cap
    } else if classico || disputa > 1.0 {
        (recorde as f64 * disputa * if classico { 1.5 } else { 1.0 }) as i32
    } else {
        recorde
    };
    let visitante = dem_fora.min((cap as f64 * 0.15) as i32);
    let mandante = dem_casa.min(teto).min(cap - visitante).max(1);
    let total = mandante + visitante;
    *saida = total;
    *saida.add(1) = mandante;
    *saida.add(2) = visitante;
    let mut novo = recorde;
    if p.recorde_real && mandante > novo {
        novo = mandante;
    }
    let alvo = dem_casa.min(cap);
    if p.reprimida > 0.0 && alvo > novo {
        novo += ((alvo - novo) as f64 * p.reprimida) as i32;
    }
    if novo != recorde {
        pi(casa + 0x7b, novo);
    }
    somar_jogo(casa, mandante, dem_casa, cap, mata, classico);
    registro_jogo(partida, dem_casa, dem_fora, teto, mandante, visitante, recorde, novo, disputa);
    total
}

/// Mata-mata: categoria de copa (4 copa nacional, 5 repescagem da liga, 6 fase final, 7/8 continental, 23 mundial) fora
/// da liga do mandante, com o código de rodada de eliminatória (10 a 0xe1, variante 0 a 5), fora dos grupos (prefixo
/// 0x418 a 0x426 em +0x32).
unsafe fn eh_mata(partida: usize, da_liga: bool, codigo: i32) -> bool {
    let cat = *((partida + 0x3a) as *const u8);
    let prefixo = w(partida + 0x32) as u16;
    !da_liga && matches!(cat, 4..=8 | 23) && !(0x418..=0x426).contains(&prefixo) && (10..=0xe1).contains(&codigo) && codigo % 10 <= 5
}

/// O jogo grande pela tabela (title_race = on): os dois ainda podem ser campeões (melhor posição possível, +0x39 da
/// linha da tabela, = 1). O fator cresce com a temporada, 1 + 0,5 × jogos feitos / jogos da temporada, e +0,25 com os
/// dois nos dois primeiros: até 1,75 na reta final de um 1º × 2º. Soma ao clássico do banco (×1,5): um clássico
/// valendo o título chega a 1,5 × 1,75.
unsafe fn fator_disputa(liga: usize, casa: usize, fora: usize) -> f64 {
    if liga == 0 || d(liga + 0xb1) == 0 {
        return 1.0;
    }
    let n = w(liga + 0x3e).max(0) as usize;
    let total = w(liga + 0xa7) as f64;
    let (mut a, mut b) = (0usize, 0usize);
    for k in 0..n.min(200) {
        let l = d(liga + 0xb1) + 0x3b * k;
        if d(l) == casa {
            a = l;
        } else if d(l) == fora {
            b = l;
        }
    }
    if a == 0 || b == 0 || total <= 0.0 {
        return 1.0;
    }
    let melhor = |l: usize| *((l + 0x39) as *const u8);
    let pos = |l: usize| *((l + 4) as *const u8);
    if melhor(a) != 1 || melhor(b) != 1 {
        return 1.0;
    }
    let feitos = w(a + 0x11).max(w(b + 0x11)) as f64;
    let mut f = 1.0 + 0.5 * (feitos / total).clamp(0.0, 1.0);
    if pos(a) <= 2 && pos(b) <= 2 {
        f += 0.25;
    }
    f
}

unsafe fn somar_jogo(casa: usize, mandante: i32, demanda: i32, cap: i32, mata: bool, classico: bool) {
    let e = somas().entry(d(casa)).or_default();
    e.jogos += 1;
    e.publico += mandante as i64;
    e.demanda += demanda as i64;
    e.capacidade += cap as i64;
    e.cheios += (mandante as f64 >= cap as f64 * 0.92) as i32;
    e.mata += mata as i32;
    e.classicos += classico as i32;
}

#[allow(clippy::too_many_arguments)]
unsafe fn registro_jogo(partida: usize, dem_casa: i32, dem_fora: i32, teto: i32, mandante: i32, visitante: i32, recorde: i32, novo: i32, disputa: f64) {
    if !REGRAS.registrar {
        return;
    }
    let (casa, fora, est) = (d(partida + 0x1c), d(partida + 0x20), d(partida + 0x18));
    let comp = d(partida + 0x14);
    let nivel = if comp != 0 { w(comp + 0x69) as i32 } else { 0 };
    let da_liga = comp != 0 && d(casa + 0x57) == comp;
    let codigo = w(partida + 0x30) as u16 as i32;
    let mata = eh_mata(partida, da_liga, codigo);
    let f = if da_liga { ficha(objeto(comp), casa) } else { 0 };
    let (prev, atual) = if f != 0 { (*((f + 0x72) as *const u8) as i32, *((f + 0x73) as *const u8) as i32) } else { (-1, -1) };
    gravar("publico_jogos.tsv",
        "data	comp	nivel	tipo	rodada	liga	mata	classico	casa	nome	fora	rep_casa	rep_fora	prevista	atual	media	minimo	recorde	capacidade	demanda_casa	demanda_fora	teto	mandante	visitante	recorde_novo	disputa",
        &format!("{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{}	{:.2}",
            hoje(), if comp != 0 { d(comp) as i64 } else { -1 }, nivel, *((partida + 0x3a) as *const u8), codigo, da_liga as u8, mata as u8,
            rival(casa, fora) as u8, d(casa), nome(casa), d(fora), w(casa + 0x80), w(fora + 0x80), prev, atual, i(casa + 0x73), i(casa + 0x77),
            recorde, i(est + 0x3c), dem_casa, dem_fora, teto, mandante, visitante, novo, disputa));
}

/// A tabela ordenada da liga (0x4a4b20: entradas de 0x76 bytes, +0 o id do clube, +5 jogos em casa, +6 público somado,
/// +0x72 a posição prevista, +0x73 a final), com o número de entradas; soltar com 0x9452ca.
unsafe fn tabela(liga: usize) -> (usize, usize) {
    let ordenada: unsafe extern "thiscall" fn(usize, u32, *mut i32, u32) -> usize = core::mem::transmute(0x4a4b20usize);
    let mut n = 0i32;
    let t = if d(liga + 8) != 0 { ordenada(d(liga + 8), 1, &mut n, 1000) } else { 0 };
    (t, if t != 0 { n.max(0) as usize } else { 0 })
}

/// { 0x5d6280 } embrulhada: a média de público no fim da temporada da liga.
unsafe extern "C" fn media(liga: usize) {
    let original: unsafe extern "C" fn(usize) = core::mem::transmute(0x5d6280usize);
    let (t, n) = tabela(liga);
    struct Antes {
        c: usize,
        media: i32,
        minimo: i32,
        recorde: i32,
        prevista: i32,
        fim: i32,
        jogos: i32,
        soma: i32,
    }
    let mut antes = Vec::with_capacity(n);
    for k in 0..n {
        let e = t + 0x76 * k;
        let c = d(0xae23b4) + 0x245 * d(e);
        antes.push(Antes {
            c, media: i(c + 0x73), minimo: i(c + 0x77), recorde: i(c + 0x7b), prevista: *((e + 0x72) as *const u8) as i32,
            fim: *((e + 0x73) as *const u8) as i32, jogos: *((e + 5) as *const u8) as i32, soma: i(e + 6),
        });
    }
    original(liga);
    let p = P;
    let nivel = w(d(liga + 4) + 0x69) as i32;
    for a in &antes {
        let c = a.c;
        medias_antes().insert(d(c), a.media);
        let da_original = i(c + 0x73);
        let recorde = i(c + 0x7b);
        let est = d(c + 0x69);
        let cap = if est != 0 { i(est + 0x3c) } else { 0 };
        let real = if a.jogos > 0 { a.soma / a.jogos } else { a.media };
        let dif = (a.prevista - a.fim).clamp(-5, 5);
        let mut nova = if p.tetos_da_liga {
            da_original as f64
        } else {
            (real + a.media) as f64 / 2.0 * (1.0 + 0.05 * dif as f64)
        };
        if p.tradicao > 0.0 && dif > 0 && (recorde as f64) > nova {
            nova += p.tradicao * (recorde as f64 - nova) * dif as f64 / 10.0;
        }
        let mut teto = recorde;
        if cap > 0 {
            teto = teto.min(cap);
        }
        let piso = a.minimo.max(500);
        let nova = (nova as i32).min(teto).max(piso.min(teto.max(500)));
        if nova != da_original {
            pi(c + 0x73, nova);
        }
        let s = somas().remove(&d(c)).unwrap_or_default();
        ultima().insert(d(c), s);
        if REGRAS.registrar {
            gravar("publico_temporada.tsv",
                "data\tliga\tnivel\tclube\tnome\trep\tprevista\tfinal\tjogos\tpublico_real\tmedia_antes\tmedia_original\tmedia_nova\tminimo\trecorde_antes\trecorde\tcapacidade\tdll_jogos\tdll_publico\tdll_demanda\tdll_cheios\tdll_mata\tdll_classicos",
                &format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    hoje(), d(d(liga + 4)), nivel, d(c), nome(c), w(c + 0x80), a.prevista, a.fim, a.jogos, real, a.media, da_original, nova,
                    a.minimo, a.recorde, recorde, cap, s.jogos, if s.jogos > 0 { s.publico / s.jogos as i64 } else { 0 },
                    if s.jogos > 0 { s.demanda / s.jogos as i64 } else { 0 }, s.cheios, s.mata, s.classicos));
        }
    }
    if t != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(t);
    }
}

/// A média de público do clube na temporada: só a liga, ou todas as competições (pela estatística de cada uma, +5
/// jogos em casa e +6 público somado), com o número de jogos.
unsafe fn media_da_temporada(c: usize, tudo: bool) -> (i32, i32) {
    let (mut jogos, mut soma) = (0i64, 0i64);
    let mut somar = |obj: usize| {
        let f = ficha(obj, c);
        if f != 0 {
            jogos += *((f + 5) as *const u8) as i64;
            soma += i(f + 6) as i64;
        }
    };
    if tudo {
        let n = d(0xae2378);
        for id in 0..n.min(10_000) {
            let obj = d(d(0xadadfc) + 4 * id);
            if obj != 0 {
                somar(obj);
            }
        }
    } else {
        somar(objeto(d(c + 0x57)));
    }
    (if jogos > 0 { (soma / jogos) as i32 } else { 0 }, jogos as i32)
}

/// { 0x59cfa0 } embrulhada: (this = a tabela de finanças, clube), no fim da temporada.
unsafe extern "thiscall" fn estadio(this: usize, clube: usize) {
    let original: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x59cfa0usize);
    let est = if clube != 0 { d(clube + 0x69) } else { 0 };
    if est == 0 || *((clube + 0x6d) as *const u8) == 0 || d(clube + 0x57) == 0 {
        return original(this, clube);
    }
    let p = P;
    let (cap0, sent0, teto0) = (i(est + 0x3c), i(est + 0x40), i(est + 0x44));
    if !p.sentados_crescem {
        pi(est + 0x40, cap0); // sem lugar em pé, a conversão não acontece
    }
    let assume = p.assume_obra();
    if assume {
        pi(est + 0x44, cap0); // sem folga no teto, a original não amplia
    }
    original(this, clube);
    let cap1 = i(est + 0x3c);
    if !p.sentados_crescem {
        let sent1 = i(est + 0x40);
        pi(est + 0x40, if cap1 != cap0 { sent0 + (sent1 - cap0) } else { sent0 });
    }
    let teto_original = i(est + 0x44);
    if assume {
        pi(est + 0x44, teto0);
    }
    if !assume {
        if REGRAS.registrar {
            let (media_t, jogos_t) = media_da_temporada(clube, true);
            let antes = medias_antes().remove(&d(clube)).unwrap_or(i(clube + 0x73));
            let ocup = if cap0 > 0 { media_t * 100 / cap0 } else { 0 };
            let motivo = if cap1 == cap0 { "original: none" } else if i(est + 0x40) - sent0 == cap1 - cap0 && teto_original == teto0 { "original: seating or build" } else { "original: build" };
            registro_obra(clube, cap0, teto0, sent0, media_t, jogos_t, antes, -1, false, 0, cap1 - cap0, 0, ocup, motivo);
        }
        return;
    }
    // a obra pela regra nova
    let (media_t, jogos_t) = media_da_temporada(clube, p.ocupacao_tudo);
    let ocup = if cap0 > 0 { media_t as f64 / cap0 as f64 } else { 0.0 };
    let antes = medias_antes().remove(&d(clube)).unwrap_or(i(clube + 0x73));
    let ocup_antes = if cap0 > 0 { antes as f64 / cap0 as f64 } else { 0.0 };
    let rep = w(clube + 0x80) as i32;
    let mut apto = jogos_t > 0 && ocup >= 0.92 && (p.temporadas_cheias <= 1 || ocup_antes >= 0.92);
    if p.gatilho_reputacao && rep * 2 > cap0 && rep > 4250 {
        apto = true;
    }
    // a soma desta temporada (o estádio antes da média da liga) ou a da que acabou (depois)
    let s = somas().get(&d(clube)).copied().or_else(|| ultima().remove(&d(clube))).unwrap_or_default();
    let demanda = if s.jogos > 0 { (s.demanda / s.jogos as i64) as i32 } else { (media_t as f64 * 1.1) as i32 };
    let mut a = 0u32;
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let pc = parceiro(clube, &mut a, 1);
    let quem = if pc != 0 && a == 0 { pc } else { clube };
    let fin = d(this) + (i(quem) as u32 as usize) * 0x167;
    let saldo = core::ptr::read_unaligned(fin as *const i64);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let reserva = 5_250_000i64 - if humano(clube, 0, 1, 0, 0) != 0 { rep as i64 * 1000 } else { 0 };
    let profissional = *((clube + 0x64) as *const u8) == 1;
    let unidade = if p.custo_lugar < 0.0 { custo_auto() } else { p.custo_lugar };
    let mut motivo = "ok";
    let mut n = 0i32;
    let mut custo = 0i64;
    if !apto {
        motivo = "not eligible";
    } else if saldo <= reserva {
        motivo = "no money";
    } else if cap0 > p.maximo {
        motivo = "at max";
    } else {
        let quer = if p.obra_demanda {
            demanda - cap0
        } else if saldo > 20_000_000 {
            if profissional { 16000 } else { 21000 }
        } else if profissional {
            (saldo as f64 * 0.1) as i32 / 125
        } else {
            (saldo as f64 * 0.2) as i32 / 50
        };
        let teto = if teto0 > cap0 { teto0 } else if p.teto_demanda { cap0 + quer.max(0) } else { teto0 };
        let terco = (teto / 3).max(5000);
        n = quer.min(teto - cap0).min(terco).min(p.maximo - cap0);
        // o custo por lugar: o da chave, ou o da original (profissional: ((n/1000+1)×6000 + n) × 125; senão (n + (n/1000+1)×1000) × 50)
        let custo_de = |n: i32| -> i64 {
            if unidade > 0.0 {
                (n as f64 * unidade) as i64
            } else if profissional {
                (((n / 1000 + 1) * 6000 + n) as i64) * 125
            } else {
                ((n + (n / 1000 + 1) * 1000) as i64) * 50
            }
        };
        if unidade > 0.0 {
            let cabe = ((saldo - reserva) as f64 / unidade) as i32;
            n = n.min(cabe);
        }
        n = n / 500 * 500;
        if n < 1000 {
            motivo = if quer < 1000 { "no demand" } else if teto - cap0 < 1000 { "ceiling" } else { "too small" };
            n = 0;
            if !p.teto_demanda && cap0 < 30000 && teto0 <= cap0 {
                pi(est + 0x44, teto0 + 1500); // como a original
            }
        } else {
            custo = custo_de(n);
            pi(est + 0x3c, cap0 + n);
            if profissional {
                pi(est + 0x40, i(est + 0x40) + n);
            }
            let custo32 = custo.min(i32::MAX as i64) as i32;
            pi(fin + 0x8c, i(fin + 0x8c).wrapping_add(custo32));
            pi(fin + 0x12c, i(fin + 0x12c).wrapping_add(custo32));
            core::ptr::write_unaligned(fin as *mut i64, saldo - custo32 as i64);
            let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(0x59ffb0usize);
            noticia(this, clube, n, if profissional { 1 } else { 2 });
        }
    }
    if REGRAS.registrar {
        registro_obra(clube, cap0, teto0, sent0, media_t, jogos_t, antes, demanda, apto, saldo, n, custo, (ocup * 100.0) as i32, motivo);
    }
}

/// O custo por lugar `auto`: 1.500 no banco de 2001 (uma obra real da época, de 1.000 a 3.500 libras por lugar), na
/// proporção do caixa mediano dos clubes de reputação 8.000 ou mais do banco carregado (+0x65 do clube; 15,4 milhões no
/// de 2001; 56 milhões no do CLZ 6.1, 3,6 vezes). Calculado uma vez.
unsafe fn custo_auto() -> f64 {
    static mut VALOR: f64 = 0.0;
    if VALOR > 0.0 {
        return VALOR;
    }
    let (base, n) = (d(0xae23b4), d(0xae235c));
    let mut caixas: Vec<i32> = (0..n.min(100_000)).map(|k| base + 0x245 * k)
        .filter(|&c| w(c + 0x80) >= 8000 && i(c + 0x65) > 0).map(|c| i(c + 0x65)).collect();
    caixas.sort_unstable();
    let mediana = caixas.get(caixas.len() / 2).copied().unwrap_or(15_400_000);
    let v = (1500.0 * mediana as f64 / 15_400_000.0).clamp(100.0, 1e6);
    VALOR = v;
    registro::linha(&format!("stadium seat cost auto: {:.0} per place (median cash of the {} clubs with reputation 8000+: {}; 2001 database: 15400000 -> 1500)",
        v, caixas.len(), mediana));
    v
}

#[allow(clippy::too_many_arguments)]
unsafe fn registro_obra(c: usize, cap0: i32, teto0: i32, sent0: i32, media_t: i32, jogos_t: i32, antes: i32, demanda: i32, apto: bool,
    saldo: i64, n: i32, custo: i64, ocup: i32, motivo: &str) {
    gravar("publico_estadio.tsv",
        "data\tclube\tnome\trep\tcapacidade\tteto\tsentados\tmedia_temporada\tjogos\tocupacao_pct\tmedia_antes\tdemanda\tapto\tsaldo\tlugares\tcusto\tmotivo",
        &format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", hoje(), d(c), nome(c), w(c + 0x80), cap0, teto0, sent0,
            media_t, jogos_t, ocup, antes, demanda, apto as u8, saldo, n, custo, motivo));
}
