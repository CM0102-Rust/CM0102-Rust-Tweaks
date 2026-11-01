//! DLL de ajustes do CM 01/02 3.9.68, sem alterar o exe: a distribuição dos regens e o save acima de 2 GB.
//!
//! Entra no jogo como `ddraw.dll`, como a do save2gb, e lê `regens.cfg` ao lado do exe. Cada parte só liga se a
//! assinatura do código de que depende bater com o 3.9.68:
//! - `save_over_2gb = 1` (padrão): as três rotinas do contêiner do save (`zopen` 0x9326e0, `zseek` 0x933080, `ztell`
//!   0x933170) vão para as de `src/conteiner.rs`, como na DLL do save2gb (`TRECHOS_DO_SAVE`);
//! - `rule = new`: `PLAYER_REGEN::pick_best_club_for_player` (0x7ad0e0) vai para `escolher_clube` (`TRECHOS`).
//! O `DirectDrawCreate` é repassado a
//! `ddraw_chain.dll` na pasta do jogo, se existir, senão à `ddraw.dll` do Windows. O registro vai para `regens.log`.
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
        let caminho = std::env::current_exe().map(|p| p.with_file_name("regens.log")).unwrap_or_else(|_| "regens.log".into());
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
}

/// A entrada da 0x7ad0e0, a chamada dela na 0x7abac0 e o começo das rotinas do jogo que a nova chama. O corpo da 0x7ad0e0
/// fica de fora: ele não roda mais, e assim o exe com o Regen Tweak (0x7ad1bd) também serve.
const TRECHOS: [(usize, usize); 7] = [(0x7ad0e0, 7), (0x7ace9b, 0x1a), (0x53d980, 32), (0x540a50, 32), (0x53c590, 32), (0x8ce990, 32), (0x90cfc0, 32)];
/// FNV-1a de 64 bits dos trechos no cm0102.exe 3.9.68.
const ASSINATURA: u64 = 0xb9c6540ed49c8bff;

/// O save: o contêiner inteiro (zipdir.cpp) e o começo das funções da CRT e do jogo que as rotinas chamam (os mesmos
/// trechos e a mesma assinatura de save/src/lib.rs).
const TRECHOS_DO_SAVE: [(usize, usize); 15] = [
    (0x9326e0, 0x1180), (0x944c94, 32), (0x944c9f, 32), (0x944cf1, 32), (0x944cff, 32), (0x945a09, 32),
    (0x945b00, 32), (0x945bfe, 32), (0x945ce6, 32), (0x946a22, 32), (0x94af9e, 32), (0x94fded, 32),
    (0x94c552, 32), (0x90d130, 32), (0x5e8290, 32),
];
const ASSINATURA_DO_SAVE: u64 = 0x232589cfe29f6240;

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
    let mut antiga = 0u32;
    VirtualProtect(de as *mut c_void, 5, 0x40, &mut antiga);
    *(de as *mut u8) = 0xe9;
    core::ptr::write_unaligned((de + 1) as *mut i32, para.wrapping_sub(de + 5) as i32);
    VirtualProtect(de as *mut c_void, 5, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), de as *const c_void, 5);
}

// ---------- as regras (regens.cfg) ----------

#[derive(Clone, Copy)]
struct Regras {
    save: bool,
    novo: bool,
    alvo_k: f64,
    alvo_ca: f64,
    sigma: f64,
    piso_nivel: f64,
    inst_base: f64,
    inst_k: f64,
    com_min: f64,
    com_max: f64,
    elenco_teto: f64,
    elenco_exp: f64,
    teto_temporada: f64,
    temperatura: f64,
    registrar: bool,
}

const PADRAO: Regras = Regras {
    save: true,
    novo: true,
    alvo_k: 50.0,
    alvo_ca: 0.0,
    sigma: 1500.0,
    piso_nivel: 0.002,
    inst_base: 0.5,
    inst_k: 0.05,
    com_min: 0.8,
    com_max: 1.2,
    elenco_teto: 50.0,
    elenco_exp: 0.5,
    teto_temporada: 3.0,
    temperatura: 1.0,
    registrar: false,
};

static mut REGRAS: Regras = PADRAO;

