//! O configurador do CM0102-Rust-Tweaks: edita o `tweaks.cfg` ao lado do exe do jogo (docs/CONFIGURADOR.md). Um exe só,
//! sem instalador. O `.cfg` continua a única fonte: só o valor das linhas mudadas é trocado.
//! Uso: `tweaks-config.exe [pasta do jogo]` (sem argumento: a pasta do próprio configurador).

#![windows_subsystem = "windows"]

mod cfg;
mod chaves;
mod exe;
mod guia;
mod nomes;

use cfg::Cfg;
use chaves::{Aba, Tipo, ABAS, CHAVES, IDADES_PADRAO, PREDEFINICOES};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use exe::{Estado, Exe};
use std::path::{Path, PathBuf};

const VERDE: Color32 = Color32::from_rgb(0x4c, 0xc3, 0x6a);
const AMARELO: Color32 = Color32::from_rgb(0xe8, 0xb9, 0x3c);
const AZUL: Color32 = Color32::from_rgb(0x5a, 0x9b, 0xe8);
const VERMELHO: Color32 = Color32::from_rgb(0xe0, 0x5a, 0x4f);
const CINZA: Color32 = Color32::from_rgb(0x80, 0x88, 0x84);
const FUNDO: Color32 = Color32::from_rgb(0x12, 0x1a, 0x16);
const CARTAO: Color32 = Color32::from_rgb(0x1a, 0x25, 0x1f);
/// A frase "com este valor..." embaixo de cada opção.
const FRASE: Color32 = Color32::from_rgb(0xb4, 0xc6, 0xbc);

fn cor(e: Estado) -> Color32 {
    match e {
        Estado::Compativel => VERDE,
        Estado::Sobrescreve => AMARELO,
        Estado::FicaComOutro => AZUL,
        Estado::Desconhecido => VERMELHO,
        Estado::Desligado => CINZA,
    }
}

/// `tweaks-config.lang`, ao lado do configurador: "en" ou "pt".
fn arquivo_do_idioma() -> PathBuf {
    std::env::current_exe().map(|p| p.with_file_name("tweaks-config.lang")).unwrap_or_else(|_| PathBuf::from("tweaks-config.lang"))
}

fn t2(en: bool, pt: &'static str, e: &'static str) -> &'static str {
    if en { e } else { pt }
}

/// A data do computador, AAAA-MM-DD (para o nome da cópia do histórico).
fn hoje_no_computador() -> String {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetLocalTime(t: *mut [u16; 8]);
    }
    let mut t = [0u16; 8];
    unsafe { GetLocalTime(&mut t) };
    format!("{:04}-{:02}-{:02}", t[0], t[1], t[3])
}

/// Uma bolinha da cor do estado, desenhada (a fonte padrão do egui não tem o "●").
fn bolinha(ui: &mut egui::Ui, c: Color32) {
    let (r, p) = ui.allocate_painter(Vec2::new(12.0, 12.0), Sense::hover());
    p.circle_filled(r.rect.center(), 5.0, c);
}

/// Um interruptor de verdade (o desenho de liga/desliga): verde e bolinha à direita = ligado. Devolve a resposta; o
/// clique troca o estado.
fn interruptor(ui: &mut egui::Ui, on: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(38.0, 20.0), Sense::click());
    if ui.is_rect_visible(rect) {
        let t = ui.ctx().animate_bool(resp.id, on);
        let fundo = if on { Color32::from_rgb(0x2d, 0x9a, 0x52) } else { Color32::from_rgb(0x4a, 0x52, 0x4e) };
        let r = rect.height() / 2.0;
        ui.painter().rect_filled(rect, r, fundo);
        let x = egui::lerp((rect.left() + r)..=(rect.right() - r), t);
        ui.painter().circle_filled(egui::pos2(x, rect.center().y), r - 3.0, Color32::WHITE);
    }
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Um texto com as partes entre ⟦ ⟧ em destaque: o resultado da conta, que muda quando se mexe no controle.
fn texto_com_resultado(texto: &str, tam: f32) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let normal = egui::TextFormat { font_id: egui::FontId::proportional(tam), color: FRASE, ..Default::default() };
    let destaque = egui::TextFormat { font_id: egui::FontId::proportional(tam + 0.5), color: AMARELO, background: Color32::from_rgb(0x33, 0x2b, 0x12), ..Default::default() };
    for (i, bloco) in texto.split('⟦').enumerate() {
        if i == 0 {
            job.append(bloco, 0.0, normal.clone());
        } else {
            let (d, resto) = bloco.split_once('⟧').unwrap_or((bloco, ""));
            job.append(d, 0.0, destaque.clone());
            job.append(resto, 0.0, normal.clone());
        }
    }
    job
}

/// A unidade do número de uma chave, escrita ao lado do campo.
fn unidade(nome: &str, en: bool) -> &'static str {
    match nome {
        "attendance_toward_real" | "attendance_per_place" | "tradition_per_place" | "weak_league_keep" | "pent_up_demand" | "title_race_max"
        | "title_race_top2" | "stadium_occupancy_min" => "%",
        "attendance_cap_weak" | "attendance_cap_mid" | "attendance_cap_strong" | "weak_league_limit" => t2(en, "pessoas por ponto de nível", "people per level point"),
        "stadium_full_seasons" => t2(en, "temporada(s)", "season(s)"),
        "stadium_max" => t2(en, "lugares", "places"),
        "stadium_reserve" => t2(en, "no dinheiro do banco", "in the database's money"),
        _ => "",
    }
}

/// A chave que se mostra como interruptor: um liga/desliga de duas posições (on/off, 1/0) ou a chave de uma aba.
fn como_interruptor(nome: &str, op: &[(&str, &str, &str)]) -> bool {
    op.len() == 2 && (matches!((op[0].0, op[1].0), ("off", "on") | ("on", "off") | ("0", "1"))
        || chaves::ABAS.iter().any(|a| chaves::mestres(a.0).iter().any(|m| m.0 == nome)))
}

/// Um texto numa largura fixa, alinhado à esquerda (o `add_sized` do egui centraliza).
fn rotulo_fixo(ui: &mut egui::Ui, largura: f32, texto: impl Into<egui::WidgetText>) -> egui::Response {
    ui.allocate_ui_with_layout(Vec2::new(largura, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_min_width(largura);
        ui.label(texto)
    }).inner
}

/// O "?" ao lado de uma opção ou de um termo: passe o mouse e a explicação aparece, sem clicar.
fn ajuda_icone(ui: &mut egui::Ui, texto: &str) {
    if texto.is_empty() {
        return;
    }
    let (r, p) = ui.allocate_painter(Vec2::new(18.0, 18.0), Sense::hover());
    let cor = if r.hovered() { AMARELO } else { AZUL };
    p.circle_stroke(r.rect.center(), 7.5, Stroke::new(1.5f32, cor));
    p.text(r.rect.center(), Align2::CENTER_CENTER, "?", FontId::proportional(11.0), cor);
    r.on_hover_text(texto);
}

fn nome_do_estado(e: Estado, en: bool) -> &'static str {
    match (e, en) {
        (Estado::Compativel, false) => "compatível",
        (Estado::Compativel, true) => "compatible",
        (Estado::Sobrescreve, false) => "atenção: sobrescreve",
        (Estado::Sobrescreve, true) => "attention: overrides",
        (Estado::FicaComOutro, false) => "fica com o outro",
        (Estado::FicaComOutro, true) => "left to the other",
        (Estado::Desconhecido, false) => "não compatível",
        (Estado::Desconhecido, true) => "not compatible",
        (Estado::Desligado, false) => "desligado",
        (Estado::Desligado, true) => "off",
    }
}

/// O clube de exemplo da aba Disputa (fictício, como na maquete).
struct Clube {
    nome: (&'static str, &'static str),
    rep: f64,
    training: f64,
    comissao: f64,
    elenco: f64,
    competicao: bool,
    vezes: f64,
}

const CLUBES: [Clube; 4] = [
    Clube { nome: ("Grande (exemplo)", "Big (example)"), rep: 9000.0, training: 18.0, comissao: 16.0, elenco: 38.0, competicao: true, vezes: 1.0 },
    Clube { nome: ("Médio (exemplo)", "Mid-sized (example)"), rep: 5000.0, training: 12.0, comissao: 10.0, elenco: 30.0, competicao: true, vezes: 1.0 },
    Clube { nome: ("Pequeno (exemplo)", "Small (example)"), rep: 2000.0, training: 6.0, comissao: 5.0, elenco: 25.0, competicao: true, vezes: 1.0 },
    Clube { nome: ("120 clubes pequenos somados", "120 small clubs added up"), rep: 1200.0, training: 4.0, comissao: 4.0, elenco: 22.0, competicao: false, vezes: 120.0 },
];

struct App {
    pasta: PathBuf,
    pasta_digitada: String,
    cfg: Cfg,
    lido: String,
    existe: bool,
    exes: Vec<PathBuf>,
    exe: Option<Exe>,
    aba: Aba,
    en: bool,
    foco: Option<&'static str>,
    idade: usize,
    rep_exemplo: f64,
    pa: f64,
    ca: f64,
    mensagem: String,
    /// A predefinição escolhida e as mudanças que ela faria, à espera da confirmação.
    pendente: Option<(usize, Vec<(String, String, String)>)>,
    /// A busca no histórico de regens e a confirmação de limpar.
    busca: String,
    /// Os nomes por nação do banco da pasta (aba Nomes dos regens).
    nomes: Option<Vec<nomes::Nacao>>,
    limpar: bool,
}

impl App {
    fn new(pasta: PathBuf) -> App {
        let mut a = App {
            pasta_digitada: pasta.display().to_string(),
            pasta,
            cfg: Cfg::ler(""),
            lido: String::new(),
            existe: false,
            exes: Vec::new(),
            exe: None,
            aba: Aba::Inicio,
            en: false,
            foco: None,
            idade: 18,
            rep_exemplo: 3000.0,
            pa: 150.0,
            ca: 60.0,
            mensagem: String::new(),
            pendente: None,
            busca: String::new(),
            limpar: false,
            nomes: None,
        };
        a.abrir_pasta();
        // só para as capturas de tela da documentação: a aba inicial e o idioma
        if let Ok(v) = std::env::var("TWEAKS_CONFIG_ABA") {
            if let Some(&(aba, ..)) = v.parse::<usize>().ok().and_then(|i| ABAS.get(i)) {
                a.aba = aba;
            }
        }
        // o idioma: inglês por padrão; a escolha fica em tweaks-config.lang, ao lado do configurador (ou
        // TWEAKS_CONFIG_LANG=pt|en, para as capturas de tela)
        a.en = std::env::var("TWEAKS_CONFIG_LANG").ok().or_else(|| std::fs::read_to_string(arquivo_do_idioma()).ok())
            .map(|v| !v.trim().eq_ignore_ascii_case("pt")).unwrap_or(true);
        a.foco = std::env::var("TWEAKS_CONFIG_FOCO").ok().and_then(|k| chaves::chave(&k)).map(|c| c.nome);
        a.busca = std::env::var("TWEAKS_CONFIG_BUSCA").unwrap_or_default();
        a
    }

    fn t(&self, pt: &str, en: &str) -> String {
        if self.en { en.into() } else { pt.into() }
    }

