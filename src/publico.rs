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
//!   metade do caminho até o público real dos jogos comuns da liga na temporada (sem clássico e sem jogo grande), ×
//!   (1 + 0,03 × (prevista − final)), sem os tetos de 2.000, 3.000 ou 5.000 × o nível; `tradition_weight`: quem terminou
//!   2 posições ou mais acima do previsto anda essa fração × (prevista − final) / 10 do caminho até o recorde (o
//!   histórico do banco, a torcida que já existiu).
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
    /// os tetos da média da temporada, em pessoas por ponto de nível da divisão (0 = sem teto): divisões de nível até 7,
    /// de 8 a 12 e acima (o original: 2.000, 3.000 e 5.000)
    teto_fraco: f64,
    teto_medio: f64,
    teto_forte: f64,
    /// o corte do público de cada jogo nas divisões de nível até 4: o limite em pessoas por ponto de nível (0 = sem corte)
    /// e quanto do excesso fica, em % (o original: 1.500 e 20%)
    fraca_limite: f64,
    fraca_fica: f64,
    /// a média do fim da temporada: quanto anda na direção do público real (%) e quanto vale cada posição acima ou abaixo
    /// do previsto (%); o piso do original (a sombra)
    rumo_real: f64,
    por_posicao: f64,
    piso: bool,
    /// o jogo grande: o acréscimo na última rodada (%) e o do 1º × 2º (%)
    disputa_max: f64,
    disputa_top2: f64,
    /// o estádio: a ocupação mínima (%) e a reserva de caixa que a obra não toca
    ocupacao_min: f64,
    reserva: f64,
    /// o bônus do campeão da liga na média da temporada seguinte (%; 0 = sem)
    titulo: f64,
}

/// O jogo original (sem tweaks.cfg nenhum, nada muda).
const ORIGINAL: Publico = Publico {
    sentados_crescem: true, gatilho_reputacao: true, tetos_da_liga: true, recorde_real: false, reprimida: 0.0, tradicao: 0.0,
    mata_estadio: false, temporadas_cheias: 1, ocupacao_tudo: false, obra_demanda: false, custo_lugar: 0.0, teto_demanda: false,
    maximo: 125_000, disputa: false, teto_fraco: 2000.0, teto_medio: 3000.0, teto_forte: 5000.0, fraca_limite: 1500.0, fraca_fica: 20.0,
    rumo_real: 50.0, por_posicao: 3.0, piso: true, disputa_max: 50.0, disputa_top2: 25.0, ocupacao_min: 92.0, reserva: 5_250_000.0, titulo: 0.0,
};
/// As nossas regras, o padrão de cada chave ausente de um tweaks.cfg (08/11, Lucas: o padrão é sempre o nosso). O custo
/// por lugar −1 = `auto` (calibrado pelo caixa dos clubes do banco).
const NOSSO: Publico = Publico {
    sentados_crescem: false, gatilho_reputacao: false, tetos_da_liga: false, recorde_real: true, reprimida: 0.1, tradicao: 0.025,
    mata_estadio: true, temporadas_cheias: 2, ocupacao_tudo: true, obra_demanda: true, custo_lugar: -1.0, teto_demanda: true,
    maximo: 125_000, disputa: true, teto_fraco: 0.0, teto_medio: 0.0, teto_forte: 0.0, fraca_limite: 0.0, fraca_fica: 20.0,
    rumo_real: 50.0, por_posicao: 3.0, piso: true, disputa_max: 50.0, disputa_top2: 25.0, ocupacao_min: 92.0, reserva: 5_250_000.0, titulo: 0.0,
};
static mut P: Publico = ORIGINAL;

