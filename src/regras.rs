//! As regras: o tweaks.cfg lido na abertura, e as chaves do diagnóstico (`log = 1`).

use super::*;

/// O diagnóstico (as linhas para nós, fora do cfg do release): só com `log = 1`.
pub(crate) fn diagnostico() -> bool {
    unsafe { REGRAS.registrar }
}

/// O registro da camada de vídeo: só com `log = 1`.
pub(crate) fn linha_de_video(texto: &str) {
    if unsafe { REGRAS.registrar } {
        registro::linha(texto);
    }
}


#[derive(Clone, Copy)]
pub(crate) struct Regras {
    /// override_exe_changes: sobrescrever a rotina que o exe já alterou (só o corpo; as bordas sempre têm de bater).
    pub(crate) sobrescrever: bool,
    pub(crate) save: bool,
    pub(crate) idades: bool,
    pub(crate) idades_fora: bool,
    /// O peso (%) e o teto do CA (−1: sem teto) de cada idade, de IDADE_MIN em diante.
    pub(crate) peso_da_idade: [f64; IDADES],
    pub(crate) teto_da_idade: [i16; IDADES],
    pub(crate) idade_max_disputa: i32,
    pub(crate) pular_teto: f64,
    /// O jovem regen sorteado com `mais_novo_a_partir` anos ou mais fica `mais_novo_em` anos mais novo (0: não): a ideia de
    /// petermf23 (item 14 do mod das divisões polonesas), 22 anos ou mais ficam 6 anos mais novos.
    pub(crate) mais_novo_a_partir: i32,
    pub(crate) mais_novo_em: i32,
    pub(crate) novo: bool,
    pub(crate) alvo_k: f64,
    pub(crate) alvo_ca: f64,
    pub(crate) sigma: f64,
    /// sigma_below: a largura da curva para o clube menor que o alvo (0: a mesma de cima).
    pub(crate) sigma_abaixo: f64,
    pub(crate) piso_nivel: f64,
    pub(crate) inst_base: f64,
    pub(crate) inst_k: f64,
    pub(crate) com_min: f64,
    pub(crate) com_max: f64,
    pub(crate) elenco_teto: f64,
    pub(crate) elenco_exp: f64,
    pub(crate) teto_temporada: f64,
    /// retire_idle_players: a aposentadoria do desempregado com as chaves abaixo (a 0x7a8050 trocada).
    pub(crate) inativos: bool,
    /// idle_days, idle_min_age, idle_rep, idle_rep_per_year: com idle_min_age anos ou mais e sem clube há mais de idle_days
    /// dias, aposenta com sorteio(reputação atual) < idle_rep + idle_rep_per_year × (idade − idle_min_age).
    pub(crate) inativo_dias: i32,
    pub(crate) inativo_idade: i32,
    pub(crate) inativo_rep: f64,
    pub(crate) inativo_rep_por_ano: f64,
    /// protect_young_unemployed: abaixo dessa idade a regra do desempregado não aposenta jogador (0: sem proteção).
    pub(crate) proteger_jovens: i32,
    /// surprise: % dos jovens sorteados sem o fator do nível, só entre clubes de competição que roda no jogo.
    pub(crate) surpresa: f64,
    /// outside_competitions: o peso dos clubes fora das competições que rodam no jogo (1 = como o resto; 0 = nunca).
    pub(crate) fora_competicao: f64,
    pub(crate) temperatura: f64,
    pub(crate) registrar: bool,
    /// unattached = only_if_no_room: todo jovem de nação no jogo vai à disputa (qualquer idade, sem o 1 em 4); fica sem
    /// clube só quando nenhum clube do país é elegível.
    pub(crate) so_sem_vaga: bool,
    /// portable_folder: o Location do registro é a pasta do exe (sem SetupReg).
    pub(crate) pasta: bool,
    /// multiple_instances: a trava de jogo único (o mutex) vale por pasta: jogos de pastas diferentes abrem juntos.
    pub(crate) instancias: bool,
    /// clear_admin_flag: tirar a marca WIN95 RUNASADMIN que a NickDLL grava para o exe.
    pub(crate) limpar_marca: bool,
    /// video = window: a camada de vídeo do projeto CM0102 (o quadro numa janela comum) em vez do DirectDraw do sistema.
    pub(crate) video: bool,
    /// resolution = 1080: a resolução 1920×1080 do projeto CM0102, em memória (liga também a camada de vídeo).
    pub(crate) resolucao: bool,
    /// no_cd = 1: as 33 checagens do CD contam como achadas, como no exe portátil do projeto CM0102.
    pub(crate) sem_cd: bool,
    /// regen_origins = 1: de quem é cada regen, em regen_origins.csv.
    pub(crate) origens: bool,
    /// grey_players = squad: o CA dos jogadores cinzentos (VIRTUAL_STAFF) pelo nível do clube (o maior entre o elenco e a reputação).
    pub(crate) cinzentos: bool,
    /// grey_players_spread: a largura da faixa do CA do cinzento, centrada no nível.
    pub(crate) cinzentos_espalhar: i32,
    /// regen_names = own: a nação usa os próprios nomes a partir de `nomes_minimo` (a original, só acima de 100).
    pub(crate) nomes: bool,
    pub(crate) nomes_minimo: i32,
}

