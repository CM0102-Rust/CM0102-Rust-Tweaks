//! As chaves do `tweaks.cfg` (docs/CONFIGURADOR.md §3): o controle, a faixa e o padrão da DLL sem a linha (`PADRAO` em
//! `src/lib.rs`). As faixas que o código não limita são as propostas do projeto.

pub enum Tipo {
    /// Interruptor: grava "1" ou "0" (a DLL liga só com "1"; `log` com qualquer número ≠ 0).
    Liga,
    /// Seletor: (valor gravado, rótulo pt, rótulo en); o primeiro é o original.
    Escolha(&'static [(&'static str, &'static str, &'static str)]),
    /// Número: (mínimo e máximo do campo, faixa da barra, inteiro).
    Numero { min: f64, max: f64, barra: (f64, f64), inteiro: bool },
}

pub struct Chave {
    pub nome: &'static str,
    pub aba: Aba,
    pub tipo: Tipo,
    /// O valor da DLL sem a linha.
    pub padrao: &'static str,
    pub rotulo: (&'static str, &'static str),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Aba {
    Inicio,
    Jogo,
    Idades,
    Inativos,
    Disputa,
    Registro,
    Cfg,
}

pub const ABAS: [(Aba, &str, &str); 7] = [
    (Aba::Inicio, "Início", "Start"),
    (Aba::Jogo, "Jogo", "Game"),
    (Aba::Idades, "Idades", "Ages"),
    (Aba::Inativos, "Inativos", "Idle players"),
    (Aba::Disputa, "Disputa", "Contest"),
    (Aba::Registro, "Registro", "Log"),
    (Aba::Cfg, "tweaks.cfg", "tweaks.cfg"),
];

const fn num(min: f64, max: f64, b0: f64, b1: f64, inteiro: bool) -> Tipo {
    Tipo::Numero { min, max, barra: (b0, b1), inteiro }
}

const INF: f64 = 1e9;

pub const CHAVES: &[Chave] = &[
    Chave { nome: "save_over_2gb", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Save acima de 2 GB", "Saves above 2 GB") },
    Chave { nome: "portable_folder", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Pasta portátil (sem SetupReg)", "Portable folder (no SetupReg)") },
    Chave { nome: "clear_admin_flag", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Tirar a marca de administrador da NickDLL", "Clear NickDLL's admin flag") },
    Chave { nome: "override_exe_changes", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Sobrescrever as mudanças do exe", "Override the exe's changes") },
    Chave { nome: "video", aba: Aba::Jogo, tipo: Tipo::Escolha(&[("original", "DirectDraw do Windows", "Windows DirectDraw"), ("window", "Janela do projeto (Print Screen, escala)", "Project window (Print Screen, scaling)")]), padrao: "original", rotulo: ("Vídeo", "Video") },
    Chave { nome: "resolution", aba: Aba::Jogo, tipo: Tipo::Escolha(&[("original", "A do exe", "The exe's"), ("1080", "1920×1080", "1920×1080")]), padrao: "original", rotulo: ("Resolução", "Resolution") },
    Chave { nome: "no_cd", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Sem o CD", "No CD") },
    Chave { nome: "ages", aba: Aba::Idades, tipo: Tipo::Escolha(&[("original", "Original", "Original"), ("custom", "Tabela", "Table")]), padrao: "original", rotulo: ("Idades dos regens", "Regen ages") },
    Chave { nome: "younger_from_age", aba: Aba::Idades, tipo: num(0.0, 126.0, 14.0, 35.0, true), padrao: "22", rotulo: ("Mais novos: a partir de", "Younger: from age") },
    Chave { nome: "younger_by", aba: Aba::Idades, tipo: num(0.0, 126.0, 0.0, 15.0, true), padrao: "0", rotulo: ("Mais novos: anos a menos (0 = desligado)", "Younger: years less (0 = off)") },
    Chave { nome: "ages_out_of_game", aba: Aba::Idades, tipo: Tipo::Escolha(&[("original", "Sorteio original", "Original draw"), ("table", "A tabela também", "The table too")]), padrao: "original", rotulo: ("Nações fora do jogo", "Nations not in the game") },
    Chave { nome: "ca_cap_skip", aba: Aba::Idades, tipo: num(0.0, 100.0, 0.0, 5.0, false), padrao: "0.5", rotulo: ("Pular o teto do CA (%)", "Skip the CA cap (%)") },
    Chave { nome: "contest_max_age", aba: Aba::Idades, tipo: num(0.0, 126.0, 14.0, 25.0, true), padrao: "18", rotulo: ("Idade máxima da disputa", "Contest max age") },
    Chave { nome: "unattached", aba: Aba::Idades, tipo: Tipo::Escolha(&[("original", "Original", "Original"), ("only_if_no_room", "Sem clube só sem vaga", "Clubless only without room")]), padrao: "original", rotulo: ("Quem vai à disputa", "Who goes to the contest") },
    Chave { nome: "retire_idle_players", aba: Aba::Inativos, tipo: Tipo::Liga, padrao: "0", rotulo: ("Aposentar os inativos", "Retire idle players") },
    Chave { nome: "idle_days", aba: Aba::Inativos, tipo: num(0.0, INF, 0.0, 730.0, true), padrao: "180", rotulo: ("Dias sem clube", "Days without a club") },
    Chave { nome: "idle_min_age", aba: Aba::Inativos, tipo: num(0.0, 126.0, 16.0, 40.0, true), padrao: "28", rotulo: ("Idade mínima", "Minimum age") },
    Chave { nome: "idle_rep", aba: Aba::Inativos, tipo: num(0.0, INF, 0.0, 10000.0, false), padrao: "1500", rotulo: ("Reputação-limite", "Reputation threshold") },
    Chave { nome: "idle_rep_per_year", aba: Aba::Inativos, tipo: num(0.0, INF, 0.0, 2000.0, false), padrao: "250", rotulo: ("A mais por ano de idade", "More per year of age") },
    Chave { nome: "protect_young_unemployed", aba: Aba::Inativos, tipo: num(0.0, 126.0, 0.0, 30.0, true), padrao: "0", rotulo: ("Proteger os menores de (0 = não)", "Protect those under (0 = no)") },
    Chave { nome: "rule", aba: Aba::Disputa, tipo: Tipo::Escolha(&[("original", "Original", "Original"), ("new", "Sorteio proporcional", "Proportional draw")]), padrao: "original", rotulo: ("A disputa dos clubes", "The clubs' contest") },
    Chave { nome: "target_k", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 100.0, false), padrao: "50", rotulo: ("Alvo: reputação por ponto de PA", "Target: reputation per PA point") },
    Chave { nome: "target_ca", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 1.0, false), padrao: "0", rotulo: ("Alvo: peso do CA (0 = PA, 1 = CA)", "Target: CA weight (0 = PA, 1 = CA)") },
    Chave { nome: "sigma", aba: Aba::Disputa, tipo: num(1.0, INF, 100.0, 10000.0, false), padrao: "2500", rotulo: ("Largura acima do alvo", "Width above the target") },
    Chave { nome: "sigma_below", aba: Aba::Disputa, tipo: num(0.0, INF, 0.0, 10000.0, false), padrao: "1200", rotulo: ("Largura abaixo do alvo (0 = igual)", "Width below the target (0 = same)") },
    Chave { nome: "level_floor", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 0.05, false), padrao: "0.002", rotulo: ("Piso do nível", "Level floor") },
    Chave { nome: "facilities_base", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 2.0, false), padrao: "0.4", rotulo: ("Instalações: base", "Facilities: base") },
    Chave { nome: "facilities_k", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 0.2, false), padrao: "0.06", rotulo: ("Instalações: por ponto de Training", "Facilities: per Training point") },
    Chave { nome: "staff_min", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 2.0, false), padrao: "0.8", rotulo: ("Comissão: o pior", "Staff: the worst") },
    Chave { nome: "staff_max", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 2.0, false), padrao: "1.2", rotulo: ("Comissão: o melhor", "Staff: the best") },
    Chave { nome: "squad_limit", aba: Aba::Disputa, tipo: num(1.0, 50.0, 20.0, 50.0, false), padrao: "48", rotulo: ("Elenco: teto", "Squad limit") },
    Chave { nome: "squad_exp", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 3.0, false), padrao: "0.5", rotulo: ("Elenco: expoente da vaga", "Squad: room exponent") },
    Chave { nome: "season_cap", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 10.0, false), padrao: "2", rotulo: ("Teto por clube a cada 365 dias (0 = sem)", "Cap per club every 365 days (0 = none)") },
    Chave { nome: "surprise", aba: Aba::Disputa, tipo: num(0.0, 100.0, 0.0, 50.0, false), padrao: "15", rotulo: ("Surpresa (%)", "Surprise (%)") },
    Chave { nome: "temperature", aba: Aba::Disputa, tipo: num(-INF, INF, 0.25, 3.0, false), padrao: "1", rotulo: ("Temperatura", "Temperature") },
    Chave { nome: "log", aba: Aba::Registro, tipo: Tipo::Liga, padrao: "0", rotulo: ("Registro (tweaks.log)", "Log (tweaks.log)") },
];

