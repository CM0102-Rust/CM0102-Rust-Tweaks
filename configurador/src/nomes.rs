//! Os nomes por nação do banco da pasta do jogo (`Data/nation.dat`, `first_names.dat`, `second_names.dat`), para a aba
//! Nomes dos regens mostrar, país por país, de onde vêm os nomes hoje e o que muda com `regen_names = own`. A regra é a
//! de `database_pick_first_name_for_nation` 0x539c60 (ver `src/nomes.rs` da DLL).

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

/// De onde a nação tira os nomes no original (com 100 ou menos): a nação de referência da língua, um sorteio entre as da
/// mesma língua, ou a tabela do mundo inteiro.
pub fn de_onde(n: &Nacao, en: bool) -> String {
    match n.grupo {
        0 => if en { "any country (first name from the surnames)".into() } else { "qualquer país (primeiro nome dos sobrenomes)".into() },
        1..=32 => format!("{} {}", if en { "from" } else { "de" }, REFERENCIA[(n.grupo - 1) as usize]),
        _ => if en { "drawn, same language".into() } else { "sorteado, mesma língua".into() },
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
