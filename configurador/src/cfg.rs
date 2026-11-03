//! O `tweaks.cfg` como lista de linhas (docs/CONFIGURADOR.md §5): lido e gravado sem reconstruir o arquivo. Só o valor
//! das linhas mudadas troca; comentários, ordem, linhas em branco e chaves desconhecidas ficam. A última ocorrência de
//! uma chave vence, como em `ler_regras` da DLL.

/// O `.cfg` distribuído, embutido: a ordem das chaves, a ajuda e o arquivo novo quando a pasta não tem um.
pub const DISTRIBUIDO: &str = include_str!("../../tweaks.cfg");

pub struct Cfg {
    linhas: Vec<String>,
    eol: &'static str,
    termina_com_eol: bool,
}

/// A chave e o valor de uma linha `chave = valor  # comentário`, ou nada.
fn chave_valor(linha: &str) -> Option<(&str, &str)> {
    let codigo = linha.split('#').next().unwrap_or("");
    let (k, v) = codigo.split_once('=')?;
    let k = k.trim();
    (!k.is_empty()).then_some((k, v.trim()))
}

impl Cfg {
    pub fn ler(texto: &str) -> Cfg {
        let eol = if texto.contains("\r\n") { "\r\n" } else { "\n" };
        let termina_com_eol = texto.ends_with('\n');
        let corpo = texto.strip_suffix(eol).unwrap_or(texto.strip_suffix('\n').unwrap_or(texto));
        let linhas = if texto.is_empty() { Vec::new() } else { corpo.split(eol).map(str::to_string).collect() };
        Cfg { linhas, eol, termina_com_eol }
    }

    pub fn texto(&self) -> String {
        let mut s = self.linhas.join(self.eol);
        if self.termina_com_eol {
            s.push_str(self.eol);
        }
        s
    }

    fn posicao(&self, chave: &str) -> Option<usize> {
        self.linhas.iter().rposition(|l| chave_valor(l).is_some_and(|(k, _)| k == chave))
    }

    pub fn valor(&self, chave: &str) -> Option<String> {
        self.posicao(chave).and_then(|i| chave_valor(&self.linhas[i]).map(|(_, v)| v.to_string()))
    }

    /// Troca o valor de `chave`. Sem a linha: entra depois da chave anterior na ordem do `.cfg` distribuído que exista
    /// aqui, ou no fim, sob um comentário. (Quem chama decide se vale acrescentar: só se for diferente do padrão.)
    pub fn definir(&mut self, chave: &str, novo: &str) {
        if let Some(i) = self.posicao(chave) {
            let l = &self.linhas[i];
            let (codigo, comentario) = match l.find('#') {
                Some(p) => (&l[..p], &l[p..]),
                None => (l.as_str(), ""),
            };
            let igual = codigo.find('=').unwrap();
            let depois = &codigo[igual + 1..];
            let espaco_antes = &depois[..depois.len() - depois.trim_start().len()];
            let espaco_depois = &depois[depois.trim_end().len()..];
            let espaco_depois = if depois.trim().is_empty() { "" } else { espaco_depois };
            self.linhas[i] = format!("{}={}{}{}{}", &codigo[..igual], if espaco_antes.is_empty() { " " } else { espaco_antes }, novo, espaco_depois, comentario);
            return;
        }
        let nova = format!("{} = {}", chave, novo);
        // uma idade que o .cfg distribuído não lista: depois da maior idade abaixo dela
        if let Some(n) = chave.strip_prefix("age_").and_then(|n| n.parse::<u32>().ok()) {
            if let Some(i) = (14..n).rev().find_map(|m| self.posicao(&format!("age_{}", m))) {
                self.linhas.insert(i + 1, nova);
                return;
            }
        }
        let ordem: Vec<&str> = DISTRIBUIDO.lines().filter_map(chave_valor).map(|(k, _)| k).collect();
        let antes = ordem.iter().position(|&k| k == chave).map(|p| &ordem[..p]).unwrap_or(&[]);
        for k in antes.iter().rev() {
            if let Some(i) = self.posicao(k) {
                self.linhas.insert(i + 1, nova);
                return;
            }
        }
        if !self.linhas.iter().any(|l| l == "# added by the configurator") {
            self.linhas.push("# added by the configurator".into());
        }
        self.linhas.push(nova);
        self.termina_com_eol = true;
    }
}

/// A ajuda de uma chave: as linhas de comentário logo acima dela no `.cfg` distribuído (até uma linha em branco ou de
/// chave); sem nenhuma, a da chave anterior do mesmo bloco.
pub fn ajuda(chave: &str) -> String {
    let linhas: Vec<&str> = DISTRIBUIDO.lines().collect();
    let Some(i) = linhas.iter().position(|l| chave_valor(l).is_some_and(|(k, _)| k == chave)) else { return String::new() };
    let mut j = i;
    while j > 0 && chave_valor(linhas[j - 1]).is_some() && linhas[j - 1].trim_start().chars().next() != Some('#') {
        j -= 1; // chaves seguidas sem comentário entre elas (a tabela de idades): a ajuda do bloco
    }
    let mut ajuda: Vec<&str> = Vec::new();
    let mut k = j;
    while k > 0 {
        let l = linhas[k - 1].trim_start();
        if let Some(c) = l.strip_prefix('#') {
            if c.trim_start().starts_with("----") {
                break;
            }
            ajuda.push(c.strip_prefix(' ').unwrap_or(c));
            k -= 1;
        } else {
            break;
        }
    }
    ajuda.reverse();
    ajuda.join("\n")
}

/// Número no formato do `.cfg`: ponto decimal, sem zeros sobrando, até 4 casas.
pub fn numero(x: f64) -> String {
    let s = format!("{:.4}", x);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" { "0".into() } else { s.into() }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn ida_e_volta_sem_mudar_nada() {
        assert_eq!(Cfg::ler(DISTRIBUIDO).texto(), DISTRIBUIDO);
        let crlf = DISTRIBUIDO.replace("\r\n", "\n").replace('\n', "\r\n");
        assert_eq!(Cfg::ler(&crlf).texto(), crlf);
    }

    #[test]
    fn troca_so_o_valor() {
        let mut c = Cfg::ler("a = 1   # um\nb=2\n");
        c.definir("a", "5");
        c.definir("b", "7");
        assert_eq!(c.texto(), "a = 5   # um\nb= 7\n");
        assert_eq!(c.valor("a").as_deref(), Some("5"));
    }

    #[test]
    fn a_ultima_vence() {
        let mut c = Cfg::ler("log = 0\nlog = 1\n");
        assert_eq!(c.valor("log").as_deref(), Some("1"));
        c.definir("log", "0");
        assert_eq!(c.texto(), "log = 0\nlog = 0\n");
    }

    #[test]
    fn chave_nova_na_ordem() {
        let mut c = Cfg::ler("age_26 = 0.04\nlog = 0\n");
        c.definir("age_27", "1");
        assert_eq!(c.texto(), "age_26 = 0.04\nage_27 = 1\nlog = 0\n");
    }

    #[test]
    fn numeros() {
        assert_eq!(numero(0.0025), "0.0025");
        assert_eq!(numero(50.0), "50");
        assert_eq!(numero(29.11), "29.11");
    }

    #[test]
    fn ajuda_das_idades_e_do_log() {
        assert!(ajuda("age_20").contains("age_NN"));
        assert!(!ajuda("log").is_empty());
    }
}
