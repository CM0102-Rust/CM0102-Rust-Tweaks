//! A disputa dos clubes pelo regen (`rule = new`, 0x7ad0e0) e o modo de observação (`rule = original` com `log = 1`).

use super::*;

/// O regen basco só vai para os clubes bascos. A 0x7ad0e0 olha só a segunda nacionalidade (+0x1e) e aceita 5 clubes
/// (Athletic e o B, Real Sociedad e o B, Alavés); aqui vale também a primeira (+0x1a; bancos novos trazem bascos nela) e
/// também os clubes que o próprio jogo marca como "só contrata bascos" (0x53d7f0: Eibar, Barakaldo, Sestao..., que a
/// original deixava sem regen basco).
pub(crate) unsafe fn basco(jogador: usize) -> bool {
    let e = |n: usize| n != 0 && d(n) == d(0x9cf424);
    e(d(jogador + 0x1a)) || e(d(jogador + 0x1e))
}

pub(crate) unsafe fn vetado(jogador: usize, clube: usize) -> bool {
    if !basco(jogador) {
        return false;
    }
    let so_bascos: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d7f0usize);
    let id = d(clube);
    ![0x9d0120usize, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124].iter().any(|&g| d(g) == id) && so_bascos(clube) == 0
}

/// Os filtros da 0x7ad0e0: não estrangeiro (0x53d980), não é o time B (0x540a50), não vetado, pode chegar ao jogador
/// (0x8ce990). Devolve o elenco (0x53c590) do clube elegível.
pub(crate) unsafe fn elegivel(jogador: usize, clube: usize) -> Option<i32> {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    // o basco não é estrangeiro na Espanha (só aqui, na escolha do clube do regen: num banco com bascos na primeira
    // nacionalidade, a 0x53d980 original o barraria em todo clube espanhol; o desvio no jogo inteiro está no ROADMAP)
    let nacao = d(clube + 0x53);
    let espanhol = nacao != 0 && d(nacao) == d(0x9cf488) && basco(jogador);
    if (estrangeiro(jogador, nacao, 1) != 0 && !espanhol) || time_b(clube) || vetado(jogador, clube) {
        return None;
    }
    let mut motivo = 0i16;
    if pode(d(0xde1f54), jogador, clube, &mut motivo, 0) == 0 {
        return None;
    }
    Some(elenco_de(clube, 0, 0) as i8 as i32)
}

/// Diagnóstico da fornada: por que o clube não pode receber o jovem ("ok" se pode). `bid N` = o motivo da 0x8ce990.
pub(crate) unsafe fn motivo_recusa(jogador: usize, clube: usize) -> String {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    if estrangeiro(jogador, d(clube + 0x53), 1) != 0 {
        return "foreign".into();
    }
    if vetado(jogador, clube) {
        return "vetoed".into();
    }
    let mut motivo = 0i16;
    if pode(d(0xde1f54), jogador, clube, &mut motivo, 0) == 0 {
        return format!("bid {}", motivo);
    }
    "ok".into()
}

/// O time B de outro clube (0x540a50: tem par e não é o time A); a disputa não manda jovem para ele.
pub(crate) unsafe fn time_b(clube: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a = 0u32;
    parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0
}

/// O melhor "Working with Youngsters" (+0x22 da ficha de não jogador +0x69) entre o técnico (+0xcf) e os 5 treinadores
/// (+0x19f); sem ninguém, 1.
pub(crate) unsafe fn melhor_com_jovens(clube: usize) -> i32 {
    let mut m = 1;
    for p in core::iter::once(d(clube + 0xcf)).chain((0..5).map(|k| d(clube + 0x19f + 4 * k))) {
        if p != 0 && d(p + 0x69) != 0 {
            m = m.max(b(d(p + 0x69) + 0x22) as i32);
        }
    }
    m
}

/// O clube (id + 1) para o qual a disputa mandou cada pessoa, pela regra nova ou pela do jogo: a linha `season` separa
/// os regens que a disputa deu ao clube dos que passaram por ela rumo a outro clube e chegaram depois (transferência ou
/// empréstimo). Vetor fixo, sem alocar na linha do jogo (o jogo lê memória não inicializada, e a heap muda a trajetória
/// da carreira; TESTES §4.5). ponytail: um id reaproveitado por outro regen herda o destino; ids acima de 2^18 não contam.
pub(crate) static DESTINO: [core::sync::atomic::AtomicU16; 1 << 18] = [const { core::sync::atomic::AtomicU16::new(0) }; 1 << 18];

/// Por onde o regen chegou pela fornada (diagnóstico da linha `season`): o clube + 1 nos 16 bits de baixo, 1 (cota)
/// ou 2 (sobra) nos de cima. ponytail: o mesmo limite de ids do DESTINO.
pub(crate) static FORNADA: [core::sync::atomic::AtomicU32; 1 << 18] = [const { core::sync::atomic::AtomicU32::new(0) }; 1 << 18];

