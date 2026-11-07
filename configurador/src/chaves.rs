//! As chaves do `tweaks.cfg` (docs/CONFIGURADOR.md §3): o controle, a faixa e o padrão da DLL sem a linha (`PADRAO` em
//! `src/regras.rs`). As faixas que o código não limita são as propostas do projeto.

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
    Cinzentos,
    Fornada,
    Nomes,
    Registro,
    Cfg,
    Log,
}

/// O menu da esquerda, na ordem: (aba, nome pt, nome en, grupo pt, grupo en); o grupo vazio não tem título. Os grupos
/// (03/11, Lucas): o que muda a carreira, o que só registra, a instalação e os arquivos.
pub const ABAS: [(Aba, &str, &str, &str, &str); 11] = [
    (Aba::Inicio, "Início", "Start", "", ""),
    (Aba::Fornada, "Fornada de regens", "Youth intake", "Ajustes da carreira", "Career tweaks"),
    (Aba::Idades, "Idades dos regens", "Regen ages", "Ajustes da carreira", "Career tweaks"),
    (Aba::Disputa, "Disputa dos clubes", "Clubs' contest", "Ajustes da carreira", "Career tweaks"),
    (Aba::Inativos, "Aposentadoria", "Retirement", "Ajustes da carreira", "Career tweaks"),
    (Aba::Nomes, "Nomes dos regens", "Regen names", "Ajustes da carreira", "Career tweaks"),
    (Aba::Cinzentos, "Jogadores cinzentos", "Grey players", "Ajustes da carreira", "Career tweaks"),
    (Aba::Registro, "De quem é cada regen", "Regen origins", "Registro", "Records"),
    (Aba::Jogo, "Tela, save e pasta", "Screen, save, folder", "Instalação", "Install"),
    (Aba::Cfg, "tweaks.cfg", "tweaks.cfg", "Arquivos", "Files"),
    (Aba::Log, "tweaks.log", "tweaks.log", "Arquivos", "Files"),
];

const fn num(min: f64, max: f64, b0: f64, b1: f64, inteiro: bool) -> Tipo {
    Tipo::Numero { min, max, barra: (b0, b1), inteiro }
}

const INF: f64 = 1e9;

