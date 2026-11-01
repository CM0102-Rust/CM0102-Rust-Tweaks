//! CM0102-Rust-Tweaks: a `ddraw.dll` de ajustes do CM 01/02 3.9.68, sem alterar o exe (hoje: o save acima de 2 GB, a idade
//! e a distribuição dos regens).
//!
//! Entra no jogo como `ddraw.dll`, como a do save2gb, e lê `tweaks.cfg` ao lado do exe. Cada parte só liga se a
//! assinatura do código de que depende bater com o 3.9.68:
//! - `save_over_2gb = 1` (padrão): as três rotinas do contêiner do save (`zopen` 0x9326e0, `zseek` 0x933080, `ztell`
//!   0x933170) vão para as de `src/conteiner.rs`, como na DLL do save2gb (`TRECHOS_DO_SAVE`);
//! - `rule = new`: `PLAYER_REGEN::pick_best_club_for_player` (0x7ad0e0) vai para `escolher_clube` (`TRECHOS`).
//! O `DirectDrawCreate` é repassado a
//! `ddraw_chain.dll` na pasta do jogo, se existir, senão à `ddraw.dll` do Windows. O registro vai para `tweaks.log`.
//!
//! A regra original (docs/DISTRIBUICAO_DOS_REGENS.md): cada clube do país com elenco abaixo de 50 recebe uma nota
//! (0x7ad220) e ganha o maior sorteio(nota). A nova: os mesmos clubes elegíveis, mas o clube é sorteado na proporção
//! do peso = nível × instalações × comissão × elenco, à potência da temperatura, com um teto de jovens por clube nos
//! últimos 365 dias. As mesmas contas de `tools/calibrar.py` (`peso_novo`).

#![allow(linker_messages)]

use core::ffi::c_void;

// O save acima de 2 GB (o do save2gb): as três rotinas do contêiner.
mod conteiner;

/// Os contadores de ramo do projeto: aqui não contam nada.
mod comparacao {
    pub fn ramo(_: &'static str) {}
}

mod registro {
    use std::io::Write;
    pub fn linha(texto: &str) {
        let caminho = std::env::current_exe().map(|p| p.with_file_name("tweaks.log")).unwrap_or_else(|_| "tweaks.log".into());
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(caminho) {
            let _ = writeln!(f, "{}", texto);
        }
    }
}

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryW(nome: *const u16) -> *mut c_void;
    fn GetProcAddress(modulo: *mut c_void, nome: *const u8) -> *mut c_void;
    fn GetSystemDirectoryW(buf: *mut u16, tam: u32) -> u32;
    fn VirtualProtect(end: *mut c_void, tam: usize, nova: u32, antiga: *mut u32) -> i32;
    fn FlushInstructionCache(processo: *mut c_void, end: *const c_void, tam: usize) -> i32;
    fn GetCurrentProcess() -> *mut c_void;
    fn VirtualAlloc(end: *mut c_void, tam: usize, tipo: u32, protecao: u32) -> *mut c_void;
    fn AddVectoredExceptionHandler(primeiro: u32, tratador: unsafe extern "system" fn(*mut usize) -> i32) -> *mut c_void;
}

/// Uma parte da DLL e o que ela confere no exe antes de ligar (FNV-1a de 64 bits dos trechos no cm0102.exe 3.9.68):
/// - as **bordas**: o que o desvio usa (a entrada, a volta, as rotinas chamadas). Se não batem, a parte nunca liga;
/// - o **corpo**: a rotina original que a parte troca. Se só ele difere, o exe já tem outra modificação ali: como no resto
///   do projeto, fica a do exe (e o log diz), a menos que `override_exe_changes = 1` mande sobrescrever.
struct Parte {
    nome: &'static str,
    bordas: &'static [(usize, usize)],
    assinatura_das_bordas: u64,
    corpo: &'static [(usize, usize)],
    assinatura_do_corpo: u64,
}

