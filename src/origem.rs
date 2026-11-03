//! `regen_origins = 1`: de quem é cada regen. O jogo cria o regen (`regenerate_as_youth_player` 0x7abac0) numa vaga
//! (o id e a personalidade de A, o aposentado da frente da fila) com uma ficha de jogador (o PA, as posições, os pés, a
//! nação e o perfil dos atributos de B, o jogador aposentado dono da primeira ficha da reserva); nada disso fica no save
//! (docs/MARCA_DO_REGEN.md no projeto CM0102, §4 e §8). A DLL desvia a única chamada (0x7a664f, em
//! `process_regen_queue`), lê A e B antes da criação, o regen depois, e grava uma linha em `regen_origins.csv`, ao lado
//! do exe, com a data do computador, a data do jogo e a carreira (o save carregado ou gravado por último; antes do
//! primeiro save, "new career" com a data em que a DLL abriu).

use core::ffi::c_void;
use std::io::Write;

#[link(name = "kernel32")]
extern "system" {
    fn GetLocalTime(t: *mut [u16; 8]);
}

unsafe fn d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A data e a hora do computador, AAAA-MM-DD HH:MM:SS.
pub fn agora() -> String {
    let mut t = [0u16; 8];
    unsafe { GetLocalTime(&mut t) };
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", t[0], t[1], t[3], t[4], t[5], t[6])
}

/// Um texto do jogo (até 51 bytes, Latin-1), sem tabulação nem ponto e vírgula.
unsafe fn texto(p: usize) -> String {
    if p == 0 {
        return String::new();
    }
    let s = core::slice::from_raw_parts(p as *const u8, 51);
    s.iter().take_while(|&&c| c != 0).map(|&c| if c == b';' || c == b'\t' { ' ' } else { c as char }).collect()
}

/// O nome da pessoa: o nome comum, se houver; senão o primeiro nome e o sobrenome (+4, +8, +0xc apontam para os nomes).
unsafe fn nome_da_pessoa(p: usize) -> String {
    let comum = texto(d(p + 0xc));
    if !comum.trim().is_empty() {
        return comum;
    }
    format!("{} {}", texto(d(p + 4)), texto(d(p + 8))).trim().to_string()
}

unsafe fn nacao(p: usize) -> String {
    let n = d(p + 0x1a);
    if n == 0 { String::new() } else { texto(n + 4) }
}

/// A posição principal da ficha (a maior nota entre +0xf e +0x16: GK, SW, D, DM, M, AM, F, WB).
unsafe fn posicao(ficha: usize) -> &'static str {
    const NOMES: [&str; 8] = ["GK", "SW", "D", "DM", "M", "AM", "F", "WB"];
    let notas: Vec<i8> = (0..8).map(|k| *((ficha + 0xf + k) as *const i8)).collect();
    let i = (0..8).max_by_key(|&k| notas[k]).unwrap_or(0);
    NOMES[i]
}

unsafe fn pessoa(id: usize) -> Option<usize> {
    (id < d(0xae2364)).then(|| d(0xae23bc) + 110 * id)
}

/// A carreira: o nome do save carregado ou gravado por último, ou "new career (AAAA-MM-DD HH:MM:SS)". Texto fixo, sem
/// alocar no gancho do CreateFileA (que roda para todo arquivo do jogo).
static mut CARREIRA: [u8; 260] = [0; 260];

fn carreira() -> String {
    let c = unsafe { &*core::ptr::addr_of!(CARREIRA) };
    String::from_utf8_lossy(&c[..c.iter().position(|&b| b == 0).unwrap_or(c.len())]).into_owned()
}

fn definir_carreira(nome: &[u8]) {
    let c = unsafe { &mut *core::ptr::addr_of_mut!(CARREIRA) };
    let n = nome.len().min(c.len() - 1);
    c[..n].copy_from_slice(&nome[..n]);
    c[n] = 0;
}

fn caminho_do_arquivo() -> std::path::PathBuf {
    std::env::current_exe().map(|p| p.with_file_name("regen_origins.csv")).unwrap_or_else(|_| "regen_origins.csv".into())
}

/// Uma linha no `regen_origins.csv` (cria o arquivo com o cabeçalho na primeira vez).
fn gravar(linha: &str) {
    let caminho = caminho_do_arquivo();
    let novo = !caminho.exists();
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&caminho) {
        if novo {
            let _ = writeln!(f, "real_time;game_date;career;event;regen_id;regen_name;age;nation;position;PA;CA;club;slot_from_id;slot_from_name;slot_from_age;slot_from_was;record_from_id;record_from_name;record_from_age_at_retirement;record_from_CA;record_from_PA;record_from_nation");
        }
        let _ = writeln!(f, "{}", linha);
    }
}

