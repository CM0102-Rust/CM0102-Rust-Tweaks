//! As partes da DLL e as assinaturas que cada uma confere no exe (FNV-1a de 64 bits, no cm0102.exe 3.9.68). Usado pela
//! DLL (`src/lib.rs`) e pelo configurador (`configurador/`), que confere o arquivo do exe da mesma maneira.

/// Uma parte da DLL e o que ela confere no exe antes de ligar (FNV-1a de 64 bits dos trechos no cm0102.exe 3.9.68):
/// - as **bordas**: o que o desvio usa (a entrada, a volta, as rotinas chamadas). Se não batem, a parte nunca liga;
/// - o **corpo**: a rotina original que a parte troca. Se só ele difere, o exe já tem outra modificação ali: como no resto
///   do projeto, fica a do exe (e o log diz), a menos que `override_exe_changes = 1` mande sobrescrever.
pub struct Parte {
    pub nome: &'static str,
    pub bordas: &'static [(usize, usize)],
    pub assinatura_das_bordas: u64,
    pub corpo: &'static [(usize, usize)],
    pub assinatura_do_corpo: u64,
}

/// O save: as entradas das três rotinas do contêiner e o começo das funções da CRT e do jogo que elas chamam; o corpo é
/// o contêiner inteiro (zipdir.cpp), como em save/src/lib.rs.
pub const SAVE: Parte = Parte {
    nome: "save over 2 GB",
    bordas: &[
        (0x9326e0, 5), (0x933080, 5), (0x933170, 5), (0x944c94, 32), (0x944c9f, 32), (0x944cf1, 32), (0x944cff, 32),
        (0x945a09, 32), (0x945b00, 32), (0x945bfe, 32), (0x945ce6, 32), (0x946a22, 32), (0x94af9e, 32), (0x94fded, 32),
        (0x94c552, 32), (0x90d130, 32), (0x5e8290, 32),
    ],
    assinatura_das_bordas: 0x1d251608b137c13d,
    corpo: &[(0x9326e0, 0x1180)],
    assinatura_do_corpo: 0x20103c4e062d80ac,
};

/// A idade: as bordas do bloco do sorteio da idade e do teto do CA na 0x7abac0 (a entrada 0x7ac58e..0x7ac59f, com o
/// `call sorteio(10000)` e o `add esp, 8`, e a volta em 0x7ac6f6; ramo das nações no jogo ou de 350 pessoas ou mais) e o corte
/// da idade para a disputa (0x7ace65, `cmp byte [esp+0x1c], 0x13`); o corpo é o bloco. Os exes do CLZ mudam o corpo
/// (14 a 17 e 20 a 25 nas faixas raras, 0x7ac5b1 e 0x7ac699).
pub const IDADE: Parte = Parte {
    nome: "regen ages",
    bordas: &[(0x7ac58e, 0x11), (0x7ac6f6, 4), (0x7ace65, 6)],
    assinatura_das_bordas: 0x8cf4d2529f52726b,
    corpo: &[(0x7ac58e, 0x7ac6f6 - 0x7ac58e)],
    assinatura_do_corpo: 0x117c76f665c5e095,
};

/// A idade nas nações fora do jogo com menos de 350 pessoas (+0x88, NumberStaff; 0x7abea0..0x7abef1): sorteio(pessoas) < sorteio(1000) dá 23 a 27,
/// senão 17 a 23; depois a fórmula do CA pela idade (que fica como está). Os exes do CLZ pulam esse ramo (`jmp` em
/// 0x7abeab, no corpo); sobrescrito, o desvio fica lá, mas o ramo continua sem rodar.
pub const IDADE_FORA: Parte = Parte {
    nome: "regen ages (nations not in the game)",
    bordas: &[(0x7abeb1, 6), (0x7abeea, 7)],
    assinatura_das_bordas: 0x3850462e62889e48,
    corpo: &[(0x7abea0, 0x51)],
    assinatura_do_corpo: 0x3c0e1bab273bfa24,
};