/// O save: as entradas das três rotinas do contêiner e o começo das funções da CRT e do jogo que elas chamam; o corpo é
/// o contêiner inteiro (zipdir.cpp), como em save/src/lib.rs.
const SAVE: Parte = Parte {
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
const IDADE: Parte = Parte {
    nome: "regen ages",
    bordas: &[(0x7ac58e, 0x11), (0x7ac6f6, 4), (0x7ace65, 6)],
    assinatura_das_bordas: 0x8cf4d2529f52726b,
    corpo: &[(0x7ac58e, 0x7ac6f6 - 0x7ac58e)],
    assinatura_do_corpo: 0x117c76f665c5e095,
};

/// A idade nas nações fora do jogo com menos de 350 pessoas (+0x88, NumberStaff; 0x7abea0..0x7abef1): sorteio(pessoas) < sorteio(1000) dá 23 a 27,
/// senão 17 a 23; depois a fórmula do CA pela idade (que fica como está). Os exes do CLZ pulam esse ramo (`jmp` em
/// 0x7abeab, no corpo); sobrescrito, o desvio fica lá, mas o ramo continua sem rodar.
const IDADE_FORA: Parte = Parte {
    nome: "regen ages (nations not in the game)",
    bordas: &[(0x7abeb1, 6), (0x7abeea, 7)],
    assinatura_das_bordas: 0x3850462e62889e48,
    corpo: &[(0x7abea0, 0x51)],
    assinatura_do_corpo: 0x3c0e1bab273bfa24,
};

/// A disputa: a entrada da 0x7ad0e0, a chamada dela na 0x7abac0 e o começo das rotinas que a nova chama; o corpo é a
/// 0x7ad0e0 inteira. Os exes do CLZ mudam o corpo com o Regen Tweak de xeno/Saturn (elenco 45, 0x7ad1bd).
const DISPUTA: Parte = Parte {
    nome: "regens 0.1",
    bordas: &[(0x7ad0e0, 7), (0x7ace9b, 0x1a), (0x53d980, 32), (0x540a50, 32), (0x53c590, 32), (0x8ce990, 32), (0x90cfc0, 32)],
    assinatura_das_bordas: 0xb9c6540ed49c8bff,
    corpo: &[(0x7ad0e0, 0x140)],
    assinatura_do_corpo: 0xa59f8acea76c7152,
};

/// A aposentadoria do desempregado (`player_regen_check_unemployed_retirement`, 0x7a8050..0x7a8193, stdcall ret 8): a
/// entrada e o começo das rotinas que a nova chama (a lista de observados 0x842080 e o sorteio); o corpo é a rotina.
const INATIVOS: Parte = Parte {
    nome: "retire idle players",
    bordas: &[(0x7a8050, 5), (0x842080, 32), (0x90cfc0, 32)],
    assinatura_das_bordas: 0x2e5b640ea518fc61,
    corpo: &[(0x7a8050, 0x143)],
    assinatura_do_corpo: 0x286a5dd95fa250e1,
};
/// Pode ligar a parte? Escreve no log o motivo quando não liga, ou quando sobrescreve uma modificação do exe.
unsafe fn pode_ligar(p: &Parte, sobrescrever: bool) -> bool {
    let b = assinatura(p.bordas);
    if b != p.assinatura_das_bordas {
        registro::linha(&format!("{}: this exe differs from 3.9.68 where this part hooks in (signature {:016x}); nothing was changed", p.nome, b));
        return false;
    }
    let c = assinatura(p.corpo);
    if c == p.assinatura_do_corpo {
        return true;
    }
    // o Regen Tweak de xeno/Saturn (elenco 45 em 0x7ad1bd) é o caso conhecido; nomeá-lo ajuda quem lê o log
    let conhecido = if p.nome == DISPUTA.nome && *(0x7ad1bf as *const u8) == 0x2d { " (the Regen Tweak by xeno/Saturn: squad limit 45)" } else { "" };
    if sobrescrever {
        registro::linha(&format!("{}: this exe already changes this routine{} (signature {:016x}); overriding it (override_exe_changes = 1)", p.nome, conhecido, c));
        true
    } else {
        registro::linha(&format!("{}: this exe already changes this routine{} (signature {:016x}); keeping the exe's own routine (override_exe_changes = 0)", p.nome, conhecido, c));
        false
    }
}
/// As idades que o `.cfg` aceita (`age_14` a `age_35`).
const IDADE_MIN: usize = 14;
const IDADES: usize = 22;

unsafe fn assinatura(trechos: &[(usize, usize)]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &(va, n) in trechos {
        for i in 0..n {
            h = (h ^ *((va + i) as *const u8) as u64).wrapping_mul(0x100000001b3);
        }
    }
    h
}

/// Salto (E9) da entrada da rotina original para a versão em Rust.
unsafe fn desviar(de: usize, para: usize) {
    (*core::ptr::addr_of_mut!(DESVIOS)).push((de, para));
    let mut antiga = 0u32;
    VirtualProtect(de as *mut c_void, 5, 0x40, &mut antiga);
    *(de as *mut u8) = 0xe9;
    core::ptr::write_unaligned((de + 1) as *mut i32, para.wrapping_sub(de + 5) as i32);
    VirtualProtect(de as *mut c_void, 5, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), de as *const c_void, 5);
}

// ---------- diagnóstico (só com log = 1) ----------

static mut FALHAS: u32 = 0;
/// Os desvios instalados: (endereço no exe, destino), para conferir depois que as outras DLLs do exe carregaram.
static mut DESVIOS: Vec<(usize, usize)> = Vec::new();

