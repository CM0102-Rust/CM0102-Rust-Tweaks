//! Medição para os estudos do mercado e da reputação e do público (só com `TWEAKS_ESTUDO=1` no ambiente; não é chave do cfg).
//! Só lê a memória, na batida mensal, e grava três tabelas ao lado do exe:
//! - `estudo_clubes.tsv`, todo mês: cada clube de competição no jogo com a reputação, o público (médio, mínimo, máximo),
//!   o estádio (capacidade, teto de expansão) e a competição (id, reputação);
//! - `estudo_jogadores.tsv`, em julho: cada jogador em clube de competição com idade, CA, PA, reputações e o clube;
//! - `estudo_mudancas.tsv`, todo mês: quem mudou de clube desde o mês anterior (de, para, as reputações e o CA).
//! ponytail: leitura sem trava de outra linha de execução, como a linha `season`; desvios nas rotinas só se a foto mensal
//! não bastar para os estudos.

use super::*;
use std::collections::HashMap;
use std::io::Write;

pub(crate) fn ligado() -> bool {
    std::env::var("TWEAKS_ESTUDO").map(|v| v == "1").unwrap_or(false)
}

fn gravar(arquivo: &str, cabecalho: &str, linhas: &[String]) {
    let caminho = std::env::current_exe().map(|p| p.with_file_name(arquivo)).unwrap_or_else(|_| arquivo.into());
    let novo = !caminho.exists();
    if let Ok(f) = std::fs::OpenOptions::new().create(true).append(true).open(caminho) {
        let mut f = std::io::BufWriter::new(f);
        if novo {
            let _ = writeln!(f, "{}", cabecalho);
        }
        for l in linhas {
            let _ = writeln!(f, "{}", l);
        }
    }
}

unsafe fn u(a: usize) -> u16 {
    *(a as *const u16)
}

/// Clube de competição: (id, reputação, id e reputação da competição).
unsafe fn clube(c: usize) -> String {
    let comp = d(c + 0x57);
    let estadio = d(c + 0x69);
    let (cap, teto) = if estadio != 0 { (d(estadio + 0x3c) as i32, d(estadio + 0x44) as i32) } else { (-1, -1) };
    // o saldo de verdade fica nas finanças (0x5a0590, thiscall em [0xae2a38], ret 4; BALANCE +0, i64); +0x65 é o
    // dinheiro do banco de dados, que a carreira não atualiza. A verba: 0x598800 thiscall(fin, 0, 0, 0, 1, 0) -> i64.
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let fin = financas(d(0xae2a38), c);
    let (saldo, v) = if fin != 0 { (core::ptr::read_unaligned(fin as *const i64), verba(fin, 0, 0, 0, 1, 0)) } else { (0, 0) };
    // Business (+0xf) e Ambition do presidente (+0xbf, ficha de não jogador em +0x69); -1 sem presidente
    let pres = d(c + 0xbf);
    let np = if pres != 0 { d(pres + 0x69) } else { 0 };
    let business = if np != 0 { b(np + 0xf) as i32 } else { -1 };
    format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", d(c), nome(c), d(comp), w(comp + 0x69), w(c + 0x80),
        d(c + 0x73) as i32, d(c + 0x77) as i32, d(c + 0x7b) as i32, cap, teto, d(c + 0x65) as i32, saldo, v, business)
}

static mut ANTERIOR: Option<HashMap<usize, usize>> = None;