    fn abrir_pasta(&mut self) {
        let caminho = self.pasta.join("tweaks.cfg");
        match std::fs::read_to_string(&caminho) {
            Ok(t) => {
                self.existe = true;
                self.lido = t;
            }
            Err(_) => {
                self.existe = false;
                self.lido = cfg::DISTRIBUIDO.to_string();
            }
        }
        self.cfg = Cfg::ler(&self.lido);
        self.exes = std::fs::read_dir(&self.pasta).map(|d| d.filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("exe")))
            .filter(|p| p.file_name().is_some_and(|n| { let n = n.to_string_lossy().to_lowercase(); n.starts_with("cm") && !n.contains("tweaks-config") }))
            .collect()).unwrap_or_default();
        self.exes.sort();
        let preferido = self.exes.iter().find(|p| p.file_name().is_some_and(|n| n.eq_ignore_ascii_case("cm0102.exe"))).or(self.exes.first()).cloned();
        self.exe = preferido.and_then(|p| Exe::abrir(&p));
        self.mensagem.clear();
        self.nomes = nomes::ler(&self.pasta);
    }

    fn valor(&self, nome: &str) -> String {
        self.cfg.valor(nome).unwrap_or_else(|| chaves::chave(nome).map(|c| c.padrao.to_string()).unwrap_or_default())
    }

    fn ligado(&self, nome: &str) -> bool {
        chaves::ligado(nome, &self.valor(nome))
    }

    /// Grava o valor no `.cfg` em memória; sem a linha, só acrescenta se for diferente do padrão da DLL.
    fn definir(&mut self, nome: &str, v: &str) {
        if self.cfg.valor(nome).is_none() && chaves::chave(nome).is_some_and(|c| c.padrao == v) {
            return;
        }
        if self.cfg.valor(nome).as_deref() != Some(v) {
            self.cfg.definir(nome, v);
        }
    }

    fn idade_de(&self, n: usize) -> (f64, i32) {
        let p = IDADES_PADRAO[n - 14];
        match self.cfg.valor(&format!("age_{}", n)) {
            Some(v) => {
                let mut partes = v.split(',').map(str::trim);
                match partes.next().and_then(|x| x.parse::<f64>().ok()) {
                    Some(c) => (c.max(0.0), partes.next().and_then(|t| t.parse::<i32>().ok()).unwrap_or(-1)),
                    None => p,
                }
            }
            None => p,
        }
    }

    fn definir_idade(&mut self, n: usize, chance: f64, teto: i32) {
        let v = if teto >= 0 { format!("{}, {}", cfg::numero(chance), teto) } else { cfg::numero(chance) };
        let p = IDADES_PADRAO[n - 14];
        let chave = format!("age_{}", n);
        if self.cfg.valor(&chave).is_none() && (p.0 - chance).abs() < 1e-9 && p.1 == teto {
            return;
        }
        if self.cfg.valor(&chave).as_deref() != Some(v.as_str()) {
            self.cfg.definir(&chave, &v);
        }
    }

    fn mudancas(&self) -> usize {
        let a: Vec<&str> = self.lido.lines().collect();
        let texto = self.cfg.texto();
        let b: Vec<&str> = texto.lines().collect();
        b.iter().enumerate().filter(|(i, l)| a.get(*i) != Some(l)).count() + a.len().saturating_sub(b.len())
    }

    fn gravar(&mut self) {
        let caminho = self.pasta.join("tweaks.cfg");
        if self.existe {
            if let Ok(atual) = std::fs::read_to_string(&caminho) {
                if atual != self.lido {
                    self.mensagem = self.t("O tweaks.cfg mudou no disco desde a leitura: abra a pasta de novo antes de gravar.", "tweaks.cfg changed on disk since it was read: open the folder again before saving.");
                    return;
                }
            }
        }
        let texto = self.cfg.texto();
        let tmp = self.pasta.join("tweaks.cfg.tmp");
        let existe = self.existe;
        let r = std::fs::write(&tmp, &texto).and_then(|_| {
            if existe {
                std::fs::copy(&caminho, self.pasta.join("tweaks.cfg.bak"))?;
            }
            std::fs::rename(&tmp, &caminho)
        });
        match r {
            Ok(_) => {
                self.lido = texto;
                self.existe = true;
                self.mensagem = self.t("Gravado (a cópia anterior ficou em tweaks.cfg.bak).", "Saved (the previous copy is in tweaks.cfg.bak).");
            }
            Err(e) => self.mensagem = format!("{}: {}", self.t("Não foi possível gravar", "Could not save"), e),
        }
    }

    fn predefinicao(&self) -> Option<usize> {
        PREDEFINICOES.iter().position(|(_, _, vs)| vs.iter().all(|(k, v)| {
            let a = self.valor(k);
            a == *v || a.parse::<f64>().ok().zip(v.parse::<f64>().ok()).is_some_and(|(x, y)| (x - y).abs() < 1e-9)
        }))
    }

    fn estados(&self) -> Vec<exe::Linha> {
        match &self.exe {
            Some(e) => e.estados(&|k| self.ligado(k), self.ligado("override_exe_changes")),
            None => Vec::new(),
        }
    }

    // ---------- controles ----------

    fn controle(&mut self, ui: &mut egui::Ui, nome: &'static str, ativo: bool) {
        let Some(c) = chaves::chave(nome) else { return };
        let rotulo = if self.en { c.rotulo.1 } else { c.rotulo.0 };
        let valor = self.valor(nome);
        let mut novo: Option<String> = None;
        let en = self.en;
        let (a_pt, a_en) = guia::ajuda(nome);
        let texto_ajuda = if en { a_en } else { a_pt };
        // o "?" fica fora da parte desligável: a explicação aparece mesmo com a opção cinza
        let resp = ui.horizontal(|ui| {
            let r = ui.add_enabled_ui(ativo, |ui| ui.horizontal(|ui| match &c.tipo {
                Tipo::Liga => {
                    let on = chaves::ligado(nome, &valor);
                    let r = interruptor(ui, on);
                    ui.label(rotulo);
                    if r.clicked() {
                        novo = Some(if on { "0" } else { "1" }.into());
                    }
                    r
                }
                Tipo::Escolha(op) if como_interruptor(nome, op) => {
                    let i = op.iter().position(|o| o.0 == valor).unwrap_or(0);
                    // ligado = a opção que não é a do jogo original (a segunda), ou "on"/"1"
                    let ligado_idx = if op[0].0 == "on" { 0 } else { 1 };
                    let on = i == ligado_idx;
                    let r = interruptor(ui, on);
                    ui.label(RichText::new(rotulo).strong());
                    ui.label(RichText::new(if en { op[i].2 } else { op[i].1 }).size(12.0).color(CINZA));
                    if r.clicked() {
                        novo = Some(op[1 - i].0.into());
                    }
                    r
                }
                Tipo::Escolha(op) => {
                    rotulo_fixo(ui, 260.0, rotulo);
                    let mut atual = op.iter().position(|o| o.0 == valor).unwrap_or(0);
                    let antes = atual;
                    let r = egui::ComboBox::from_id_salt(nome).width(300.0)
                        .selected_text(if en { op[atual].2 } else { op[atual].1 })
                        .show_ui(ui, |ui| {
                            for (i, o) in op.iter().enumerate() {
                                ui.selectable_value(&mut atual, i, if en { o.2 } else { o.1 });
                            }
                        }).response;
                    if atual != antes {
                        novo = Some(op[atual].0.into());
                    }
                    r
                }
                Tipo::Numero { min, max, barra, inteiro } => {
                    rotulo_fixo(ui, 260.0, rotulo);
                    let mut x: f64 = valor.parse().unwrap_or_else(|_| c.padrao.parse().unwrap_or(0.0));
                    let mut xb = x.clamp(barra.0, barra.1);
                    let r1 = ui.add(egui::Slider::new(&mut xb, barra.0..=barra.1).show_value(false));
                    if r1.changed() {
                        x = xb;
                    }
                    let passo = if *inteiro { 0.2 } else { ((barra.1 - barra.0) / 300.0).max(0.0001) };
                    // o número como se lê: "2" e não "2.00"; no português, com vírgula (e a vírgula também vale ao digitar)
                    let r2 = ui.add(egui::DragValue::new(&mut x).range(*min..=*max).speed(passo).max_decimals(if *inteiro { 0 } else { 4 })
                        .custom_formatter(move |v, _| { let s = format!("{:.4}", v); let s = s.trim_end_matches('0').trim_end_matches('.'); if en { s.to_string() } else { s.replace('.', ",") } })
                        .custom_parser(|s| s.trim().replace(',', ".").parse::<f64>().ok()));
                    let u = unidade(nome, en);
                    if !u.is_empty() {
                        ui.label(RichText::new(u).color(CINZA));
                    }
                    if r1.changed() || r2.changed() {
                        novo = Some(cfg::numero(if *inteiro { x.round() } else { x }));
                    }
                    r1.union(r2)
                }
            }).inner).inner;
            ajuda_icone(ui, texto_ajuda);
            r
        });
        if let Some(v) = novo {
            self.definir(nome, &v);
        }
        // o que o valor quer dizer no jogo, por extenso (03/11, Lucas: nenhum número sem explicação)
        if let Some(f) = self.frase(nome) {
            ui.horizontal(|ui| {
                ui.add_space(22.0);
                ui.add(egui::Label::new(texto_com_resultado(&f, 12.5)).wrap());
            });
            ui.add_space(4.0);
        }
        if resp.response.hovered() || resp.inner.hovered() || resp.inner.has_focus() {
            self.foco = Some(nome);
        }
    }

    /// Um número no jeito do idioma (vírgula no português), sem zeros sobrando.
    fn n(&self, x: f64) -> String {
        let s = format!("{:.2}", x);
        let s = s.trim_end_matches('0').trim_end_matches('.').to_string();
        if self.en { s } else { s.replace('.', ",") }
    }

    /// O que o valor atual de uma opção faz no jogo, em uma frase (ou nada, para as que se explicam sozinhas).
    fn frase(&self, nome: &str) -> Option<String> {
        let g = |k: &str| self.valor(k).parse::<f64>().unwrap_or_else(|_| chaves::chave(k).and_then(|c| c.padrao.parse().ok()).unwrap_or(0.0));
        let en = self.en;
        let f = |pt: String, e: String| Some(if en { e } else { pt });
        let x = g(nome);
        match nome {
            "regen_names_min" => f(format!("Um país com {:.0} nomes ou mais no banco usa os próprios; com menos, pega emprestado de outro (o original exige mais de 100).", x),
                format!("A country with {:.0} or more names in the database uses its own; with fewer, it borrows from another (the original needs more than 100).", x)),
            "multiple_instances" => {
                let mut s = if self.ligado(nome) {
                    (String::from("Ligado: cada pasta do jogo pode ter um jogo aberto (por exemplo, o 3.9.68 numa pasta e o CLZ em outra, ou duas carreiras em duas cópias). Dois jogos da mesma pasta continuam barrados."),
                     String::from("On: each game folder can have one game open (for example, 3.9.68 in one folder and CLZ in another, or two careers in two copies). Two games from the same folder are still blocked."))
                } else {
                    (String::from("Desligado (como o original): só um jogo aberto no computador; o segundo para na caixa \"Already Running\"."),
                     String::from("Off (as the original): only one game open on the computer; the second one stops at the \"Already Running\" box."))
                };
                if self.ligado(nome) && !self.ligado("portable_folder") {
                    s.0 += " Atenção: ligue também a pasta portátil, para cada cópia usar a própria pasta.";
                    s.1 += " Note: turn on the portable folder too, so each copy uses its own folder.";
                }
                f(s.0, s.1)
            }
            "younger_from_age" | "younger_by" => {
                let (a, b) = (g("younger_from_age"), g("younger_by"));
                if nome == "younger_from_age" {
                    return f(format!("Vale para quem nasceria com {} anos ou mais.", a), format!("It applies to those who would be born aged {} or more.", a));
                }
                if b <= 0.0 {
                    f("0: desligado, as idades ficam como na tabela.".into(), "0: off, the ages stay as in the table.".into())
                } else {
                    f(format!("Quem nasceria com {} anos ou mais nasce {} anos mais novo: um de {} vira um de {}.", a, b, a, (a - b).max(14.0)),
                      format!("Those who would be born aged {} or more are born {} years younger: a {}-year-old becomes a {}-year-old.", a, b, a, (a - b).max(14.0)))
                }
            }
            "ca_cap_skip" => if x <= 0.0 {
                f("0: todo jovem respeita o teto do CA da idade dele.".into(), "0: every youngster keeps his age's CA cap.".into())
            } else {
                f(format!("{}% = cerca de 1 em {:.0} jovens nasce sem o teto do CA da idade (pode nascer já pronto).", self.n(x), 100.0 / x),
                  format!("{}% = about 1 in {:.0} youngsters is born without his age's CA cap (he may be born ready).", self.n(x), 100.0 / x))
            },
            "contest_max_age" => if self.valor("unattached") == "only_if_no_room" {
                f("Sem efeito agora: com \"Sem clube só sem vaga\", todos vão à disputa.".into(), "No effect now: with \"Clubless only without room\", everyone goes to the contest.".into())
            } else {
                f(format!("Vão à disputa dos clubes os jovens de até {} anos; os mais velhos começam sem clube.", x), format!("Youngsters up to {} go to the clubs' contest; older ones start without a club.", x))
            },
            "idle_days" => f(format!("{} dias = uns {:.0} meses sem clube antes de a regra valer.", x, x / 30.4), format!("{} days = about {:.0} months without a club before the rule applies.", x, x / 30.4)),
            "idle_min_age" => f(format!("Só vale para quem tem {} anos ou mais; os mais novos seguem a regra original.", x), format!("It only applies from age {} up; younger players keep the original rule.", x)),
            "idle_rep" => {
                let i = g("idle_min_age");
                f(format!("Aos {} anos, quem tem reputação abaixo de {:.0} se aposenta quase sempre; acima, a chance cai (a reputação vai de 0 a 10000).", i, x),
                  format!("At {}, a player with reputation below {:.0} retires almost always; above it, the chance drops (reputation goes from 0 to 10000).", i, x))
            }
            "idle_rep_per_year" => {
                let (i, r) = (g("idle_min_age"), g("idle_rep"));
                f(format!("O limite sobe {:.0} por ano de idade: {:.0} aos {}, {:.0} aos {}, {:.0} aos {}.", x, r + x, i + 1.0, r + 2.0 * x, i + 2.0, r + 5.0 * x, i + 5.0),
                  format!("The threshold rises {:.0} per year of age: {:.0} at {}, {:.0} at {}, {:.0} at {}.", x, r + x, i + 1.0, r + 2.0 * x, i + 2.0, r + 5.0 * x, i + 5.0))
            }
            "protect_young_unemployed" => if x <= 0.0 {
                f("0: ninguém fica protegido (o jogo já dá 365 dias ao jovem sem clube).".into(), "0: nobody is protected (the game already gives a clubless youngster 365 days).".into())
            } else {
                f(format!("Ninguém com menos de {} anos se aposenta por esta regra.", x), format!("Nobody under {} retires through this rule.", x))
            },
            "target_k" => f(format!("Um jovem de PA {:.0} (o de exemplo) mira um clube de reputação {:.0}: o PA × {}.", self.pa, x * self.pa, self.n(x)),
                format!("A PA {:.0} youngster (the example) aims at a club of reputation {:.0}: the PA × {}.", self.pa, x * self.pa, self.n(x))),
            "target_ca" => {
                let c = x.clamp(0.0, 1.0);
                f(format!("O alvo olha {:.0}% o potencial (PA) e {:.0}% o que ele já sabe (CA).", 100.0 * (1.0 - c), 100.0 * c),
                  format!("The target looks {:.0}% at the potential (PA) and {:.0}% at what he can do now (CA).", 100.0 * (1.0 - c), 100.0 * c))
            }
            "sigma" => f(format!("Um clube {:.0} de reputação acima do alvo fica com uns 60% da chance de um clube no alvo; {:.0} acima, uns 14%.", x, 2.0 * x),
                format!("A club {:.0} reputation above the target keeps about 60% of a club on target's chance; {:.0} above, about 14%.", x, 2.0 * x)),
            "sigma_below" => {
                let y = if x > 0.0 { x } else { g("sigma") };
                f(format!("{}Um clube {:.0} abaixo do alvo fica com uns 60% da chance; {:.0} abaixo, uns 14%.", if x > 0.0 { "" } else { "0 = igual à de cima. " }, y, 2.0 * y),
                  format!("{}A club {:.0} below the target keeps about 60% of the chance; {:.0} below, about 14%.", if x > 0.0 { "" } else { "0 = same as above. " }, y, 2.0 * y))
            }
            "level_floor" => if x <= 0.0 {
                f("0: um clube muito longe do alvo não tem chance nenhuma.".into(), "0: a club far from the target has no chance at all.".into())
            } else {
                f(format!("Um clube muito longe do alvo ainda fica com {}% da chance de um clube no alvo (1 em {:.0}).", self.n(100.0 * x), 1.0 / x),
                  format!("A club far from the target still keeps {}% of a club on target's chance (1 in {:.0}).", self.n(100.0 * x), 1.0 / x))
            },
            "facilities_base" => f(format!("O ponto de partida: um clube com Training 0 multiplica a chance por {}. Cada ponto de Training soma o valor de baixo.", self.n(x)),
                format!("The starting point: a club with Training 0 multiplies its chance by {}. Each Training point adds the value below.", self.n(x))),
            "facilities_k" => {
                let (b, k) = (g("facilities_base"), g("facilities_k"));
                f(format!("A chance do clube é multiplicada pelas instalações de base: Training 1 = ×{}, 10 = ×{}, 20 = ×{}.", self.n(b + k), self.n(b + 10.0 * k), self.n(b + 20.0 * k)),
                  format!("The club's chance is multiplied by its youth facilities: Training 1 = ×{}, 10 = ×{}, 20 = ×{}.", self.n(b + k), self.n(b + 10.0 * k), self.n(b + 20.0 * k)))
            }
            "staff_min" => f(format!("A comissão que pior trabalha com jovens (Working with Youngsters 1) multiplica a chance do clube por {} ({:+.0}%).", self.n(x), 100.0 * (x - 1.0)),
                format!("The staff worst with youngsters (Working with Youngsters 1) multiplies the club's chance by {} ({:+.0}%).", self.n(x), 100.0 * (x - 1.0))),
            "staff_max" => f(format!("A comissão que melhor trabalha com jovens (Working with Youngsters 20) multiplica a chance do clube por {} ({:+.0}%).", self.n(x), 100.0 * (x - 1.0)),
                format!("The staff best with youngsters (Working with Youngsters 20) multiplies the club's chance by {} ({:+.0}%).", self.n(x), 100.0 * (x - 1.0))),
            "squad_limit" => f(format!("Um clube com {:.0} jogadores ou mais não recebe jovem.", x), format!("A club with {:.0} players or more gets no youngster.", x)),
            "squad_exp" => {
                let l = g("squad_limit").clamp(1.0, 50.0);
                if x <= 0.0 {
                    f("0: a vaga não conta; só o teto do elenco.".into(), "0: the room doesn't count; only the squad limit.".into())
                } else if l > 40.0 {
                    let r = ((l - 24.0) / (l - 40.0)).powf(x);
                    f(format!("Um clube com 24 jogadores tem ×{} a chance de um com 40.", self.n(r)), format!("A club with 24 players has ×{} the chance of one with 40.", self.n(r)))
                } else {
                    None
                }
            }
            "season_cap" => if x <= 0.0 {
                f("0: sem teto; um clube pode receber quantos jovens ganhar.".into(), "0: no cap; a club can get as many youngsters as it wins.".into())
            } else {
                f(format!("Nenhum clube recebe mais de {} jovens pela disputa a cada 365 dias.", x), format!("No club gets more than {} youngsters from the contest every 365 days.", x))
            },
            "surprise" => f(format!("{} em cada 100 jovens vão a um clube sorteado sem olhar o nível.", self.n(x)), format!("{} in every 100 youngsters go to a club drawn without looking at the level.", self.n(x))),
            "outside_competitions" => {
                let w = x.clamp(0.0, 1.0);
                if w >= 1.0 {
                    f("1: a mesma chance dos clubes que jogam.".into(), "1: the same chance as the clubs that play.".into())
                } else if w <= 0.0 {
                    f("0: nunca recebem jovem.".into(), "0: they never get a youngster.".into())
                } else {
                    f(format!("{}% da chance de um clube igual que joga.", self.n(100.0 * w)), format!("{}% of the chance of a similar club that plays.", self.n(100.0 * w)))
                }
            }
            "temperature" => {
                let r = 2f64.powf(x);
                if (x - 1.0).abs() < 1e-9 {
                    f("1: normal; um clube com o dobro do peso tem o dobro da chance.".into(), "1: normal; a club with twice the weight has twice the chance.".into())
                } else {
                    f(format!("{}: um clube com o dobro do peso tem ×{} a chance do outro ({}).", self.n(x), self.n(r), if x > 1.0 { "acentua as diferenças" } else { "suaviza as diferenças" }),
                      format!("{}: a club with twice the weight has ×{} the other's chance ({}).", self.n(x), self.n(r), if x > 1.0 { "sharper differences" } else { "softer differences" }))
                }
            }
            "attendance_toward_real" => f(format!("Média de 10.000 que levou 14.000 por jogo na temporada: passa a ⟦{}⟧ no ano seguinte.", self.milhar(10000.0 + 4000.0 * x / 100.0)),
                format!("An average of 10,000 that drew 14,000 a match in the season: becomes ⟦{}⟧ the next year.", self.milhar(10000.0 + 4000.0 * x / 100.0))),
            "attendance_per_place" => f(format!("4 posições acima do previsto: ⟦+{}%⟧ (10.000 → ⟦{}⟧); 4 abaixo: ⟦-{}%⟧.", self.n(4.0 * x), self.milhar(10000.0 * (1.0 + 0.04 * x)), self.n(4.0 * x)),
                format!("4 places above the prediction: ⟦+{}%⟧ (10,000 → ⟦{}⟧); 4 below: ⟦-{}%⟧.", self.n(4.0 * x), self.milhar(10000.0 * (1.0 + 0.04 * x)), self.n(4.0 * x))),
            "tradition_per_place" => if x <= 0.0 {
                f("0: o recorde histórico não pesa.".into(), "0: the historical record doesn't count.".into())
            } else {
                f(format!("{}% × 5 posições acima = ⟦{}%⟧ da distância até o recorde histórico. Média de 12.000 e recorde histórico de 26.000 (distância de 14.000): voltam ⟦{}⟧ pessoas.", self.n(x), self.n((5.0 * x).min(100.0)), self.milhar(14000.0 * (5.0 * x / 100.0).min(1.0))),
                    format!("{}% × 5 places above = ⟦{}%⟧ of the distance to the historical record. Average 12,000 and historical record 26,000 (a distance of 14,000): ⟦{}⟧ people come back.", self.n(x), self.n((5.0 * x).min(100.0)), self.milhar(14000.0 * (5.0 * x / 100.0).min(1.0))))
            },
            "attendance_floor" => if self.ligado(nome) {
                f("Clube que cresceu de 10.000 para 16.000 e foi rebaixado: pode voltar até ⟦uns 10.000⟧ (o que o jogo original daria), nunca abaixo.".into(),
                    "A club that grew from 10,000 to 16,000 and was relegated: it can go back to ⟦about 10,000⟧ (what the original game would give), never below.".into())
            } else {
                f("Sem piso: quem vai mal cai pelo valor por posição, mesmo abaixo do que o jogo original daria.".into(), "No floor: a club doing badly falls by the per-place value, even below what the original game would give.".into())
            },
            "attendance_cap_weak" | "attendance_cap_mid" | "attendance_cap_strong" => {
                let nivel = match nome { "attendance_cap_weak" => 3.0, "attendance_cap_mid" => 10.0, _ => 16.0 };
                if x <= 0.0 {
                    f("⟦Sem teto⟧: a média sobe o quanto o desempenho levar.".into(), "⟦No cap⟧: the average goes as high as performance takes it.".into())
                } else {
                    f(format!("Numa divisão de nível {}: a média não passa de ⟦{}⟧ pessoas.", nivel, self.milhar(x * nivel)), format!("In a level-{} division: the average doesn't go past ⟦{}⟧ people.", nivel, self.milhar(x * nivel)))
                }
            }
            "weak_league_limit" => if x <= 0.0 {
                f("⟦Sem corte⟧: o clube popular de divisão baixa pode encher o estádio.".into(), "⟦No cut⟧: a popular low-division club can fill its stadium.".into())
            } else {
                f(format!("Numa divisão de nível 3: acima de ⟦{}⟧ pessoas, o público de cada jogo é cortado.", self.milhar(3.0 * x)), format!("In a level-3 division: above ⟦{}⟧ people, each match crowd is cut.", self.milhar(3.0 * x)))
            },
            "weak_league_keep" => {
                let lim = 3.0 * g("weak_league_limit");
                if lim <= 0.0 {
                    f("Só vale com o corte acima ligado (maior que 0).".into(), "Only applies with the cut above on (above 0).".into())
                } else {
                    let vai = lim + (10000.0 - lim).max(0.0) * x / 100.0;
                    f(format!("10.000 pessoas querendo ir a um jogo com limite de {}: vão ⟦{}⟧.", self.milhar(lim), self.milhar(vai.min(10000.0))),
                        format!("10,000 people wanting to go to a match with a {} limit: ⟦{}⟧ go.", self.milhar(lim), self.milhar(vai.min(10000.0))))
                }
            }
            "pent_up_demand" => if x <= 0.0 {
                f("0: o recorde só muda como no original.".into(), "0: the record only changes as in the original.".into())
            } else {
                f(format!("10.000 pessoas de fora de um jogo sobem o teto do clube em ⟦{}⟧ para os próximos.", self.milhar(100.0 * x)),
                    format!("10,000 people left out of a match raise the club's ceiling by ⟦{}⟧ for the next ones.", self.milhar(100.0 * x)))
            },
            "title_race" => {
                let tot = g("title_race_max") + g("title_race_top2");
                if self.ligado(nome) {
                    f(format!("Um 1º × 2º na reta final: ⟦+{}%⟧ de público; se também for clássico: ⟦+{}%⟧ (sempre até a capacidade do estádio).", self.n(tot), self.n((1.5 * (1.0 + tot / 100.0) - 1.0) * 100.0)),
                        format!("A 1st v 2nd in the run-in: ⟦+{}%⟧ crowd; if it is also a derby: ⟦+{}%⟧ (always up to the stadium's capacity).", self.n(tot), self.n((1.5 * (1.0 + tot / 100.0) - 1.0) * 100.0)))
                } else {
                    f("Só o clássico do jogo aumenta o público (+50%), como no original.".into(), "Only the game's derby raises the crowd (+50%), as in the original.".into())
                }
            }
            "title_race_max" => f(format!("Jogo de 20.000 entre dois candidatos ao título: ⟦{}⟧ na última rodada, ⟦{}⟧ no meio da temporada.", self.milhar(20000.0 * (1.0 + x / 100.0)), self.milhar(20000.0 * (1.0 + x / 200.0))),
                format!("A 20,000 match between two title candidates: ⟦{}⟧ in the last round, ⟦{}⟧ half way through the season.", self.milhar(20000.0 * (1.0 + x / 100.0)), self.milhar(20000.0 * (1.0 + x / 200.0)))),
            "title_race_top2" => f(format!("Num 1º × 2º na última rodada: ⟦+{}%⟧ no total (o de cima mais este).", self.n(g("title_race_max") + x)),
                format!("In a 1st v 2nd in the last round: ⟦+{}%⟧ in all (the one above plus this).", self.n(g("title_race_max") + x))),
            "stadium_occupancy_min" => f(format!("Estádio de 30.000: precisa de média de ⟦{}⟧ por jogo para a temporada contar como cheia.", self.milhar(300.0 * x)),
                format!("A 30,000 stadium: needs an average of ⟦{}⟧ a match for the season to count as full.", self.milhar(300.0 * x))),
            "stadium_full_seasons" => f(format!("⟦{}⟧ temporada(s) seguida(s) acima de {}% antes de poder ampliar.", x as i32, self.n(g("stadium_occupancy_min"))),
                format!("⟦{}⟧ season(s) in a row above {}% before it can expand.", x as i32, self.n(g("stadium_occupancy_min")))),
            "stadium_reserve" => f(format!("Clube com saldo de 20 milhões: a obra pode usar até ⟦{}⟧.", self.milhar((20_000_000.0 - x).max(0.0))),
                format!("A club with a 20 million balance: the build can use up to ⟦{}⟧.", self.milhar((20_000_000.0 - x).max(0.0)))),
            "stadium_max" => f(format!("Um estádio com mais de ⟦{}⟧ lugares não amplia mais.", self.milhar(x)), format!("A stadium with more than ⟦{}⟧ places doesn't expand any more.", self.milhar(x))),
            _ => None,
        }
    }

    /// A explicação de um termo da aba (o nome em português, como em `guia::termos`), no idioma da tela.
    fn termo(&self, aba: Aba, nome: &str) -> &'static str {
        guia::termos(aba).iter().find(|t| t.0 == nome).map(|t| if self.en { t.3 } else { t.2 }).unwrap_or("")
    }

    /// O topo de cada aba: para que ela serve e o que quer dizer cada termo que ela usa.
    fn cabecalho(&self, ui: &mut egui::Ui, aba: Aba) {
        let (pt, en) = guia::intro(aba);
        let termos = guia::termos(aba);
        let ingles = self.en;
        egui::Frame::group(ui.style()).fill(CARTAO).inner_margin(10.0).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(if ingles { en } else { pt }).size(13.5));
            if !termos.is_empty() {
                ui.add_space(6.0);
                egui::Grid::new(("termos", aba as u8)).num_columns(2).spacing([12.0, 5.0]).show(ui, |ui| {
                    for &(n_pt, n_en, t_pt, t_en) in termos {
                        ui.label(RichText::new(if ingles { n_en } else { n_pt }).strong().color(AMARELO));
                        ui.add(egui::Label::new(if ingles { t_en } else { t_pt }).wrap());
                        ui.end_row();
                    }
                });
            }
        });
        ui.add_space(10.0);
    }

    fn cartao(ui: &mut egui::Ui, titulo: &str, corpo: impl FnOnce(&mut egui::Ui)) {
        egui::Frame::group(ui.style()).fill(CARTAO).inner_margin(10.0).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(titulo).strong().size(15.0).color(VERDE));
            ui.add_space(4.0);
            corpo(ui);
        });
        ui.add_space(8.0);
    }

    // ---------- abas ----------

    fn aba_inicio(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Inicio);
        let en = self.en;
        let mut escolhido: Option<PathBuf> = None;
        let exes = self.exes.clone();
        let atual = self.exe.as_ref().map(|e| e.caminho.clone());
        App::cartao(ui, &self.t("O exe", "The exe"), |ui| {
            if exes.is_empty() {
                ui.colored_label(AMARELO, if en { "No cm*.exe in this folder: open the game folder above." } else { "Nenhum cm*.exe nesta pasta: abra a pasta do jogo no topo." });
            }
            ui.horizontal_wrapped(|ui| {
                for p in &exes {
                    let nome = p.file_name().unwrap().to_string_lossy().into_owned();
                    if ui.selectable_label(atual.as_ref() == Some(p), nome).clicked() {
                        escolhido = Some(p.clone());
                    }
                }
            });
        });
        if let Some(p) = escolhido {
            self.exe = Exe::abrir(&p);
        }
        if let Some(e) = &self.exe {
            App::cartao(ui, &self.t("Retrato", "Snapshot"), |ui| {
                ui.label(if e.eh_original() {
                    if en { "The code the tweaks use is as in 3.9.68." } else { "O código que os ajustes usam está como no 3.9.68." }
                } else if en { "This exe changes code the tweaks use (see below)." } else { "Este exe muda código que os ajustes usam (veja abaixo)." });
                for c in &e.conhecidos {
                    ui.label(format!("• {}", c));
                }
                if e.nick {
                    ui.label(format!("• NickDLL.dll{}", match &e.nick_resolucao {
                        Some(r) => format!(" ({}: \"resolucao\": \"{}\")", if en { "with its own resolution" } else { "com resolução própria" }, r),
                        None => String::new(),
                    }));
                }
                if e.lamo {
                    ui.label("• LaMOHatesCD.dll");
                }
            });
        }
        let estados = self.estados();
        App::cartao(ui, &self.t("O que cada ajuste vai fazer neste exe", "What each tweak will do on this exe"), |ui| {
            for l in &estados {
                // o nome da opção como nas abas; a chave do tweaks.cfg em cinza, para quem edita à mão
                let nome = chaves::chave(l.ajuste).map(|c| if en { c.rotulo.1 } else { c.rotulo.0 }).unwrap_or(l.ajuste);
                ui.horizontal(|ui| {
                    rotulo_fixo(ui, 260.0, nome);
                    rotulo_fixo(ui, 135.0, RichText::new(l.ajuste).monospace().size(11.0).color(CINZA));
                    bolinha(ui, cor(l.estado));
                    rotulo_fixo(ui, 115.0, RichText::new(nome_do_estado(l.estado, en)).color(cor(l.estado)));
                    ui.add(egui::Label::new(if en { &l.motivo.1 } else { &l.motivo.0 }).wrap());
                });
            }
        });
    }

    fn aba_jogo(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Jogo);
        ui.label(RichText::new(self.t("O save e a instalação", "The save and the install")).strong().size(15.0).color(VERDE));
        for k in ["save_over_2gb", "portable_folder", "multiple_instances", "clear_admin_flag", "override_exe_changes", "squad_guard"] {
            self.controle(ui, k, true);
        }
        ui.add_space(8.0);
        ui.label(RichText::new(self.t("Tela e CD", "Screen and CD")).strong().size(15.0).color(VERDE));
        self.controle(ui, "resolution", true);
        let pela_resolucao = self.ligado("resolution") && self.estados().iter().any(|l| l.ajuste == "resolution" && l.estado == Estado::Compativel);
        self.controle(ui, "video", !pela_resolucao);
        if pela_resolucao {
            ui.colored_label(AZUL, self.t("O vídeo em janela vem ligado pela resolução.", "The windowed video is turned on by the resolution."));
        }
        self.controle(ui, "no_cd", true);
    }

    fn aba_idades(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Idades);
        self.controle(ui, "ages", true);
        let ativo = self.ligado("ages");
        if !ativo {
            ui.colored_label(CINZA, self.t("A tabela não vale: o jogo usa as idades do 3.9.68.", "The table doesn't apply: the game uses 3.9.68's ages."));
        }
        // o gráfico: barras arrastáveis, escala de raiz quadrada
        let idades: Vec<(f64, i32)> = (14..36).map(|n| self.idade_de(n)).collect();
        let total: f64 = idades.iter().map(|x| x.0).sum::<f64>().max(1e-9);
        let maior = 40f64.max(idades.iter().map(|x| x.0).fold(0.0, f64::max));
        let largura = ui.available_width().min(900.0);
        let (resp, p) = ui.allocate_painter(Vec2::new(largura, 240.0), Sense::click_and_drag());
        let r = resp.rect;
        p.rect_filled(r, 4, CARTAO);
        let base = r.bottom() - 34.0;
        let alto = base - r.top() - 18.0;
        let w = r.width() / 22.0;
        let mais_novo_a = self.valor("younger_from_age").parse::<i32>().unwrap_or(22);
        let mais_novo_em = self.valor("younger_by").parse::<i32>().unwrap_or(0);
        // a distribuição depois de "regens mais novos"
        let mut final_ = [0f64; 22];
        for (i, x) in idades.iter().enumerate() {
            let idade = 14 + i as i32;
            let j = if mais_novo_em > 0 && idade >= mais_novo_a { (idade - mais_novo_em).clamp(14, 35) } else { idade };
            final_[(j - 14) as usize] += x.0;
        }
        for (i, x) in idades.iter().enumerate() {
            let h = (x.0 / maior).sqrt() as f32 * alto;
            let x0 = r.left() + i as f32 * w;
            let barra = Rect::from_min_max(Pos2::new(x0 + 3.0, base - h), Pos2::new(x0 + w - 3.0, base));
            let c = if !ativo { CINZA } else if i == self.idade - 14 { AMARELO } else { VERDE };
            p.rect_filled(barra, 2, c);
            if mais_novo_em > 0 && ativo {
                let hf = (final_[i] / maior).sqrt() as f32 * alto;
                p.rect_stroke(Rect::from_min_max(Pos2::new(x0 + 1.0, base - hf), Pos2::new(x0 + w - 1.0, base)), 2, Stroke::new(1.0f32, AZUL), egui::StrokeKind::Outside);
            }
            p.text(Pos2::new(x0 + w / 2.0, base + 2.0), Align2::CENTER_TOP, format!("{}", 14 + i), FontId::proportional(11.0), Color32::LIGHT_GRAY);
            if x.1 >= 0 {
                p.text(Pos2::new(x0 + w / 2.0, base + 16.0), Align2::CENTER_TOP, format!("{}", x.1), FontId::proportional(9.0), CINZA);
            }
            if x.0 > 0.0 {
                p.text(Pos2::new(x0 + w / 2.0, base - h - 2.0), Align2::CENTER_BOTTOM, format!("{:.1}%", 100.0 * x.0 / total), FontId::proportional(9.0), Color32::WHITE);
            }
        }
        if ativo {
            if let Some(pos) = resp.interact_pointer_pos() {
                let i = (((pos.x - r.left()) / w).max(0.0) as usize).min(21);
                self.idade = 14 + i;
                if resp.dragged() {
                    let frac = ((base - pos.y) / alto).clamp(0.0, 1.0) as f64;
                    let chance = (frac * frac * maior * 100.0).round() / 100.0;
                    self.definir_idade(14 + i, chance, idades[i].1);
                }
            }
        }
        if resp.hovered() {
            self.foco = Some("age_16");
        }
        let resp = resp.on_hover_text(if self.en { guia::ajuda("age_NN").1 } else { guia::ajuda("age_NN").0 });
        let _ = resp;
        ui.label(RichText::new(self.t("Clique numa barra para escolher a idade e arraste para mudar a chance. Embaixo de cada barra: a idade e o teto do CA. O contorno azul é como fica a distribuição depois de \"regens mais novos\".",
            "Click a bar to pick the age and drag to change the chance. Below each bar: the age and the CA cap. The blue outline is the distribution after \"younger regens\".")).size(11.5).color(CINZA));
        ui.add_space(6.0);
        // a idade escolhida, por extenso
        let n = self.idade;
        let (mut chance, teto) = self.idade_de(n);
        let soma: f64 = (14..36).map(|k| self.idade_de(k).0).sum::<f64>().max(1e-9);
        let en = self.en;
        let termo = |nome: &str| guia::termos(Aba::Idades).iter().find(|t| t.0 == nome).map(|t| if en { t.3 } else { t.2 }).unwrap_or("");
        let (ajuda_chance, ajuda_teto, ajuda_linha) = (termo("Chance"), termo("Teto do CA"), termo("age_18 no tweaks.cfg"));
        let mut mudou = None;
        App::cartao(ui, &format!("{} {} {}", self.t("Idade escolhida:", "Chosen age:"), n, self.t("anos", "years")), |ui| {
            ui.add_enabled_ui(ativo, |ui| {
                ui.horizontal(|ui| {
                    rotulo_fixo(ui, 150.0, t2(en, "Chance", "Chance"));
                    let virgula = move |s: String| if en { s } else { s.replace('.', ",") };
                    let r = ui.add(egui::DragValue::new(&mut chance).range(0.0..=1000.0).speed(0.05).max_decimals(4)
                        .custom_formatter(move |v, _| { let s = format!("{:.4}", v); virgula(s.trim_end_matches('0').trim_end_matches('.').to_string()) })
                        .custom_parser(|s| s.trim().replace(',', ".").parse::<f64>().ok()));
                    let pct = 100.0 * chance / soma;
                    ui.label(RichText::new(if en {
                        format!("= {:.1}% of the table's regens (about 1 in {:.0})", pct, 100.0 / pct.max(1e-9))
                    } else {
                        format!("= {}% dos regens da tabela (cerca de 1 em {:.0})", format!("{:.1}", pct).replace('.', ","), 100.0 / pct.max(1e-9))
                    }).color(CINZA));
                    ajuda_icone(ui, ajuda_chance);
                    if r.changed() {
                        mudou = Some((chance, teto));
                    }
                });
                ui.horizontal(|ui| {
                    let mut tem_teto = teto >= 0;
                    let mut t = if teto >= 0 { teto } else { 100 };
                    let r2 = ui.allocate_ui_with_layout(Vec2::new(150.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.set_min_width(150.0);
                        ui.checkbox(&mut tem_teto, t2(en, "Teto do CA", "CA cap"))
                    }).inner;
                    let r3 = ui.add_enabled(tem_teto, egui::DragValue::new(&mut t).range(1..=200));
                    ui.label(RichText::new(if tem_teto {
                        if en { format!("no {}-year-old regen is born with CA above {}", n, t) } else { format!("nenhum regen de {} anos nasce com CA acima de {}", n, t) }
                    } else if en { "no cap: the CA comes from the game's formula".to_string() } else { "sem teto: o CA sai da fórmula do jogo".to_string() }).color(CINZA));
                    ajuda_icone(ui, ajuda_teto);
                    if r2.changed() || r3.changed() {
                        mudou = Some((chance, if tem_teto { t } else { -1 }));
                    }
                });
                ui.horizontal(|ui| {
                    rotulo_fixo(ui, 150.0, t2(en, "No tweaks.cfg", "In tweaks.cfg"));
                    let linha = if teto >= 0 { format!("age_{} = {}, {}", n, cfg::numero(chance), teto) } else { format!("age_{} = {}", n, cfg::numero(chance)) };
                    ui.label(RichText::new(linha).monospace().color(AMARELO));
                    ajuda_icone(ui, ajuda_linha);
                });
            });
        });
        if let Some((c, t)) = mudou {
            self.definir_idade(n, c, t);
        }
        ui.add_space(8.0);
        for k in ["younger_from_age", "younger_by", "ages_out_of_game", "ca_cap_skip"] {
            self.controle(ui, k, ativo);
        }
        let sem_vaga = self.valor("unattached") == "only_if_no_room";
        self.controle(ui, "contest_max_age", ativo && !sem_vaga);
        self.controle(ui, "unattached", true);
    }

    fn aba_inativos(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Inativos);
        self.controle(ui, "retire_idle_players", true);
        let ativo = self.ligado("retire_idle_players");
        for k in ["idle_days", "idle_min_age", "idle_rep", "idle_rep_per_year", "protect_young_unemployed"] {
            self.controle(ui, k, ativo);
        }
        ui.add_space(8.0);
        let rotulo = self.t("Reputação de exemplo", "Example reputation");
        let ajuda = self.termo(Aba::Inativos, "Reputação de exemplo");
        ui.horizontal(|ui| {
            ui.label(rotulo);
            ui.add(egui::Slider::new(&mut self.rep_exemplo, 500.0..=10000.0).step_by(100.0));
            ajuda_icone(ui, ajuda);
        });
        let g = |k: &str, d: f64| self.valor(k).parse::<f64>().unwrap_or(d);
        let (idade_min, rep, por_ano, protege) = (g("idle_min_age", 28.0), g("idle_rep", 1500.0), g("idle_rep_per_year", 250.0), g("protect_young_unemployed", 0.0));
        let rep_ex = self.rep_exemplo;
        let largura = ui.available_width().min(900.0);
        let (resp, p) = ui.allocate_painter(Vec2::new(largura, 220.0), Sense::hover());
        let r = resp.rect.shrink2(Vec2::new(34.0, 20.0));
        p.rect_filled(resp.rect, 4, CARTAO);
        let x = |idade: f64| r.left() + ((idade - 18.0) / 22.0) as f32 * r.width();
        let y = |chance: f64| r.bottom() - chance.clamp(0.0, 1.0) as f32 * r.height();
        let original: Vec<Pos2> = (18..=40).map(|i| Pos2::new(x(i as f64), y((750.0 / rep_ex).min(1.0)))).collect();
        p.extend(egui::Shape::dashed_line(&original, Stroke::new(1.5f32, CINZA), 6.0, 4.0));
        if ativo {
            let nova: Vec<Pos2> = (18..=40).map(|i| {
                let i = i as f64;
                let c = if protege > 0.0 && i < protege { 0.0 } else if i >= idade_min { (rep + por_ano * (i - idade_min)) / rep_ex } else { 750.0 / rep_ex };
                Pos2::new(x(i), y(c.min(1.0)))
            }).collect();
            p.add(egui::Shape::line(nova, Stroke::new(2.5f32, VERDE)));
        }
        for i in (18..=40).step_by(2) {
            p.text(Pos2::new(x(i as f64), r.bottom() + 4.0), Align2::CENTER_TOP, format!("{}", i), FontId::proportional(10.0), Color32::LIGHT_GRAY);
        }
        for c in [0.0, 0.5, 1.0] {
            p.text(Pos2::new(r.left() - 4.0, y(c)), Align2::RIGHT_CENTER, format!("{:.0}%", c * 100.0), FontId::proportional(10.0), Color32::LIGHT_GRAY);
        }
        ui.label(RichText::new(self.t("Chance de aposentar por verificação (dias 0, 80, 173 e 250 do ano), por idade, para a reputação de exemplo. Tracejado: a regra original (750 / reputação, depois de 365 dias sem clube).",
            "Chance of retiring per check (days 0, 80, 173 and 250 of the year), by age, for the example reputation. Dashed: the original rule (750 / reputation, after 365 days without a club).")).size(11.0).color(CINZA));
    }

    /// O peso do clube para o jovem: a mesma conta de `peso()` em src/regens/disputa.rs.
    fn peso(&self, c: &Clube, surpresa: bool) -> f64 {
        let g = |k: &str, d: f64| self.valor(k).parse::<f64>().unwrap_or(d);
        let (teto, exp) = (g("squad_limit", 48.0).clamp(1.0, 50.0), g("squad_exp", 0.5));
        if c.elenco >= teto || (surpresa && !c.competicao) {
            return 0.0;
        }
        let tca = g("target_ca", 0.0);
        let alvo = g("target_k", 50.0) * ((1.0 - tca) * self.pa + tca * self.ca);
        let sigma = g("sigma", 2500.0).max(1.0);
        let abaixo = g("sigma_below", 1200.0);
        let largura = if c.rep < alvo && abaixo > 0.0 { abaixo } else { sigma };
        let z = (c.rep - alvo) / largura;
        let piso = g("level_floor", 0.002);
        let nivel = if surpresa { 1.0 } else { piso + (1.0 - piso) * (-0.5 * z * z).exp() };
        let inst = g("facilities_base", 0.4) + g("facilities_k", 0.06) * c.training;
        let (cmin, cmax) = (g("staff_min", 0.8), g("staff_max", 1.2));
        let com = cmin + (cmax - cmin) * (c.comissao - 1.0) / 19.0;
        let vaga = ((teto - c.elenco) / teto).powf(exp);
        let p = (nivel * inst * com * vaga).max(0.0).powf(g("temperature", 1.0));
        if !surpresa && !c.competicao { p * g("outside_competitions", 1.0).clamp(0.0, 1.0) } else { p }
    }

    fn aba_disputa(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Disputa);
        self.controle(ui, "rule", true);
        let ativo = self.ligado("rule");
        if !ativo {
            ui.colored_label(CINZA, self.t("O jogo usa a disputa do 3.9.68.", "The game uses 3.9.68's contest."));
        }
        let fornada = self.fornada_ligada();
        if fornada {
            ui.colored_label(AZUL, self.t("A fornada está ligada: esta curva decide para qual clube vai cada regen, mas o número de regens por clube é o da fornada. O máximo por ano e o teto do elenco não valem (aba Fornada).",
                "The intake is on: this curve decides which club each regen goes to, but the number of regens per club is the intake's. The yearly maximum and the squad limit don't apply (Intake tab)."));
        }
        let rotulo = self.t("Jovem de exemplo: PA", "Example youngster: PA");
        let (a_jovem, a_pa, a_ca) = (self.termo(Aba::Disputa, "Jovem de exemplo"), self.termo(Aba::Idades, "PA"), self.termo(Aba::Idades, "CA"));
        ui.horizontal(|ui| {
            ajuda_icone(ui, a_jovem);
            ui.label(rotulo);
            ui.add(egui::Slider::new(&mut self.pa, 40.0..=200.0).step_by(1.0));
            ajuda_icone(ui, a_pa);
            ui.add_space(10.0);
            ui.label("CA");
            ui.add(egui::Slider::new(&mut self.ca, 20.0..=150.0).step_by(1.0));
            ajuda_icone(ui, a_ca);
        });
        // a curva do nível
        let g = |k: &str, d: f64| self.valor(k).parse::<f64>().unwrap_or(d);
        let tca = g("target_ca", 0.0);
        let alvo = g("target_k", 50.0) * ((1.0 - tca) * self.pa + tca * self.ca);
        let (sigma, abaixo, piso) = (g("sigma", 2500.0).max(1.0), g("sigma_below", 1200.0), g("level_floor", 0.002));
        let largura = ui.available_width().min(900.0);
        let (resp, p) = ui.allocate_painter(Vec2::new(largura, 200.0), Sense::hover());
        p.rect_filled(resp.rect, 4, CARTAO);
        let r = resp.rect.shrink2(Vec2::new(34.0, 20.0));
        let x = |rep: f64| r.left() + (rep / 10000.0).clamp(0.0, 1.0) as f32 * r.width();
        let y = |v: f64| r.bottom() - v.clamp(0.0, 1.0) as f32 * r.height();
        let nivel = |rep: f64| {
            let l = if rep < alvo && abaixo > 0.0 { abaixo } else { sigma };
            let z = (rep - alvo) / l;
            piso + (1.0 - piso) * (-0.5 * z * z).exp()
        };
        p.rect_filled(Rect::from_min_max(Pos2::new(r.left(), y(piso)), r.right_bottom()), 0, Color32::from_rgba_unmultiplied(0x4c, 0xc3, 0x6a, 40));
        let curva: Vec<Pos2> = (0..=200).map(|i| Pos2::new(x(i as f64 * 50.0), y(nivel(i as f64 * 50.0)))).collect();
        p.add(egui::Shape::line(curva, Stroke::new(2.5f32, if ativo { VERDE } else { CINZA })));
        p.line_segment([Pos2::new(x(alvo), r.top()), Pos2::new(x(alvo), r.bottom())], Stroke::new(1.0f32, AMARELO));
        p.text(Pos2::new(x(alvo) + 4.0, r.top()), Align2::LEFT_TOP, format!("{} {:.0}", self.t("alvo", "target"), alvo), FontId::proportional(11.0), AMARELO);
        for c in &CLUBES {
            p.circle_filled(Pos2::new(x(c.rep), y(nivel(c.rep))), 4.0, AZUL);
        }
        for rep in [0, 2500, 5000, 7500, 10000] {
            p.text(Pos2::new(x(rep as f64), r.bottom() + 4.0), Align2::CENTER_TOP, format!("{}", rep), FontId::proportional(10.0), Color32::LIGHT_GRAY);
        }
        ui.label(RichText::new(self.t("O peso do nível pela reputação do clube (pontos azuis: os clubes de exemplo); a faixa clara é o piso.",
            "The level weight by the club's reputation (blue dots: the example clubs); the light band is the floor.")).size(11.0).color(CINZA));
        // as chances por clube
        let s = g("surprise", 15.0).clamp(0.0, 100.0) / 100.0;
        let normal: Vec<f64> = CLUBES.iter().map(|c| self.peso(c, false) * c.vezes).collect();
        let surpr: Vec<f64> = CLUBES.iter().map(|c| self.peso(c, true) * c.vezes).collect();
        let (sn, ss) = (normal.iter().sum::<f64>().max(1e-12), surpr.iter().sum::<f64>().max(1e-12));
        let en = self.en;
        App::cartao(ui, &self.t("Chance de cada clube receber o jovem", "Each club's chance of getting the youngster"), |ui| {
            for (i, c) in CLUBES.iter().enumerate() {
                let ch = (1.0 - s) * normal[i] / sn + s * surpr[i] / ss;
                ui.horizontal(|ui| {
                    rotulo_fixo(ui, 230.0, if en { c.nome.1 } else { c.nome.0 });
                    let (rr, pp) = ui.allocate_painter(Vec2::new(300.0, 14.0), Sense::hover());
                    pp.rect_filled(rr.rect, 2, FUNDO);
                    pp.rect_filled(Rect::from_min_size(rr.rect.min, Vec2::new(rr.rect.width() * ch as f32, rr.rect.height())), 2, if ativo { VERDE } else { CINZA });
                    ui.label(format!("{:.1}%", 100.0 * ch));
                });
            }
        });
        for k in ["target_k", "target_ca", "squad_limit", "season_cap", "surprise", "outside_competitions"] {
            // a fornada anula os dois tetos: a cota dela é o limite
            self.controle(ui, k, ativo && !(fornada && (k == "squad_limit" || k == "season_cap")));
        }
        ui.add_space(6.0);
        // o ajuste fino fica fechado: a predefinição Equilibrado já o deixa certo (03/11, Lucas)
        egui::CollapsingHeader::new(RichText::new(self.t("Avançado: o ajuste fino do sorteio", "Advanced: fine-tuning the draw")).strong()).default_open(std::env::var("TWEAKS_CONFIG_AVANCADO").is_ok()).show(ui, |ui| {
            ui.label(RichText::new(self.t("Quase ninguém precisa mexer aqui: a predefinição Equilibrado já deixa estes valores certos. Cada um diz embaixo o que faz.",
                "Almost nobody needs to change these: the Balanced preset already sets them right. Each one says below what it does.")).size(12.0).color(CINZA));
            for k in ["sigma", "sigma_below", "level_floor", "facilities_base", "facilities_k", "staff_min", "staff_max", "squad_exp", "temperature"] {
                self.controle(ui, k, ativo);
            }
        });
        if self.valor("staff_max").parse::<f64>().unwrap_or(1.2) < self.valor("staff_min").parse::<f64>().unwrap_or(0.8) {
            ui.colored_label(AMARELO, self.t("staff_max está abaixo de staff_min.", "staff_max is below staff_min."));
        }
    }

    fn aba_nomes(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Nomes);
        self.controle(ui, "regen_names", true);
        let ativo = self.ligado("regen_names");
        self.controle(ui, "regen_names_min", ativo);
        ui.add_space(8.0);
        let en = self.en;
        let minimo = self.valor("regen_names_min").parse::<f64>().unwrap_or(20.0) as i32;
        let Some(lista) = &self.nomes else {
            App::cartao(ui, &self.t("Os países deste jogo", "This game's countries"), |ui| {
                ui.label(t2(en, "Não achei o banco (a pasta Data com nation.dat, first_names.dat e second_names.dat) na pasta do jogo.",
                    "I couldn't find the database (the Data folder with nation.dat, first_names.dat and second_names.dat) in the game folder."));
            });
            return;
        };
        // quem já usa os próprios (mais de 100 nos dois), quem passa a usar com o mínimo, quem continua emprestando
        let afetadas: Vec<&nomes::Nacao> = lista.iter().filter(|n| n.primeiros <= 100 || n.segundos <= 100).collect();
        let passam = afetadas.iter().filter(|n| n.primeiros >= minimo && n.segundos >= minimo).count();
        let sem_nome = afetadas.iter().filter(|n| n.primeiros == 0 && n.segundos == 0).count();
        let titulo = self.t("Os países deste jogo", "This game's countries");
        App::cartao(ui, &titulo, |ui| {
            ui.label(if en {
                format!("This game's database has {} countries. {} have more than 100 names and already use their own. {} have 100 or fewer and borrow from another country; with the option and a minimum of {}, {} of them use their own. The rest keep borrowing ({} have no names at all).",
                    lista.len(), lista.len() - afetadas.len(), afetadas.len(), minimo, passam, sem_nome)
            } else {
                format!("O banco deste jogo tem {} países. {} têm mais de 100 nomes e já usam os próprios. {} têm 100 ou menos e pegam nomes de outro país; com a opção e o mínimo de {}, {} delas passam a usar os próprios. As outras continuam pegando emprestado ({} não têm nome nenhum).",
                    lista.len(), lista.len() - afetadas.len(), afetadas.len(), minimo, passam, sem_nome)
            });
            ui.add_space(6.0);
            egui::ScrollArea::both().max_height(380.0).show(ui, |ui| {
                egui::Grid::new("nomes").striped(true).spacing([12.0, 3.0]).show(ui, |ui| {
                    for h in [t2(en, "País", "Country"), t2(en, "Nomes", "Names"), t2(en, "Sobrenomes", "Surnames"), t2(en, "Com a opção", "With the option"), t2(en, "Hoje vêm", "Today from")] {
                        ui.label(RichText::new(h).strong());
                    }
                    ui.end_row();
                    for n in &afetadas {
                        let proprios = n.primeiros >= minimo && n.segundos >= minimo;
                        ui.label(&n.nome);
                        ui.label(n.primeiros.to_string());
                        ui.label(n.segundos.to_string());
                        ui.label(if proprios {
                            RichText::new(t2(en, "os próprios", "its own")).color(if ativo { VERDE } else { CINZA })
                        } else if n.grupo == 0 {
                            RichText::new(t2(en, "qualquer país (tabela certa)", "any country (right table)")).color(CINZA)
                        } else {
                            RichText::new(t2(en, "igual (poucos nomes)", "the same (too few names)")).color(CINZA)
                        });
                        ui.label(RichText::new(nomes::de_onde(n, lista, en)).color(AMARELO));
                        ui.end_row();
                    }
                });
            });
        });
    }

    fn aba_cinzentos(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Cinzentos);
        self.controle(ui, "grey_players", true);
        let ativo = self.ligado("grey_players");
        let largura = self.valor("grey_players_spread").parse::<f64>().unwrap_or(10.0).clamp(0.0, 100.0);
        // a barra mostra o ± (a metade); o tweaks.cfg guarda a largura toda
        let mut mais_menos = (largura / 2.0).round() as i32;
        let en = self.en;
        let rotulo = self.t("Variação em volta do nível", "Variation around the level");
        let (a_pt, a_en) = guia::ajuda("grey_players_spread");
        let mut mudou = false;
        let r = ui.horizontal(|ui| {
            let r = ui.add_enabled_ui(ativo, |ui| ui.horizontal(|ui| {
                rotulo_fixo(ui, 260.0, rotulo);
                let r = ui.add(egui::Slider::new(&mut mais_menos, 0..=25).show_value(false));
                ui.label(RichText::new(format!("± {}", mais_menos)).strong());
                mudou = r.changed();
                r
            }).inner).inner;
            ajuda_icone(ui, if en { a_en } else { a_pt });
            r
        });
        if mudou {
            self.definir("grey_players_spread", &(2 * mais_menos).to_string());
        }
        if r.response.hovered() || r.inner.hovered() {
            self.foco = Some("grey_players_spread");
        }
        let m = mais_menos;
        ui.horizontal(|ui| {
            ui.add_space(22.0);
            ui.add(egui::Label::new(RichText::new(if m == 0 {
                t2(en, "± 0: todo cinzento vem exatamente no nível do clube.", "± 0: every grey player comes exactly at the club's level.").to_string()
            } else if en {
                format!("± {m}: each grey player comes with a CA from {m} below to {m} above the club's level. At a level-82 club: from {} to {}.", 82 - m, 82 + m)
            } else {
                format!("± {m}: cada cinzento vem com um CA de {m} abaixo a {m} acima do nível do clube. Num clube de nível 82: de {} a {}.", 82 - m, 82 + m)
            }).size(12.5).color(FRASE)).wrap());
        });
        ui.add_space(10.0);
        App::cartao(ui, &self.t("Exemplos (de uma carreira de teste, Inglaterra carregada, 2001)", "Examples (from a test career, England loaded, 2001)"), |ui| {
            egui::Grid::new("cinzentos").striped(true).spacing([18.0, 4.0]).show(ui, |ui| {
                for h in [t2(en, "Clube", "Club"), t2(en, "Reputação", "Reputation"), t2(en, "Nível do clube", "Club's level"), t2(en, "CA no original", "CA in the original"), t2(en, "CA com esta opção", "CA with this option")] {
                    ui.label(RichText::new(h).strong());
                }
                ui.end_row();
                for (clube, rep, nivel, fonte, orig) in [("Rochdale", 3000, 82, 0, -82), ("Mansfield Town", 2500, 95, 0, -95), ("Wimbledon", 5000, 107, 0, 1), ("Corinthians", 9000, 164, 1, 143), ("Tonga", 500, 45, 1, -124)] {
                    ui.label(if clube == "Tonga" { t2(en, "Tonga (seleção)", "Tonga (national team)") } else { clube });
                    ui.label(rep.to_string());
                    ui.label(format!("{} ({})", nivel, if fonte == 0 { t2(en, "elenco", "squad") } else { t2(en, "reputação", "reputation") }));
                    ui.label(RichText::new(orig.to_string()).color(if orig < 0 { VERMELHO } else { Color32::LIGHT_GRAY }));
                    ui.label(RichText::new(if m == 0 { nivel.to_string() } else { format!("{} {} {}", nivel - m, t2(en, "a", "to"), nivel + m) }).color(if ativo { VERDE } else { CINZA }));
                    ui.end_row();
                }
            });
            ui.label(RichText::new(t2(en, "O CA negativo do original é tratado pelo motor da partida como o pior jogador possível.", "The original's negative CA is treated by the match engine as the weakest possible player.")).size(11.5).color(CINZA));
        });
    }

    /// O histórico de regens (`regen_origins.csv`): quantos, a busca por nome e limpar.
    fn historico(&mut self, ui: &mut egui::Ui) {
        self.controle(ui, "regen_origins", true);
        let caminho = self.pasta.join("regen_origins.csv");
        let texto = std::fs::read_to_string(&caminho).unwrap_or_default();
        let linhas: Vec<Vec<&str>> = texto.lines().skip(1).map(|l| l.split(';').collect::<Vec<_>>()).filter(|c| c.len() >= 21 && c[3] == "regen").collect();
        let en = self.en;
        let titulo = self.t("Histórico de regens", "Regen history");
        let busca_rot = self.t("Buscar (nome do regen, da vaga ou da ficha):", "Search (the regen's, slot's or record's name):");
        let mut abrir_limpar = false;
        App::cartao(ui, &titulo, |ui| {
            if linhas.is_empty() {
                ui.label(if en { "No regens recorded yet in this folder (turn the option on and play)." } else { "Nenhum regen registrado nesta pasta ainda (ligue a opção e jogue)." });
                return;
            }
            ui.horizontal(|ui| {
                ui.label(format!("{} {}", linhas.len(), if en { "regens recorded" } else { "regens registrados" }));
                if ui.button(if en { "Clear the history…" } else { "Limpar o histórico…" }).clicked() {
                    abrir_limpar = true;
                }
            });
            ui.horizontal(|ui| {
                ui.label(busca_rot);
                ui.add(egui::TextEdit::singleline(&mut self.busca).desired_width(240.0));
            });
            let b = self.busca.to_lowercase();
            let achadas: Vec<&Vec<&str>> = linhas.iter().rev().filter(|c| b.is_empty() || [5, 13, 17].iter().any(|&i| c[i].to_lowercase().contains(&b))).take(300).collect();
            egui::ScrollArea::both().max_height(420.0).show(ui, |ui| {
                egui::Grid::new("historico").striped(true).spacing([10.0, 4.0]).show(ui, |ui| {
                    for h in [t2(en, "Data (jogo)", "Date (game)"), t2(en, "Regen", "Regen"), t2(en, "Idade", "Age"), t2(en, "Pos.", "Pos."), "PA", "CA",
                        t2(en, "Clube", "Club"), t2(en, "Ficha de (idade, CA, PA na aposentadoria)", "Record from (age, CA, PA at retirement)"), t2(en, "Vaga de", "Slot from"), t2(en, "Carreira", "Career")] {
                        ui.label(RichText::new(h).strong());
                    }
                    ui.end_row();
                    for c in achadas {
                        ui.label(c[1]);
                        ui.label(RichText::new(c[5]).color(VERDE));
                        ui.label(c[6]);
                        ui.label(c[8]);
                        ui.label(c[9]);
                        ui.label(c[10]);
                        // o CSV fica em inglês; na tela, os dois textos fixos dele seguem o idioma
                        ui.label(if !en && c[11] == "(no club)" { "(sem clube)" } else { c[11] });
                        let ficha = if c[17].starts_with("(unknown") {
                            t2(en, "desconhecido (save carregado sem o .regens)", "unknown (save loaded without its .regens)").to_string()
                        } else if !c[17].is_empty() {
                            format!("{} ({}, {}, {})", c[17], c[18], c[19], c[20])
                        } else {
                            "?".into()
                        };
                        ui.label(RichText::new(ficha).color(AMARELO));
                        ui.label(format!("{} ({})", c[13], match (en, c[15]) { (false, "player") => "jogador", (false, "non-player") => "não jogador", (_, x) => x }));
                        ui.label(RichText::new(c[2]).size(10.5).color(CINZA));
                        ui.end_row();
                    }
                });
            });
        });
        if abrir_limpar {
            self.limpar = true;
        }
        if self.limpar {
            let mut fechar = false;
            egui::Window::new(self.t("Limpar o histórico de regens?", "Clear the regen history?")).collapsible(false).resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(ui.ctx(), |ui| {
                    ui.label(self.t("O regen_origins.csv começa de novo vazio. Os regens já criados não voltam a ser registrados.",
                        "regen_origins.csv starts again empty. Regens already created won't be recorded again."));
                    ui.horizontal(|ui| {
                        if ui.button(self.t("Guardar uma cópia e limpar", "Keep a copy and clear")).clicked() {
                            let copia = self.pasta.join(format!("regen_origins_{}.csv", hoje_no_computador()));
                            self.mensagem = match std::fs::rename(&caminho, &copia) {
                                Ok(_) => format!("{} {}", self.t("Histórico guardado em", "History kept in"), copia.display()),
                                Err(e) => e.to_string(),
                            };
                            fechar = true;
                        }
                        if ui.button(self.t("Apagar de vez", "Delete for good")).clicked() {
                            self.mensagem = match std::fs::remove_file(&caminho) {
                                Ok(_) => self.t("Histórico apagado.", "History deleted."),
                                Err(e) => e.to_string(),
                            };
                            fechar = true;
                        }
                        if ui.button(self.t("Cancelar", "Cancel")).clicked() {
                            fechar = true;
                        }
                    });
                });
            if fechar {
                self.limpar = false;
            }
        }
    }

    /// A fornada ligada: `intake = 1`; sem a linha `intake` (um cfg da 1.2), o `intake_stock` > 0.
    fn fornada_ligada(&self) -> bool {
        match self.cfg.valor("intake") {
            Some(v) => v.trim() == "1",
            None => self.valor("intake_stock").parse::<f64>().unwrap_or(0.0) > 0.0,
        }
    }

    fn aba_fornada(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Fornada);
        self.controle(ui, "intake", true);
        let ativo = self.fornada_ligada();
        ui.colored_label(if ativo { AZUL } else { CINZA }, if ativo {
            self.t("Ligada. Regen é regen: os regens só nascem no dia da fornada, e cada clube com vaga recebe o número abaixo. Para qual clube vai cada um, quem decide é a curva da aba Disputa dos clubes. O máximo por ano e o teto do elenco da aba Disputa dos clubes não valem.",
                "On. A regen is a regen: regens are only born on intake day, and every club with room gets the number below. Which club each one goes to is up to the Clubs' contest tab's curve. The Clubs' contest tab's yearly maximum and squad limit don't apply.")
        } else {
            self.t("Desligada: os regens nascem ao longo da temporada, como no jogo original, pelas regras da aba Disputa dos clubes.",
                "Off: regens are born through the season, as in the original game, by the Clubs' contest tab's rules.")
        });
        ui.add_space(6.0);
        ui.strong(self.t("Quando", "When"));
        self.controle(ui, "intake_day", ativo);
        self.controle(ui, "retire_day", true);
        ui.add_space(6.0);
        ui.strong(self.t("Quantos", "How many"));
        for nome in ["intake_per_club", "intake_clubs"] {
            self.controle(ui, nome, ativo);
        }
        ui.add_space(6.0);
        ui.strong(self.t("A mensagem", "The message"));
        for nome in ["intake_news", "intake_news_numbers"] {
            self.controle(ui, nome, ativo);
        }
    }

    /// Um número inteiro com separador de milhar no jeito do idioma.
    fn milhar(&self, x: f64) -> String {
        let n = x.round() as i64;
        let d = n.abs().to_string();
        let mut out = String::new();
        for (i, c) in d.chars().enumerate() {
            if i > 0 && (d.len() - i) % 3 == 0 {
                out.push(if self.en { ',' } else { '.' });
            }
            out.push(c);
        }
        if n < 0 { format!("-{}", out) } else { out }
    }

    /// O custo por lugar: o do jogo, automático (pelo caixa do banco) ou um número fixo.
    fn controle_custo(&mut self, ui: &mut egui::Ui, ativo: bool) {
        let en = self.en;
        let v = self.valor("stadium_seat_cost");
        let numero = v.parse::<f64>().ok();
        let mut modo = if numero.is_some() { 2 } else if v == "original" { 0 } else { 1 };
        let antes = modo;
        let nomes = [t2(en, "O do jogo (uns 100 por lugar)", "The game's (about 100 a place)"), t2(en, "Automático, pelo caixa do banco", "Automatic, by the database's money"), t2(en, "Um valor fixo por lugar", "A fixed value per place")];
        let mut x = numero.unwrap_or(1500.0);
        let mut mudou_x = false;
        let (a_pt, a_en) = guia::ajuda("stadium_seat_cost");
        let r = ui.horizontal(|ui| {
            let r = ui.add_enabled_ui(ativo, |ui| ui.horizontal(|ui| {
                rotulo_fixo(ui, 260.0, t2(en, "Custo por lugar novo", "Cost per new place"));
                let r = egui::ComboBox::from_id_salt("stadium_seat_cost").width(300.0).selected_text(nomes[modo]).show_ui(ui, |ui| {
                    for (i, n) in nomes.iter().enumerate() {
                        ui.selectable_value(&mut modo, i, *n);
                    }
                }).response;
                if modo == 2 {
                    mudou_x = ui.add(egui::DragValue::new(&mut x).range(1.0..=1e7).speed(10.0).max_decimals(0)).changed();
                }
                r
            }).inner).inner;
            ajuda_icone(ui, if en { a_en } else { a_pt });
            r
        });
        if modo != antes || mudou_x {
            let novo = match modo { 0 => "original".to_string(), 1 => "auto".to_string(), _ => cfg::numero(x.round()) };
            self.definir("stadium_seat_cost", &novo);
        }
        if r.response.hovered() || r.inner.hovered() {
            self.foco = Some("stadium_seat_cost");
        }
        let frase = match modo {
            0 => t2(en, "Uns 100 por lugar (875 num clube profissional): 21 mil lugares saem por uns 2 milhões, e qualquer clube rico amplia sem sentir.",
                "About 100 a place (875 for a professional club): 21,000 places cost about 2 million, and any rich club expands without noticing.").to_string(),
            1 => t2(en, "1.500 por lugar no banco de 2001 (10 mil lugares: 15 milhões, mais ou menos o caixa de um clube grande da época). Num banco com o triplo de dinheiro, como o do CLZ, a obra custa o triplo. O valor calculado aparece no tweaks.log.",
                "1,500 a place in the 2001 database (10,000 places: 15 million, about a big club's balance of the time). In a database with three times the money, like CLZ's, a build costs three times as much. The worked-out value shows in tweaks.log.").to_string(),
            _ => if en { format!("{} a place, in the money of the database you play: 10,000 places cost {}.", self.milhar(x), self.milhar(x * 10000.0)) }
                else { format!("{} por lugar, no dinheiro do banco que você joga: 10 mil lugares custam {}.", self.milhar(x), self.milhar(x * 10000.0)) },
        };
        ui.horizontal(|ui| {
            ui.add_space(22.0);
            ui.add(egui::Label::new(RichText::new(frase).size(12.5).color(FRASE)).wrap());
        });
        ui.add_space(4.0);
    }

    fn aba_publico(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Publico);
        let en = self.en;
        // ---- o público ----
        let p = self.ligado("attendance");
        let titulo = self.t("O público", "Attendance");
        App::cartao(ui, &titulo, |_| {});
        self.controle(ui, "attendance", true);
        ui.colored_label(if p { AZUL } else { CINZA }, if p {
            self.t("Ligado: quem termina onde se previa mantém o público; quem vai melhor cresce; quem vai pior perde o que ganhou por esta regra.",
                "On: a club that finishes where it was predicted keeps its crowd; one that does better grows; one that does worse loses what it gained by this rule.")
        } else {
            self.t("Desligado: o público de cada jogo e da temporada é o do jogo original; os controles abaixo ficam sem efeito.", "Off: every match and season crowd is the original game's; the controls below have no effect.")
        });
        ui.add_space(8.0);
        ui.strong(self.t("A média da temporada (no fim de cada temporada da liga)", "The season average (at the end of each league season)"));
        for k in ["attendance_toward_real", "attendance_per_place", "tradition_per_place", "attendance_floor"] {
            self.controle(ui, k, p);
        }
        ui.add_space(6.0);
        ui.strong(self.t("Tetos pela divisão (0 = sem teto; o jogo original usa 2.000, 3.000 e 5.000)", "Caps by division (0 = no cap; the original game uses 2,000, 3,000 and 5,000)"));
        for k in ["attendance_cap_weak", "attendance_cap_mid", "attendance_cap_strong"] {
            self.controle(ui, k, p);
        }
        ui.add_space(6.0);
        ui.strong(self.t("Cada jogo", "Each match"));
        for k in ["attendance_record", "pent_up_demand", "weak_league_limit"] {
            self.controle(ui, k, p);
        }
        let corte = p && self.valor("weak_league_limit").parse::<f64>().unwrap_or(0.0) > 0.0;
        self.controle(ui, "weak_league_keep", corte);
        self.controle(ui, "knockout_ceiling", p);
        self.controle(ui, "title_race", p);
        let grande = p && self.ligado("title_race");
        for k in ["title_race_max", "title_race_top2"] {
            self.controle(ui, k, grande);
        }
        ui.add_space(14.0);
        // ---- o estádio ----
        let e = self.ligado("stadium");
        let titulo = self.t("O estádio", "The stadium");
        App::cartao(ui, &titulo, |_| {});
        self.controle(ui, "stadium", true);
        ui.colored_label(if e { AZUL } else { CINZA }, if e {
            self.t("Ligado: só amplia quem lota de forma sustentada e tem dinheiro, do tamanho que a demanda pede e pagando um preço de verdade.",
                "On: only clubs that fill their ground steadily and have the money expand, by what the demand asks for and paying a real price.")
        } else {
            self.t("Desligado: as obras do estádio são as do jogo original; os controles abaixo ficam sem efeito.", "Off: stadium works are the original game's; the controls below have no effect.")
        });
        ui.add_space(8.0);
        ui.strong(self.t("O que o jogo faz sem motivo", "What the game does for no reason"));
        for k in ["stadium_seating_growth", "stadium_reputation_trigger"] {
            self.controle(ui, k, e);
        }
        ui.add_space(6.0);
        ui.strong(self.t("Quando pode ampliar", "When it can expand"));
        for k in ["stadium_occupancy_min", "stadium_full_seasons", "stadium_occupancy", "stadium_reserve"] {
            self.controle(ui, k, e);
        }
        ui.add_space(6.0);
        ui.strong(self.t("A obra", "The build"));
        self.controle(ui, "stadium_size", e);
        self.controle_custo(ui, e);
        for k in ["stadium_ceiling_zero", "stadium_max"] {
            self.controle(ui, k, e);
        }
        ui.add_space(10.0);
        // ---- o clube de exemplo ----
        let g = |s: &App, k: &str| s.valor(k).parse::<f64>().unwrap_or_else(|_| chaves::chave(k).and_then(|c| c.padrao.parse().ok()).unwrap_or(0.0));
        let (trad, reprimida, por_pos, piso) = (g(self, "tradition_per_place") / 100.0, g(self, "pent_up_demand") / 100.0, g(self, "attendance_per_place") / 100.0, self.ligado("attendance_floor"));
        let (mata, grande) = (p && self.ligado("knockout_ceiling"), p && self.ligado("title_race"));
        let extra = 1.0 + (g(self, "title_race_max") + g(self, "title_race_top2")) / 100.0;
        let (cap, media, recorde, historico) = (30000.0f64, 12000.0f64, 18000.0f64, 26000.0f64);
        let um = |x: f64| self.milhar(x);
        let mut linhas: Vec<(String, String)> = Vec::new();
        for (dif, pt, en_) in [(0i32, "Terminou onde se previa", "Finished where predicted"), (4, "Terminou 4 posições acima", "Finished 4 places above"), (-4, "Terminou 4 posições abaixo", "Finished 4 places below")] {
            let r = if !p {
                match dif { 0 => um(media * 1.025), d if d > 0 => um(media * 1.08), _ => um(media) }
            } else {
                let mut nova = media * (1.0 + por_pos * dif as f64);
                if dif >= 2 && trad > 0.0 {
                    nova += (trad * dif as f64).min(1.0) * (historico - nova);
                }
                if dif < 0 && piso {
                    nova = nova.max(media); // o piso: este clube ainda não tinha ganho nada pela nossa regra
                }
                um(nova.min(cap))
            };
            linhas.push((t2(en, pt, en_).to_string(), r));
        }
        let cl = if grande { (recorde * 1.5 * extra).min(cap) } else { (recorde * 1.5).min(cap) };
        linhas.push((t2(en, "Teto de um jogo comum de liga", "Ceiling of an ordinary league match").to_string(), um(recorde)));
        linhas.push((t2(en, "Clássico valendo o título", "Derby deciding the title").to_string(), um(cl)));
        linhas.push((t2(en, "Semifinal de copa", "Cup semi-final").to_string(), um(if mata { cap } else { recorde })));
        if reprimida > 0.0 && p {
            linhas.push((t2(en, "Jogo com 6.000 pessoas de fora", "Match with 6,000 people left out").to_string(),
                if en { format!("record goes from {} to {}", um(recorde), um(recorde + 6000.0 * reprimida)) } else { format!("recorde vai de {} a {}", um(recorde), um(recorde + 6000.0 * reprimida)) }));
        }
        let custo = match self.valor("stadium_seat_cost").as_str() { "original" => 100.0, "auto" => 1500.0, x => x.parse().unwrap_or(1500.0) };
        let titulo = self.t("Um clube de exemplo", "An example club");
        let descr = if en {
            format!("Stadium of {} places, average crowd of {}, record of {} and a historical record of {}. Approximate values for the next season, with the settings above.", um(cap), um(media), um(recorde), um(historico))
        } else {
            format!("Estádio de {} lugares, média de {} pessoas, recorde de {} e recorde histórico de {}. Valores aproximados da temporada seguinte, com os ajustes acima.", um(cap), um(media), um(recorde), um(historico))
        };
        let obra = if !e {
            t2(en, "Obras: as do jogo original (pode ampliar só pela reputação, e o estádio com lugares em pé cresce todo ano).", "Works: the original game's (it can expand on reputation alone, and a stadium with standing room grows every year).").to_string()
        } else {
            let lugares = 8000.0;
            let (temp, occ) = (g(self, "stadium_full_seasons"), g(self, "stadium_occupancy_min"));
            if en { format!("Works: after {} season(s) in a row above {}% full ({} a match), wanting about {} more places, it builds them for about {} (if it has that above its reserve).", temp, self.n(occ), um(cap * occ / 100.0), um(lugares), um(lugares * custo)) }
            else { format!("Obras: depois de {} temporada(s) seguida(s) acima de {}% de ocupação ({} por jogo), com uns {} lugares a mais de demanda, constrói por uns {} (se tiver isso acima da reserva).", temp, self.n(occ), um(cap * occ / 100.0), um(lugares), um(lugares * custo)) }
        };
        App::cartao(ui, &titulo, |ui| {
            ui.label(RichText::new(descr).size(12.5).color(FRASE));
            ui.add_space(4.0);
            egui::Grid::new("publico_exemplo").striped(true).spacing([18.0, 4.0]).show(ui, |ui| {
                for (a, b) in &linhas {
                    ui.label(a);
                    ui.label(RichText::new(b).strong().color(if p { AMARELO } else { CINZA }));
                    ui.end_row();
                }
            });
            ui.add_space(4.0);
            ui.label(RichText::new(obra).size(12.5).color(if e { AMARELO } else { CINZA }));
            ui.label(RichText::new(t2(en, "Conta simplificada, para dar a ideia: no jogo, cada partida também pesa o adversário, a distância e a fase da temporada.", "A simplified sum, to give the idea: in the game, each match also weighs the opponent, the distance and the stage of the season.")).size(11.5).color(CINZA));
        });
    }

    fn aba_registro(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Registro);
        self.historico(ui);
    }

    fn aba_log(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Log);
        self.controle(ui, "log", true);
        let log = self.exe.as_ref().map(|e| e.caminho.with_file_name("tweaks.log")).unwrap_or_else(|| self.pasta.join("tweaks.log"));
        App::cartao(ui, &self.t("O último tweaks.log (o exe, a memória e cada ajuste)", "The last tweaks.log (the exe, memory and each tweak)"), |ui| match std::fs::read_to_string(&log) {
            Ok(texto) => {
                let linhas: Vec<&str> = texto.lines().filter(|l| !["exe-", "runtime-fn", "regen\t", "birth\t", "idle\t", "tick\t", "season\t"].iter().any(|p| l.starts_with(p))).take(80).collect();
                ui.label(RichText::new(linhas.join("\n")).monospace().size(11.0));
            }
            Err(_) => {
                ui.label("—");
            }
        });
    }

    fn aba_cfg(&mut self, ui: &mut egui::Ui) {
        self.cabecalho(ui, Aba::Cfg);
        let texto = self.cfg.texto();
        let antes: Vec<String> = self.lido.lines().map(str::to_string).collect();
        ui.horizontal(|ui| {
            ui.label(format!("{} {}", self.mudancas(), self.t("linhas mudadas ou acrescentadas; nenhuma apagada.", "lines changed or added; none removed.")));
            if ui.button(self.t("Desfazer tudo", "Undo all")).clicked() {
                self.cfg = Cfg::ler(&self.lido);
            }
        });
        egui::Frame::group(ui.style()).fill(FUNDO).show(ui, |ui| {
            for (i, l) in texto.lines().enumerate() {
                let mudou = antes.get(i).map(String::as_str) != Some(l);
                let rt = RichText::new(l).monospace().size(11.5);
                ui.label(if mudou { rt.color(AMARELO).strong() } else if l.trim_start().starts_with('#') { rt.color(CINZA) } else { rt });
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let estados = self.estados();
        let alerta: Vec<&str> = estados.iter().filter(|l| matches!(l.estado, Estado::Sobrescreve | Estado::Desconhecido)).map(|l| l.ajuste).collect();
        egui::TopBottomPanel::top("topo").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("CM0102 Rust Tweaks").strong().size(18.0).color(VERDE));
                ui.separator();
                ui.label(self.t("Pasta do jogo:", "Game folder:"));
                ui.add(egui::TextEdit::singleline(&mut self.pasta_digitada).desired_width(320.0));
                if ui.button("…").on_hover_text(self.t("Escolher a pasta do jogo", "Choose the game folder")).clicked() {
                    if let Some(p) = rfd::FileDialog::new().set_directory(&self.pasta).pick_folder() {
                        self.pasta_digitada = p.display().to_string();
                        self.pasta = p;
                        self.abrir_pasta();
                    }
                }
                if ui.button(self.t("Abrir", "Open")).clicked() {
                    self.pasta = PathBuf::from(self.pasta_digitada.trim());
                    self.abrir_pasta();
                }
                ui.separator();
                let atual = self.predefinicao();
                let rotulo = match atual {
                    Some(i) => if self.en { PREDEFINICOES[i].1 } else { PREDEFINICOES[i].0 }.to_string(),
                    None => self.t("Personalizado", "Custom"),
                };
                let mut escolha = None;
                let en = self.en;
                egui::ComboBox::from_id_salt("predef").width(240.0).selected_text(rotulo).show_ui(ui, |ui| {
                    for (i, p) in PREDEFINICOES.iter().enumerate() {
                        if ui.selectable_label(atual == Some(i), if en { p.1 } else { p.0 }).clicked() {
                            escolha = Some(i);
                        }
                    }
                });
                if let Some(i) = escolha {
                    let mut lista: Vec<(String, String, String)> = PREDEFINICOES[i].2.iter().map(|(k, v)| (k.to_string(), self.valor(k), v.to_string()))
                        .filter(|(_, a, b)| a != b && !a.parse::<f64>().ok().zip(b.parse::<f64>().ok()).is_some_and(|(x, y)| (x - y).abs() < 1e-9)).collect();
                    if i >= 2 {
                        for n in 14..36 {
                            let (p, atual) = (IDADES_PADRAO[n - 14], self.idade_de(n));
                            if (p.0 - atual.0).abs() > 1e-9 || p.1 != atual.1 {
                                lista.push((format!("age_{}", n), format!("{}, {}", cfg::numero(atual.0), atual.1), format!("{}, {}", cfg::numero(p.0), p.1)));
                            }
                        }
                    }
                    self.pendente = Some((i, lista));
                }
                ui.separator();
                let m = self.mudancas();
                if ui.add_enabled(m > 0 || !self.existe, egui::Button::new(RichText::new(self.t("Gravar", "Save")).strong())).clicked() {
                    self.gravar();
                }
                let antes = self.en;
                egui::ComboBox::from_id_salt("idioma").width(110.0).selected_text(if self.en { "English" } else { "Português" }).show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.en, true, "English");
                    ui.selectable_value(&mut self.en, false, "Português");
                });
                if self.en != antes {
                    std::fs::write(arquivo_do_idioma(), if self.en { "en" } else { "pt" }).ok();
                }
            });
            if !alerta.is_empty() {
                egui::Frame::new().fill(Color32::from_rgb(0x4a, 0x3a, 0x12)).inner_margin(6.0).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.colored_label(AMARELO, format!("{} {}: {}", alerta.len(),
                        self.t("ajuste(s) pedem atenção ou não são compatíveis com este exe/patch", "tweak(s) need attention or are not compatible with this exe/patch"), alerta.join(", ")));
                });
            }
            ui.add_space(4.0);
        });
        if let Some((i, lista)) = self.pendente.clone() {
            let mut fechar = false;
            egui::Window::new(self.t("Aplicar a predefinição?", "Apply the preset?")).collapsible(false).resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(ctx, |ui| {
                    ui.label(format!("{} — {} {}", if self.en { PREDEFINICOES[i].1 } else { PREDEFINICOES[i].0 }, lista.len(), self.t("valor(es) vão mudar:", "value(s) will change:")));
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for (k, a, b) in &lista {
                            ui.label(RichText::new(format!("{:<26} {}  →  {}", k, a, b)).monospace());
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button(self.t("Aplicar", "Apply")).clicked() {
                            for (k, _, v) in &lista {
                                match k.strip_prefix("age_").and_then(|n| n.parse::<usize>().ok()) {
                                    Some(n) => {
                                        let p = IDADES_PADRAO[n - 14];
                                        self.definir_idade(n, p.0, p.1);
                                    }
                                    None => self.definir(k, v),
                                }
                            }
                            fechar = true;
                        }
                        if ui.button(self.t("Cancelar", "Cancel")).clicked() {
                            fechar = true;
                        }
                    });
                });
            if fechar {
                self.pendente = None;
            }
        }
        egui::TopBottomPanel::bottom("rodape").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let m = self.mudancas();
                ui.label(format!("{} {}", m, self.t("linha(s) não gravada(s)", "unsaved line(s)")));
                ui.separator();
                ui.label(format!("{}{}", self.pasta.join("tweaks.cfg").display(), if self.existe { "" } else if self.en { " (new: from the distributed file)" } else { " (novo: a partir do distribuído)" }));
                if !self.mensagem.is_empty() {
                    ui.separator();
                    ui.colored_label(AMARELO, &self.mensagem);
                }
            });
        });
        egui::SidePanel::left("abas").resizable(false).exact_width(270.0).show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let mut tudo: Option<bool> = None;
                if ui.button(self.t("Ligar tudo", "Turn all on")).on_hover_text(self.t("Liga todos os ajustes da carreira e a correção do elenco, com os nossos valores.", "Turns on every career tweak and the squad fix, with our values.")).clicked() {
                    tudo = Some(true);
                }
                if ui.button(self.t("Desligar tudo", "Turn all off")).on_hover_text(self.t("Desliga todos os ajustes da carreira e a correção do elenco: o jogo fica como o original. A instalação (save, pasta, tela) fica como está.", "Turns off every career tweak and the squad fix: the game stays as the original. The install (save, folder, screen) stays as it is.")).clicked() {
                    tudo = Some(false);
                }
                if let Some(on) = tudo {
                    for (aba, ..) in ABAS {
                        for &(k, sim, nao) in chaves::mestres(aba) {
                            self.definir(k, if on { sim } else { nao });
                        }
                    }
                    self.definir("squad_guard", if on { "1" } else { "0" });
                }
            });
            ui.add_space(4.0);
            let mut grupo_atual = "";
            for (aba, pt, en, g_pt, g_en) in ABAS {
                let grupo = if self.en { g_en } else { g_pt };
                if grupo != grupo_atual {
                    grupo_atual = grupo;
                    ui.add_space(8.0);
                    ui.label(RichText::new(grupo.to_uppercase()).size(11.5).strong().color(CINZA));
                    ui.add_space(2.0);
                }
                let pior = CHAVES.iter().filter(|c| c.aba == aba).filter_map(|c| estados.iter().find(|l| l.ajuste == c.nome)).map(|l| l.estado)
                    .filter(|e| *e != Estado::Desligado).max();
                ui.horizontal(|ui| {
                    if !grupo.is_empty() {
                        ui.add_space(4.0);
                    }
                    let mestres = chaves::mestres(aba);
                    if mestres.is_empty() {
                        ui.add_space(42.0);
                    } else {
                        let on = mestres.iter().any(|&(k, ..)| chaves::ligado(k, &self.valor(k)));
                        if interruptor(ui, on).on_hover_text(if on { self.t("Ligado: clique para desligar (fica como o jogo original).", "On: click to turn off (stays as the original game).") } else { self.t("Desligado: clique para ligar com os nossos valores.", "Off: click to turn on with our values.") }).clicked() {
                            for &(k, sim, nao) in mestres {
                                self.definir(k, if on { nao } else { sim });
                            }
                        }
                    }
                    if ui.selectable_label(self.aba == aba, RichText::new(if self.en { en } else { pt }).size(14.5)).clicked() {
                        self.aba = aba;
                    }
                    if let Some(e) = pior {
                        bolinha(ui, cor(e));
                        ui.label("").on_hover_text(nome_do_estado(e, self.en));
                    }
                });
                ui.add_space(3.0);
            }
        });
        egui::SidePanel::right("ajuda").resizable(true).default_width(340.0).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.foco {
                Some(k) => {
                    // o nome do que a opção faz, o texto prático e, embaixo, a chave e o comentário técnico do .cfg
                    let guia_k = if k.starts_with("age_") { "age_NN" } else { k };
                    let titulo = match chaves::chave(k) {
                        Some(c) => if self.en { c.rotulo.1 } else { c.rotulo.0 }.to_string(),
                        None => self.t("Tabela de idades", "Age table"),
                    };
                    ui.label(RichText::new(titulo).strong().size(15.0).color(VERDE));
                    let (pt, en) = guia::ajuda(guia_k);
                    ui.label(if self.en { en } else { pt });
                    ui.add_space(6.0);
                    egui::CollapsingHeader::new(self.t("Detalhe técnico (tweaks.cfg)", "Technical detail (tweaks.cfg)")).show(ui, |ui| {
                        ui.label(RichText::new(if k.starts_with("age_") { "age_14 … age_35" } else { k }).monospace().color(CINZA));
                        ui.label(RichText::new(cfg::ajuda(k)).size(11.5).color(CINZA));
                    });
                    let alvo = if k.starts_with("age_") { "ages" } else { k };
                    if let Some(l) = estados.iter().find(|l| l.ajuste == alvo) {
                        ui.add_space(8.0);
                        ui.label(RichText::new(self.t("Neste exe:", "On this exe:")).strong());
                        ui.horizontal(|ui| {
                            bolinha(ui, cor(l.estado));
                            ui.colored_label(cor(l.estado), nome_do_estado(l.estado, self.en));
                        });
                        ui.label(if self.en { &l.motivo.1 } else { &l.motivo.0 });
                    }
                }
                None => {
                    ui.label(self.t("Passe o mouse sobre uma opção para ver a ajuda (o comentário dela no tweaks.cfg).", "Hover an option to see its help (its comment in tweaks.cfg)."));
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut area = egui::ScrollArea::vertical().auto_shrink([false, false]);
            // só para as capturas de tela da documentação: começar rolado
            if let Some(y) = std::env::var("TWEAKS_CONFIG_ROLAR").ok().and_then(|v| v.parse::<f32>().ok()) {
                area = area.vertical_scroll_offset(y);
            }
            area.show(ui, |ui| match self.aba {
                Aba::Inicio => self.aba_inicio(ui),
                Aba::Jogo => self.aba_jogo(ui),
                Aba::Idades => self.aba_idades(ui),
                Aba::Inativos => self.aba_inativos(ui),
                Aba::Disputa => self.aba_disputa(ui),
                Aba::Cinzentos => self.aba_cinzentos(ui),
                Aba::Nomes => self.aba_nomes(ui),
                Aba::Publico => self.aba_publico(ui),
                Aba::Fornada => self.aba_fornada(ui),
                Aba::Registro => self.aba_registro(ui),
                Aba::Cfg => self.aba_cfg(ui),
                Aba::Log => self.aba_log(ui),
            });
        });
    }
}