/// A primeira violação de acesso (até 3) vai para o log com o endereço, os registradores e o topo da pilha, antes de o
/// jogo cair: sem isso, uma queda num exe com outra DLL (a NickDLL do CLZ) não deixa rastro.
unsafe extern "system" fn ao_falhar(ponteiros: *mut usize) -> i32 {
    let registro_da_excecao = *ponteiros;
    let contexto = *ponteiros.add(1);
    let codigo = *(registro_da_excecao as *const u32);
    if codigo == 0xc000_0005 && FALHAS < 3 {
        FALHAS += 1;
        let r = |o: usize| *((contexto + o) as *const u32);
        let alvo = *((registro_da_excecao + 0x18) as *const u32);
        let modo = *((registro_da_excecao + 0x14) as *const u32);
        let esp = r(0xc4) as usize;
        let pilha: Vec<String> = (0..16).map(|k| format!("{:08x}", core::ptr::read_volatile((esp + 4 * k) as *const u32))).collect();
        registro::linha(&format!("crash\t{}\teip {:08x} {} {:08x}\teax {:08x} ebx {:08x} ecx {:08x} edx {:08x} esi {:08x} edi {:08x} ebp {:08x} esp {:08x}\tdll {:08x}\tstack {}",
            data_do_jogo(), r(0xb8), if modo == 1 { "writing" } else { "reading" }, alvo, r(0xb0), r(0xa4), r(0xac), r(0xa8), r(0xa0), r(0x9c), r(0xb4), r(0xc4),
            ao_falhar as *const () as usize, pilha.join(" ")));
    }
    0 // EXCEPTION_CONTINUE_SEARCH: o jogo segue como seguiria
}

/// No primeiro DirectDrawCreate, quando as DLLs do exe já carregaram: cada desvio continua no lugar?
unsafe fn conferir_desvios() {
    for &(de, para) in (*core::ptr::addr_of!(DESVIOS)).iter() {
        let ok = *(de as *const u8) == 0xe9 && (core::ptr::read_unaligned((de + 1) as *const i32) as usize).wrapping_add(de + 5) == para;
        if !ok {
            let b: Vec<String> = (0..8).map(|k| format!("{:02x}", *((de + k) as *const u8))).collect();
            registro::linha(&format!("check: the hook at {:08x} was overwritten after loading ({}); another DLL changed it", de, b.join(" ")));
        }
    }
    registro::linha(&format!("check: {} hooks checked after the exe's DLLs loaded", (*core::ptr::addr_of!(DESVIOS)).len()));
}
// ---------- as regras (tweaks.cfg) ----------

#[derive(Clone, Copy)]
struct Regras {
    /// override_exe_changes: sobrescrever a rotina que o exe já alterou (só o corpo; as bordas sempre têm de bater).
    sobrescrever: bool,
    save: bool,
    idades: bool,
    idades_fora: bool,
    /// O peso (%) e o teto do CA (−1: sem teto) de cada idade, de IDADE_MIN em diante.
    peso_da_idade: [f64; IDADES],
    teto_da_idade: [i16; IDADES],
    idade_max_disputa: i32,
    pular_teto: f64,
    /// O jovem regen sorteado com `mais_novo_a_partir` anos ou mais fica `mais_novo_em` anos mais novo (0: não): a ideia de
    /// petermf23 (item 14 do mod das divisões polonesas), 22 anos ou mais ficam 6 anos mais novos.
    mais_novo_a_partir: i32,
    mais_novo_em: i32,
    novo: bool,
    alvo_k: f64,
    alvo_ca: f64,
    sigma: f64,
    /// sigma_below: a largura da curva para o clube menor que o alvo (0: a mesma de cima).
    sigma_abaixo: f64,
    piso_nivel: f64,
    inst_base: f64,
    inst_k: f64,
    com_min: f64,
    com_max: f64,
    elenco_teto: f64,
    elenco_exp: f64,
    teto_temporada: f64,
    /// retire_idle_players: a aposentadoria do desempregado com as chaves abaixo (a 0x7a8050 trocada).
    inativos: bool,
    /// idle_days, idle_min_age, idle_rep, idle_rep_per_year: com idle_min_age anos ou mais e sem clube há mais de idle_days
    /// dias, aposenta com sorteio(reputação atual) < idle_rep + idle_rep_per_year × (idade − idle_min_age).
    inativo_dias: i32,
    inativo_idade: i32,
    inativo_rep: f64,
    inativo_rep_por_ano: f64,
    /// protect_young_unemployed: abaixo dessa idade a regra do desempregado não aposenta jogador (0: sem proteção).
    proteger_jovens: i32,
    /// surprise: % dos jovens sorteados sem o fator do nível, só entre clubes de competição que roda no jogo.
    surpresa: f64,
    temperatura: f64,
    registrar: bool,
}

// As três chaves de ligar começam desligadas: sem o tweaks.cfg (ou sem a linha), a DLL não muda nada no jogo.
const PADRAO: Regras = Regras {
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
    proteger_jovens: 23,
    surpresa: 15.0,
    temperatura: 1.0,
    registrar: false,
};

static mut REGRAS: Regras = PADRAO;

fn ler_regras() -> Regras {
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
        if k == "save_over_2gb" {
            r.save = v == "1";
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
            "temperature" => r.temperatura = x,
            "log" => r.registrar = x != 0.0,
            "contest_max_age" => r.idade_max_disputa = (x as i32).clamp(0, 126),
            "ca_cap_skip" => r.pular_teto = x,
            "younger_from_age" => r.mais_novo_a_partir = x as i32,
            "younger_by" => r.mais_novo_em = (x as i32).max(0),
            _ => {}
        }
    }
    r
}