/// Chamada pela batida mensal (data "AAAA-DDD"; `julho` = o mês 6).
pub(crate) unsafe fn medir(data: &str, julho: bool) {
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    let (clubes, nc) = (d(0xae23b4), d(0xae235c));
    // [0xadadfc] (as competições que rodam, lida por em_competicao) ainda é vazia no primeiro dia da carreira
    if pessoas == 0 || clubes == 0 || d(0xadadfc) == 0 || n > 2_000_000 || nc > 100_000 {
        return;
    }
    let linhas: Vec<String> = (0..nc).map(|i| clubes + 0x245 * i).filter(|&c| em_competicao(c))
        .map(|c| format!("{}\t{}", data, clube(c))).collect();
    gravar("estudo_clubes.tsv", "data\tclube\tnome\tcomp\tcomp_rep\trep\tpublico\tpub_min\tpub_max\tcapacidade\tteto_estadio\tcaixa\tsaldo\tverba\tbusiness", &linhas);

    let mut agora = HashMap::new();
    let (mut jogadores, mut mudancas) = (Vec::new(), Vec::new());
    let mut emprestados = Vec::new();
    let mut jovens = Vec::new();
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    // os jogos de cada (pessoa, clube) nas ligas da temporada: o Comp_Stats de cada liga (objeto em [[0xadadfc] + 4 × id] + 8),
    // tabela de jogadores em +0x19 (45 bytes: +4 a pessoa, +8 o clube da última partida, +0xc os jogos), quantidade em +0x21
    let mut jogos_liga: HashMap<(usize, usize), u32> = HashMap::new();
    let mut ligas = std::collections::HashSet::new();
    for i in 0..nc {
        let c = clubes + 0x245 * i;
        let comp = d(c + 0x57);
        if comp != 0 && em_competicao(c) && ligas.insert(d(comp)) {
            let obj = d(d(0xadadfc) + 4 * d(comp));
            let est = if obj != 0 { d(obj + 8) } else { 0 };
            if est == 0 {
                continue;
            }
            let (tab, q) = (d(est + 0x19), d(est + 0x21));
            if tab == 0 || q > 100_000 {
                continue;
            }
            for r in 0..q {
                let e = tab + 45 * r;
                *jogos_liga.entry((d(e + 4), d(e + 8))).or_insert(0) += *((e + 0xc) as *const u8) as u32;
            }
        }
    }
    let anterior = (*core::ptr::addr_of_mut!(ANTERIOR)).take();
    for s in 0..n {
        let p = pessoas + 110 * s;
        let ficha = d(p + 0x61);
        if ficha == 0 {
            continue;
        }
        let c = d(p + 0x39);
        let id_clube = if c != 0 { d(c) } else { usize::MAX };
        agora.insert(s, id_clube);
        // os jogos do contrato com o clube atual (+0x3b) de quem não está num contrato comum (tipo 5 = empréstimo)
        if c != 0 {
            let k = contrato(d(0xae1050), p, c);
            if k != 0 && *((k + 0x35) as *const u8) & 0x3f != 2 {
                emprestados.push(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", data, s, d(c), w(c + 0x80), *((k + 0x35) as *const u8) & 0x3f,
                    jogos_liga.get(&(d(p), d(c))).copied().unwrap_or(0), b(p + 0x18), w(ficha + 5), w(ficha + 7)));
            }
        }
        // os jovens promissores (até 21, PA − CA ≥ 30) e os jogos de liga no clube atual, para comparar com os emprestados
        if c != 0 && b(p + 0x18) <= 21 && w(ficha + 7) - w(ficha + 5) >= 30 {
            jovens.push(format!("{}\t{}\t{}\t{}", data, s, d(c), jogos_liga.get(&(d(p), d(c))).copied().unwrap_or(0)));
        }
        let dados = || format!("{}\t{}\t{}\t{}\t{}\t{}", b(p + 0x18), w(ficha + 5), w(ficha + 7), u(ficha + 9), u(ficha + 0xb), u(ficha + 0xd));
        if julho && c != 0 && em_competicao(c) {
            jogadores.push(format!("{}\t{}\t{}\t{}\t{}", data, s, dados(), d(c), w(c + 0x80)));
        }
        if let Some(antes) = anterior.as_ref().and_then(|a| a.get(&s)) {
            if *antes != id_clube {
                let rep = |id: usize| if id < nc { w(clubes + 0x245 * id + 0x80) as i32 } else { -1 };
                mudancas.push(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}", data, s, dados(), *antes as i64, rep(*antes), id_clube as i64, rep(id_clube)));
            }
        }
    }
    *core::ptr::addr_of_mut!(ANTERIOR) = Some(agora);
    if julho {
        gravar("estudo_jogadores.tsv", "data\tpessoa\tidade\tca\tpa\trep_casa\trep_atual\trep_mundial\tclube\tclube_rep", &jogadores);
    }
    gravar_mercado();
    gravar("estudo_emprestados.tsv", "data\tpessoa\tclube\tclube_rep\ttipo\tjogos\tidade\tca\tpa", &emprestados);
    gravar("estudo_jovens.tsv", "data\tpessoa\tclube\tjogos", &jovens);
    gravar("estudo_mudancas.tsv", "data\tpessoa\tidade\tca\tpa\trep_casa\trep_atual\trep_mundial\tde\tde_rep\tpara\tpara_rep", &mudancas);
}