/// O dono de cada ficha que entra na reserva, com o nome e a idade **do momento da aposentadoria** (o id pode virar
/// outra pessoa depois, quando ele mesmo vira regen). Tabela fixa da DLL, indexada pela ficha: nada muda na memória do
/// jogo nem no save. Fichas que já estavam na reserva antes desta sessão (num save carregado) ficam sem dono conhecido.
#[derive(Clone, Copy)]
struct Dono {
    id_mais_1: u32,
    idade: u8,
    ca: i16,
    pa: i16,
    nome: [u8; 43],
}

const SEM_DONO: Dono = Dono { id_mais_1: 0, idade: 0, ca: 0, pa: 0, nome: [0; 43] };
static mut FICHAS: [Dono; 1 << 18] = [SEM_DONO; 1 << 18];

/// Anota o dono da ficha (o jogador `p`), se a ficha cabe na tabela.
unsafe fn anotar_dono(ficha: usize, p: usize) {
    if ficha >= 1 << 18 {
        return;
    }
    // o CA e o PA da ficha dele, agora (o regen fica com o PA; o CA é sorteado de novo)
    let f = d(0xae23c4) + 70 * ficha;
    let (ca, pa) = (core::ptr::read_unaligned((f + 5) as *const i16), core::ptr::read_unaligned((f + 7) as *const i16));
    let mut dono = Dono { id_mais_1: d(p) as u32 + 1, idade: *((p + 0x18) as *const u8), ca, pa, nome: [0; 43] };
    let nome = nome_da_pessoa(p);
    let bytes: Vec<u8> = nome.chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect();
    let n = bytes.len().min(42);
    dono.nome[..n].copy_from_slice(&bytes[..n]);
    (*core::ptr::addr_of_mut!(FICHAS))[ficha] = dono;
}

/// { chamada em 0x7a6335 } (thiscall, ret 0xc): `add_player_data_to_pool(this, ficha, nação, segunda nação)`, quando o
/// jogador que se aposenta vira técnico. Depois da chamada o jogador ainda aponta para a ficha (o jogo zera o +0x61 na
/// instrução seguinte, 0x7a633a): é ele o dono.
unsafe extern "thiscall" fn guardar_ficha(this: usize, ficha: usize, nacao: u32, nacao2: u32) {
    let original: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x7a8350usize);
    original(this, ficha, nacao, nacao2);
    let alvo = d(0xae23c4) + 70 * ficha;
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    if let Some(s) = (0..n).find(|&s| d(pessoas + 110 * s + 0x61) == alvo) {
        anotar_dono(ficha, pessoas + 110 * s);
    }
}

/// { chamadas em 0x7a64c9 e 0x7a64e2 } (thiscall, ret 4): `queue_staff_for_regen(this, registro)`, o registro com o id
/// da pessoa em +0. O jogador que entra na fila põe a ficha dele na reserva (o dono em +0xc da entrada): anota o nome
/// dele agora, antes de virar regen.
unsafe extern "thiscall" fn guardar_fila(this: usize, registro: usize) -> u32 {
    let p = match pessoa(d(registro)) {
        Some(p) => p,
        None => {
            let original: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7a87d0usize);
            return original(this, registro);
        }
    };
    let antes = d(this + 0x40) as i32;
    let original: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7a87d0usize);
    let r = original(this, registro);
    let depois = d(this + 0x40) as i32;
    if depois > antes && depois > 0 {
        let e = d(this + 0x3c) + 16 * (depois as usize - 1);
        if d(e + 0xc) == d(p) {
            anotar_dono(d(e), p);
        }
    }
    r
}

static mut CREATE_FILE_ORIGINAL: usize = 0;

/// `CreateFileA` do exe: quando o jogo abre um `.sav`, anota a carreira (o nome do arquivo) e, na primeira gravação de
/// uma carreira nova, liga as duas no arquivo.
unsafe extern "system" fn criar_arquivo(nome: *const u8, acesso: u32, comp: u32, seg: *mut c_void, disp: u32, flags: u32, modelo: *mut c_void) -> *mut c_void {
    type F = unsafe extern "system" fn(*const u8, u32, u32, *mut c_void, u32, u32, *mut c_void) -> *mut c_void;
    let f: F = core::mem::transmute(CREATE_FILE_ORIGINAL);
    if !nome.is_null() {
        let n = (0..520).position(|k| *nome.add(k) == 0).unwrap_or(0);
        let s = core::slice::from_raw_parts(nome, n);
        if n > 4 && s[n - 4..].eq_ignore_ascii_case(b".sav") {
            let arquivo = s.rsplit(|&c| c == b'\\' || c == b'/').next().unwrap_or(s);
            let antes = carreira();
            let gravando = acesso & 0x4000_0000 != 0; // GENERIC_WRITE
            if antes.as_bytes() != arquivo {
                definir_carreira(arquivo);
                gravar(&format!("{};{};{};{} {}", agora(), crate::data_do_jogo(), carreira(), if gravando { "saved as" } else { "loaded" },
                    if antes.starts_with("new career") { format!("(was {})", antes) } else { String::new() }));
            }
        }
    }
    f(nome, acesso, comp, seg, disp, flags, modelo)
}