// ---------- o jogo ----------

unsafe fn d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}
unsafe fn w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}
unsafe fn b(a: usize) -> i8 {
    *(a as *const i8)
}

/// sorteio(n) do jogo (0x90cfc0): 0..n−1.
unsafe fn sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O jovem com a segunda nacionalidade (+0x1e) basca só vai para os clubes bascos (como a 0x7ad0e0).
unsafe fn vetado(jogador: usize, clube: usize) -> bool {
    let e = d(jogador + 0x1e);
    if e == 0 || d(e) != d(0x9cf424) {
        return false;
    }
    let id = d(clube);
    ![0x9d0120usize, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124].iter().any(|&g| d(g) == id)
}

/// Os filtros da 0x7ad0e0: não estrangeiro (0x53d980), não é o time B (0x540a50), não vetado, pode chegar ao jogador
/// (0x8ce990). Devolve o elenco (0x53c590) do clube elegível.
unsafe fn elegivel(jogador: usize, clube: usize) -> Option<i32> {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let mut eh_a = 0u32;
    if estrangeiro(jogador, d(clube + 0x53), 1) != 0 || (parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0) || vetado(jogador, clube) {
        return None;
    }
    let mut motivo = 0i16;
    if pode(d(0xde1f54), jogador, clube, &mut motivo, 0) == 0 {
        return None;
    }
    Some(elenco_de(clube, 0, 0) as i8 as i32)
}

/// O melhor "Working with Youngsters" (+0x22 da ficha de não jogador +0x69) entre o técnico (+0xcf) e os 5 treinadores
/// (+0x19f); sem ninguém, 1.
unsafe fn melhor_com_jovens(clube: usize) -> i32 {
    let mut m = 1;
    for p in core::iter::once(d(clube + 0xcf)).chain((0..5).map(|k| d(clube + 0x19f + 4 * k))) {
        if p != 0 && d(p + 0x69) != 0 {
            m = m.max(b(d(p + 0x69) + 0x22) as i32);
        }
    }
    m
}

/// Regens (bit 0x40 do +0xb do registro de 79 bytes) no elenco (+0xd7) que entraram no clube (+0x3e dia, +0x40 ano)
/// nos últimos 365 dias.
unsafe fn recebidos_no_ano(clube: usize) -> i32 {
    let (dia, ano) = (w(0xae2c90) as i32, w(0xae2c92) as i32);
    (0..50usize)
        .map(|k| d(clube + 0xd7 + 4 * k))
        .filter(|&p| p != 0 && *((d(0xae2d0c) + 79 * d(p) + 0xb) as *const u8) & 0x40 != 0)
        .filter(|&p| (ano - w(p + 0x40) as i32) * 365 + dia - (w(p + 0x3e) as i32) < 365)
        .count() as i32
}

/// O clube joga uma competição que roda no jogo (a divisão +0x57 com `[[0xadadfc] + 4 × id]`, como na 0x7ad220)?
unsafe fn em_competicao(clube: usize) -> bool {
    let comp = d(clube + 0x57);
    comp != 0 && d(d(0xadadfc) + 4 * d(comp)) != 0
}

/// O peso do clube para o jovem (o mesmo de `peso_novo` em calibrar.py). Na surpresa, o nível não conta e só entram os
/// clubes de competição que roda no jogo.
unsafe fn peso(r: &Regras, pa: f64, ca: f64, clube: usize, elenco: i32, surpresa: bool) -> f64 {
    if elenco as f64 >= r.elenco_teto || (r.teto_temporada > 0.0 && recebidos_no_ano(clube) as f64 >= r.teto_temporada) {
        return 0.0;
    }
    if surpresa && !em_competicao(clube) {
        return 0.0;
    }
    let alvo = r.alvo_k * ((1.0 - r.alvo_ca) * pa + r.alvo_ca * ca);
    let rep = w(clube + 0x80) as f64;
    // a curva é assimétrica: o clube menor que o alvo cai mais depressa (sigma_below), senão as centenas de clubes
    // pequenos de um país, somados, levam os jovens bons
    let largura = if rep < alvo && r.sigma_abaixo > 0.0 { r.sigma_abaixo } else { r.sigma };
    let z = (rep - alvo) / largura;
    let nivel = if surpresa { 1.0 } else { r.piso_nivel + (1.0 - r.piso_nivel) * (-0.5 * z * z).exp() };
    let inst = r.inst_base + r.inst_k * b(clube + 0x7f) as f64;
    let comissao = r.com_min + (r.com_max - r.com_min) * (melhor_com_jovens(clube) - 1) as f64 / 19.0;
    let vaga = ((r.elenco_teto - elenco as f64) / r.elenco_teto).powf(r.elenco_exp);
    (nivel * inst * comissao * vaga).max(0.0).powf(r.temperatura)
}

/// Um número uniforme em [0, 1) com dois sorteios do jogo (30 bits).
unsafe fn uniforme() -> f64 {
    ((sorteio(0x8000) as u32) << 15 | sorteio(0x8000) as u32) as f64 / (1u32 << 30) as f64
}