static mut VISTOS: Option<std::collections::HashSet<(usize, usize)>> = None;

/// Todo dia de jogo: o mesmo jogador duas vezes no elenco de um clube (+0xd7, 50 ponteiros), a causa da caixa
/// "match_man.cpp 1225" (a escalação automática 0x6dbc50 confere jogador repetido). Uma linha por par novo em
/// `estudo_duplicados.tsv`, com a pessoa, os dois clubes que ela aponta e a idade.
pub(crate) unsafe fn duplicados(data: &str) {
    let (clubes, nc) = (d(0xae23b4), d(0xae235c));
    if clubes == 0 || nc > 100_000 {
        return;
    }
    let vistos = (*core::ptr::addr_of_mut!(VISTOS)).get_or_insert_with(Default::default);
    let mut linhas = Vec::new();
    for k in 0..nc {
        let c = clubes + 0x245 * k;
        let mut elenco: Vec<usize> = (0..50).map(|j| d(c + 0xd7 + 4 * j)).filter(|&p| crate::elenco::pessoa_valida(p)).collect();
        elenco.sort_unstable();
        for par in elenco.windows(2) {
            if par[0] == par[1] && vistos.insert((d(c), par[0])) {
                let p = par[0];
                let onde = d(p + 0x39);
                linhas.push(format!("{}	{}	{}	{}	{}	{}	{}", data, d(c), nome(c), d(p), b(p + 0x18),
                    if onde != 0 { d(onde) as i64 } else { -1 }, if d(p + 0x61) != 0 { "player" } else { "non-player" }));
            }
        }
    }
    if !linhas.is_empty() {
        for l in &linhas {
            registro::linha(&format!("duplicate in squad	{}", l));
        }
        gravar("estudo_duplicados.tsv", "data	clube	nome	pessoa	idade	clube_da_pessoa	tipo", &linhas);
    }
}

/// A comissão escalando agora (0x6dbc50), para a caixa "match_man.cpp 1225" dizer de que clube é.
static mut ESCALANDO: usize = 0;

/// Desvia as 5 chamadas da escalação automática (0x6dbc50) para anotar o clube; com o mesmo jogador duas vezes no
/// elenco, uma linha antes de escalar.
pub(crate) unsafe fn instalar() {
    let sitios = [0x6db8fe, 0x6db91b, 0x6db973, 0x6db98d, 0x6dba73];
    let chama = |de: usize| *(de as *const u8) == 0xe8 && (de + 5).wrapping_add(core::ptr::read_unaligned((de + 1) as *const i32) as usize) == 0x6dbc50;
    if sitios.iter().all(|&de| chama(de)) {
        for de in sitios {
            chamar(de, escalar as *const () as usize);
        }
        registro::linha("study: watching the automatic team selection (0x6dbc50) for the repeated-player box");
    }
    instalar_conferencia();
    instalar_mercado();
}

unsafe fn elenco_repetido(c: usize) -> Vec<usize> {
    let mut e: Vec<usize> = (0..50).map(|j| d(c + 0xd7 + 4 * j)).filter(|&p| crate::elenco::pessoa_valida(p)).collect();
    e.sort_unstable();
    e.windows(2).filter(|p| p[0] == p[1]).map(|p| p[0]).collect()
}

unsafe extern "thiscall" fn escalar(t: usize, outro: usize, modo: u32) {
    let original: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6dbc50usize);
    let c = d(t + 0x2f);
    if c != 0 {
        for p in elenco_repetido(c) {
            registro::linha(&format!("study: repeated in the squad at selection	{}	club {} {}	person {} age {} points to club {}",
                crate::data_do_jogo(), d(c), nome(c), d(p), b(p + 0x18), if d(p + 0x39) != 0 { d(d(p + 0x39)) as i64 } else { -1 }));
        }
    }
    crate::elenco::limpar_clube(c, "before the team selection");
    ESCALANDO = t;
    original(t, outro, modo);
    ESCALANDO = 0;
}