pub const CHAVES: &[Chave] = &[
    Chave { nome: "save_over_2gb", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Save acima de 2 GB", "Saves above 2 GB") },
    Chave { nome: "portable_folder", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Pasta portátil (sem SetupReg)", "Portable folder (no SetupReg)") },
    Chave { nome: "multiple_instances", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Abrir mais de um jogo ao mesmo tempo", "Open more than one game at a time") },
    Chave { nome: "clear_admin_flag", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Tirar a marca de administrador da NickDLL", "Clear NickDLL's admin flag") },
    Chave { nome: "override_exe_changes", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Sobrescrever as mudanças do exe", "Override the exe's changes") },
    Chave { nome: "video", aba: Aba::Jogo, tipo: Tipo::Escolha(&[("original", "DirectDraw do Windows", "Windows DirectDraw"), ("window", "Janela do projeto (Print Screen, escala)", "Project window (Print Screen, scaling)")]), padrao: "original", rotulo: ("Vídeo", "Video") },
    Chave { nome: "resolution", aba: Aba::Jogo, tipo: Tipo::Escolha(&[("original", "A do exe", "The exe's"), ("1080", "1920×1080", "1920×1080")]), padrao: "original", rotulo: ("Resolução", "Resolution") },
    Chave { nome: "no_cd", aba: Aba::Jogo, tipo: Tipo::Liga, padrao: "0", rotulo: ("Sem o CD", "No CD") },
    Chave { nome: "ages", aba: Aba::Idades, tipo: Tipo::Escolha(&[("original", "Original", "Original"), ("custom", "Tabela", "Table")]), padrao: "original", rotulo: ("Idades dos regens", "Regen ages") },
    Chave { nome: "younger_from_age", aba: Aba::Idades, tipo: num(0.0, 126.0, 14.0, 35.0, true), padrao: "22", rotulo: ("Mais novos: a partir de", "Younger: from age") },
    Chave { nome: "younger_by", aba: Aba::Idades, tipo: num(0.0, 126.0, 0.0, 15.0, true), padrao: "0", rotulo: ("Mais novos: anos a menos", "Younger: years less") },
    Chave { nome: "ages_out_of_game", aba: Aba::Idades, tipo: Tipo::Escolha(&[("original", "Sorteio original", "Original draw"), ("table", "A tabela também", "The table too")]), padrao: "original", rotulo: ("Nações fora do jogo", "Nations not in the game") },
    Chave { nome: "ca_cap_skip", aba: Aba::Idades, tipo: num(0.0, 100.0, 0.0, 5.0, false), padrao: "0.5", rotulo: ("Jovens sem o teto do CA (%)", "Youngsters without the CA cap (%)") },
    Chave { nome: "contest_max_age", aba: Aba::Idades, tipo: num(0.0, 126.0, 14.0, 25.0, true), padrao: "18", rotulo: ("Idade máxima da disputa", "Contest max age") },
    Chave { nome: "unattached", aba: Aba::Idades, tipo: Tipo::Escolha(&[("original", "Original", "Original"), ("only_if_no_room", "Sem clube só sem vaga", "Clubless only without room")]), padrao: "original", rotulo: ("Quem vai à disputa", "Who goes to the contest") },
    Chave { nome: "retire_idle_players", aba: Aba::Inativos, tipo: Tipo::Liga, padrao: "0", rotulo: ("Aposentar os inativos", "Retire idle players") },
    Chave { nome: "idle_days", aba: Aba::Inativos, tipo: num(0.0, INF, 0.0, 730.0, true), padrao: "180", rotulo: ("Dias sem clube", "Days without a club") },
    Chave { nome: "idle_min_age", aba: Aba::Inativos, tipo: num(0.0, 126.0, 16.0, 40.0, true), padrao: "28", rotulo: ("Idade mínima", "Minimum age") },
    Chave { nome: "idle_rep", aba: Aba::Inativos, tipo: num(0.0, INF, 0.0, 10000.0, false), padrao: "1500", rotulo: ("Reputação-limite", "Reputation threshold") },
    Chave { nome: "idle_rep_per_year", aba: Aba::Inativos, tipo: num(0.0, INF, 0.0, 2000.0, false), padrao: "250", rotulo: ("A mais por ano de idade", "More per year of age") },
    Chave { nome: "protect_young_unemployed", aba: Aba::Inativos, tipo: num(0.0, 126.0, 0.0, 30.0, true), padrao: "0", rotulo: ("Proteger os jovens: menores de", "Protect the young: under") },
    Chave { nome: "rule", aba: Aba::Disputa, tipo: Tipo::Escolha(&[("original", "Original", "Original"), ("new", "Sorteio proporcional", "Proportional draw")]), padrao: "original", rotulo: ("A disputa dos clubes", "The clubs' contest") },
    Chave { nome: "target_k", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 100.0, false), padrao: "50", rotulo: ("Alvo: reputação por ponto de PA", "Target: reputation per PA point") },
    Chave { nome: "target_ca", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 1.0, false), padrao: "0", rotulo: ("Alvo: potencial ou o que já sabe", "Target: potential or current ability") },
    Chave { nome: "sigma", aba: Aba::Disputa, tipo: num(1.0, INF, 100.0, 10000.0, false), padrao: "2500", rotulo: ("Largura acima do alvo", "Width above the target") },
    Chave { nome: "sigma_below", aba: Aba::Disputa, tipo: num(0.0, INF, 0.0, 10000.0, false), padrao: "1200", rotulo: ("Largura abaixo do alvo", "Width below the target") },
    Chave { nome: "level_floor", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 0.05, false), padrao: "0.002", rotulo: ("Piso do nível", "Level floor") },
    Chave { nome: "facilities_base", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 2.0, false), padrao: "0.4", rotulo: ("Instalações: base", "Facilities: base") },
    Chave { nome: "facilities_k", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 0.2, false), padrao: "0.06", rotulo: ("Instalações: por ponto de Training", "Facilities: per Training point") },
    Chave { nome: "staff_min", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 2.0, false), padrao: "0.8", rotulo: ("Comissão: o pior", "Staff: the worst") },
    Chave { nome: "staff_max", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 2.0, false), padrao: "1.2", rotulo: ("Comissão: o melhor", "Staff: the best") },
    Chave { nome: "squad_limit", aba: Aba::Disputa, tipo: num(1.0, 50.0, 20.0, 50.0, false), padrao: "48", rotulo: ("Elenco: teto", "Squad limit") },
    Chave { nome: "squad_exp", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 3.0, false), padrao: "0.5", rotulo: ("Quanto pesa a vaga no elenco", "How much squad room counts") },
    Chave { nome: "season_cap", aba: Aba::Disputa, tipo: num(-INF, INF, 0.0, 10.0, false), padrao: "2", rotulo: ("Máximo de jovens por clube por ano", "Most youngsters per club per year") },
    Chave { nome: "surprise", aba: Aba::Disputa, tipo: num(0.0, 100.0, 0.0, 50.0, false), padrao: "15", rotulo: ("Surpresa (%)", "Surprise (%)") },
    Chave { nome: "outside_competitions", aba: Aba::Disputa, tipo: num(0.0, 1.0, 0.0, 1.0, false), padrao: "1", rotulo: ("Clubes de divisões que não rodam", "Clubs in divisions not running") },
    Chave { nome: "temperature", aba: Aba::Disputa, tipo: num(-INF, INF, 0.25, 3.0, false), padrao: "1", rotulo: ("Diferença entre os clubes (temperatura)", "Difference between clubs (temperature)") },
    Chave { nome: "regen_names", aba: Aba::Nomes, tipo: Tipo::Escolha(&[("original", "Original (só acima de 100 nomes)", "Original (only above 100 names)"), ("own", "Os da própria nação", "The nation's own")]), padrao: "original", rotulo: ("Os nomes das pessoas novas", "New people's names") },
    Chave { nome: "regen_names_min", aba: Aba::Nomes, tipo: num(1.0, 101.0, 1.0, 100.0, true), padrao: "20", rotulo: ("Nomes próprios a partir de", "Own names from") },
    Chave { nome: "grey_players", aba: Aba::Cinzentos, tipo: Tipo::Escolha(&[("original", "Original (a conta do jogo)", "Original (the game's formula)"), ("squad", "No nível do clube", "At the club's level")]), padrao: "original", rotulo: ("O nível dos cinzentos", "The grey players' level") },
    Chave { nome: "grey_players_spread", aba: Aba::Cinzentos, tipo: num(0.0, 100.0, 0.0, 50.0, true), padrao: "10", rotulo: ("Variação em volta do nível", "Variation around the level") },
    Chave { nome: "intake", aba: Aba::Fornada, tipo: Tipo::Escolha(&[("0", "Desligada (regens ao longo da temporada)", "Off (regens through the season)"), ("1", "Ligada (só numa carreira nova)", "On (new career only)")]), padrao: "0", rotulo: ("Fornada", "Intake") },
    Chave { nome: "intake_per_club", aba: Aba::Fornada, tipo: num(1.0, 20.0, 1.0, 10.0, true), padrao: "3", rotulo: ("Regens por clube em cada fornada", "Regens per club per intake") },
    Chave { nome: "intake_day", aba: Aba::Fornada, tipo: Tipo::Escolha(&[("season_end", "Fim da temporada de cada nação", "Each nation's season end"), ("181", "1º de julho (dia 181), para todos", "1 July (day 181), for everyone"), ("0", "1º de janeiro (dia 0), para todos", "1 January (day 0), for everyone")]), padrao: "season_end", rotulo: ("Dia da fornada", "Intake day") },
    Chave { nome: "intake_clubs", aba: Aba::Fornada, tipo: Tipo::Escolha(&[("playable", "Ligas jogáveis", "Playable leagues"), ("selected", "Também as ligas de fundo", "Background leagues too")]), padrao: "playable", rotulo: ("Clubes que recebem", "Clubs that get them") },
    Chave { nome: "retire_day", aba: Aba::Fornada, tipo: Tipo::Escolha(&[("original", "Datas do próprio jogo", "The game's own dates"), ("174", "24 de junho (dia 174, uma semana antes do 1º de julho)", "24 June (day 174, a week before 1 July)"), ("358", "24 de dezembro (dia 358)", "24 December (day 358)")]), padrao: "original", rotulo: ("Dia das aposentadorias", "Retirement day") },
    Chave { nome: "intake_news", aba: Aba::Fornada, tipo: Tipo::Escolha(&[("1", "Mensagem na caixa de entrada", "Inbox message"), ("0", "Sem mensagem", "No message")]), padrao: "1", rotulo: ("Mensagem da fornada", "Intake message") },
    Chave { nome: "intake_news_numbers", aba: Aba::Fornada, tipo: Tipo::Escolha(&[("0", "Só o parecer do treinador", "Only the coach's view"), ("1", "Parecer, CA e PA", "View, CA and PA")]), padrao: "0", rotulo: ("Na mensagem", "In the message") },
    Chave { nome: "regen_origins", aba: Aba::Registro, tipo: Tipo::Liga, padrao: "0", rotulo: ("Registrar de quem é cada regen", "Record where each regen comes from") },
    Chave { nome: "log", aba: Aba::Log, tipo: Tipo::Liga, padrao: "0", rotulo: ("Registro (tweaks.log)", "Log (tweaks.log)") },
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
    ("Original", "Original", &[("ages", "original"), ("ages_out_of_game", "original"), ("unattached", "original"), ("retire_idle_players", "0"), ("rule", "original"), ("grey_players", "original"), ("regen_names", "original"), ("intake", "0")]),
    ("Só o save", "Save only", &[("save_over_2gb", "1"), ("ages", "original"), ("ages_out_of_game", "original"), ("unattached", "original"), ("retire_idle_players", "0"), ("rule", "original"), ("grey_players", "original"), ("regen_names", "original"), ("intake", "0")]),
    ("Equilibrado", "Balanced", &[("ages", "custom"), ("younger_from_age", "22"), ("younger_by", "6"), ("ages_out_of_game", "original"), ("ca_cap_skip", "0.5"), ("contest_max_age", "18"), ("unattached", "original"),
        ("retire_idle_players", "1"), ("idle_days", "180"), ("idle_min_age", "28"), ("idle_rep", "1500"), ("idle_rep_per_year", "250"), ("protect_young_unemployed", "0"), ("rule", "new"),
        ("target_k", "50"), ("target_ca", "0"), ("sigma", "2500"), ("sigma_below", "1200"), ("level_floor", "0.002"), ("facilities_base", "0.4"), ("facilities_k", "0.06"), ("staff_min", "0.8"),
        ("staff_max", "1.2"), ("squad_limit", "48"), ("squad_exp", "0.5"), ("season_cap", "2"), ("surprise", "15"), ("temperature", "1"), ("intake", "1"), ("intake_per_club", "3"), ("intake_day", "season_end"), ("intake_clubs", "playable")]),
    ("Equilibrado, todos com clube", "Balanced, everyone with a club", &[("ages", "custom"), ("younger_from_age", "22"), ("younger_by", "6"), ("ages_out_of_game", "original"), ("ca_cap_skip", "0.5"), ("contest_max_age", "18"), ("unattached", "only_if_no_room"),
        ("retire_idle_players", "1"), ("idle_days", "180"), ("idle_min_age", "28"), ("idle_rep", "1500"), ("idle_rep_per_year", "250"), ("protect_young_unemployed", "0"), ("rule", "new"),
        ("target_k", "50"), ("target_ca", "0"), ("sigma", "2500"), ("sigma_below", "1200"), ("level_floor", "0.002"), ("facilities_base", "0.4"), ("facilities_k", "0.06"), ("staff_min", "0.8"),
        ("staff_max", "1.2"), ("squad_limit", "48"), ("squad_exp", "0.5"), ("season_cap", "2"), ("surprise", "15"), ("temperature", "1"), ("intake", "1"), ("intake_per_club", "3"), ("intake_day", "season_end"), ("intake_clubs", "playable")]),
];