unsafe fn nome(clube: usize) -> String {
    let s = core::slice::from_raw_parts((clube + 4) as *const u8, 51);
    s.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect()
}

/// { 0x7ad0e0 } (thiscall, ret 4): a regra nova da disputa. Mesmos clubes e filtros da original; o clube é sorteado na
/// proporção do peso. Nenhum peso positivo: 0 (o jovem fica sem clube, como na original sem clube elegível).
unsafe extern "thiscall" fn escolher_clube(_this: usize, jogador: usize) -> usize {
    let r = REGRAS;
    let ficha = d(jogador + 0x61);
    let (pa, ca) = (w(ficha + 7) as f64, w(ficha + 5) as f64);
    let total_de_clubes = (d(0xae235c) as i32).wrapping_sub((d(0xae2350) as i32).wrapping_mul(2));
    let base = d(0xae23b4);
    let mut surpresa = r.surpresa > 0.0 && (sorteio(10000) as f64) < r.surpresa * 100.0;
    let mut pesos: Vec<(usize, f64)> = Vec::new();
    // a surpresa sem nenhum clube de competição no país (as ligas dele não estão no jogo) volta ao sorteio normal, senão
    // o jovem ficaria sem clube só por causa dela
    for _ in 0..2 {
        pesos.clear();
        for i in 0..total_de_clubes.max(0) as usize {
            let clube = base + 0x245 * i;
            if let Some(elenco) = elegivel(jogador, clube) {
                let p = peso(&r, pa, ca, clube, elenco, surpresa);
                if p > 0.0 {
                    pesos.push((clube, p));
                }
            }
        }
        if !pesos.is_empty() || !surpresa {
            break;
        }
        surpresa = false;
    }
    let soma: f64 = pesos.iter().map(|x| x.1).sum();
    if soma <= 0.0 {
        if r.registrar {
            registrar_escolha("new", jogador, 0, "-\t0\t-");
        }
        return 0;
    }
    let mut x = uniforme() * soma;
    let mut escolhido = pesos[pesos.len() - 1];
    for &(c, p) in &pesos {
        x -= p;
        if x < 0.0 {
            escolhido = (c, p);
            break;
        }
    }
    if r.registrar {
        registrar_escolha("new", jogador, escolhido.0, &format!("{:.1}%\t{}\t{}", 100.0 * escolhido.1 / soma, pesos.len(), if surpresa { "surprise" } else { "-" }));
    }
    escolhido.0
}

/// A data do jogo como AAAA-ddd (o dia do ano em [0xae2c90], o ano em [0xae2c92]).
unsafe fn data_do_jogo() -> String {
    format!("{}-{:03}", w(0xae2c92) as u16, w(0xae2c90) as u16)
}

/// Uma linha do log por jovem da disputa, separada por tabulação (para contar depois):
/// `regen  regra  data  idade  PA  CA  reputação  Training  elenco  clube  [chance  disputaram  surpresa]`.
unsafe fn registrar_escolha(regra: &str, jogador: usize, clube: usize, extra: &str) {
    let ficha = d(jogador + 0x61);
    let (rep, tr, nome_clube) = if clube != 0 { (w(clube + 0x80) as i32, b(clube + 0x7f) as i32, nome(clube)) } else { (-1, -1, "(no club)".into()) };
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let elenco = if clube != 0 { elenco_de(clube, 0, 0) as i32 } else { -1 };
    registro::linha(&format!("regen\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", regra, data_do_jogo(), b(jogador + 0x18), w(ficha + 7), w(ficha + 5), rep, tr, elenco, nome_clube, extra));
}

/// A entrada original da 0x7ad0e0, para o modo de observação (`rule = original` com `log = 1`): os 7 bytes do prólogo
/// (`sub esp, 0x14; mov [esp+0x10], ecx`, sem salto relativo) copiados para um trecho que volta em 0x7ad0e7.
static mut DISPUTA_ORIGINAL: usize = 0;

/// { 0x7ad0e0 } no modo de observação: a disputa da original escolhe, e a escolha vai para o log.
unsafe extern "thiscall" fn observar_clube(this: usize, jogador: usize) -> usize {
    let original: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(DISPUTA_ORIGINAL);
    let clube = original(this, jogador);
    registrar_escolha("original", jogador, clube, "-\t-\t-");
    clube
}

unsafe fn instalar_observacao() -> bool {
    let t = VirtualAlloc(core::ptr::null_mut(), 16, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return false;
    }
    core::ptr::copy_nonoverlapping(0x7ad0e0 as *const u8, t, 7);
    *t.add(7) = 0xe9;
    core::ptr::write_unaligned(t.add(8) as *mut i32, 0x7ad0e7usize.wrapping_sub(t as usize + 12) as i32);
    DISPUTA_ORIGINAL = t as usize;
    desviar(0x7ad0e0, observar_clube as *const () as usize);
    true
}