/// A tabela de idades sem as linhas (`PADRAO.peso_da_idade` e `teto_da_idade`), de 14 a 35.
pub const IDADES_PADRAO: [(f64, i32); 22] = [
    (0.0, 80), (0.0025, 80), (16.67, 80), (29.11, 80), (29.11, 100), (12.44, 110), (12.44, 110), (0.04, -1), (0.04, -1),
    (0.04, -1), (0.04, -1), (0.04, -1), (0.04, -1), (0.0, -1), (0.0, -1), (0.0, -1), (0.0, -1), (0.0, -1), (0.0, -1),
    (0.0, -1), (0.0, -1), (0.0, -1),
];

pub fn chave(nome: &str) -> Option<&'static Chave> {
    CHAVES.iter().find(|c| c.nome == nome)
}

/// Ligado como a DLL entende (`ler_regras`): os interruptores só com "1" (o `log` com qualquer número ≠ 0); os seletores
/// com qualquer valor que não seja o original.
pub fn ligado(nome: &str, valor: &str) -> bool {
    match chave(nome).map(|c| &c.tipo) {
        Some(Tipo::Liga) if nome == "log" => valor.parse::<f64>().is_ok_and(|x| x != 0.0),
        Some(Tipo::Liga) => valor == "1",
        Some(Tipo::Escolha(op)) => op.iter().skip(1).any(|o| o.0 == valor),
        _ => false,
    }
}