// As três chaves de ligar começam desligadas: sem o tweaks.cfg (ou sem a linha), a DLL não muda nada no jogo.
pub(crate) const PADRAO: Regras = Regras {
    sobrescrever: false,
    save: false,
    idades: false,
    idades_fora: false,
    // a distribuição original (0x7ac58e): 15 a 18 com sorteio(10000) = 0, 16 a 18 abaixo de 5000, 17 a 20 abaixo de 9976, 21
    // a 26 no resto; o teto do CA por idade é um valor só (a original muda o de 17 e 18 conforme a faixa sorteada)
    peso_da_idade: [0.0, 0.0025, 16.67, 29.11, 29.11, 12.44, 12.44, 0.04, 0.04, 0.04, 0.04, 0.04, 0.04, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    teto_da_idade: [80, 80, 80, 80, 100, 110, 110, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    idade_max_disputa: 18,
    pular_teto: 0.5,
    mais_novo_a_partir: 22,
    mais_novo_em: 0,
    novo: false,
    alvo_k: 50.0,
    alvo_ca: 0.0,
    sigma: 2500.0,
    sigma_abaixo: 1200.0,
    piso_nivel: 0.002,
    inst_base: 0.4,
    inst_k: 0.06,
    com_min: 0.8,
    com_max: 1.2,
    elenco_teto: 48.0,
    elenco_exp: 0.5,
    teto_temporada: 2.0,
    inativos: false,
    inativo_dias: 180,
    inativo_idade: 28,
    inativo_rep: 1500.0,
    inativo_rep_por_ano: 250.0,
    proteger_jovens: 0,
    surpresa: 15.0,
    fora_competicao: 1.0,
    temperatura: 1.0,
    registrar: false,
    so_sem_vaga: false,
    pasta: false,
    instancias: false,
    limpar_marca: false,
    video: false,
    resolucao: false,
    sem_cd: false,
    origens: false,
    cinzentos: false,
    cinzentos_espalhar: 10,
    nomes: false,
    nomes_minimo: 20,
};

pub(crate) static mut REGRAS: Regras = PADRAO;

pub(crate) fn ler_regras() -> Regras {
    let mut r = PADRAO;
    let caminho = std::env::current_exe().map(|p| p.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return r };
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        if k == "rule" {
            r.novo = v == "new"; // rule = new | original
            continue;
        }
        if k == "ages" {
            r.idades = v == "custom"; // ages = custom | original
            continue;
        }
        if k == "ages_out_of_game" {
            r.idades_fora = v == "table"; // ages_out_of_game = table | original
            continue;
        }
        if let Some(n) = k.strip_prefix("age_").and_then(|n| n.parse::<usize>().ok()) {
            // age_NN = peso[, teto do CA]
            if (IDADE_MIN..IDADE_MIN + IDADES).contains(&n) {
                let mut partes = v.split(',').map(str::trim);
                if let Some(p) = partes.next().and_then(|p| p.parse::<f64>().ok()) {
                    r.peso_da_idade[n - IDADE_MIN] = p.max(0.0);
                    r.teto_da_idade[n - IDADE_MIN] = partes.next().and_then(|t| t.parse::<i16>().ok()).unwrap_or(-1);
                }
            }
            continue;
        }
        if k == "override_exe_changes" {
            r.sobrescrever = v == "1";
            continue;
        }
        if k == "retire_idle_players" {
            r.inativos = v == "1";
            continue;
        }
        if k == "unattached" {
            r.so_sem_vaga = v == "only_if_no_room"; // unattached = original | only_if_no_room
            continue;
        }
        if k == "save_over_2gb" {
            r.save = v == "1";
            continue;
        }
        if k == "portable_folder" {
            r.pasta = v == "1";
            continue;
        }
        if k == "multiple_instances" {
            r.instancias = v == "1";
            continue;
        }
        if k == "video" {
            r.video = v == "window"; // video = original | window
            continue;
        }
        if k == "resolution" {
            r.resolucao = v == "1080"; // resolution = original | 1080
            continue;
        }
        if k == "regen_origins" {
            r.origens = v == "1";
            continue;
        }
        if k == "regen_names" {
            r.nomes = v == "own"; // regen_names = original | own
            continue;
        }
        if k == "grey_players" {
            r.cinzentos = v == "squad"; // grey_players = original | squad
            continue;
        }
        if k == "no_cd" {
            r.sem_cd = v == "1";
            continue;
        }
        if k == "clear_admin_flag" {
            r.limpar_marca = v == "1";
            continue;
        }
        let Ok(x) = v.parse::<f64>() else { continue };
        match k {
            "target_k" => r.alvo_k = x,
            "target_ca" => r.alvo_ca = x,
            "sigma" => r.sigma = x.max(1.0),
            "sigma_below" => r.sigma_abaixo = x.max(0.0),
            "level_floor" => r.piso_nivel = x,
            "facilities_base" => r.inst_base = x,
            "facilities_k" => r.inst_k = x,
            "staff_min" => r.com_min = x,
            "staff_max" => r.com_max = x,
            "squad_limit" => r.elenco_teto = x.clamp(1.0, 50.0),
            "squad_exp" => r.elenco_exp = x,
            "season_cap" => r.teto_temporada = x,
            "idle_days" => r.inativo_dias = (x as i32).max(0),
            "idle_min_age" => r.inativo_idade = x as i32,
            "idle_rep" => r.inativo_rep = x.max(0.0),
            "idle_rep_per_year" => r.inativo_rep_por_ano = x.max(0.0),
            "protect_young_unemployed" => r.proteger_jovens = (x as i32).max(0),
            "surprise" => r.surpresa = x.clamp(0.0, 100.0),
            "outside_competitions" => r.fora_competicao = x.clamp(0.0, 1.0),
            "temperature" => r.temperatura = x,
            "log" => r.registrar = x != 0.0,
            "contest_max_age" => r.idade_max_disputa = (x as i32).clamp(0, 126),
            "ca_cap_skip" => r.pular_teto = x,
            "younger_from_age" => r.mais_novo_a_partir = x as i32,
            "younger_by" => r.mais_novo_em = (x as i32).max(0),
            "regen_names_min" => r.nomes_minimo = (x as i32).clamp(1, 101),
            "grey_players_spread" => r.cinzentos_espalhar = (x as i32).clamp(0, 100),
            _ => {}
        }
    }
    r
}

/// O catálogo das chaves: cada chave do `tweaks.cfg` distribuído tem de ser lida pela DLL, estar no configurador e ter
/// a sua seção no guia das chaves nas duas línguas (as `age_NN` contam como uma só, `age_14` no guia).
#[cfg(test)]
mod catalogo {
    #[test]
    fn cada_chave_do_cfg_no_codigo_no_configurador_e_nos_guias() {
        let cfg = include_str!("../tweaks.cfg");
        let codigo = [
            include_str!("regras.rs"), include_str!("regens/fornada.rs"), include_str!("regens/nomes.rs"),
            include_str!("regens/cinzentos.rs"), include_str!("regens/origem.rs"), include_str!("lib.rs"),
        ]
        .concat();
        let configurador = [include_str!("../configurador/src/chaves.rs"), include_str!("../configurador/src/cfg.rs")].concat();
        let guias = [include_str!("../docs/GUIA_DAS_CHAVES.md"), include_str!("../docs/GUIA_DAS_CHAVES.en.md")];
        let mut faltas = Vec::new();
        for linha in cfg.lines() {
            let Some((k, _)) = linha.split_once(" = ") else { continue };
            if k.is_empty() || !k.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_') {
                continue;
            }
            let (no_codigo, no_guia) = if k.starts_with("age_") { ("\"age_".to_string(), "`age_14`".to_string()) } else { (format!("\"{}\"", k), format!("`{}`", k)) };
            if !codigo.contains(&no_codigo) {
                faltas.push(format!("{}: a DLL não lê", k));
            }
            if !configurador.contains(&no_codigo) {
                faltas.push(format!("{}: fora do configurador", k));
            }
            for (g, lingua) in guias.iter().zip(["pt", "en"]) {
                if !g.contains(&no_guia) {
                    faltas.push(format!("{}: fora do guia ({})", k, lingua));
                }
            }
        }
        assert!(faltas.is_empty(), "{}", faltas.join("\n"));
    }
}