fn ler() -> Publico {
    let caminho = std::env::current_exe().map(|x| x.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return ORIGINAL };
    let mut p = NOSSO;
    let (mut publico, mut estadio) = (true, true);
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        let n = v.parse::<f64>().ok();
        match k {
            // os interruptores de cada ajuste (o switch do configurador): off = o jogo original, qualquer que seja o resto
            "attendance" => publico = v != "off",
            "stadium" => estadio = v != "off",
            "stadium_seating_growth" => p.sentados_crescem = v == "on",
            "stadium_reputation_trigger" => p.gatilho_reputacao = v == "on",
            "attendance_record" => p.recorde_real = v != "fixed",
            // em %: 10 = 10% de quem ficou de fora vira recorde
            "pent_up_demand" => p.reprimida = n.map(|x| x / 100.0).unwrap_or(p.reprimida).clamp(0.0, 1.0),
            // em % da distância até o recorde histórico, por posição acima do previsto
            "tradition_per_place" => p.tradicao = n.map(|x| x / 100.0).unwrap_or(p.tradicao).clamp(0.0, 0.5),
            "attendance_cap_weak" => p.teto_fraco = n.unwrap_or(p.teto_fraco).max(0.0),
            "attendance_cap_mid" => p.teto_medio = n.unwrap_or(p.teto_medio).max(0.0),
            "attendance_cap_strong" => p.teto_forte = n.unwrap_or(p.teto_forte).max(0.0),
            "weak_league_limit" => p.fraca_limite = n.unwrap_or(p.fraca_limite).max(0.0),
            "weak_league_keep" => p.fraca_fica = n.unwrap_or(p.fraca_fica).clamp(0.0, 100.0),
            "attendance_toward_real" => p.rumo_real = n.unwrap_or(p.rumo_real).clamp(0.0, 100.0),
            "attendance_per_place" => p.por_posicao = n.unwrap_or(p.por_posicao).clamp(0.0, 20.0),
            "attendance_floor" => p.piso = v != "off",
            "title_race_max" => p.disputa_max = n.unwrap_or(p.disputa_max).clamp(0.0, 300.0),
            "title_race_top2" => p.disputa_top2 = n.unwrap_or(p.disputa_top2).clamp(0.0, 300.0),
            "stadium_occupancy_min" => p.ocupacao_min = n.unwrap_or(p.ocupacao_min).clamp(10.0, 100.0),
            "stadium_reserve" => p.reserva = n.unwrap_or(p.reserva).max(0.0),
            "knockout_ceiling" => p.mata_estadio = v != "record",
            "stadium_full_seasons" => p.temporadas_cheias = n.unwrap_or(p.temporadas_cheias as f64).clamp(1.0, 10.0) as u32,
            "stadium_occupancy" => p.ocupacao_tudo = v != "league",
            "stadium_size" => p.obra_demanda = v != "original",
            "stadium_seat_cost" => p.custo_lugar = match v {
                "original" => 0.0,
                "auto" => -1.0,
                _ => n.map(|x| x.clamp(1.0, 1e7)).unwrap_or(p.custo_lugar),
            },
            "stadium_ceiling_zero" => p.teto_demanda = v != "original",
            "title_race" => p.disputa = v != "off",
            // EXPERIMENTAL (1.5): % a mais na média do ano seguinte para o campeão da liga
            "title_bonus" => p.titulo = n.unwrap_or(p.titulo).clamp(0.0, 100.0),
            "stadium_max" => p.maximo = n.unwrap_or(p.maximo as f64).clamp(1_000.0, 1e7) as i32,
            _ => {}
        }
    }
    if !publico {
        let o = ORIGINAL;
        (p.tetos_da_liga, p.recorde_real, p.reprimida, p.tradicao, p.mata_estadio, p.disputa) =
            (o.tetos_da_liga, o.recorde_real, o.reprimida, o.tradicao, o.mata_estadio, o.disputa);
    }
    if !estadio {
        let o = ORIGINAL;
        (p.sentados_crescem, p.gatilho_reputacao, p.temporadas_cheias, p.ocupacao_tudo, p.obra_demanda, p.custo_lugar, p.teto_demanda, p.maximo) =
            (o.sentados_crescem, o.gatilho_reputacao, o.temporadas_cheias, o.ocupacao_tudo, o.obra_demanda, o.custo_lugar, o.teto_demanda, o.maximo);
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
            || self.teto_demanda || self.maximo != 125_000 || self.ocupacao_min != 92.0 || self.reserva != 5_250_000.0
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
        // a sombra (o piso da queda) vai num arquivo ao lado do save: os desvios da carga e da gravação
        if !p.tetos_da_liga && !crate::origem::ganchos_do_save() {
            registro::linha("season attendance: the save and load routines differ from 3.9.68; the original-game floor lasts only until the game closes");
        }
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
    /// os jogos da liga, sem o acréscimo do clássico e do jogo grande: o alvo da média do fim da temporada
    comuns: i32,
    publico_comuns: i64,
}
static mut SOMAS: Option<HashMap<usize, Soma>> = None;
/// A soma da temporada que acabou (a média do fim da temporada a passa para cá; o estádio pode vir depois).
static mut ULTIMA: Option<HashMap<usize, Soma>> = None;
/// A sombra: a média que cada clube teria no jogo original (o piso da queda: quem cresceu pela nossa regra cai, mas não
/// abaixo do que o original daria). Começa na média do banco, anda pela rotina original no fim de cada temporada e vai
/// num arquivo ao lado do save (X.sav → X.publico), gravado junto com ele e lido na carga.
static mut SOMBRA: Option<HashMap<usize, i32>> = None;