/// As predefinições (docs/CONFIGURADOR.md §4): (nome pt, nome en, valores). As chaves de instalação (pasta, marca,
/// sobrescrever, log, vídeo, resolução, CD) ficam fora: dependem da instalação, não do gosto.
pub const PREDEFINICOES: &[(&str, &str, &[(&str, &str)])] = &[
    ("Original", "Original", &[("ages", "original"), ("ages_out_of_game", "original"), ("unattached", "original"), ("retire_idle_players", "0"), ("rule", "original")]),
    ("Só o save", "Save only", &[("save_over_2gb", "1"), ("ages", "original"), ("ages_out_of_game", "original"), ("unattached", "original"), ("retire_idle_players", "0"), ("rule", "original")]),
    ("Equilibrado", "Balanced", &[("ages", "custom"), ("younger_from_age", "22"), ("younger_by", "6"), ("ages_out_of_game", "original"), ("ca_cap_skip", "0.5"), ("contest_max_age", "18"), ("unattached", "original"),
        ("retire_idle_players", "1"), ("idle_days", "180"), ("idle_min_age", "28"), ("idle_rep", "1500"), ("idle_rep_per_year", "250"), ("protect_young_unemployed", "0"), ("rule", "new"),
        ("target_k", "50"), ("target_ca", "0"), ("sigma", "2500"), ("sigma_below", "1200"), ("level_floor", "0.002"), ("facilities_base", "0.4"), ("facilities_k", "0.06"), ("staff_min", "0.8"),
        ("staff_max", "1.2"), ("squad_limit", "48"), ("squad_exp", "0.5"), ("season_cap", "2"), ("surprise", "15"), ("temperature", "1")]),
    ("Equilibrado, todos com clube", "Balanced, everyone with a club", &[("ages", "custom"), ("younger_from_age", "22"), ("younger_by", "6"), ("ages_out_of_game", "original"), ("ca_cap_skip", "0.5"), ("contest_max_age", "18"), ("unattached", "only_if_no_room"),
        ("retire_idle_players", "1"), ("idle_days", "180"), ("idle_min_age", "28"), ("idle_rep", "1500"), ("idle_rep_per_year", "250"), ("protect_young_unemployed", "0"), ("rule", "new"),
        ("target_k", "50"), ("target_ca", "0"), ("sigma", "2500"), ("sigma_below", "1200"), ("level_floor", "0.002"), ("facilities_base", "0.4"), ("facilities_k", "0.06"), ("staff_min", "0.8"),
        ("staff_max", "1.2"), ("squad_limit", "48"), ("squad_exp", "0.5"), ("season_cap", "2"), ("surprise", "15"), ("temperature", "1")]),
];
