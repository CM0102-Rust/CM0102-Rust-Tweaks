//! O estado de cada ajuste num exe, sem rodar o jogo (docs/CONFIGURADOR.md §6, docs/COMPATIBILIDADE.md): as mesmas
//! conferências da DLL, lendo o arquivo em vez da memória, mais a pasta (LaMOHatesCD.dll, NickDLL.dll e o "resolucao"
//! dela). As tabelas são as da DLL (`src/partes.rs`, `src/trocas.rs`), sem cópia.

#[path = "../../src/partes.rs"]
#[allow(dead_code)]
mod partes;
#[path = "../../src/trocas.rs"]
mod trocas;

use partes::*;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Estado {
    Desligado,
    Compativel,
    FicaComOutro,
    Sobrescreve,
    Desconhecido,
}

/// Um ajuste no exe: o estado e o motivo (em português e em inglês).
pub struct Linha {
    pub ajuste: &'static str,
    pub estado: Estado,
    pub motivo: (String, String),
}

pub struct Exe {
    pub caminho: PathBuf,
    dados: Vec<u8>,
    secoes: Vec<(u32, u32, u32)>, // (VA relativo, tamanho na memória, deslocamento no arquivo)
    pub conhecidos: Vec<&'static str>,
    pub lamo: bool,
    pub nick: bool,
    pub nick_resolucao: Option<String>,
}

impl Exe {
    pub fn abrir(caminho: &Path) -> Option<Exe> {
        let dados = std::fs::read(caminho).ok()?;
        let rd16 = |o: usize| -> Option<u32> { Some(u16::from_le_bytes(dados.get(o..o + 2)?.try_into().ok()?) as u32) };
        let rd32 = |o: usize| -> Option<u32> { Some(u32::from_le_bytes(dados.get(o..o + 4)?.try_into().ok()?)) };
        let nt = rd32(0x3c)? as usize;
        let n = rd16(nt + 6)?;
        let opc = rd16(nt + 20)?;
        let mut secoes = Vec::new();
        for i in 0..n as usize {
            let s = nt + 24 + opc as usize + 40 * i;
            secoes.push((rd32(s + 12)?, rd32(s + 8)?.max(rd32(s + 16)?), rd32(s + 20)?));
        }
        let pasta = caminho.parent().unwrap_or(Path::new("."));
        let nick_dll = std::fs::read(pasta.join("NickDLL.dll")).ok();
        let nick_resolucao = nick_dll.as_ref().filter(|d| d.windows(19).any(|w| w == b"layout Nick Patcher")).and_then(|_| {
            let cfg = std::fs::read_to_string(pasta.join("NickDLL_config.json")).ok()?;
            let v = cfg.split("\"resolucao\"").nth(1)?.split('"').nth(1)?.trim().to_string();
            Some(v)
        });
        let mut e = Exe {
            caminho: caminho.to_path_buf(),
            dados,
            secoes,
            conhecidos: Vec::new(),
            lamo: pasta.join("LaMOHatesCD.dll").exists(),
            nick: nick_dll.is_some(),
            nick_resolucao,
        };
        e.conhecidos = CONHECIDOS.iter().filter(|&&(_, va, b)| e.bytes(va as usize, b.len()) == Some(b)).map(|c| c.0).collect();
        Some(e)
    }

    fn bytes(&self, va: usize, n: usize) -> Option<&[u8]> {
        let rva = va.checked_sub(0x400000)? as u32;
        let &(base, _, arq) = self.secoes.iter().find(|&&(b, t, _)| rva >= b && rva < b + t)?;
        let o = (arq + rva - base) as usize;
        self.dados.get(o..o + n)
    }

    fn assinatura(&self, trechos: &[(usize, usize)]) -> Option<u64> {
        let mut h = 0xcbf29ce484222325u64;
        for &(va, n) in trechos {
            for &b in self.bytes(va, n)? {
                h = (h ^ b as u64).wrapping_mul(0x100000001b3);
            }
        }
        Some(h)
    }

