//! A pasta do jogo e a marca de compatibilidade, sem mexer no exe nem nas outras DLLs (pedido do CLZ, 02/11/2026).
//!
//! - `portable_folder = 1`: o `WinMain` lê `HKLM\Software\Sports Interactive Ltd\Championship Manager 01-02\Location`
//!   (gravado pelo SetupReg) e faz chdir para lá. A DLL troca, na tabela de importações do exe, o `RegOpenKeyExA` e o
//!   `RegQueryValueExA`: a chave do jogo abre sempre (um identificador próprio) e o `Location` é a pasta do exe. Outras
//!   chaves e valores seguem para o Windows. Dispensa o SetupReg e faz duas instalações conviverem.
//! - `clear_admin_flag = 1`: a NickDLL grava `WIN95 RUNASADMIN` em `HKCU\...\AppCompatFlags\Layers` para o caminho do exe
//!   a cada execução; com ela o jogo pede elevação e, elevado, ia para o `Location` de outra instalação. Na primeira
//!   `DirectDrawCreate` (depois da NickDLL) a DLL tira o `RUNASADMIN` e o `WIN95` da entrada desse exe e mantém o resto
//!   (o `DWM8And16BitMitigation` do Windows); vazia, apaga. Vale da execução seguinte em diante.

use core::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(nome: *const u16) -> *mut c_void;
    fn GetModuleFileNameA(modulo: *mut c_void, buf: *mut u8, tam: u32) -> u32;
    fn VirtualProtect(end: *mut c_void, tam: usize, nova: u32, antiga: *mut u32) -> i32;
}

#[link(name = "advapi32")]
extern "system" {
    fn RegOpenKeyExA(chave: usize, sub: *const u8, opcoes: u32, acesso: u32, saida: *mut usize) -> i32;
    fn RegQueryValueExA(chave: usize, nome: *const u8, r: *mut u32, tipo: *mut u32, dados: *mut u8, tam: *mut u32) -> i32;
    fn RegSetValueExA(chave: usize, nome: *const u8, r: u32, tipo: u32, dados: *const u8, tam: u32) -> i32;
    fn RegDeleteValueA(chave: usize, nome: *const u8) -> i32;
    fn RegCloseKey(chave: usize) -> i32;
}

const CHAVE_DO_JOGO: &str = "software\\sports interactive ltd\\championship manager 01-02";
/// O identificador falso da chave do jogo (nunca é um identificador do Windows, que são múltiplos de 4).
const FALSO: usize = 0x0CF0_1021;
const HKCU: usize = 0x8000_0001;

static mut ABRIR_ORIGINAL: usize = 0;
static mut LER_ORIGINAL: usize = 0;

unsafe fn texto(p: *const u8) -> String {
    if p.is_null() {
        return String::new();
    }
    let n = (0..1024).position(|k| *p.add(k) == 0).unwrap_or(1024);
    core::slice::from_raw_parts(p, n).iter().map(|&c| c as char).collect()
}

/// O caminho do exe, em ANSI, sem o zero final.
unsafe fn caminho_do_exe() -> Vec<u8> {
    let mut buf = [0u8; 520];
    let n = GetModuleFileNameA(core::ptr::null_mut(), buf.as_mut_ptr(), buf.len() as u32) as usize;
    buf[..n.min(buf.len())].to_vec()
}

unsafe extern "system" fn abrir(chave: usize, sub: *const u8, opcoes: u32, acesso: u32, saida: *mut usize) -> i32 {
    if texto(sub).to_ascii_lowercase().trim_end_matches('\\') == CHAVE_DO_JOGO && !saida.is_null() {
        *saida = FALSO;
        return 0;
    }
    let f: unsafe extern "system" fn(usize, *const u8, u32, u32, *mut usize) -> i32 = core::mem::transmute(ABRIR_ORIGINAL);
    f(chave, sub, opcoes, acesso, saida)
}

unsafe extern "system" fn ler(chave: usize, nome: *const u8, r: *mut u32, tipo: *mut u32, dados: *mut u8, tam: *mut u32) -> i32 {
    if chave != FALSO {
        let f: unsafe extern "system" fn(usize, *const u8, *mut u32, *mut u32, *mut u8, *mut u32) -> i32 = core::mem::transmute(LER_ORIGINAL);
        return f(chave, nome, r, tipo, dados, tam);
    }
    if !texto(nome).eq_ignore_ascii_case("Location") {
        return 2; // ERROR_FILE_NOT_FOUND: a chave do jogo só tem o Location
    }
    let mut pasta = caminho_do_exe();
    let barra = pasta.iter().rposition(|&c| c == b'\\').unwrap_or(0);
    pasta.truncate(barra);
    pasta.push(0);
    if !tipo.is_null() {
        *tipo = 1; // REG_SZ
    }
    if tam.is_null() {
        return if dados.is_null() { 0 } else { 87 }; // ERROR_INVALID_PARAMETER
    }
    let cabe = *tam as usize >= pasta.len();
    *tam = pasta.len() as u32;
    if dados.is_null() {
        return 0;
    }
    if !cabe {
        return 234; // ERROR_MORE_DATA
    }
    core::ptr::copy_nonoverlapping(pasta.as_ptr(), dados, pasta.len());
    0
}

