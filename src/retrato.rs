//! O retrato do exe: o que ele muda em relação ao cm0102.exe 3.9.68 (fase 2b do roadmap).
//!
//! - **No disco**, na abertura: o arquivo do exe contra `referencia.rs` (o hash de cada função do .text e de cada bloco de
//!   4 KB das outras seções), mais o ponto de entrada e as seções novas. O que um patcher gravou no arquivo.
//! - **Na memória**, no primeiro DirectDrawCreate (as DLLs do exe já carregaram): o .text e o .rdata em memória contra o
//!   próprio arquivo, sem os trechos que esta DLL escreveu e sem a tabela de importações. O que outra DLL (a NickDLL do
//!   CLZ, por exemplo) mudou ao rodar.
//!
//! O resumo vai sempre para o tweaks.log; a lista das funções e blocos, só com `log = 1`. Os nomes das funções e dos
//! patches do catálogo do NickPatcher ficam fora da DLL: `tools/ler_retrato.py` os acrescenta a partir do log.
use crate::referencia::{BLOCOS, ENTRADA, FUNCOES, SECOES};
use crate::registro::linha;

struct Secao {
    nome: String,
    va: u32,
    tam: u32,
    cru: u32,
    tam_cru: u32,
}

static mut ARQUIVO: Vec<u8> = Vec::new();
static mut SECOES_DO_EXE: Vec<Secao> = Vec::new();
static mut IAT: (u32, u32) = (0, 0);

fn fnv32(b: &[u8]) -> u32 {
    b.iter().fold(0x811c9dc5u32, |h, &x| (h ^ x as u32).wrapping_mul(0x01000193))
}

fn u32_em(d: &[u8], o: usize) -> u32 {
    d.get(o..o + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).unwrap_or(0)
}

/// Lê o cabeçalho PE: as seções, o ponto de entrada e a tabela de importações (o IAT, que o Windows preenche).
fn ler_pe(d: &[u8]) -> Option<(u32, Vec<Secao>, (u32, u32))> {
    let pe = u32_em(d, 0x3c) as usize;
    if d.get(pe..pe + 4)? != b"PE\0\0" {
        return None;
    }
    let n = u16::from_le_bytes([d[pe + 6], d[pe + 7]]) as usize;
    let opcional = pe + 0x18;
    let tam_opcional = u16::from_le_bytes([d[pe + 0x14], d[pe + 0x15]]) as usize;
    let base = u32_em(d, opcional + 0x1c);
    let entrada = base + u32_em(d, opcional + 0x10);
    let iat = (base + u32_em(d, opcional + 0x60 + 12 * 8), u32_em(d, opcional + 0x60 + 12 * 8 + 4));
    let mut secoes = Vec::new();
    for k in 0..n {
        let s = opcional + tam_opcional + 40 * k;
        let nome = String::from_utf8_lossy(&d.get(s..s + 8)?.split(|&c| c == 0).next()?.to_vec()).into_owned();
        secoes.push(Secao { nome, va: base + u32_em(d, s + 12), tam: u32_em(d, s + 8), cru: u32_em(d, s + 20), tam_cru: u32_em(d, s + 16) });
    }
    Some((entrada, secoes, iat))
}

/// Os bytes de [va, va + n) no arquivo, se estiverem inteiros numa seção.
fn no_arquivo<'a>(d: &'a [u8], secoes: &[Secao], va: u32, n: u32) -> Option<&'a [u8]> {
    let s = secoes.iter().find(|s| va >= s.va && va + n <= s.va + s.tam_cru)?;
    let o = (va - s.va + s.cru) as usize;
    d.get(o..o + n as usize)
}

use crate::partes::CONHECIDOS;