/// { chamada em 0x7a664f } (thiscall, ret 4): `regenerate_as_youth_player(this, pessoa)` com o registro da origem.
unsafe extern "thiscall" fn criar_regen(this: usize, p: usize) -> u32 {
    // A, o dono da vaga, antes de o jogo sobrescrever
    let (a_id, a_nome, a_idade) = (d(p), nome_da_pessoa(p), *((p + 0x18) as *const u8));
    let a_era = if d(p + 0x61) != 0 { "player" } else { "non-player" };
    // B, o dono da primeira ficha da reserva (this+0x3c, entradas de 16 bytes; +0 a ficha, +0xc o dono, quando o
    // jogador entrou na reserva pela fila; quem virou técnico entra sem o dono)
    let mut b = (String::new(), ";;;".to_string(), String::new());
    if (d(this + 0x40) as i32) > 0 {
        let e = d(this + 0x3c);
        let ficha = d(e);
        let dono = if ficha < 1 << 18 { (*core::ptr::addr_of!(FICHAS))[ficha] } else { SEM_DONO };
        b = if dono.id_mais_1 != 0 {
            let nome: String = dono.nome.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect();
            ((dono.id_mais_1 - 1).to_string(), format!("{};{};{};{}", nome, dono.idade, dono.ca, dono.pa), texto(d(0xae23a8) + 0x122 * d(e + 4) + 4))
        } else if let Some(pb) = pessoa(d(e + 0xc)).filter(|&pb| d(pb + 0x61) == d(0xae23c4) + 70 * ficha) {
            // a ficha já estava na reserva (save carregado), mas o dono gravado pela fila ainda aponta para ela
            (d(pb).to_string(), format!("{};{};{};{}", nome_da_pessoa(pb), *((pb + 0x18) as *const u8),
                core::ptr::read_unaligned((d(pb + 0x61) + 5) as *const i16), core::ptr::read_unaligned((d(pb + 0x61) + 7) as *const i16)), nacao(pb))
        } else {
            ("?".into(), "(unknown: in the pool before this session);;;".into(), String::new())
        };
    }
    let original: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7abac0usize);
    let r = original(this, p);
    // o regen, se nasceu (o bit 0x40 do registro de 79 bytes)
    if *((d(0xae2d0c) + 79 * d(p) + 0xb) as *const u8) & 0x40 != 0 && d(p + 0x61) != 0 {
        let ficha = d(p + 0x61);
        let clube = d(p + 0x39);
        gravar(&format!("{};{};{};regen;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}", agora(), crate::data_do_jogo(), carreira(),
            d(p), nome_da_pessoa(p), *((p + 0x18) as *const u8), nacao(p), posicao(ficha),
            core::ptr::read_unaligned((ficha + 7) as *const i16), core::ptr::read_unaligned((ficha + 5) as *const i16),
            if clube != 0 { texto(clube + 4) } else { "(no club)".into() },
            a_id, a_nome, a_idade, a_era, b.0, b.1, b.2));
    }
    r
}

/// A chamada em 0x7a664f (`call 0x7abac0`): troca o destino; o CreateFileA do exe, para a carreira.
pub unsafe fn ligar(registrar: bool) -> bool {
    const CHAMADA: [u8; 5] = [0xe8, 0x6c, 0x54, 0x00, 0x00];
    if core::slice::from_raw_parts(0x7a664f as *const u8, 5) != CHAMADA {
        if registrar {
            crate::registro::linha("regen origins: this exe differs from 3.9.68 where the regen is created (0x7a664f); not recorded");
        }
        return false;
    }
    definir_carreira(format!("new career ({})", agora()).as_bytes());
    if let Some(a) = crate::pasta::trocar_importacao("KERNEL32.dll", "CreateFileA", criar_arquivo as *const () as usize) {
        CREATE_FILE_ORIGINAL = a;
    }
    crate::chamar(0x7a664f, criar_regen as *const () as usize);
    // os dois caminhos por onde a ficha de um jogador aposentado entra na reserva
    for (de, para, bytes) in [(0x7a6335usize, guardar_ficha as *const () as usize, 0x7a8350usize), (0x7a64c9, guardar_fila as *const () as usize, 0x7a87d0),
        (0x7a64e2, guardar_fila as *const () as usize, 0x7a87d0)] {
        let rel = core::ptr::read_unaligned((de + 1) as *const i32);
        if *(de as *const u8) == 0xe8 && (de + 5).wrapping_add(rel as usize) == bytes {
            crate::chamar(de, para);
        }
    }
    true
}
