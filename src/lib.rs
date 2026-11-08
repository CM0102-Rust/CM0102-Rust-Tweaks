//! CM0102-Rust-Tweaks: a `ddraw.dll` de ajustes do CM 01/02 3.9.68, sem alterar o exe (hoje: o save acima de 2 GB, a idade
//! e a distribuição dos regens).
//!
//! Entra no jogo como `ddraw.dll`, como a do save2gb, e lê `tweaks.cfg` ao lado do exe. Cada parte só liga se a
//! assinatura do código de que depende bater com o 3.9.68:
//! - `save_over_2gb = 1` (padrão): as três rotinas do contêiner do save (`zopen` 0x9326e0, `zseek` 0x933080, `ztell`
//!   0x933170) vão para as de `src/save/conteiner.rs`, como na DLL do save2gb (`TRECHOS_DO_SAVE`);
//! - `rule = new`: `PLAYER_REGEN::pick_best_club_for_player` (0x7ad0e0) vai para `escolher_clube` (`TRECHOS`).
//! O `DirectDrawCreate` é repassado a
//! `ddraw_chain.dll` na pasta do jogo, se existir, senão à `ddraw.dll` do Windows. O registro vai para `tweaks.log`.
//!
//! A regra original (docs/DISTRIBUICAO_DOS_REGENS.md): cada clube do país com elenco abaixo de 50 recebe uma nota
//! (0x7ad220) e ganha o maior sorteio(nota). A nova: os mesmos clubes elegíveis, mas o clube é sorteado na proporção
//! do peso = nível × instalações × comissão × elenco, à potência da temperatura, com um teto de jovens por clube nos
//! últimos 365 dias. As mesmas contas de `tools/regens/calibrar.py` (`peso_novo`).

#![allow(linker_messages)]

use core::ffi::c_void;

// Cada ajuste no seu módulo, numa pasta por assunto (os caminhos com #[path] mantêm `crate::nome` de sempre; video.rs,
// apresentacao.rs, reducao.rs, win.rs e conteiner.rs são cópias do projeto CM0102).
#[path = "save/conteiner.rs"]
mod conteiner;
#[path = "exe/referencia.rs"]
mod referencia;
#[path = "exe/retrato.rs"]
mod retrato;
#[path = "exe/partes.rs"]
mod partes;
use partes::*;
#[path = "exe/desvios.rs"]
mod desvios;
use desvios::*;
#[path = "jogo/pasta.rs"]
mod pasta;
#[path = "jogo/instancias.rs"]
mod instancias;
#[path = "jogo/elenco.rs"]
mod elenco;
use instancias::*;
#[path = "video/apresentacao.rs"]
mod apresentacao;
#[path = "video/reducao.rs"]
mod reducao;
#[path = "video/trocas.rs"]
mod trocas;
#[path = "video/video.rs"]
mod video;
#[path = "video/win.rs"]
mod win;
#[path = "video/tela.rs"]
mod tela;
use tela::*;
#[path = "regens/disputa.rs"]
mod disputa;
use disputa::*;
#[path = "regens/idades.rs"]
mod idades;
use idades::*;
#[path = "regens/inativos.rs"]
mod inativos;
use inativos::*;
#[path = "regens/fornada.rs"]
mod fornada;
#[path = "regens/origem.rs"]
mod origem;
#[path = "regens/nomes.rs"]
mod nomes;
#[path = "regens/cinzentos.rs"]
mod cinzentos;
mod regras;
use regras::*;
mod vigia;
use vigia::*;
mod estudo;
mod sucesso;
mod publico;

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
    fn GetCurrentThreadId() -> u32;
    fn OpenThread(acesso: u32, herdar: i32, id: u32) -> *mut c_void;
    fn SuspendThread(linha: *mut c_void) -> u32;
    fn ResumeThread(linha: *mut c_void) -> u32;
    fn GetThreadContext(linha: *mut c_void, contexto: *mut u8) -> i32;
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