pub(crate) fn marcar_fornada(pessoa: usize, clube: usize, sobra: bool) {
    if pessoa < 1 << 18 && clube < 0xffff {
        FORNADA[pessoa].store((clube as u32 + 1) | if sobra { 2 << 16 } else { 1 << 16 }, core::sync::atomic::Ordering::Relaxed);
    }
}

/// 0: não chegou ao `clube` pela fornada; 1: pela cota; 2: como sobra.
pub(crate) fn pela_fornada(pessoa: usize, clube: usize) -> u32 {
    let v = if pessoa < 1 << 18 { FORNADA[pessoa].load(core::sync::atomic::Ordering::Relaxed) } else { 0 };
    if v & 0xffff == clube as u32 + 1 { v >> 16 } else { 0 }
}

pub(crate) fn marcar_destino(pessoa: usize, clube: usize) {
    if pessoa < 1 << 18 && clube < 0xffff {
        DESTINO[pessoa].store(clube as u16 + 1, core::sync::atomic::Ordering::Relaxed);
    }
}

/// `Some(clube)` se a pessoa passou pela disputa.
pub(crate) fn destino(pessoa: usize) -> Option<usize> {
    let v = if pessoa < 1 << 18 { DESTINO[pessoa].load(core::sync::atomic::Ordering::Relaxed) } else { 0 };
    (v != 0).then(|| v as usize - 1)
}

/// A carreira foi criada com a linha `ARSE` no `player_setup.cfg`? A linha (tipo 0x20 da 0x7a3a10, laço 0x7a3c0c) liga o
/// bit 0x40 do regen em todas as pessoas no fim da `create_game` (0x5cc850, chamada em 0x829fd9, lê o arquivo com a
/// máscara 0x203b em 0x5ccc9c), e nada desliga o bit. Os técnicos humanos (classificação +0x5f = 0, as últimas vagas da
/// tabela) nunca viram regen (a 0x7abac0 grava +0x5f = 2): um deles com o bit só pode ser obra do ARSE. Vale para a
/// carreira nova e para o save carregado, sem contar quantos têm o bit nem ler o .cfg de agora.
pub(crate) unsafe fn arse() -> bool {
    let (pessoas, n, t) = (d(0xae23bc), d(0xae2364), d(0xae2d0c));
    if pessoas == 0 || t == 0 || n > 2_000_000 {
        return false;
    }
    // ponytail: só as últimas 64 vagas, onde a criação põe os 16 técnicos humanos
    (n.saturating_sub(64)..n).rev().map(|s| pessoas + 110 * s).find(|&p| *((p + 0x5f) as *const u8) == 0)
        .is_some_and(|p| *((t + 79 * d(p) + 0xb) as *const u8) & 0x40 != 0)
}

/// Os jovens que a disputa nova entregou: (id do clube, dia corrido = ano × 365 + dia).
pub(crate) static mut ENTREGUES: Vec<(usize, i32)> = Vec::new();

pub(crate) unsafe fn hoje() -> i32 {
    w(0xae2c92) as i32 * 365 + w(0xae2c90) as i32
}

/// Quantos jovens o clube recebeu nos últimos 365 dias: os que esta DLL entregou (mesmo que o clube já os tenha
/// emprestado, vendido ou mandado para o time B; contar só o elenco deixava o teto vazar, 15 no Real Madrid em 4 anos)
/// ou, se for maior, os regens (bit 0x40 do +0xb do registro de 79 bytes) do elenco (+0xd7) que entraram no clube
/// (+0x3e dia, +0x40 ano) no período (o que vale logo depois de carregar um save, com a lista vazia). Numa carreira
/// criada com ARSE (`arse`), só a lista.
/// ponytail: a lista não vai para o save; depois da carga, só a contagem pelo elenco até a lista encher de novo.
pub(crate) unsafe fn recebidos_no_ano(clube: usize) -> i32 {
    let (id, agora) = (d(clube), hoje());
    let pela_lista = (*core::ptr::addr_of!(ENTREGUES)).iter().filter(|&&(c, t)| c == id && agora - t < 365).count() as i32;
    if arse() {
        return pela_lista; // com ARSE todo jogador tem o bit: o elenco não diz quem é regen
    }
    let pelo_elenco = (0..50usize)
        .map(|k| d(clube + 0xd7 + 4 * k))
        .filter(|&p| p != 0 && *((d(0xae2d0c) + 79 * d(p) + 0xb) as *const u8) & 0x40 != 0)
        .filter(|&p| agora - (w(p + 0x40) as i32 * 365 + w(p + 0x3e) as i32) < 365)
        .count() as i32;
    pela_lista.max(pelo_elenco)
}