/// Chamada pela caixa de mensagem: se é a da escalação, o clube e o elenco inteiro.
pub(crate) unsafe fn caixa_da_escalacao(texto: &str) {
    if !texto.contains("match_man") || ESCALANDO == 0 {
        return;
    }
    let t = ESCALANDO;
    let c = d(t + 0x2f);
    if c == 0 {
        return;
    }
    let jogo = d(t + 0x37);
    registro::linha(&format!("study: the box came while selecting club {} {} (staff {:#x}, match {:#x}, human {})", d(c), nome(c), t, jogo, *((t + 0x19) as *const u8)));
    for j in 0..50 {
        let p = d(c + 0xd7 + 4 * j);
        if crate::elenco::pessoa_valida(p) {
            let ficha = d(p + 0x61);
            registro::linha(&format!("study: squad {}	person {}	age {}	club {}	player {}	ca {}", j, d(p), b(p + 0x18),
                if d(p + 0x39) != 0 { d(d(p + 0x39)) as i64 } else { -1 }, (ficha != 0) as u8, if ficha != 0 { w(ficha + 5) } else { -1 }));
        }
    }
}

/// Os dois caminhos que põem uma pessoa num elenco sem conferir se ela já está lá: `database_add_staff_to_club`
/// (0x53bd10, cdecl(clube, pessoa, ...)) e `player_regen_sign_for_club` (0x7a6f10, thiscall(this, pessoa, clube)). Na
/// entrada de cada um, um trecho em assembly passa o endereço de retorno (quem chamou), o clube e a pessoa para
/// `conferir` e segue para a original pelo trampolim.
static mut TRAMPOLIM_ADD: usize = 0;
static mut TRAMPOLIM_REGEN: usize = 0;

unsafe extern "C" fn conferir(retorno: usize, clube: usize, pessoa: usize) {
    if clube < 0x10000 || !crate::elenco::pessoa_valida(pessoa) {
        return;
    }
    if (0..50).any(|j| d(clube + 0xd7 + 4 * j) == pessoa) {
        registro::linha(&format!("study: adding a person already in the squad\t{}\tclub {} {}\tperson {} age {}\tcalled from {:#x}",
            crate::data_do_jogo(), d(clube), nome(clube), d(pessoa), b(pessoa + 0x18), retorno.wrapping_sub(5)));
    }
}

#[unsafe(naked)]
unsafe extern "C" fn entrada_add() {
    // na entrada: [esp] retorno, [esp+4] clube, [esp+8] pessoa
    core::arch::naked_asm!(
        "pushad",
        "push dword ptr [esp+40]",
        "push dword ptr [esp+40]",
        "push dword ptr [esp+40]",
        "call {c}",
        "add esp, 12",
        "popad",
        "jmp dword ptr [{t}]",
        c = sym conferir,
        t = sym TRAMPOLIM_ADD,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn entrada_regen() {
    // thiscall: [esp] retorno, [esp+4] pessoa, [esp+8] clube
    core::arch::naked_asm!(
        "pushad",
        "push dword ptr [esp+36]",
        "push dword ptr [esp+44]",
        "push dword ptr [esp+40]",
        "call {c}",
        "add esp, 12",
        "popad",
        "jmp dword ptr [{t}]",
        c = sym conferir,
        t = sym TRAMPOLIM_REGEN,
    );
}

pub(crate) unsafe fn instalar_conferencia() {
    if crate::elenco::ligado() {
        return; // a proteção do elenco já fica nessas entradas e registra cada cópia apagada
    }
    match crate::origem::trampolim(0x53bd10, &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00], entrada_add as *const () as usize) {
        Some(t) => TRAMPOLIM_ADD = t,
        None => registro::linha("study: 0x53bd10 differs; not watched"),
    }
    match crate::origem::trampolim(0x7a6f10, &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00], entrada_regen as *const () as usize) {
        Some(t) => TRAMPOLIM_REGEN = t,
        None => registro::linha("study: 0x7a6f10 differs; not watched"),
    }
    registro::linha("study: watching every addition to a squad (0x53bd10, 0x7a6f10) for repeated people");
}

// ---- Mercado (1.5): cada decisão da 0x84af00 e cada transferência concluída (0x8cb810) ----

