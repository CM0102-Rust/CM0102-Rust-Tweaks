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
    format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", d(c), nome(c), d(comp), w(comp + 0x69), w(c + 0x80),
        d(c + 0x73) as i32, d(c + 0x77) as i32, d(c + 0x7b) as i32, cap, teto)
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
    gravar("estudo_clubes.tsv", "data\tclube\tnome\tcomp\tcomp_rep\trep\tpublico\tpub_min\tpub_max\tcapacidade\tteto_estadio", &linhas);

    let mut agora = HashMap::new();
    let (mut jogadores, mut mudancas) = (Vec::new(), Vec::new());
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
        let mut elenco: Vec<usize> = (0..50).map(|j| d(c + 0xd7 + 4 * j)).filter(|&p| p != 0).collect();
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