/// O clube joga uma competição que roda no jogo (a divisão +0x57 com `[[0xadadfc] + 4 × id]`, como na 0x7ad220)?
pub(crate) unsafe fn em_competicao(clube: usize) -> bool {
    let comp = d(clube + 0x57);
    comp != 0 && d(d(0xadadfc) + 4 * d(comp)) != 0
}

/// O peso do clube para o jovem (o mesmo de `peso_novo` em calibrar.py). Na surpresa, o nível não conta e só entram os
/// clubes de competição que roda no jogo.
pub(crate) unsafe fn peso(r: &Regras, pa: f64, ca: f64, clube: usize, elenco: i32, surpresa: bool) -> f64 {
    if elenco as f64 >= r.elenco_teto || (r.teto_temporada > 0.0 && recebidos_no_ano(clube) as f64 >= r.teto_temporada) {
        return 0.0;
    }
    if surpresa && !em_competicao(clube) {
        return 0.0;
    }
    let alvo = r.alvo_k * ((1.0 - r.alvo_ca) * pa + r.alvo_ca * ca);
    let rep = w(clube + 0x80) as f64;
    // a curva é assimétrica: o clube menor que o alvo cai mais depressa (sigma_below), senão as centenas de clubes
    // pequenos de um país, somados, levam os jovens bons
    let largura = if rep < alvo && r.sigma_abaixo > 0.0 { r.sigma_abaixo } else { r.sigma };
    let z = (rep - alvo) / largura;
    let nivel = if surpresa { 1.0 } else { r.piso_nivel + (1.0 - r.piso_nivel) * (-0.5 * z * z).exp() };
    let inst = r.inst_base + r.inst_k * b(clube + 0x7f) as f64;
    let comissao = r.com_min + (r.com_max - r.com_min) * (melhor_com_jovens(clube) - 1) as f64 / 19.0;
    let vaga = ((r.elenco_teto - elenco as f64) / r.elenco_teto).powf(r.elenco_exp);
    let p = (nivel * inst * comissao * vaga).max(0.0).powf(r.temperatura);
    // os clubes fora das competições que rodam (a divisão não roda no jogo): o peso vezes `outside_competitions`; na
    // surpresa eles já ficaram de fora acima
    if r.fora_competicao < 1.0 && !surpresa && !em_competicao(clube) { p * r.fora_competicao } else { p }
}

/// A fornada: o peso do clube para o jovem pela curva da aba Regens, sem o teto por temporada e sem o espaço no elenco
/// (a cota da fornada já é o limite; elenco cheio, 50, fica de fora). `None`: o clube não pode recebê-lo (os filtros
/// da disputa: estrangeiro, time B, vetado).
pub(crate) unsafe fn peso_na_fornada(jogador: usize, clube: usize, surpresa: bool) -> Option<f64> {
    elegivel(jogador, clube)?;
    // as 50 vagas do próprio clube; a 0x53c590 soma o time B (na Espanha, em Portugal e na Alemanha, A + B passa de 50)
    let elenco = (0..50).filter(|&k| d(clube + 0xd7 + 4 * k) != 0).count() as i32;
    if elenco >= 50 {
        return None;
    }
    let mut r = REGRAS;
    r.elenco_teto = 50.0;
    r.elenco_exp = 0.0;
    r.teto_temporada = 0.0;
    let ficha = d(jogador + 0x61);
    Some(peso(&r, w(ficha + 7) as f64, w(ficha + 5) as f64, clube, elenco, surpresa))
}

/// A surpresa da aba Regens (`surprise`, %) para um jovem.
pub(crate) unsafe fn sorteia_surpresa() -> bool {
    REGRAS.surpresa > 0.0 && (sorteio(10000) as f64) < REGRAS.surpresa * 100.0
}

/// Um número uniforme em [0, 1) com dois sorteios do jogo (30 bits).
pub(crate) unsafe fn uniforme() -> f64 {
    ((sorteio(0x8000) as u32) << 15 | sorteio(0x8000) as u32) as f64 / (1u32 << 30) as f64
}

pub(crate) unsafe fn nome(clube: usize) -> String {
    let s = core::slice::from_raw_parts((clube + 4) as *const u8, 51);
    s.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect()
}