/// O retrato do arquivo, na abertura da DLL.
pub unsafe fn do_disco(detalhe: bool) {
    let caminho = match std::env::current_exe() {
        Ok(c) => c,
        Err(_) => return,
    };
    let Ok(d) = std::fs::read(&caminho) else {
        linha("exe: could not read the exe file; no snapshot");
        return;
    };
    let Some((entrada, secoes, iat)) = ler_pe(&d) else {
        linha("exe: not a PE file?; no snapshot");
        return;
    };
    let nome = caminho.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    linha(&format!("exe\t{}\t{} bytes\tfnv {:08x}", nome, d.len(), fnv32(&d)));
    if entrada != ENTRADA {
        linha(&format!("exe\tentry point {:08x} (3.9.68: {:08x})", entrada, ENTRADA));
    }
    let novas: Vec<String> = secoes.iter().filter(|s| !SECOES.iter().any(|o| o.0 == s.nome && o.1 == s.va))
        .map(|s| format!("{} at {:08x}, {} KB", s.nome, s.va, s.tam.max(s.tam_cru) / 1024)).collect();
    if !novas.is_empty() {
        linha(&format!("exe\tnew sections: {}", novas.join("; ")));
    }
    let funcoes: Vec<&(u32, u32, u32)> = FUNCOES.iter().filter(|&&(va, n, h)| no_arquivo(&d, &secoes, va, n).map(fnv32) != Some(h)).collect();
    let blocos: Vec<&(u32, u32, u32)> = BLOCOS.iter().filter(|&&(va, n, h)| no_arquivo(&d, &secoes, va, n).map(fnv32) != Some(h)).collect();
    let conhecidos: Vec<&str> = CONHECIDOS.iter().filter(|&&(_, va, b)| no_arquivo(&d, &secoes, va, b.len() as u32) == Some(b)).map(|c| c.0).collect();
    if funcoes.is_empty() && blocos.is_empty() && novas.is_empty() && entrada == ENTRADA {
        linha("exe\tthe same code and data as the original 3.9.68");
    } else {
        linha(&format!("exe\tdiffers from 3.9.68: {} of {} functions, {} of {} data blocks{}", funcoes.len(), FUNCOES.len(), blocos.len(), BLOCOS.len(),
            if detalhe { " (listed below as exe-fn and exe-data; tools/ler_retrato.py names them)" } else { " (log = 1 lists them)" }));
    }
    for c in &conhecidos {
        linha(&format!("exe\tknown change: {}", c));
    }
    if detalhe {
        for &&(va, n, _) in &funcoes {
            linha(&format!("exe-fn\t{:08x}\t{}", va, n));
        }
        for &&(va, n, _) in &blocos {
            linha(&format!("exe-data\t{:08x}\t{}", va, n));
        }
    }
    ARQUIVO = d;
    SECOES_DO_EXE = secoes;
    IAT = iat;
}

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(bandeiras: u32, endereco: *const u8, modulo: *mut *mut core::ffi::c_void) -> i32;
    fn GetModuleFileNameW(modulo: *mut core::ffi::c_void, buf: *mut u16, tam: u32) -> u32;
}

/// Desvios de outras DLLs já lidos (endereço, módulo, o que muda para os ajustes): viram nota em vez de aviso.
const DESVIOS_CONHECIDOS: [(u32, &str, &str); 2] = [
    (0x540a50, "NickDLL.dll", "NickDLL's own club_get_linked_b_team, with its pair list for the loaded database; the contest uses it to leave B teams out, as the exe's own routine would (fine)"),
    (0x5e8290, "LaMOHatesCD.dll", "a message-box filter (by its name, the no-CD helper); other messages, like the save container's errors, still show (fine)"),
];