    /// Bordas diferentes: não serve; corpo igual: liga; corpo diferente: fica com o exe, ou sobrescreve com override.
    fn parte(&self, p: &Parte, sobrescrever: bool) -> (Estado, (String, String)) {
        if self.assinatura(p.bordas) != Some(p.assinatura_das_bordas) {
            return (Estado::Desconhecido, (
                "o exe difere do 3.9.68 onde o ajuste se liga; fica de fora".into(),
                "the exe differs from 3.9.68 where this hooks in; left out".into()));
        }
        if self.assinatura(p.corpo) == Some(p.assinatura_do_corpo) {
            return (Estado::Compativel, (String::new(), String::new()));
        }
        let nome = self.nome_da_mudanca(p);
        match (nome, sobrescrever) {
            (Some(n), false) => (Estado::FicaComOutro, (format!("o exe já muda esta rotina ({}); fica a do exe", n), format!("the exe already changes this routine ({}); the exe's stays", n))),
            (Some(n), true) => (Estado::Sobrescreve, (format!("troca a mudança do exe: {}", n), format!("replaces the exe's change: {}", n))),
            (None, false) => (Estado::Desconhecido, ("o exe muda esta rotina, sem nome conhecido; fica a do exe".into(), "the exe changes this routine, not a known patch; the exe's stays".into())),
            (None, true) => (Estado::Desconhecido, ("troca uma mudança do exe que não conhecemos".into(), "replaces a change in the exe we don't know".into())),
        }
    }