/// { 0x7ad0e0 } (thiscall, ret 4): a regra nova da disputa. Mesmos clubes e filtros da original; o clube é sorteado na
/// proporção do peso. Nenhum peso positivo: 0 (o jovem fica sem clube, como na original sem clube elegível).
pub(crate) unsafe extern "thiscall" fn escolher_clube(_this: usize, jogador: usize) -> usize {
    let r = REGRAS;
    let ficha = d(jogador + 0x61);
    let (pa, ca) = (w(ficha + 7) as f64, w(ficha + 5) as f64);
    let total_de_clubes = (d(0xae235c) as i32).wrapping_sub((d(0xae2350) as i32).wrapping_mul(2));
    let base = d(0xae23b4);
    let mut surpresa = r.surpresa > 0.0 && (sorteio(10000) as f64) < r.surpresa * 100.0;
    let mut pesos: Vec<(usize, f64)> = Vec::new();
    // a surpresa sem nenhum clube de competição no país (as ligas dele não estão no jogo) volta ao sorteio normal, senão
    // o jovem ficaria sem clube só por causa dela
    for _ in 0..2 {
        pesos.clear();
        for i in 0..total_de_clubes.max(0) as usize {
            let clube = base + 0x245 * i;
            if let Some(elenco) = elegivel(jogador, clube) {
                let p = peso(&r, pa, ca, clube, elenco, surpresa);
                if p > 0.0 {
                    pesos.push((clube, p));
                }
            }
        }
        if !pesos.is_empty() || !surpresa {
            break;
        }
        surpresa = false;
    }
    let soma: f64 = pesos.iter().map(|x| x.1).sum();
    if soma <= 0.0 {
        if r.registrar {
            registrar_escolha("new", jogador, 0, "-\t0\t-");
        }
        return 0;
    }
    let mut x = uniforme() * soma;
    let mut escolhido = pesos[pesos.len() - 1];
    for &(c, p) in &pesos {
        x -= p;
        if x < 0.0 {
            escolhido = (c, p);
            break;
        }
    }
    if r.registrar {
        registrar_escolha("new", jogador, escolhido.0, &format!("{:.1}%\t{}\t{}", 100.0 * escolhido.1 / soma, pesos.len(), if surpresa { "surprise" } else { "-" }));
    }
    let lista = &mut *core::ptr::addr_of_mut!(ENTREGUES);
    let agora = hoje();
    lista.retain(|&(_, t)| agora - t < 365);
    lista.push((d(escolhido.0), agora));
    escolhido.0
}

/// A data do jogo como AAAA-ddd (o dia do ano em [0xae2c90], o ano em [0xae2c92]).
pub(crate) unsafe fn data_do_jogo() -> String {
    format!("{}-{:03}", w(0xae2c92) as u16, w(0xae2c90) as u16)
}

/// Uma linha do log por jovem da disputa, separada por tabulação (para contar depois):
/// `regen  regra  data  idade  PA  CA  reputação  Training  elenco  clube  [chance  disputaram  surpresa]`.
pub(crate) unsafe fn registrar_escolha(regra: &str, jogador: usize, clube: usize, extra: &str) {
    JOVENS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    if clube != 0 {
        marcar_destino(d(jogador), d(clube));
    }
    let ficha = d(jogador + 0x61);
    let (rep, tr, nome_clube) = if clube != 0 { (w(clube + 0x80) as i32, b(clube + 0x7f) as i32, nome(clube)) } else { (-1, -1, "(no club)".into()) };
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let elenco = if clube != 0 { elenco_de(clube, 0, 0) as i32 } else { -1 };
    registro::linha(&format!("regen\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", regra, data_do_jogo(), b(jogador + 0x18), w(ficha + 7), w(ficha + 5), rep, tr, elenco, nome_clube, extra));
}

/// A entrada original da 0x7ad0e0, para o modo de observação (`rule = original` com `log = 1`): os 7 bytes do prólogo
/// (`sub esp, 0x14; mov [esp+0x10], ecx`, sem salto relativo) copiados para um trecho que volta em 0x7ad0e7.
pub(crate) static mut DISPUTA_ORIGINAL: usize = 0;
/// "original" (rule = original) ou "exe" (a disputa ficou com o exe modificado, override 0).
pub(crate) static mut DISPUTA_ROTULO: &str = "original";

/// { 0x7ad0e0 } no modo de observação: a disputa da original escolhe, e a escolha vai para o log.
pub(crate) unsafe extern "thiscall" fn observar_clube(this: usize, jogador: usize) -> usize {
    let original: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(DISPUTA_ORIGINAL);
    let clube = original(this, jogador);
    registrar_escolha(DISPUTA_ROTULO, jogador, clube, "-\t-\t-");
    clube
}

pub(crate) unsafe fn instalar_observacao() -> bool {
    let t = VirtualAlloc(core::ptr::null_mut(), 16, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return false;
    }
    core::ptr::copy_nonoverlapping(0x7ad0e0 as *const u8, t, 7);
    *t.add(7) = 0xe9;
    core::ptr::write_unaligned(t.add(8) as *mut i32, 0x7ad0e7usize.wrapping_sub(t as usize + 12) as i32);
    DISPUTA_ORIGINAL = t as usize;
    desviar(0x7ad0e0, observar_clube as *const () as usize);
    true
}