/// Se o trecho começa com um salto (E9 ou E8 rel32, ou FF 25 [endereço]), para onde ele vai e em que módulo.
unsafe fn destino(va: u32) -> String {
    let p = va as *const u8;
    let alvo = match *p {
        0xe9 | 0xe8 => (va + 5).wrapping_add(core::ptr::read_unaligned(p.add(1) as *const u32)),
        0xff if *p.add(1) == 0x25 => *(core::ptr::read_unaligned(p.add(2) as *const u32) as *const u32),
        _ => return format!("; now {:02x} {:02x} {:02x} {:02x} {:02x}", *p, *p.add(1), *p.add(2), *p.add(3), *p.add(4)),
    };
    let mut m = core::ptr::null_mut();
    // GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT
    if GetModuleHandleExW(0x4 | 0x2, alvo as *const u8, &mut m) == 0 {
        return format!("; it now jumps to {:08x} (outside any module)", alvo);
    }
    let mut buf = [0u16; 260];
    let n = GetModuleFileNameW(m, buf.as_mut_ptr(), 260) as usize;
    let caminho = String::from_utf16_lossy(&buf[..n]);
    let nome = caminho.rsplit('\\').next().unwrap_or("");
    format!("; it now jumps to {} +{:x}", nome, alvo as usize - m as usize)
}

/// O retrato da memória, depois das DLLs do exe: o .text e o .rdata contra o arquivo, sem as `nossas` escritas.
/// `partes`: os ajustes ligados e os trechos que cada um conferiu na abertura (as bordas); uma mudança de outra DLL
/// dentro deles vira aviso, porque a conferência foi antes dela.
pub unsafe fn da_memoria(nossas: &[(usize, usize)], partes: &[(&str, &[(usize, usize)])], detalhe: bool) {
    let d = core::mem::take(&mut *core::ptr::addr_of_mut!(ARQUIVO));
    let secoes = core::mem::take(&mut *core::ptr::addr_of_mut!(SECOES_DO_EXE));
    if d.is_empty() {
        return;
    }
    let iat = IAT;
    let nosso = |va: u32| nossas.iter().any(|&(de, n)| (va as usize) >= de && (va as usize) < de + n);
    let mut trechos: Vec<(u32, u32)> = Vec::new();
    for s in secoes.iter().filter(|s| s.nome == ".text" || s.nome == ".rdata") {
        let n = s.tam.min(s.tam_cru);
        let memoria = core::slice::from_raw_parts(s.va as *const u8, n as usize);
        let arquivo = &d[s.cru as usize..(s.cru + n) as usize];
        for k in 0..n {
            let va = s.va + k;
            if memoria[k as usize] == arquivo[k as usize] || nosso(va) || (va >= iat.0 && va < iat.0 + iat.1) {
                continue;
            }
            match trechos.last_mut() {
                Some(t) if va - (t.0 + t.1) <= 8 => t.1 = va + 1 - t.0,
                _ => trechos.push((va, 1)),
            }
        }
    }
    if trechos.is_empty() {
        linha("runtime\tno other DLL changed the exe's code after loading");
        return;
    }
    for &(nome, bordas) in partes {
        for &(va, n) in &trechos {
            if let Some(&(b, _)) = bordas.iter().find(|&&(b, m)| (va as usize) < b + m && b < (va + n) as usize) {
                let para = destino(va);
                match DESVIOS_CONHECIDOS.iter().find(|c| c.0 == va && para.contains(c.1)) {
                    Some(c) => linha(&format!("runtime\tnote: {}: {:08x}{}: {}", nome, va, para, c.2)),
                    None => linha(&format!("runtime\twarning: {}: another DLL changed {:08x} ({} bytes) after loading, inside code this part checked at start ({:08x}){}", nome, va, n, b, para)),
                }
            }
        }
    }
    let bytes: u32 = trechos.iter().map(|t| t.1).sum();
    linha(&format!("runtime\tanother DLL changed the exe after loading: {} ranges, {} bytes{}", trechos.len(), bytes, if detalhe { " (listed below as runtime-fn)" } else { "" }));
    if detalhe {
        for (va, n) in trechos {
            // a função do 3.9.68 onde o trecho começa (o início dela, para tools/ler_retrato.py)
            let k = FUNCOES.partition_point(|f| f.0 <= va);
            let f = if k > 0 { FUNCOES[k - 1].0 } else { 0 };
            linha(&format!("runtime-fn\t{:08x}\t{}\tin {:08x}", va, n, f));
        }
    }
}