/// Uma idade da tabela do `.cfg` (`age_NN`), pelo sorteio do jogo.
unsafe fn idade_da_tabela(r: &Regras) -> usize {
    let soma: f64 = r.peso_da_idade.iter().sum();
    let mut idade = 17usize; // sem peso nenhum: a idade mais comum da original
    if soma > 0.0 {
        // os pesos em inteiros de até 60000 no total, para o sorteio do jogo ficar no ramo de 16 bits
        let inteiros: Vec<i32> = r.peso_da_idade.iter().map(|p| (p / soma * 60000.0).round() as i32).collect();
        let total: i32 = inteiros.iter().sum();
        if total > 0 {
            let mut x = sorteio(total);
            for (k, &p) in inteiros.iter().enumerate() {
                if x < p {
                    idade = IDADE_MIN + k;
                    break;
                }
                x -= p;
            }
        }
    }
    idade
}

/// No ramo das nações fora do jogo com menos de 350 pessoas (pelo trecho de `instalar_idade_fora`): só a idade, da mesma
/// tabela; o CA sai depois da fórmula da original pela idade.
/// A regra de petermf23: com `mais_novo_a_partir` anos ou mais, `mais_novo_em` anos a menos (nunca abaixo de IDADE_MIN).
fn rejuvenescer(r: &Regras, idade: usize) -> usize {
    if r.mais_novo_em > 0 && idade as i32 >= r.mais_novo_a_partir {
        (idade as i32 - r.mais_novo_em).max(IDADE_MIN as i32) as usize
    } else {
        idade
    }
}

unsafe extern "C" fn sortear_idade_fora(pessoa: usize) -> u32 {
    let r = REGRAS;
    let idade = if r.idades_fora {
        idade_da_tabela(&r)
    } else {
        // o sorteio da original (0x7abeb1..0x7abee7): sorteio(pessoas da nação, +0x88) < sorteio(1000) dá 23 a 27, senão 17 a 23
        let pessoas = core::ptr::read_unaligned((d(pessoa + 0x1a) + 0x88) as *const i32);
        if sorteio(pessoas) < sorteio(1000) { 23 + sorteio(5) as usize } else { 17 + sorteio(7) as usize }
    };
    let idade = rejuvenescer(&r, idade);
    if r.registrar {
        registro::linha(&format!("birth\t{}\t{}\t{}\tout of game", data_do_jogo(), idade, w(d(pessoa + 0x61) + 7)));
    }
    idade as u32
}

/// No lugar do bloco 0x7ac597..0x7ac6f6 da 0x7abac0 (pelo trecho de `instalar_idade`): a idade do jovem, sorteada
/// pelos pesos do `.cfg`, e o teto do CA dessa idade (ficha +5), que fica de fora em `pular_teto`% das vezes (a original
/// pula em 1 de 250 ou de 150). Devolve a idade, que o trecho põe no `bl` e na local da original.
unsafe extern "C" fn sortear_idade(pessoa: usize) -> u32 {
    let r = REGRAS;
    let idade = rejuvenescer(&r, idade_da_tabela(&r));
    if r.registrar {
        registro::linha(&format!("birth\t{}\t{}\t{}", data_do_jogo(), idade, w(d(pessoa + 0x61) + 7)));
    }
    let teto = r.teto_da_idade[idade - IDADE_MIN];
    let ficha = d(pessoa + 0x61);
    if teto >= 0 && w(ficha + 5) > teto && (sorteio(10000) as f64) >= r.pular_teto * 100.0 {
        core::ptr::write_unaligned((ficha + 5) as *mut i16, teto);
    }
    idade as u32
}