/// A disputa: a entrada da 0x7ad0e0, a chamada dela na 0x7abac0 e o começo das rotinas que a nova chama; o corpo é a
/// 0x7ad0e0 inteira. Os exes do CLZ mudam o corpo com o Regen Tweak de xeno/Saturn (elenco 45, 0x7ad1bd).
pub const DISPUTA: Parte = Parte {
    nome: "regens 0.1",
    bordas: &[(0x7ad0e0, 7), (0x7ace9b, 0x1a), (0x53d980, 32), (0x540a50, 32), (0x53c590, 32), (0x8ce990, 32), (0x90cfc0, 32)],
    assinatura_das_bordas: 0xb9c6540ed49c8bff,
    corpo: &[(0x7ad0e0, 0x140)],
    assinatura_do_corpo: 0xa59f8acea76c7152,
};

/// A aposentadoria do desempregado (`player_regen_check_unemployed_retirement`, 0x7a8050..0x7a8193, stdcall ret 8): a
/// entrada e o começo das rotinas que a nova chama (a lista de observados 0x842080 e o sorteio); o corpo é a rotina.
pub const INATIVOS: Parte = Parte {
    nome: "retire idle players",
    bordas: &[(0x7a8050, 5), (0x842080, 32), (0x90cfc0, 32)],
    assinatura_das_bordas: 0x2e5b640ea518fc61,
    corpo: &[(0x7a8050, 0x143)],
    assinatura_do_corpo: 0x286a5dd95fa250e1,
};

/// Quem vai à disputa (fim da 0x7abac0): na nação no jogo, só com menos de 19 anos (`jge` em 0x7ace6a, depois do `cmp` que a
/// IDADE ajusta) e 3 vezes em 4 (`sorteio(4)` em 0x7ace89..0x7ace9b); os outros ficam sem clube. Sem corpo à parte: o trecho é
/// a borda, e um exe que o mude não liga a parte.
pub const SEM_CLUBE: Parte = Parte {
    nome: "unattached",
    bordas: &[(0x7ace6a, 0x31)],
    assinatura_das_bordas: 0x78a11e46a6bf2458,
    corpo: &[],
    assinatura_do_corpo: 0xcbf29ce484222325,
};

/// Os casos que este projeto já identificou, pelo byte que cada um muda (os outros ficam com o endereço), com o nome do
/// patch no catálogo do NickPatcher (MiscPatches.zip e as opções do Patcher.cs; `tools/nickpatcher_vs_tweaks.py`).
pub const CONHECIDOS: [(&str, u32, &[u8]); 6] = [
    ("Regen tweak 2 (Saturn) / Regen fixes (NickPatcher): squad limit 45 in the regens' contest", 0x7ad1bf, &[0x2d]),
    ("RetirePatchV4: no retirement under 34", 0x7a5806, &[0xe9]),
    ("AlwaysRegenToNewGoodCountry: a PA 120+ regen moves to a strong nation", 0x7abcd7, &[0x8b, 0x46, 0x61]),
    ("Younger Regens (Saturn) / Regen fixes (NickPatcher): the age draw of nations not in the game skipped", 0x7abeab, &[0xe9]),
    ("Regen CA level reset (Saturn): regen age bands changed (14 to 17)", 0x7ac5b1, &[0x0e]),
    ("Regen CA level reset (Saturn): regen age bands changed (20 to 25)", 0x7ac699, &[0x14]),
];

/// Os jogadores cinzentos (`VIRTUAL_STAFF::create_virtual_player` 0x90e0a0): a entrada e o quadro da rotina (até a ficha
/// guardada em [esp+0x50], 0x90e10a), o ponto do desvio (0x90e6d4, as duas leituras antes de gravar o CA), a seleção
/// (0x53b900) e o `rand` da CRT; o corpo é a conta original do CA (0x90e5f9..0x90e6d4).
pub const CINZENTOS: Parte = Parte {
    nome: "grey players",
    bordas: &[(0x90e0a0, 0x70), (0x90e6d4, 8), (0x53b900, 32), (0x946a04, 32)],
    assinatura_das_bordas: 0x13e59acd609def6e,
    corpo: &[(0x90e5f9, 0x90e6d4 - 0x90e5f9)],
    assinatura_do_corpo: 0xcad8528356a77961,
};