#[no_mangle]
pub unsafe extern "system" fn DllMain(_modulo: *mut c_void, motivo: u32, _reservado: *mut c_void) -> i32 {
    if motivo == 1 {
        let r = ler_regras();
        REGRAS = r;
        LINHA_PRINCIPAL = GetCurrentThreadId();
        if r.registrar {
            AddVectoredExceptionHandler(1, ao_falhar);
        }
        retrato::do_disco(r.registrar);
        if r.instancias {
            trava_por_pasta();
        }
        mutex_dos_testes();
        let s = r.sobrescrever;
        if r.pasta {
            pasta::ligar_pasta();
        }
        ligar_tela(&mut *core::ptr::addr_of_mut!(REGRAS));
        if r.origens && origem::ligar(r.registrar) {
            registro::linha("regen origins: on (regen_origins.csv next to the exe: each regen, the slot he took and the player record he inherited)");
        }
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
            let protecao = if r.proteger_jovens > 0 { format!("players under {} protected", r.proteger_jovens) } else { "no protection for young players".to_string() };
            registro::linha(&format!("retire idle players: on ({}+ years and {}+ days without a club; {})", r.inativo_idade, r.inativo_dias, protecao));
            // o NickPatcher (exes do CLZ) pula, na busca de aposentadorias (0x7a5790), quem tem menos de 34 anos: um `jmp` em
            // 0x7a5806 para a seção dele. A regra dos inativos continua ligada, mas quase não acha ninguém para avaliar
            if *(0x7a5806 as *const u8) == 0xe9 {
                registro::linha("retire idle players: note: this exe skips retirements under 34 (a NickPatcher option), so this rule will rarely find anyone");
            }
        }
        if !r.nomes {
            registro::linha("regen names: original (a nation uses its own names only above 100; otherwise it borrows)");
        } else if pode_ligar(&NOMES, s) {
            nomes::MINIMO = r.nomes_minimo;
            if nomes::instalar() {
                registro::linha(&format!("regen names: own (a nation with {} or more names uses its own; fewer, the original's borrowing)", r.nomes_minimo));
            } else {
                registro::linha("regen names: could not allocate the code stub; nothing was changed");
            }
        }
        if !r.cinzentos {
            registro::linha("grey players: original (ability from the club reputation)");
        } else if pode_ligar(&CINZENTOS, s) {
            cinzentos::ESPALHAR = r.cinzentos_espalhar;
            cinzentos::REGISTRAR = r.registrar;
            if cinzentos::instalar() {
                registro::linha(&format!("grey players: squad (ability = the club's level, the higher of its own squad and its reputation, within a spread of {})", r.cinzentos_espalhar));
            } else {
                registro::linha("grey players: could not allocate the code stub; nothing was changed");
            }
        }
        // a fornada (experimental): depois da disputa, cujo call em 0x7acea2 ela encadeia; ver o fim do DllMain
        // depois da IDADE, que confere 0x7ace65..0x7ace6b antes de mexer no corte
        if !r.so_sem_vaga {
            registro::linha("unattached: original (under 19, 3 times in 4, go to the clubs' contest; the rest start without a club)");
        } else if pode_ligar(&SEM_CLUBE, s) {
            nops(0x7ace6a, 6); // o corte da idade
            nops(0x7ace89, 0x12); // o sorteio de 1 em 4
            registro::linha("unattached: only_if_no_room (every youngster from a nation in the game goes to the contest; without a club only if no club there has room)");
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
        } else if r.registrar && assinatura(DISPUTA.bordas) == DISPUTA.assinatura_das_bordas {
            // a disputa ficou com o exe (override 0): observar a escolha dele, no mesmo formato
            DISPUTA_ROTULO = "exe";
            if instalar_observacao() {
                registro::linha("regens 0.1: logging the exe's own picks (log = 1)");
            }
        }
        if !fornada::instalar() {
            registro::linha("intake: off (intake = 0)");
        }
        fornada::instalar_aposentadoria();
        sucesso::instalar();
        publico::instalar();
        elenco::instalar();
        if estudo::ligado() {
            estudo::instalar();
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
    if !CONFERIDO {
        CONFERIDO = true;
        if REGRAS.limpar_marca {
            pasta::limpar_marca();
        }
        retrato::da_memoria(&*core::ptr::addr_of!(MARCAS), &*core::ptr::addr_of!(LIGADAS), REGRAS.registrar);
        if REGRAS.registrar {
            conferir_desvios();
            iniciar_batida();
            instalar_caixa(); // depois do retrato da memória, que assim não o conta
        }
    }
    if REGRAS.video {
        *saida = video::criar();
        return 0;
    }
    match proxima_ddraw() {
        Some(f) => f(guid, saida, externo),
        None => {
            registro::linha("regens: could not load the next ddraw.dll");
            0x8000_4005_u32 as i32
        }
    }
}