unsafe fn sombra() -> &'static mut HashMap<usize, i32> {
    (*core::ptr::addr_of_mut!(SOMBRA)).get_or_insert_with(HashMap::new)
}

const MARCA_SOMBRA: &[u8; 8] = b"CMRTPUB2";
/// As temporadas seguidas com o estádio acima da ocupação mínima, por clube (vão no mesmo arquivo).
static mut CHEIAS: Option<HashMap<usize, u32>> = None;

unsafe fn cheias() -> &'static mut HashMap<usize, u32> {
    (*core::ptr::addr_of_mut!(CHEIAS)).get_or_insert_with(HashMap::new)
}

/// Depois da carga de um save: a sombra dele, se houver o arquivo; sem ele, cada clube começa na média de agora.
pub(crate) unsafe fn carregou(save: &std::path::Path) {
    let m = sombra();
    m.clear();
    somas().clear();
    ultima().clear();
    medias_antes().clear();
    let Ok(v) = std::fs::read(save.with_extension("publico")) else {
        let p = P;
        if p.mexe_na_media() {
            registro::linha("season attendance: no .publico file next to the save; the original-game floor starts at the current averages");
        }
        return;
    };
    cheias().clear();
    let antigo = v.len() >= 8 && &v[..8] == b"CMRTPUB1" && (v.len() - 8) % 8 == 0;
    if antigo {
        for r in v[8..].chunks_exact(8) {
            m.insert(u32::from_le_bytes([r[0], r[1], r[2], r[3]]) as usize, i32::from_le_bytes([r[4], r[5], r[6], r[7]]));
        }
        return;
    }
    if v.len() < 8 || &v[..8] != MARCA_SOMBRA || (v.len() - 8) % 12 != 0 {
        registro::linha("season attendance: the .publico file next to the save is not valid; ignored");
        return;
    }
    for r in v[8..].chunks_exact(12) {
        let id = u32::from_le_bytes([r[0], r[1], r[2], r[3]]) as usize;
        m.insert(id, i32::from_le_bytes([r[4], r[5], r[6], r[7]]));
        cheias().insert(id, u32::from_le_bytes([r[8], r[9], r[10], r[11]]));
    }
    registro::linha(&format!("season attendance: loaded the original-game floor of {} clubs", m.len()));
}

/// Depois de gravar um save (.sav): a sombra ao lado dele.
pub(crate) unsafe fn gravou(save: &std::path::Path) {
    let m = sombra();
    if m.is_empty() {
        return;
    }
    let mut v = MARCA_SOMBRA.to_vec();
    let mut ids: Vec<_> = m.iter().collect();
    ids.sort();
    for (&id, &x) in ids {
        v.extend_from_slice(&(id as u32).to_le_bytes());
        v.extend_from_slice(&x.to_le_bytes());
        v.extend_from_slice(&cheias().get(&id).copied().unwrap_or(0).to_le_bytes());
    }
    if std::fs::write(save.with_extension("publico"), &v).is_err() {
        registro::linha("season attendance: could not write the .publico file next to the save");
    }
}

