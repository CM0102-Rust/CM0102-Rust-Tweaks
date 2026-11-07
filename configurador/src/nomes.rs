//! Os nomes por nação do banco da pasta do jogo (`Data/nation.dat`, `first_names.dat`, `second_names.dat`), para a aba
//! Nomes dos regens mostrar, país por país, de onde vêm os nomes hoje e o que muda com `regen_names = own`. A regra é a
//! de `database_pick_first_name_for_nation` 0x539c60 (ver `src/regens/nomes.rs` da DLL).

use std::path::Path;

/// A nação de referência de cada grupo de língua (nação +0x77, 1 a 32): as chaves fixas do exe que a 0x539c60 lê
/// (`g_key_nation_*` em 0x9cf204.., ver docs/CHAVES_DO_EXE.md do projeto CM0102). O 17 sorteia entre as duas Coreias;
/// o 0 e os acima de 32 não têm referência.
const REFERENCIA: [&str; 32] = [
    "Albania", "Saudi Arabia", "Russia", "China", "Croatia", "Czech Republic", "Holland", "England", "Finland", "France",
    "Germany", "Greece", "Israel", "Hungary", "Italy", "Japan", "South Korea / North Korea", "Poland", "Portugal",
    "Romania", "Russia", "Norway", "Croatia", "Spain", "Turkey", "Burma", "Malaysia", "Peru", "Slovakia", "Sweden",
    "Iceland", "Denmark",
];

pub struct Nacao {
    pub nome: String,
    pub primeiros: i32,
    pub segundos: i32,
    pub grupo: i8,
}

/// De onde a nação tira os nomes no original (com 100 ou menos): a nação de referência da língua (1 a 32), a tabela do
/// mundo inteiro (0) ou, sem referência (acima de 32), 40 sorteios entre TODOS os países atrás de um da mesma língua com
/// mais de 5 nomes; se nenhum sair, fica o último sorteado, de qualquer língua (0x539c60, `nacao_que_empresta_nomes`).
pub fn de_onde(n: &Nacao, todas: &[Nacao], en: bool) -> String {
    match n.grupo {
        0 => if en { "any country (first name from the surnames)".into() } else { "qualquer país (primeiro nome dos sobrenomes)".into() },
        1..=32 => format!("{} {}", if en { "from" } else { "de" }, REFERENCIA[(n.grupo - 1) as usize]),
        _ => {
            let mesmas: Vec<&str> = todas.iter().filter(|o| o.grupo == n.grupo && o.primeiros > 5).map(|o| o.nome.as_str()).collect();
            let p = 1.0 - (1.0 - mesmas.len() as f64 / todas.len().max(1) as f64).powi(40);
            let quem: Vec<&str> = mesmas.iter().map(|&o| if o == n.nome { if en { "itself" } else { "ele mesmo" } } else { o }).collect();
            if quem.is_empty() {
                if en { "any country".into() } else { "qualquer país".into() }
            } else if en {
                format!("{:.0}% {}; {:.0}% any country", 100.0 * p, quem.join(", "), 100.0 * (1.0 - p))
            } else {
                format!("{:.0}% {}; {:.0}% qualquer país", 100.0 * p, quem.join(", "), 100.0 * (1.0 - p))
            }
        }
    }
}

fn texto(b: &[u8]) -> String {
    let fim = b.iter().position(|&c| c == 0).unwrap_or(b.len());
    b[..fim].iter().map(|&c| c as char).collect() // latin-1
}

/// Lê o banco da pasta; `None` se faltar algum arquivo.
pub fn ler(pasta: &Path) -> Option<Vec<Nacao>> {
    let data = pasta.join("Data");
    let nacoes = std::fs::read(data.join("nation.dat")).ok()?;
    let contar = |arq: &str| -> Option<std::collections::HashMap<i32, i32>> {
        let d = std::fs::read(data.join(arq)).ok()?;
        let mut m = std::collections::HashMap::new();
        for r in d.chunks_exact(60) {
            *m.entry(i32::from_le_bytes(r[0x37..0x3b].try_into().unwrap())).or_insert(0) += 1;
        }
        Some(m)
    };
    let (p1, p2) = (contar("first_names.dat")?, contar("second_names.dat")?);
    let mut v: Vec<Nacao> = nacoes.chunks_exact(290).map(|r| {
        let id = i32::from_le_bytes(r[0..4].try_into().unwrap());
        Nacao { nome: texto(&r[4..55]), primeiros: *p1.get(&id).unwrap_or(&0), segundos: *p2.get(&id).unwrap_or(&0), grupo: r[0x77] as i8 }
    }).collect();
    v.sort_by(|a, b| a.nome.cmp(&b.nome));
    Some(v)
}