/// O guia das chaves em Markdown, dos mesmos textos da interface: `tweaks-config.exe --guia pt|en ARQUIVO`.
fn escrever_guia(en: bool, arquivo: &Path) -> std::io::Result<()> {
    use std::fmt::Write as _;
    let mut s = String::new();
    let t = |pt: &'static str, e: &'static str| if en { e } else { pt };
    let _ = writeln!(s, "# {}\n", t("Guia das chaves do tweaks.cfg", "tweaks.cfg key guide"));
    let _ = writeln!(s, "*{}*\n", t("English version: [GUIA_DAS_CHAVES.en.md](GUIA_DAS_CHAVES.en.md). Gerado por `tweaks-config.exe --guia pt` dos mesmos textos da ajuda do configurador; não editar à mão.",
        "Versão em português: [GUIA_DAS_CHAVES.md](GUIA_DAS_CHAVES.md). Generated by `tweaks-config.exe --guia en` from the same texts as the configurator's help; don't edit by hand."));
    let _ = writeln!(s, "{}\n", t("Cada opção: o que faz, quando usar e o que acontece no jogo. Sem a linha no `tweaks.cfg`, vale o padrão indicado (o da DLL). O comentário de cada linha do `tweaks.cfg` traz o detalhe técnico.",
        "Each option: what it does, when to use it and what happens in the game. Without the line in `tweaks.cfg`, the default shown applies (the DLL's). Each `tweaks.cfg` line's comment has the technical detail."));
    for (aba, pt, e, ..) in ABAS {
        let chaves: Vec<_> = CHAVES.iter().filter(|c| c.aba == aba).collect();
        if chaves.is_empty() {
            continue;
        }
        let _ = writeln!(s, "## {}\n", if en { e } else { pt });
        let (i_pt, i_en) = guia::intro(aba);
        let _ = writeln!(s, "{}\n", if en { i_en } else { i_pt });
        for &(n_pt, n_en, x_pt, x_en) in guia::termos(aba) {
            let _ = writeln!(s, "- **{}**: {}", if en { n_en } else { n_pt }, if en { x_en } else { x_pt });
        }
        if !guia::termos(aba).is_empty() {
            s.push('\n');
        }
        for c in chaves {
            let rotulo = if en { c.rotulo.1 } else { c.rotulo.0 };
            let (a_pt, a_en) = guia::ajuda(c.nome);
            let valores = match &c.tipo {
                Tipo::Liga => t("`1` liga, `0` desliga", "`1` on, `0` off").to_string(),
                Tipo::Escolha(op) => op.iter().map(|o| format!("`{}` ({})", o.0, if en { o.2 } else { o.1 })).collect::<Vec<_>>().join(", "),
                Tipo::Numero { min, max, inteiro, .. } => {
                    let f = |x: f64| if x.abs() >= 1e8 { "…".to_string() } else { cfg::numero(x) };
                    format!("{} {} {} {}{}", t("de", "from"), f(*min), t("a", "to"), f(*max), if *inteiro { t(" (inteiro)", " (whole number)") } else { "" })
                }
            };
            let _ = writeln!(s, "### {} — `{}`\n\n{}\n\n{}: {}. {}: `{}`.\n", rotulo, c.nome, if en { a_en } else { a_pt },
                t("Valores", "Values"), valores, t("Padrão", "Default"), c.padrao);
            if c.nome == "ages" {
                let (i_pt, i_en) = guia::ajuda("age_NN");
                let _ = writeln!(s, "### {} — `age_14` … `age_35`\n\n{}\n\n{}: `age_NN = {}`.\n", t("Tabela de idades", "Age table"), if en { i_en } else { i_pt },
                    t("Formato", "Format"), t("chance, teto do CA", "chance, CA cap"));
            }
        }
    }
    std::fs::write(arquivo, s)
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--guia") {
        let en = args.get(2).map(String::as_str) == Some("en");
        let arquivo = PathBuf::from(args.get(3).cloned().unwrap_or_else(|| if en { "GUIA_DAS_CHAVES.en.md".into() } else { "GUIA_DAS_CHAVES.md".into() }));
        escrever_guia(en, &arquivo).ok();
        return Ok(());
    }
    let pasta = std::env::args().nth(1).map(PathBuf::from)
        .or_else(|| std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)))
        .unwrap_or_else(|| PathBuf::from("."));
    let opcoes = eframe::NativeOptions {
        viewport: {
            let v = egui::ViewportBuilder::default().with_inner_size([1280.0, 820.0]).with_title("CM0102 Rust Tweaks");
            // o ícone da janela (o do exe vem do build.rs)
            match eframe::icon_data::from_png_bytes(include_bytes!("../../assets/icon/cm0102-rust-tweaks-icon-256px.png")) {
                Ok(i) => v.with_icon(std::sync::Arc::new(i)),
                Err(_) => v,
            }
        },
        ..Default::default()
    };
    eframe::run_native("CM0102 Rust Tweaks", opcoes, Box::new(move |cc| {
        let mut v = egui::Visuals::dark();
        v.panel_fill = FUNDO;
        v.window_fill = FUNDO;
        v.extreme_bg_color = Color32::from_rgb(0x0c, 0x12, 0x0f);
        v.selection.bg_fill = Color32::from_rgb(0x2d, 0x7a, 0x45);
        v.hyperlink_color = VERDE;
        cc.egui_ctx.set_visuals(v);
        // as dicas do "?" trazem explicações de várias linhas: mais largas que o padrão
        cc.egui_ctx.style_mut(|s| s.spacing.tooltip_width = 480.0);
        Ok(Box::new(App::new(pasta)))
    }))
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn grava_so_o_que_mudou() {
        let pasta = std::env::temp_dir().join(format!("tweaks_config_teste_{}", std::process::id()));
        std::fs::create_dir_all(&pasta).unwrap();
        std::fs::write(pasta.join("tweaks.cfg"), cfg::DISTRIBUIDO).unwrap();
        let mut a = App::new(pasta.clone());
        assert_eq!(a.mudancas(), 0);
        a.definir("log", "1");
        a.definir("unattached", "only_if_no_room");
        a.definir_idade(27, 1.5, -1);
        a.definir("sigma", "2500"); // igual ao que está: nada muda
        a.gravar();
        let novo = std::fs::read_to_string(pasta.join("tweaks.cfg")).unwrap();
        let (antes, depois): (Vec<&str>, Vec<&str>) = (cfg::DISTRIBUIDO.lines().collect(), novo.lines().collect());
        assert_eq!(depois.len(), antes.len() + 1); // só a age_27 entrou
        let mudadas: Vec<&&str> = depois.iter().filter(|l| !antes.contains(l)).collect();
        assert_eq!(mudadas.len(), 3, "{:?}", mudadas);
        assert!(novo.contains("age_27 = 1.5"));
        assert_eq!(std::fs::read_to_string(pasta.join("tweaks.cfg.bak")).unwrap(), cfg::DISTRIBUIDO);
        assert_eq!(a.mudancas(), 0);
        std::fs::remove_dir_all(&pasta).ok();
    }
}

#[cfg(test)]
mod testes_do_guia {
    #[test]
    fn toda_chave_tem_ajuda_nas_duas_linguas() {
        for c in super::CHAVES.iter().map(|c| c.nome).chain(["age_NN"]) {
            let (pt, en) = super::guia::ajuda(c);
            assert!(!pt.is_empty() && !en.is_empty(), "sem ajuda: {}", c);
        }
    }
}