/// A entrada `funcao` de `dll` na tabela de importações do exe: troca por `nova` e devolve a antiga.
pub(crate) unsafe fn trocar_importacao(dll: &str, funcao: &str, nova: usize) -> Option<usize> {
    let base = GetModuleHandleW(core::ptr::null()) as usize;
    let rd = |o: usize| core::ptr::read_unaligned((base + o) as *const u32) as usize;
    let nt = rd(0x3c);
    let (importacoes, tam) = (rd(nt + 0x18 + 0x68), rd(nt + 0x18 + 0x6c));
    if importacoes == 0 {
        return None;
    }
    let mut desc = importacoes;
    while desc + 20 <= importacoes + tam && rd(desc + 12) != 0 {
        let (nomes, enderecos) = (rd(desc), rd(desc + 16));
        if nomes != 0 && texto((base + rd(desc + 12)) as *const u8).eq_ignore_ascii_case(dll) {
            let mut i = 0;
            while rd(nomes + 4 * i) != 0 {
                let t = rd(nomes + 4 * i);
                if t & 0x8000_0000 == 0 && texto((base + t + 2) as *const u8) == funcao {
                    let lugar = (base + enderecos + 4 * i) as *mut usize;
                    let mut antiga = 0u32;
                    VirtualProtect(lugar as *mut c_void, 4, 0x04, &mut antiga);
                    let velha = core::ptr::replace(lugar, nova);
                    VirtualProtect(lugar as *mut c_void, 4, antiga, &mut antiga);
                    return Some(velha);
                }
                i += 1;
            }
        }
        desc += 20;
    }
    None
}

/// portable_folder = 1 (no DllMain, antes do WinMain).
pub unsafe fn ligar_pasta() {
    match (trocar_importacao("ADVAPI32.dll", "RegOpenKeyExA", abrir as *const () as usize), trocar_importacao("ADVAPI32.dll", "RegQueryValueExA", ler as *const () as usize)) {
        (Some(a), Some(l)) => {
            ABRIR_ORIGINAL = a;
            LER_ORIGINAL = l;
            let mut p = caminho_do_exe();
            p.truncate(p.iter().rposition(|&c| c == b'\\').unwrap_or(0));
            crate::registro::linha(&format!("portable folder: on (the game's registry Location is this folder: {})", String::from_utf8_lossy(&p)));
        }
        (a, l) => {
            // uma achada e a outra não: desfaz, para não ficar pela metade
            if let Some(a) = a {
                trocar_importacao("ADVAPI32.dll", "RegOpenKeyExA", a);
            }
            if let Some(l) = l {
                trocar_importacao("ADVAPI32.dll", "RegQueryValueExA", l);
            }
            crate::registro::linha("portable folder: could not find the exe's registry imports; nothing was changed");
        }
    }
}

/// A entrada sem `RUNASADMIN` e sem `WIN95`; `None` se nada muda. O `WIN95` sai desde 12/11/2026: sozinho ele já deixa o
/// CLZ com o clz.dll em tela branca, e sem ele a carreira é criada. Antes ficava porque o CLZ 2026 V6.1 sem o clz.dll
/// caía ao criar a carreira (heap, na montagem do Brasileiro Sub-21 B, 02/11/2026); o configurador avisa disso.
fn sem_admin(atual: &str) -> Option<String> {
    let tirar = |t: &&str| t.eq_ignore_ascii_case("RUNASADMIN") || t.eq_ignore_ascii_case("WIN95");
    let resto: Vec<&str> = atual.split_whitespace().filter(|t| !tirar(t)).collect();
    (resto.len() != atual.split_whitespace().count()).then(|| resto.join(" "))
}

/// clear_admin_flag = 1 (na primeira DirectDrawCreate, depois da NickDLL).
pub unsafe fn limpar_marca() {
    let sub = b"Software\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers\0";
    let mut chave = 0usize;
    if RegOpenKeyExA(HKCU, sub.as_ptr(), 0, 0x0002_0006 | 0x0001, &mut chave) != 0 {
        crate::registro::linha("clear admin flag: no compatibility flags for this exe");
        return;
    }
    let mut exe = caminho_do_exe();
    exe.push(0);
    let mut buf = [0u8; 512];
    let mut tam = buf.len() as u32 - 1;
    let lido = RegQueryValueExA(chave, exe.as_ptr(), core::ptr::null_mut(), core::ptr::null_mut(), buf.as_mut_ptr(), &mut tam);
    let atual = String::from_utf8_lossy(&buf[..(tam as usize).min(buf.len())]).trim_end_matches('\0').to_string();
    let linha = match (lido, sem_admin(&atual)) {
        (0, Some(resto)) if resto.is_empty() || resto == "~" => {
            RegDeleteValueA(chave, exe.as_ptr());
            format!("clear admin flag: removed \"{}\" for this exe (takes effect next time)", atual)
        }
        (0, Some(resto)) => {
            let dados: Vec<u8> = resto.bytes().chain([0]).collect();
            RegSetValueExA(chave, exe.as_ptr(), 0, 1, dados.as_ptr(), dados.len() as u32);
            format!("clear admin flag: \"{}\" -> \"{}\" for this exe (takes effect next time)", atual, resto)
        }
        (0, None) => format!("clear admin flag: nothing to clear (\"{}\")", atual),
        _ => "clear admin flag: no compatibility flags for this exe".to_string(),
    };
    RegCloseKey(chave);
    crate::registro::linha(&linha);
}

#[cfg(test)]
mod testes {
    #[test]
    fn tira_so_as_da_nickdll() {
        assert_eq!(super::sem_admin("WIN95 RUNASADMIN").as_deref(), Some(""));
        assert_eq!(super::sem_admin("DWM8And16BitMitigation WIN95 RUNASADMIN").as_deref(), Some("DWM8And16BitMitigation"));
        assert_eq!(super::sem_admin("~ RUNASADMIN").as_deref(), Some("~"));
        assert_eq!(super::sem_admin("WIN95").as_deref(), Some(""));
        assert_eq!(super::sem_admin("HIGHDPIAWARE"), None);
    }
}
