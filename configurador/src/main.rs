//! O configurador do CM0102-Rust-Tweaks: edita o `tweaks.cfg` ao lado do exe do jogo (docs/CONFIGURADOR.md). Um exe só,
//! sem instalador. O `.cfg` continua a única fonte: só o valor das linhas mudadas é trocado.
//! Uso: `tweaks-config.exe [pasta do jogo]` (sem argumento: a pasta do próprio configurador).

#![windows_subsystem = "windows"]

mod cfg;
mod chaves;
mod exe;
mod guia;

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
        };
        a.abrir_pasta();
        // só para as capturas de tela da documentação: a aba inicial e o idioma
        if let Ok(v) = std::env::var("TWEAKS_CONFIG_ABA") {
            if let Some(&(aba, _, _)) = v.parse::<usize>().ok().and_then(|i| ABAS.get(i)) {
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
        let resp = ui.add_enabled_ui(ativo, |ui| {
            ui.horizontal(|ui| match &c.tipo {
                Tipo::Liga => {
                    let mut on = chaves::ligado(nome, &valor);
                    let r = ui.checkbox(&mut on, rotulo);
                    if r.changed() {
                        novo = Some(if on { "1" } else { "0" }.into());
                    }
                    r
                }
                Tipo::Escolha(op) => {
                    ui.add_sized([260.0, 18.0], egui::Label::new(rotulo));
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
                    ui.add_sized([260.0, 18.0], egui::Label::new(rotulo));
                    let mut x: f64 = valor.parse().unwrap_or_else(|_| c.padrao.parse().unwrap_or(0.0));
                    let mut xb = x.clamp(barra.0, barra.1);
                    let r1 = ui.add(egui::Slider::new(&mut xb, barra.0..=barra.1).show_value(false));
                    if r1.changed() {
                        x = xb;
                    }
                    let passo = if *inteiro { 0.2 } else { ((barra.1 - barra.0) / 300.0).max(0.0001) };
                    let r2 = ui.add(egui::DragValue::new(&mut x).range(*min..=*max).speed(passo).max_decimals(if *inteiro { 0 } else { 4 }));
                    if r1.changed() || r2.changed() {
                        novo = Some(cfg::numero(if *inteiro { x.round() } else { x }));
                    }
                    r1.union(r2)
                }
            }).inner
        });
        if let Some(v) = novo {
            self.definir(nome, &v);
        }
        if resp.response.hovered() || resp.inner.hovered() || resp.inner.has_focus() {
            self.foco = Some(nome);
        }
        let (pt, en) = guia::ajuda(nome);
        resp.response.on_hover_text(if self.en { en } else { pt });
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
                ui.horizontal(|ui| {
                    ui.add_sized([190.0, 18.0], egui::Label::new(RichText::new(l.ajuste).monospace()));
                    bolinha(ui, cor(l.estado));
                    ui.add_sized([150.0, 18.0], egui::Label::new(RichText::new(nome_do_estado(l.estado, en)).color(cor(l.estado))));
                    ui.add(egui::Label::new(if en { &l.motivo.1 } else { &l.motivo.0 }).wrap());
                });
            }
        });
        App::cartao(ui, &self.t("Nação no jogo", "Nation in the game"), |ui| {
            ui.label(if en {
                "A nation in the game is one whose league you chose when creating the career, playable OR background: for regens both count the same (the game only checks that NationLeagueSelected isn't 0). A nation not in the game is one whose league wasn't chosen."
            } else {
                "Nação no jogo é aquela cuja liga você escolheu ao criar a carreira, jogável OU de fundo: para os regens as duas contam igual (o jogo só olha se o NationLeagueSelected não é 0). Fora do jogo é a nação cuja liga não foi escolhida."
            });
        });
    }

    fn aba_jogo(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(self.t("O save e a instalação", "The save and the install")).strong().size(15.0).color(VERDE));
        for k in ["save_over_2gb", "portable_folder", "clear_admin_flag", "override_exe_changes"] {
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
        ui.label(RichText::new(self.t("Arraste as barras. Embaixo, a idade e o teto do CA. O contorno azul é a distribuição depois de \"regens mais novos\".",
            "Drag the bars. Below: the age and the CA cap. The blue outline is the distribution after \"younger regens\".")).size(11.0).color(CINZA));
        // a idade escolhida
        let n = self.idade;
        let (mut chance, teto) = self.idade_de(n);
        let rotulo_teto = self.t("teto do CA", "CA cap");
        let mut mudou = None;
        ui.add_enabled_ui(ativo, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("age_{}:", n));
                let r = ui.add(egui::DragValue::new(&mut chance).range(0.0..=1000.0).speed(0.05).max_decimals(4));
                let mut tem_teto = teto >= 0;
                let mut t = if teto >= 0 { teto } else { 100 };
                let r2 = ui.checkbox(&mut tem_teto, rotulo_teto);
                let r3 = ui.add_enabled(tem_teto, egui::DragValue::new(&mut t).range(1..=200));
                if r.changed() || r2.changed() || r3.changed() {
                    mudou = Some((chance, if tem_teto { t } else { -1 }));
                }
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
        self.controle(ui, "retire_idle_players", true);
        let ativo = self.ligado("retire_idle_players");
        for k in ["idle_days", "idle_min_age", "idle_rep", "idle_rep_per_year", "protect_young_unemployed"] {
            self.controle(ui, k, ativo);
        }
        ui.add_space(8.0);
        let rotulo = self.t("Reputação de exemplo", "Example reputation");
        ui.horizontal(|ui| {
            ui.label(rotulo);
            ui.add(egui::Slider::new(&mut self.rep_exemplo, 500.0..=10000.0).step_by(100.0));
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

    /// O peso do clube para o jovem: a mesma conta de `peso()` em src/lib.rs.
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
        self.controle(ui, "rule", true);
        let ativo = self.ligado("rule");
        if !ativo {
            ui.colored_label(CINZA, self.t("O jogo usa a disputa do 3.9.68.", "The game uses 3.9.68's contest."));
        }
        let rotulo = self.t("Jovem de exemplo: PA", "Example youngster: PA");
        ui.horizontal(|ui| {
            ui.label(rotulo);
            ui.add(egui::Slider::new(&mut self.pa, 40.0..=200.0).step_by(1.0));
            ui.label("CA");
            ui.add(egui::Slider::new(&mut self.ca, 20.0..=150.0).step_by(1.0));
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
                    ui.add_sized([230.0, 16.0], egui::Label::new(if en { c.nome.1 } else { c.nome.0 }));
                    let (rr, pp) = ui.allocate_painter(Vec2::new(300.0, 14.0), Sense::hover());
                    pp.rect_filled(rr.rect, 2, FUNDO);
                    pp.rect_filled(Rect::from_min_size(rr.rect.min, Vec2::new(rr.rect.width() * ch as f32, rr.rect.height())), 2, if ativo { VERDE } else { CINZA });
                    ui.label(format!("{:.1}%", 100.0 * ch));
                });
            }
        });
        for k in ["target_k", "target_ca", "sigma", "sigma_below", "level_floor", "facilities_base", "facilities_k", "staff_min", "staff_max", "squad_limit", "squad_exp", "season_cap", "surprise", "outside_competitions", "temperature"] {
            self.controle(ui, k, ativo);
        }
        if self.valor("staff_max").parse::<f64>().unwrap_or(1.2) < self.valor("staff_min").parse::<f64>().unwrap_or(0.8) {
            ui.colored_label(AMARELO, self.t("staff_max está abaixo de staff_min.", "staff_max is below staff_min."));
        }
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
                        ui.label(c[11]);
                        ui.label(RichText::new(if c.len() >= 21 && !c[17].is_empty() { format!("{} ({}, {}, {})", c[17], c[18], c[19], c[20]) } else { "?".into() }).color(AMARELO));
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

    fn aba_registro(&mut self, ui: &mut egui::Ui) {
        self.historico(ui);
        ui.add_space(8.0);
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
        egui::SidePanel::left("abas").resizable(false).exact_width(160.0).show(ctx, |ui| {
            ui.add_space(8.0);
            for (aba, pt, en) in ABAS {
                let pior = CHAVES.iter().filter(|c| c.aba == aba).filter_map(|c| estados.iter().find(|l| l.ajuste == c.nome)).map(|l| l.estado)
                    .filter(|e| *e != Estado::Desligado).max();
                ui.horizontal(|ui| {
                    bolinha(ui, pior.map(cor).unwrap_or(Color32::TRANSPARENT));
                    if ui.selectable_label(self.aba == aba, RichText::new(if self.en { en } else { pt }).size(15.0)).clicked() {
                        self.aba = aba;
                    }
                });
                ui.add_space(4.0);
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
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| match self.aba {
                Aba::Inicio => self.aba_inicio(ui),
                Aba::Jogo => self.aba_jogo(ui),
                Aba::Idades => self.aba_idades(ui),
                Aba::Inativos => self.aba_inativos(ui),
                Aba::Disputa => self.aba_disputa(ui),
                Aba::Registro => self.aba_registro(ui),
                Aba::Cfg => self.aba_cfg(ui),
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
    for (aba, pt, e) in ABAS {
        let chaves: Vec<_> = CHAVES.iter().filter(|c| c.aba == aba).collect();
        if chaves.is_empty() {
            continue;
        }
        let _ = writeln!(s, "## {}\n", if en { e } else { pt });
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
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 820.0]).with_title("CM0102 Rust Tweaks"),
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