/// Troca o bloco da idade (0x7ac597: `call sorteio(10000)`, com a pilha ainda com os dois argumentos que o `add esp, 8`
/// seguinte tiraria) por um trecho que chama `sortear_idade(esi)`, põe a idade no `ebx` e em [esp+0x1c] e volta em
/// 0x7ac6f6, onde a original segue. E o corte da disputa: "menos de 19" vira `idade_max_disputa` + 1 (0x7ace69).
unsafe fn instalar_idade(r: &Regras) -> bool {
    let t = VirtualAlloc(core::ptr::null_mut(), 64, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return false;
    }
    let f = sortear_idade as *const () as u32;
    let mut c: Vec<u8> = vec![0x83, 0xc4, 0x08, 0x56, 0xb8]; // add esp, 8; push esi; mov eax, f
    c.extend_from_slice(&f.to_le_bytes());
    c.extend_from_slice(&[0xff, 0xd0, 0x83, 0xc4, 0x04, 0x0f, 0xb6, 0xd8, 0x88, 0x5c, 0x24, 0x1c, 0xe9]); // call eax; add esp, 4; movzx ebx, al; mov [esp+0x1c], bl; jmp
    let fim = t as usize + c.len() + 4;
    c.extend_from_slice(&(0x7ac6f6usize.wrapping_sub(fim) as i32).to_le_bytes());
    core::ptr::copy_nonoverlapping(c.as_ptr(), t, c.len());
    desviar(0x7ac597, t as usize);
    let mut antiga = 0u32;
    VirtualProtect(0x7ace69 as *mut c_void, 1, 0x40, &mut antiga);
    *(0x7ace69 as *mut u8) = (r.idade_max_disputa + 1) as u8;
    VirtualProtect(0x7ace69 as *mut c_void, 1, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), 0x7ace69 as *const c_void, 1);
    true
}
/// Troca o sorteio da idade das nações fora do jogo (0x7abeb1: `push eax; call sorteio(pessoas)`, nada pendente na pilha)
/// por um trecho que chama `sortear_idade_fora(esi)`, põe a idade no `ebx` e volta em 0x7abeea, onde a original guarda
/// a idade na local e segue para a fórmula do CA.
unsafe fn instalar_idade_fora() -> bool {
    let t = VirtualAlloc(core::ptr::null_mut(), 32, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return false;
    }
    let f = sortear_idade_fora as *const () as u32;
    let mut c: Vec<u8> = vec![0x56, 0xb8]; // push esi; mov eax, f
    c.extend_from_slice(&f.to_le_bytes());
    c.extend_from_slice(&[0xff, 0xd0, 0x83, 0xc4, 0x04, 0x0f, 0xb6, 0xd8, 0xe9]); // call eax; add esp, 4; movzx ebx, al; jmp
    let fim = t as usize + c.len() + 4;
    c.extend_from_slice(&(0x7abeeausize.wrapping_sub(fim) as i32).to_le_bytes());
    core::ptr::copy_nonoverlapping(c.as_ptr(), t, c.len());
    desviar(0x7abeb1, t as usize);
    true
}
/// Está na lista de observados de algum clube (0x842080 do gerente de listas em [[0xde1f54] + 0x856])?
unsafe fn observado(pessoa: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
    f(d(d(0xde1f54) + 0x856), pessoa, 0) != 0
}