    fn nome_da_mudanca(&self, p: &Parte) -> Option<&'static str> {
        let tem = |va: u32| self.conhecidos.iter().any(|c| CONHECIDOS.iter().any(|k| k.0 == *c && k.1 == va));
        match p.nome {
            n if n == DISPUTA.nome && tem(0x7ad1bf) => Some("Regen tweak 2 / Regen fixes"),
            n if n == IDADE.nome && (tem(0x7ac5b1) || tem(0x7ac699)) => Some("Regen CA level reset"),
            n if n == IDADE_FORA.nome && tem(0x7abeab) => Some("Younger Regens / Regen fixes"),
            _ => None,
        }
    }

    /// (como no 3.9.68, já trocados, outros) de uma lista de trocas.
    fn trocas(&self, lista: &[(usize, &[u8], &[u8])]) -> (usize, usize, usize) {
        let (mut a, mut d, mut o) = (0, 0, 0);
        for &(va, antes, depois) in lista {
            match self.bytes(va, antes.len().max(depois.len())) {
                Some(b) if &b[..antes.len()] == antes => a += 1,
                Some(b) if &b[..depois.len()] == depois => d += 1,
                _ => o += 1,
            }
        }
        (a, d, o)
    }

    pub fn eh_original(&self) -> bool {
        [SAVE, IDADE, IDADE_FORA, DISPUTA, INATIVOS, SEM_CLUBE].iter().all(|p| self.assinatura(p.corpo) == Some(p.assinatura_do_corpo))
    }

    /// As linhas do Início, para os valores do `.cfg` (`ligado(chave)` diz se a chave está ligada).
    pub fn estados(&self, ligado: &dyn Fn(&str) -> bool, sobrescrever: bool) -> Vec<Linha> {
        let mut v = Vec::new();
        let mut push = |ajuste: &'static str, on: bool, (e, m): (Estado, (String, String))| {
            v.push(Linha { ajuste, estado: if on { e } else { Estado::Desligado }, motivo: m });
        };
        let ok = || (Estado::Compativel, (String::new(), String::new()));
        push("save_over_2gb", ligado("save_over_2gb"), self.parte(&SAVE, sobrescrever));
        push("portable_folder", ligado("portable_folder"), ok());
        push("clear_admin_flag", ligado("clear_admin_flag"), if self.nick { ok() } else {
            (Estado::Compativel, ("sem NickDLL.dll na pasta: não há marca para tirar".into(), "no NickDLL.dll here: no flag to clear".into()))
        });
        // a resolução: a NickDLL com resolução escolhida fica com ela; senão a lista inteira, completando o que falta
        let (a, d, o) = self.trocas(&trocas::RESOLUCAO);
        let res_nick = self.nick_resolucao.as_ref().filter(|v| v.as_str() != "800x600");
        let res = if let Some(r) = res_nick {
            (Estado::FicaComOutro, (format!("a NickDLL escolhe a resolução aqui (\"resolucao\": \"{}\"); fica com ela", r), format!("NickDLL sets the resolution here (\"resolucao\": \"{}\"); left to it", r)))
        } else if o > 0 {
            (Estado::Desconhecido, (format!("{} de {} trechos têm outro patch: nada trocado", o, a + d + o), format!("{} of {} places hold another patch: nothing changed", o, a + d + o)))
        } else if d > 0 {
            (Estado::Compativel, (format!("o exe já tem {} de {} trechos; a DLL completa os outros", d, a + d), format!("the exe already has {} of {} places; the DLL completes the rest", d, a + d)))
        } else {
            ok()
        };
        let res_vale = ligado("resolution") && matches!(res.0, Estado::Compativel);
        push("resolution", ligado("resolution"), res);
        let area = self.bytes(0x402ac0, 6) == Some(&[0x66, 0x83, 0x7c, 0x24, 0x38, 0xff][..]);
        let trava_on = ligado("resolution") || ligado("video") || res_nick.is_some();
        push("match report (30 rows)", trava_on, if area { ok() } else {
            (Estado::Desconhecido, ("o Area::init difere do 3.9.68".into(), "Area::init differs from 3.9.68".into()))
        });
        let importa = self.dados.windows(22).any(|w| w == b"ChangeDisplaySettingsA");
        push("video", ligado("video") || res_vale, if !importa {
            (Estado::Desconhecido, ("o exe não importa ChangeDisplaySettingsA".into(), "the exe doesn't import ChangeDisplaySettingsA".into()))
        } else if self.nick {
            (Estado::Desconhecido, ("não testado com a NickDLL (deve resolver o \"16 bit colour\" em janela)".into(), "not tested with NickDLL (should fix \"16 bit colour\" in a window)".into()))
        } else if res_vale && !ligado("video") {
            (Estado::Compativel, ("ligado pela resolução".into(), "turned on by the resolution".into()))
        } else {
            ok()
        });
        let (ca, cd, co) = self.trocas(&trocas::CD);
        push("no_cd", ligado("no_cd"), if self.lamo {
            (Estado::FicaComOutro, ("a LaMOHatesCD.dll cuida do CD; fica com ela".into(), "LaMOHatesCD.dll takes care of the CD; left to it".into()))
        } else {
            (Estado::Compativel, (format!("{} trocados agora, {} já no exe{}", ca, cd, if co > 0 { format!(", {} com outro patch", co) } else { String::new() }),
                format!("{} changed now, {} already in the exe{}", ca, cd, if co > 0 { format!(", {} with another patch", co) } else { String::new() })))
        });
        push("ages", ligado("ages"), self.parte(&IDADE, sobrescrever));
        push("ages_out_of_game", ligado("ages_out_of_game"), self.parte(&IDADE_FORA, sobrescrever));
        push("unattached", ligado("unattached"), self.parte(&SEM_CLUBE, sobrescrever));
        let mut idle = self.parte(&INATIVOS, sobrescrever);
        if idle.0 == Estado::Compativel && self.conhecidos.iter().any(|c| c.contains("RetirePatchV4")) {
            idle.1 = ("o RetirePatchV4 do exe não deixa quem tem menos de 34 chegar à regra".into(), "the exe's RetirePatchV4 lets nobody under 34 reach the rule".into());
        }
        push("retire_idle_players", ligado("retire_idle_players"), idle);
        push("rule", ligado("rule"), self.parte(&DISPUTA, sobrescrever));
        push("log", ligado("log"), if self.lamo || self.nick {
            (Estado::Sobrescreve, ("a caixa de mensagem (0x5e8290) também é desviada pela LaMOHatesCD/NickDLL; o registro encadeia (a confirmar no jogo)".into(),
                "the message box (0x5e8290) is also hooked by LaMOHatesCD/NickDLL; the log chains (to confirm in the game)".into()))
        } else { ok() });
        v
    }
}