/// O jogo apagou a carreira: a próxima começa do zero.
pub(crate) unsafe fn liberou() {
    sombra().clear();
    cheias().clear();
    somas().clear();
    ultima().clear();
    medias_antes().clear();
}

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

/// A original (0x85cba0) com a proteção do visitante sem nação: a distância da torcida (0x85d8a0) lê um byte da nação
/// do clube de fora (+0x53, o estágio de desenvolvimento +0x7e) sem conferir; num banco com clube sem nação, o jogo cai
/// (o erro "virtual staff 195" da comunidade, que o protect_against_virtual_staff_195_error do MiscPatches corrige no
/// exe). Aqui, durante a chamada, o visitante sem nação usa a do mandante.
unsafe fn original_protegida(this: usize, partida: usize, saida: *mut i32) -> i32 {
    let original: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> i32 = core::mem::transmute(0x85cba0usize);
    let (casa, fora) = if partida != 0 { (d(partida + 0x1c), d(partida + 0x20)) } else { (0, 0) };
    if fora == 0 || d(fora + 0x53) != 0 || casa == 0 || d(casa + 0x53) == 0 {
        return original(this, partida, saida);
    }
    pi(fora + 0x53, d(casa + 0x53) as i32);
    let r = original(this, partida, saida);
    pi(fora + 0x53, 0);
    registro::linha(&format!("match attendance: away club {} has no nation; the home nation was used for the travel distance", d(fora)));
    r
}