fn ler_regras() -> Regras {
    let mut r = PADRAO;
    let caminho = std::env::current_exe().map(|p| p.with_file_name("regens.cfg")).unwrap_or_else(|_| "regens.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return r };
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        let Some((k, v)) = linha.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        if k == "rule" {
            r.novo = v != "original"; // rule = new | original
            continue;
        }
        if k == "save_over_2gb" {
            r.save = v != "0";
            continue;
        }
        let Ok(x) = v.parse::<f64>() else { continue };
        match k {
            "target_k" => r.alvo_k = x,
            "target_ca" => r.alvo_ca = x,
            "sigma" => r.sigma = x.max(1.0),
            "level_floor" => r.piso_nivel = x,
            "facilities_base" => r.inst_base = x,
            "facilities_k" => r.inst_k = x,
            "staff_min" => r.com_min = x,
            "staff_max" => r.com_max = x,
            "squad_limit" => r.elenco_teto = x.clamp(1.0, 50.0),
            "squad_exp" => r.elenco_exp = x,
            "season_cap" => r.teto_temporada = x,
            "temperature" => r.temperatura = x,
            "log" => r.registrar = x != 0.0,
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

/// O peso do clube para o jovem (o mesmo de `peso_novo` em calibrar.py).
unsafe fn peso(r: &Regras, pa: f64, ca: f64, clube: usize, elenco: i32) -> f64 {
    if elenco as f64 >= r.elenco_teto || (r.teto_temporada > 0.0 && recebidos_no_ano(clube) as f64 >= r.teto_temporada) {
        return 0.0;
    }
    let alvo = r.alvo_k * ((1.0 - r.alvo_ca) * pa + r.alvo_ca * ca);
    let z = (w(clube + 0x80) as f64 - alvo) / r.sigma;
    let nivel = r.piso_nivel + (1.0 - r.piso_nivel) * (-0.5 * z * z).exp();
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
    let mut pesos: Vec<(usize, f64)> = Vec::new();
    for i in 0..total_de_clubes.max(0) as usize {
        let clube = base + 0x245 * i;
        if let Some(elenco) = elegivel(jogador, clube) {
            let p = peso(&r, pa, ca, clube, elenco);
            if p > 0.0 {
                pesos.push((clube, p));
            }
        }
    }
    let soma: f64 = pesos.iter().map(|x| x.1).sum();
    if soma <= 0.0 {
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
        registro::linha(&format!("PA {} CA {} -> {} (reputation {}, Training {}) chance {:.1}% among {} clubs", pa, ca, nome(escolhido.0),
            w(escolhido.0 + 0x80), b(escolhido.0 + 0x7f), 100.0 * escolhido.1 / soma, pesos.len()));
    }
    escolhido.0
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(_modulo: *mut c_void, motivo: u32, _reservado: *mut c_void) -> i32 {
    if motivo == 1 {
        let r = ler_regras();
        REGRAS = r;
        if !r.save {
            registro::linha("save over 2 GB: off (save_over_2gb = 0)");
        } else if assinatura(&TRECHOS_DO_SAVE) == ASSINATURA_DO_SAVE {
            desviar(0x9326e0, conteiner::abrir_no_conteiner as *const () as usize);
            desviar(0x933080, conteiner::buscar_no_conteiner as *const () as usize);
            desviar(0x933170, conteiner::posicao_no_conteiner as *const () as usize);
            registro::linha("save over 2 GB: on (saves up to 4 GB)");
        } else {
            registro::linha(&format!("save over 2 GB: this exe differs from 3.9.68 where the save code is (signature {:016x}); nothing was changed", assinatura(&TRECHOS_DO_SAVE)));
        }
        let a = assinatura(&TRECHOS);
        if !r.novo {
            registro::linha("regens 0.1: rule = original; nothing was changed");
        } else if a == ASSINATURA {
            desviar(0x7ad0e0, escolher_clube as *const () as usize);
            registro::linha("regens 0.1: on (proportional draw; regens.cfg)");
        } else {
            registro::linha(&format!("regens 0.1: this exe differs from 3.9.68 where the regen code is (signature {:016x}); nothing was changed", a));
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
    match proxima_ddraw() {
        Some(f) => f(guid, saida, externo),
        None => {
            registro::linha("regens: could not load the next ddraw.dll");
            0x8000_4005_u32 as i32
        }
    }
}