/// As linhas do mercado esperando a gravação (na batida mensal, ou a cada 20 mil).
static MERCADO: std::sync::Mutex<(Vec<String>, Vec<String>)> = std::sync::Mutex::new((Vec::new(), Vec::new()));
static mut T_PREVER: usize = 0;
static mut T_PASSADA: usize = 0;
static mut QUEM_PREVE: usize = 0;
/// As previsões em massa contadas (gravadas em `estudo_previsoes.tsv` na batida mensal) e os pedidos de empréstimo previstos
/// já registrados no ano (uma linha por pessoa, comprador e motivo).
static CONTAGEM: std::sync::Mutex<Option<HashMap<String, u64>>> = std::sync::Mutex::new(None);
static VISTOS_EMP: std::sync::Mutex<Option<std::collections::HashSet<(usize, usize, String, &'static str)>>> = std::sync::Mutex::new(None);

const CAB_DECISOES: &str = "data\tquem\temprestimo\tresposta\tmotivo\tvendedor\tvendedor_nome\tvendedor_rep\tvendedor_humano\tcomprador\tcomprador_rep\tpessoa\tidade\tca\tpa\trep_atual\tlistado\testatuto\tmoral\ttipo_contrato\toferta\tpedido";
const CAB_PASSADAS: &str = "data\ta0\ta1\ta2\ta3\ta4\ta5\tvalor\ttipo\tdia\tano";

fn guardar_mercado(decisao: Option<String>, passada: Option<String>) {
    let Ok(mut m) = MERCADO.lock() else { return };
    if let Some(l) = decisao {
        m.0.push(l);
    }
    if let Some(l) = passada {
        m.1.push(l);
    }
    if m.0.len() >= 20_000 || m.1.len() >= 20_000 {
        let (a, b) = (core::mem::take(&mut m.0), core::mem::take(&mut m.1));
        drop(m);
        gravar("estudo_decisoes.tsv", CAB_DECISOES, &a);
        gravar("estudo_passadas.tsv", CAB_PASSADAS, &b);
    }
}

/// Grava o que está esperando (chamada pela batida mensal).
pub(crate) fn gravar_mercado() {
    if let Ok(mut c) = CONTAGEM.lock() {
        if let Some(c) = c.take() {
            let linhas: Vec<String> = c.into_iter().map(|(k, n)| format!("{}\t{}", k, n)).collect();
            gravar("estudo_previsoes.tsv", "ano\tquem\temprestimo\tresposta\tmotivo\tn", &linhas);
        }
    }
    let Ok(mut m) = MERCADO.lock() else { return };
    let (a, b) = (core::mem::take(&mut m.0), core::mem::take(&mut m.1));
    drop(m);
    gravar("estudo_decisoes.tsv", CAB_DECISOES, &a);
    gravar("estudo_passadas.tsv", CAB_PASSADAS, &b);
}

unsafe fn f64_em(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// Por que a 0x84af00 respondeu assim a um pedido de empréstimo (a mesma ordem das regras dela; ver EMPRESTIMOS.md).
unsafe fn motivo_emprestimo(vendedor: usize, jogador: usize, comprador: usize, k: usize) -> &'static str {
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    if vendedor == 0 || (k != 0 && b(k + 0x49) != 0) {
        return "no_seller_or_flag";
    }
    if d(vendedor + 0xcf) == jogador {
        return "is_manager";
    }
    if k == 0 || b(k + 0x35) & 0x3f != 2 {
        return "no_contract";
    }
    if humano(vendedor, 0, 1, 0, 0) != 0 {
        return "human_seller";
    }
    let rep = w(comprador + 0x80) as f64;
    let rep_j = u(d(jogador + 0x61) + 0xb) as f64;
    let lis = listado(jogador, 0) != 0;
    if lis && rep * f64_em(0x968928) > rep_j {
        return "listed";
    }
    let st = b(k + 0x4f) as u8 & 0xf0;
    if st == 0x60 || st == 0x70 || b(d(jogador + 0x61) + 0xf) == 0x14 {
        if rep > rep_j * f64_em(0x969040) && (b(k + 0x3d) as i8) < -50 && d(vendedor + 0x1db) != d(0x9cf6e8) {
            return "status_6_7";
        }
        return if lis { "listed_rep_low_and_status_fail" } else { "status_fail" };
    }
    if lis { "listed_rep_low" } else { "not_listed" }
}

unsafe extern "stdcall" fn prever(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let original: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8 = core::mem::transmute(T_PREVER);
    let quem = QUEM_PREVE.wrapping_sub(5);
    let r = original(vendedor, jogador, oferta, comprador, pedido, emprestimo);
    let r = crate::mercado::prever_emprestimo(vendedor, jogador, comprador, emprestimo, r, quem);
    if jogador == 0 || comprador < 0x10000 {
        return r;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    let k = if vendedor != 0 { contrato(d(0xae1050), jogador, vendedor) } else { 0 };
    let emp = emprestimo as u8 != 0;
    let motivo = if emp { motivo_emprestimo(vendedor, jogador, comprador, k) } else { "-" };
    let ficha = d(jogador + 0x61);
    let (ca, pa, rep_j) = if ficha != 0 { (w(ficha + 5) as i32, w(ficha + 7) as i32, u(ficha + 0xb) as i32) } else { (-1, -1, -1) };
    let (vid, vnome, vrep, vh) = if vendedor != 0 { (d(vendedor) as i64, nome(vendedor), w(vendedor + 0x80) as i32, humano(vendedor, 0, 1, 0, 0)) } else { (-1, String::new(), -1, 0) };
    let (st, moral, tipo) = if k != 0 { ((b(k + 0x4f) as u8 >> 4) as i32, b(k + 0x3d) as i8 as i32, (b(k + 0x35) as u8 & 0x3f) as i32) } else { (-1, 0, -1) };
    let data = data_do_jogo();
    let massa = matches!(quem, 0x8435c3 | 0x84049c | 0x8405a2);
    if massa {
        let Ok(mut c) = CONTAGEM.lock() else { return r };
        *c.get_or_insert_with(HashMap::new).entry(format!("{}\t{:#x}\t{}\t{}\t{}", &data[..4], quem, emp as u8, r, motivo)).or_insert(0) += 1;
        drop(c);
        if quem != 0x8405a2 {
            return r;
        }
        let Ok(mut v) = VISTOS_EMP.lock() else { return r };
        if !v.get_or_insert_with(Default::default).insert((d(jogador), d(comprador), data[..4].to_string(), motivo)) {
            return r;
        }
    }
    guardar_mercado(Some(format!("{}\t{:#x}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        data, quem, emp as u8, r, motivo, vid, vnome, vrep, vh, d(comprador), w(comprador + 0x80), d(jogador), b(jogador + 0x18),
        ca, pa, rep_j, listado(jogador, 0), st, moral, tipo, oferta, pedido)), None);
    r
}

#[unsafe(naked)]
unsafe extern "C" fn entrada_prever() {
    core::arch::naked_asm!("mov eax, [esp]", "mov [{q}], eax", "jmp {f}", q = sym QUEM_PREVE, f = sym prever);
}

/// { 0x8cb810 } (thiscall, ret 4): `TRANSFER_MANAGER::add_past_transfer(registro)`: 37 bytes, +0..+0x17 seis u32 (a
/// pessoa e os clubes), +0x18 o valor, +0x1c o tipo, +0x1d a data (dia do ano u16, ano u16).
unsafe extern "thiscall" fn passada(t: usize, r: usize) {
    let original: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(T_PASSADA);
    if r != 0 {
        let a = |k: usize| d(r + 4 * k) as i32;
        guardar_mercado(None, Some(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", data_do_jogo(), a(0), a(1), a(2), a(3), a(4), a(5),
            d(r + 0x18) as i32, b(r + 0x1c) as u8, u(r + 0x1d), u(r + 0x1f))));
    }
    original(t, r)
}

pub(crate) unsafe fn instalar_mercado() {
    match crate::origem::trampolim(0x84af00, &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00], entrada_prever as *const () as usize) {
        Some(t) => T_PREVER = t,
        None => registro::linha("study: 0x84af00 differs; transfer and loan decisions not watched"),
    }
    match crate::origem::trampolim(0x8cb810, &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00], passada as *const () as usize) {
        Some(t) => T_PASSADA = t,
        None => registro::linha("study: 0x8cb810 differs; completed transfers not watched"),
    }
    registro::linha("study: watching transfer and loan decisions (0x84af00) and completed transfers (0x8cb810)");
}