/// { 0x7a8050 } (stdcall, ret 8): `player_regen_check_unemployed_retirement(pessoa, saida)`, com `retire_idle_players`.
/// A original (a mesma cópia em Rust do projeto CM0102, comparada sem divergência): o observado não se aposenta; o
/// técnico (+0x69) sem clube e sem seleção, com +0xa < 50, em 1 de 5; o jogador (+0x61) sem clube e sem seleção,
/// desempregado há mais de 365 dias, com sorteio(reputação atual +0xb) < 750; sem nenhum dos dois registros, sempre.
/// O que muda, só no jogador:
/// - abaixo de `proteger_jovens` anos (+0x18), não se aposenta por esta regra;
/// - com `inativo_idade` anos ou mais e desempregado há mais de `inativo_dias` dias, aposenta com sorteio(reputação) <
///   `inativo_rep` + `inativo_rep_por_ano` × (idade − `inativo_idade`); os outros seguem a regra original.
/// Aposenta: a saída (26 bytes) recebe a pessoa, o motivo 0x10, o clube (−1 sem) e a data de hoje, e devolve 1.
unsafe extern "stdcall" fn aposentadoria_do_desempregado(pessoa: usize, saida: usize) -> u8 {
    let r = REGRAS;
    if observado(pessoa) {
        return 0;
    }
    let tecnico = d(pessoa + 0x69);
    let sem_registro = tecnico == 0 && d(pessoa + 0x61) == 0;
    let aposenta = if tecnico != 0 {
        if d(pessoa + 0x39) != 0 || d(pessoa + 0x24) != 0 {
            return 0;
        }
        w(tecnico + 0xa) < 50 && sorteio(5) == 0 // o técnico fraco, como na original
    } else if d(pessoa + 0x61) == 0 {
        false
    } else if d(pessoa + 0x39) != 0 || d(pessoa + 0x24) != 0 {
        false
    } else {
        let idade = b(pessoa + 0x18) as i32;
        let anos = (w(0xae2c92) as u16 as i32) - (w(pessoa + 0x40) as u16 as i32);
        let dias = anos * 365 - w(pessoa + 0x3e) as i32 + w(0xae2c90) as i32;
        let rep = w(d(pessoa + 0x61) + 0xb) as i32;
        if r.proteger_jovens > 0 && idade < r.proteger_jovens {
            false
        } else if idade >= r.inativo_idade && dias > r.inativo_dias {
            let limiar = r.inativo_rep + r.inativo_rep_por_ano * (idade - r.inativo_idade) as f64;
            let sai = rep <= 0 || (sorteio(rep) as f64) < limiar;
            if sai && r.registrar {
                registro::linha(&format!("idle\t{}\t{}\t{}\t{}", data_do_jogo(), idade, rep, dias));
            }
            sai
        } else {
            dias > 365 && sorteio(rep) < 750 // a regra original
        }
    };
    if !aposenta && !sem_registro {
        return 0;
    }
    core::ptr::write_unaligned(saida as *mut usize, d(pessoa));
    *((saida + 0x18) as *mut u8) = 0x10; // o motivo: desempregado
    *((saida + 0x19) as *mut u8) = 0;
    let clube = d(pessoa + 0x39);
    core::ptr::write_unaligned((saida + 4) as *mut u32, if clube != 0 { d(clube) as u32 } else { u32::MAX });
    for o in [8usize, 0x10] {
        core::ptr::write_unaligned((saida + o) as *mut u16, w(0xae2c90) as u16);
        core::ptr::write_unaligned((saida + o + 2) as *mut u16, w(0xae2c92) as u16);
    }
    1
}
#[no_mangle]
pub unsafe extern "system" fn DllMain(_modulo: *mut c_void, motivo: u32, _reservado: *mut c_void) -> i32 {
    if motivo == 1 {
        let r = ler_regras();
        REGRAS = r;
        if r.registrar {
            AddVectoredExceptionHandler(1, ao_falhar);
        }
        let s = r.sobrescrever;
        if !r.save {
            registro::linha("save over 2 GB: off (save_over_2gb = 0)");
        } else if pode_ligar(&SAVE, s) {
            desviar(0x9326e0, conteiner::abrir_no_conteiner as *const () as usize);
            desviar(0x933080, conteiner::buscar_no_conteiner as *const () as usize);
            desviar(0x933170, conteiner::posicao_no_conteiner as *const () as usize);
            registro::linha("save over 2 GB: on (saves up to 4 GB)");
        }
        if !r.idades {
            registro::linha("regen ages: off (ages = original)");
        } else if pode_ligar(&IDADE, s) {
            if instalar_idade(&r) {
                registro::linha(&format!("regen ages: on (age table; {} or older made {} years younger; contest up to age {})", r.mais_novo_a_partir, r.mais_novo_em, r.idade_max_disputa));
            } else {
                registro::linha("regen ages: could not allocate the code stub; nothing was changed");
            }
        }
        if !r.idades || (!r.idades_fora && r.mais_novo_em == 0) {
            registro::linha("regen ages (nations not in the game): off");
        } else if pode_ligar(&IDADE_FORA, s) && instalar_idade_fora() {
            registro::linha(&format!("regen ages (nations not in the game): on ({}; {} or older made {} years younger)", if r.idades_fora { "age table" } else { "original draw" }, r.mais_novo_a_partir, r.mais_novo_em));
        }
        if !r.inativos {
            registro::linha("retire idle players: off (retire_idle_players = 0)");
        } else if pode_ligar(&INATIVOS, s) {
            desviar(0x7a8050, aposentadoria_do_desempregado as *const () as usize);
            registro::linha(&format!("retire idle players: on ({}+ years and {}+ days without a club; young players under {} protected)", r.inativo_idade, r.inativo_dias, r.proteger_jovens));
        }
        if !r.novo && r.registrar {
            if assinatura(DISPUTA.bordas) == DISPUTA.assinatura_das_bordas && instalar_observacao() {
                registro::linha("regens 0.1: off (rule = original), logging the exe's own picks (log = 1)");
            } else {
                registro::linha("regens 0.1: off (rule = original); could not log the exe's picks");
            }
        } else if !r.novo {
            registro::linha("regens 0.1: off (rule = original)");
        } else if pode_ligar(&DISPUTA, s) {
            desviar(0x7ad0e0, escolher_clube as *const () as usize);
            registro::linha("regens 0.1: on (proportional draw)");
        }
    }
    1
}

type FnDirectDrawCreate = unsafe extern "system" fn(*mut c_void, *mut *mut c_void, *mut c_void) -> i32;

/// A ddraw.dll seguinte: `ddraw_chain.dll` na pasta do exe, se existir; senão a do Windows.
unsafe fn proxima_ddraw() -> Option<FnDirectDrawCreate> {
    let local = std::env::current_exe().ok().map(|p| p.with_file_name("ddraw_chain.dll")).filter(|p| p.exists());
    let caminho: Vec<u16> = match local {
        Some(p) => p.to_string_lossy().encode_utf16().chain([0]).collect(),
        None => {
            let mut buf = [0u16; 260];
            let n = GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32) as usize;
            if n == 0 || n > 240 {
                return None;
            }
            buf[..n].iter().copied().chain("\\ddraw.dll\0".encode_utf16()).collect()
        }
    };
    let m = LoadLibraryW(caminho.as_ptr());
    if m.is_null() {
        return None;
    }
    let f = GetProcAddress(m, b"DirectDrawCreate\0".as_ptr());
    (!f.is_null()).then(|| core::mem::transmute::<*mut c_void, FnDirectDrawCreate>(f))
}

#[no_mangle]
pub unsafe extern "system" fn DirectDrawCreate(guid: *mut c_void, saida: *mut *mut c_void, externo: *mut c_void) -> i32 {
    static mut CONFERIDO: bool = false;
    if REGRAS.registrar && !CONFERIDO {
        CONFERIDO = true;
        conferir_desvios();
    }
    match proxima_ddraw() {
        Some(f) => f(guid, saida, externo),
        None => {
            registro::linha("regens: could not load the next ddraw.dll");
            0x8000_4005_u32 as i32
        }
    }
}