/// { 0x85cba0 } embrulhada: (this, partida, saída) -> total.
unsafe extern "thiscall" fn jogo(this: usize, partida: usize, saida: *mut i32) -> i32 {
    let original = |this: usize, partida: usize, saida: *mut i32| -> i32 { original_protegida(this, partida, saida) };
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
        let liga = d(partida + 0x14) != 0 && d(casa + 0x57) == d(partida + 0x14);
        somar_jogo(casa, *saida.add(1), *saida.add(1), i(est + 0x3c), mata, rival(casa, fora), if liga { Some(if rival(casa, fora) { 1.5 } else { 1.0 }) } else { None });
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
    if da_liga && nivel <= 4 {
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
    let mut dem_casa = dem_casa;
    if da_liga && nivel <= 4 && p.fraca_limite > 0.0 {
        let lim = (p.fraca_limite * nivel.max(1) as f64) as i32;
        if dem_casa > lim {
            dem_casa = lim + ((dem_casa - lim) as f64 * p.fraca_fica / 100.0) as i32;
        }
    }
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
    somar_jogo(casa, mandante, dem_casa, cap, mata, classico, if da_liga { Some(disputa * if classico { 1.5 } else { 1.0 }) } else { None });
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
    let mut f = 1.0 + P.disputa_max / 100.0 * (feitos / total).clamp(0.0, 1.0);
    if pos(a) <= 2 && pos(b) <= 2 {
        f += P.disputa_top2 / 100.0;
    }
    f
}

unsafe fn somar_jogo(casa: usize, mandante: i32, demanda: i32, cap: i32, mata: bool, classico: bool, comum: Option<f64>) {
    let e = somas().entry(d(casa)).or_default();
    // o jogo de liga entra no alvo da média sem o que o clássico (× 1,5) e o jogo grande (× o fator) somaram
    if let Some(k) = comum {
        e.comuns += 1;
        e.publico_comuns += (mandante as f64 / k.max(1.0)) as i64;
    }
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
    // a original roda sobre a sombra de cada clube (a média que o jogo original teria): o resultado é a sombra nova e o
    // piso da queda; depois a média de verdade volta e a nossa regra anda a partir dela
    let usa_sombra = !P.tetos_da_liga;
    if usa_sombra {
        for a in &antes {
            let s = *sombra().entry(d(a.c)).or_insert(a.media);
            pi(a.c + 0x73, s);
        }
    }
    original(liga);
    let mut sombra_nova = Vec::with_capacity(antes.len());
    for a in &antes {
        let s = i(a.c + 0x73);
        sombra_nova.push(s);
        if usa_sombra {
            sombra().insert(d(a.c), s);
            pi(a.c + 0x73, a.media);
        }
    }
    let p = P;
    let nivel = w(d(liga + 4) + 0x69) as i32;
    for (k, a) in antes.iter().enumerate() {
        let c = a.c;
        medias_antes().insert(d(c), a.media);
        let da_original = i(c + 0x73);
        let recorde = i(c + 0x7b);
        let est = d(c + 0x69);
        let cap = if est != 0 { i(est + 0x3c) } else { 0 };
        let s = somas().remove(&d(c)).unwrap_or_default();
        // o alvo: o público real dos jogos da liga, tirado o acréscimo do clássico e do jogo grande (que fica só no jogo); sem a
        // contagem da DLL (save carregado no meio da temporada), o da estatística da liga
        let real = if s.comuns > 0 {
            (s.publico_comuns / s.comuns as i64) as i32
        } else if a.jogos > 0 {
            a.soma / a.jogos
        } else {
            a.media
        };
        let dif = (a.prevista - a.fim).clamp(-5, 5);
        let mut nova = if p.tetos_da_liga {
            da_original as f64
        } else {
            (a.media as f64 + (real - a.media) as f64 * p.rumo_real / 100.0) * (1.0 + p.por_posicao / 100.0 * dif as f64)
        };
        // a tradição só para quem terminou 2 posições ou mais acima do previsto: % da distância por posição
        if p.tradicao > 0.0 && dif >= 2 && (recorde as f64) > nova {
            nova += (p.tradicao * dif as f64).min(1.0) * (recorde as f64 - nova);
        }
        // o campeão da liga (title_bonus): o título atrai gente mesmo para quem já era o favorito (dif = 0)
        if p.titulo > 0.0 && a.fim == 1 {
            nova *= 1.0 + p.titulo / 100.0;
        }
        // os tetos da média pela divisão (0 = sem teto)
        let teto_div = if nivel <= 7 { p.teto_fraco } else if nivel <= 12 { p.teto_medio } else { p.teto_forte };
        if teto_div > 0.0 {
            nova = nova.min(teto_div * nivel.max(1) as f64);
        }
        let mut teto = recorde;
        if cap > 0 {
            teto = teto.min(cap);
        }
        // o piso da queda: o que o jogo original daria (a sombra), nunca abaixo; quem já estava abaixo dela não cai mais
        let mut nova = nova as i32;
        if usa_sombra && p.piso && nova < a.media {
            nova = nova.max(sombra_nova[k].min(a.media));
        }
        let piso = a.minimo.max(500);
        let nova = nova.min(teto).max(piso.min(teto.max(500)));
        if nova != da_original {
            pi(c + 0x73, nova);
        }
        ultima().insert(d(c), s);
        if REGRAS.registrar {
            gravar("publico_temporada.tsv",
                "data\tliga\tnivel\tclube\tnome\trep\tprevista\tfinal\tjogos\tpublico_real\tmedia_antes\tmedia_original\tmedia_nova\tminimo\trecorde_antes\trecorde\tcapacidade\tdll_jogos\tdll_publico\tdll_demanda\tdll_cheios\tdll_mata\tdll_classicos\tsombra",
                &format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}	{}",
                    hoje(), d(d(liga + 4)), nivel, d(c), nome(c), w(c + 0x80), a.prevista, a.fim, a.jogos, real, a.media, da_original, nova,
                    a.minimo, a.recorde, recorde, cap, s.jogos, if s.jogos > 0 { s.publico / s.jogos as i64 } else { 0 },
                    if s.jogos > 0 { s.demanda / s.jogos as i64 } else { 0 }, s.cheios, s.mata, s.classicos, sombra_nova[k]));
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
    let cheia = jogos_t > 0 && ocup * 100.0 >= p.ocupacao_min;
    let seguidas = if cheia { cheias().get(&d(clube)).copied().unwrap_or(0) + 1 } else { 0 };
    cheias().insert(d(clube), seguidas);
    let _ = ocup_antes;
    let mut apto = cheia && seguidas >= p.temporadas_cheias;
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
    let reserva = p.reserva as i64 - if humano(clube, 0, 1, 0, 0) != 0 { rep as i64 * 1000 } else { 0 };
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
