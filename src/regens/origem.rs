//! `regen_origins = 1`: de quem é cada regen. O jogo cria o regen (`regenerate_as_youth_player` 0x7abac0) numa vaga
//! (o id e a personalidade de A, o aposentado da frente da fila) com uma ficha de jogador (o PA, as posições, os pés, a
//! nação e o perfil dos atributos de B, o jogador aposentado dono da primeira ficha da reserva); nada disso fica no save
//! (docs/MARCA_DO_REGEN.md no projeto CM0102, §4 e §8). A DLL desvia a única chamada (0x7a664f, em
//! `process_regen_queue`), lê A e B antes da criação, o regen depois, e grava uma linha em `regen_origins.csv`, ao lado
//! do exe, com a data do computador, a data do jogo e a carreira (o save carregado ou gravado por último; antes do
//! primeiro save, "new career" com a data em que a DLL abriu).

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

/// As linhas que não puderam ser gravadas (o arquivo aberto no Excel fica trancado): vão na próxima gravação que der
/// certo, na ordem. Só a linha de execução do jogo mexe aqui.
static mut PENDENTES: Vec<String> = Vec::new();

/// Uma linha no `regen_origins.csv` (cria o arquivo com o cabeçalho na primeira vez).
fn gravar(linha: &str) {
    let pendentes = unsafe { &mut *core::ptr::addr_of_mut!(PENDENTES) };
    pendentes.push(linha.to_string());
    descarregar();
}

/// Grava as linhas pendentes; se o arquivo está trancado, elas esperam (e o tweaks.log avisa na primeira vez).
fn descarregar() {
    let pendentes = unsafe { &mut *core::ptr::addr_of_mut!(PENDENTES) };
    if pendentes.is_empty() {
        return;
    }
    let caminho = caminho_do_arquivo();
    let novo = !caminho.exists();
    let mut texto = String::new();
    if novo {
        // o BOM: o Excel abre o arquivo com dois cliques como UTF-8 (sem ele, os acentos saem errados)
        texto.push('\u{feff}');
        texto.push_str("real_time;game_date;career;event;regen_id;regen_name;age;nation;position;PA;CA;club;slot_from_id;slot_from_name;slot_from_age;slot_from_was;record_from_id;record_from_name;record_from_age_at_retirement;record_from_CA;record_from_PA;record_from_nation\n");
    }
    for l in pendentes.iter() {
        texto.push_str(l);
        texto.push('\n');
    }
    let ok = std::fs::OpenOptions::new().create(true).append(true).open(&caminho).and_then(|mut f| f.write_all(texto.as_bytes())).is_ok();
    if ok {
        pendentes.clear();
    } else if pendentes.len() == 1 {
        crate::registro::linha("regen origins: regen_origins.csv is locked (open in Excel?); the lines wait in memory and are written when it is free");
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
        crate::registro::linha(&format!("regen origins: player record {} is beyond the owners table (262144); its owner is not kept", ficha));
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

/// { chamada em 0x7a6335 } (thiscall, ret 0xc): `add_player_data_to_pool(this, pessoa, nação, segunda nação)`, quando o
/// jogador que se aposenta tem também registro de não jogador e segue como técnico (o 1º argumento é o ÍNDICE da pessoa,
/// não a ficha: a original grava em +0 o id da ficha dela, +0x61, que o jogo só zera depois, em 0x7a633a). É ela a dona.
unsafe extern "thiscall" fn guardar_ficha(this: usize, quem: usize, nacao: u32, nacao2: u32) {
    let p = pessoa(quem).filter(|&p| d(p + 0x61) != 0);
    let ficha = p.map(|p| d(d(p + 0x61)));
    let original: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x7a8350usize);
    original(this, quem, nacao, nacao2);
    if let (Some(p), Some(ficha)) = (p, ficha) {
        anotar_dono(ficha, p);
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

/// O caminho de um texto C (até 520 bytes).
unsafe fn caminho_c(p: *const u8) -> std::path::PathBuf {
    if p.is_null() {
        return std::path::PathBuf::new();
    }
    let n = (0..520).position(|k| *p.add(k) == 0).unwrap_or(0);
    let bytes = core::slice::from_raw_parts(p, n);
    std::path::PathBuf::from(bytes.iter().map(|&c| c as char).collect::<String>())
}

/// A carreira passa a ser o save `caminho` (gravado ou carregado); uma linha de evento no arquivo quando muda.
unsafe fn trocar_carreira(caminho: &std::path::Path, gravando: bool) {
    // o nome como está no disco (o -load chega em minúsculas): a mesma carreira com um nome só
    let real = std::fs::canonicalize(caminho).unwrap_or_else(|_| caminho.to_path_buf());
    let arquivo = real.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let antes = carreira();
    if antes != arquivo {
        definir_carreira(arquivo.as_bytes());
        gravar(&format!("{};{};{};{} {}", agora(), crate::data_do_jogo(), carreira(), if gravando { "saved as" } else { "loaded" },
            if antes.starts_with("new career") { format!("(was {})", antes) } else { String::new() }));
    }
}

/// Os donos das fichas vão para um arquivo ao lado do save (`X.sav` → `X.regens`), gravado junto com ele e lido na
/// carga: assim a carreira carregada segue sabendo de quem é cada ficha que já estava na reserva. O save não muda.
/// Formato: "CMRTREG2", a reserva no momento da gravação (a quantidade u32 e as fichas u32, na ordem), depois, por ficha
/// anotada, o índice (u32) e o dono (id+1 u32, idade u8, CA i16, PA i16, nome 43). Na carga, o arquivo só vale se a
/// reserva do save for exatamente a gravada nele: um .regens de outra gravação (o save regravado com o registro
/// desligado, ou por outra DLL, ou um .regens copiado de outro save) daria donos errados, porque o id da ficha volta
/// a ser usado quando um regen se aposenta.
const MARCA_DONOS: &[u8; 8] = b"CMRTREG2";
const TAM_DONO: usize = 4 + 4 + 1 + 2 + 2 + 43;

fn caminho_dos_donos(save: &std::path::Path) -> std::path::PathBuf {
    save.with_extension("regens")
}

/// A reserva de fichas do PLAYER_REGEN ([0xb7434c], `g_player_regen`): as fichas, na ordem em que o jogo as usa.
unsafe fn reserva() -> Vec<usize> {
    let r = d(0xb7434c);
    if r == 0 || d(r + 0x3c) == 0 {
        return Vec::new();
    }
    let n = (d(r + 0x40) as i32).max(0) as usize;
    (0..n).map(|k| d(d(r + 0x3c) + 16 * k)).collect()
}

/// Grava ao lado do save os donos das fichas que estão na reserva (as que ainda vão virar regen).
unsafe fn gravar_donos(save: &std::path::Path) {
    let fichas = &*core::ptr::addr_of!(FICHAS);
    let na_reserva = reserva();
    let conhecidas = na_reserva.iter().filter(|&&i| i < fichas.len() && fichas[i].id_mais_1 != 0).count();
    crate::registro::linha(&format!("regen origins: saved; {} records in the pool, {} with a known owner", na_reserva.len(), conhecidas));
    let mut v = MARCA_DONOS.to_vec();
    v.extend_from_slice(&(na_reserva.len() as u32).to_le_bytes());
    for &i in &na_reserva {
        v.extend_from_slice(&(i as u32).to_le_bytes());
    }
    for (i, f) in na_reserva.iter().filter(|&&i| i < fichas.len()).map(|&i| (i, &fichas[i])).filter(|(_, f)| f.id_mais_1 != 0) {
        v.extend_from_slice(&(i as u32).to_le_bytes());
        v.extend_from_slice(&f.id_mais_1.to_le_bytes());
        v.push(f.idade);
        v.extend_from_slice(&f.ca.to_le_bytes());
        v.extend_from_slice(&f.pa.to_le_bytes());
        v.extend_from_slice(&f.nome);
    }
    if std::fs::write(caminho_dos_donos(save), &v).is_err() {
        crate::registro::linha("regen origins: could not write the record owners next to the save");
    }
}

/// Zera as anotações (outra carreira) e lê as do save, se ele tem o arquivo dos donos e o arquivo é desta gravação
/// (a reserva carregada igual à gravada nele). Chamada depois da carga, com a reserva do save já na memória.
unsafe fn ler_donos(save: Option<&std::path::Path>) {
    let fichas = &mut *core::ptr::addr_of_mut!(FICHAS);
    fichas.fill(SEM_DONO);
    let Some(save) = save else { return };
    let Ok(v) = std::fs::read(caminho_dos_donos(save)) else { return };
    let u = |o: usize| v.get(o..o + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    let n = if v.len() >= 12 && &v[..8] == MARCA_DONOS { u(8) } else { None };
    let Some(n) = n.filter(|&n| v.len() >= 12 + 4 * n && (v.len() - 12 - 4 * n) % TAM_DONO == 0) else {
        crate::registro::linha("regen origins: the .regens next to the save is not valid (or from an older version); ignored");
        return;
    };
    let gravada: Vec<usize> = (0..n).filter_map(|k| u(12 + 4 * k)).collect();
    if gravada != reserva() {
        crate::registro::linha("regen origins: the .regens next to the save is from another save of it (the pool differs); ignored");
        return;
    }
    for r in v[12 + 4 * n..].chunks_exact(TAM_DONO) {
        let i = u32::from_le_bytes([r[0], r[1], r[2], r[3]]) as usize;
        if i < fichas.len() {
            let mut nome = [0u8; 43];
            nome.copy_from_slice(&r[13..56]);
            fichas[i] = Dono { id_mais_1: u32::from_le_bytes([r[4], r[5], r[6], r[7]]), idade: r[8],
                ca: i16::from_le_bytes([r[9], r[10]]), pa: i16::from_le_bytes([r[11], r[12]]), nome };
        }
    }
}

/// Depois da carga: cada ficha da reserva sem dono lido do arquivo ganha o dono que a fila gravou na entrada (+0xc),
/// se ele ainda aponta para ela (não virou regen com outra ficha). O tweaks.log conta de onde veio cada dono.
unsafe fn completar_reserva() {
    let r = d(0xb7434c);
    let na_reserva = reserva();
    let (mut do_arquivo, mut do_jogo, mut sem) = (0, 0, 0);
    for (k, &i) in na_reserva.iter().enumerate() {
        if i < 1 << 18 && (*core::ptr::addr_of!(FICHAS))[i].id_mais_1 != 0 {
            do_arquivo += 1;
        } else if let Some(pb) = pessoa(d(d(r + 0x3c) + 16 * k + 0xc)).filter(|&pb| d(pb + 0x61) == d(0xae23c4) + 70 * i) {
            anotar_dono(i, pb);
            do_jogo += 1;
        } else {
            sem += 1;
        }
    }
    crate::registro::linha(&format!("regen origins: loaded; {} records in the pool: {} owners from the .regens file, {} from the game, {} unknown",
        na_reserva.len(), do_arquivo, do_jogo, sem));
}

/// Os trampolins das três rotinas desviadas na entrada (o começo original e um salto de volta).
static mut CARREGAR_ORIGINAL: usize = 0;
/// O registro das origens ligado (os ganchos da carga e da gravação também servem ao público: `ganchos_do_save`).
static mut ORIGENS: bool = false;
static mut GANCHOS: bool = false;
static mut GRAVAR_ORIGINAL: usize = 0;
static mut LIBERAR_ORIGINAL: usize = 0;

/// Desvia a entrada `entrada` para `novo` se ela começa com `esperado` (instruções inteiras, sem endereço relativo, 5
/// bytes ou mais): o trampolim devolvido roda o começo original e volta para o resto da rotina. Na entrada, e não nas
/// chamadas: outra DLL que desvie as chamadas (a NickDLL do CLZ 6.1 aponta para as da carga) chega aqui do mesmo jeito.
pub(crate) unsafe fn trampolim(entrada: usize, esperado: &[u8], novo: usize) -> Option<usize> {
    if core::slice::from_raw_parts(entrada as *const u8, esperado.len()) != esperado {
        return None;
    }
    let t = crate::VirtualAlloc(core::ptr::null_mut(), 32, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return None;
    }
    core::ptr::copy_nonoverlapping(esperado.as_ptr(), t, esperado.len());
    let n = esperado.len();
    *t.add(n) = 0xe9;
    core::ptr::write_unaligned(t.add(n + 1) as *mut i32, (entrada + n).wrapping_sub(t as usize + n + 5) as i32);
    crate::desviar(entrada, novo);
    Some(t as usize)
}

/// { 0x82a9c0 } (cdecl): `load_game_data(caminho)`, 1 se carregou.
unsafe extern "C" fn carregar_jogo(caminho: *const u8) -> u32 {
    let original: unsafe extern "C" fn(*const u8) -> u32 = core::mem::transmute(CARREGAR_ORIGINAL);
    let r = original(caminho);
    if r == 1 {
        let c = caminho_c(caminho);
        if ORIGENS {
            ler_donos(Some(&c));
            trocar_carreira(&c, false);
            completar_reserva();
        }
        crate::publico::carregou(&c);
    }
    r
}

/// { 0x82e430 } (cdecl): `save_game_data(caminho, tipo)`, 1 se gravou (tipo 0x16 `.sav`, 0x17 `.qst`). O caminho é o
/// do save final, mesmo quando o jogo grava antes num temp_save_file e renomeia (o bit 0x20000000 do game.cfg).
unsafe extern "C" fn gravar_jogo(caminho: *const u8, tipo: u32) -> u32 {
    let original: unsafe extern "C" fn(*const u8, u32) -> u32 = core::mem::transmute(GRAVAR_ORIGINAL);
    let r = original(caminho, tipo);
    if r == 1 && tipo == 0x16 {
        let c = caminho_c(caminho);
        if ORIGENS {
            gravar_donos(&c);
            trocar_carreira(&c, true);
        }
        crate::publico::gravou(&c);
    }
    if ORIGENS {
        descarregar();
    }
    r
}

/// { 0x82ff40 } (cdecl): `free_game_data(flag)`: o jogo apaga a carreira; a próxima (nova ou carregada) começa sem
/// anotações.
unsafe extern "C" fn liberar_jogo(flag: u32) -> u32 {
    let original: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(LIBERAR_ORIGINAL);
    let r = original(flag);
    if ORIGENS {
        ler_donos(None);
        definir_carreira(format!("new career ({})", agora()).as_bytes());
        guardar_pendentes();
    }
    crate::publico::liberou();
    r
}

/// O jogo fecha ou troca de carreira com o regen_origins.csv ainda trancado: as linhas pendentes vão para
/// regen_origins_pending.txt e entram no .csv na próxima vez que a DLL abrir (`ligar`).
fn guardar_pendentes() {
    descarregar();
    let pendentes = unsafe { &mut *core::ptr::addr_of_mut!(PENDENTES) };
    if !pendentes.is_empty() {
        let caminho = caminho_do_arquivo().with_file_name("regen_origins_pending.txt");
        let texto: String = pendentes.iter().map(|l| format!("{}\n", l)).collect();
        if std::fs::OpenOptions::new().create(true).append(true).open(&caminho).and_then(|mut f| f.write_all(texto.as_bytes())).is_ok() {
            pendentes.clear();
        }
    }
}

/// As linhas que ficaram em regen_origins_pending.txt numa sessão anterior.
fn recuperar_pendentes() {
    let caminho = caminho_do_arquivo().with_file_name("regen_origins_pending.txt");
    if let Ok(t) = std::fs::read_to_string(&caminho) {
        let pendentes = unsafe { &mut *core::ptr::addr_of_mut!(PENDENTES) };
        pendentes.extend(t.lines().filter(|l| !l.is_empty()).map(str::to_string));
        let _ = std::fs::remove_file(&caminho);
        descarregar();
    }
}

/// { chamada em 0x7a664f } (thiscall, ret 4): `regenerate_as_youth_player(this, pessoa)` com o registro da origem.
unsafe extern "thiscall" fn criar_regen(this: usize, p: usize) -> u32 {
    // A, o dono da vaga, antes de o jogo sobrescrever
    let (a_id, a_nome, a_idade) = (d(p), nome_da_pessoa(p), *((p + 0x18) as *const u8));
    let a_era = if d(p + 0x61) != 0 { "player" } else { "non-player" };
    // B, o dono da primeira ficha da reserva (this+0x3c, entradas de 16 bytes: +0 a ficha, +4 a nação, +0xc o dono
    // quando entrou pela fila). A 0x7abac0 usa sempre a da frente (e a tira pela 0x7a8470).
    let antes = d(this + 0x40) as i32;
    if antes <= 0 {
        let original: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7abac0usize);
        return original(this, p);
    }
    let e = d(this + 0x3c);
    let ficha_id = d(e);
    let b_nacao = if d(e + 4) < d(0xae2350) { texto(d(0xae23a8) + 0x122 * d(e + 4) + 4) } else { String::new() };
    let dono = if ficha_id < 1 << 18 { (*core::ptr::addr_of!(FICHAS))[ficha_id] } else { SEM_DONO };
    let b = if dono.id_mais_1 != 0 {
        let nome: String = dono.nome.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect();
        ((dono.id_mais_1 - 1).to_string(), format!("{};{};{};{}", nome, dono.idade, dono.ca, dono.pa))
    } else if let Some(pb) = pessoa(d(e + 0xc)).filter(|&pb| d(pb + 0x61) == d(0xae23c4) + 70 * ficha_id) {
        // sem anotação (save sem o arquivo de donos), mas o dono gravado pela fila ainda aponta para a ficha
        (d(pb).to_string(), format!("{};{};{};{}", nome_da_pessoa(pb), *((pb + 0x18) as *const u8),
            core::ptr::read_unaligned((d(pb + 0x61) + 5) as *const i16), core::ptr::read_unaligned((d(pb + 0x61) + 7) as *const i16)))
    } else {
        ("?".into(), "(unknown: the save was loaded without its .regens file);;;".into())
    };
    // a 0x7abac0 desiste (só limpa os bits 0x30) quando a pessoa não está aposentada (bits 0x30 do +0xb do registro de
    // 79 bytes) ou tem clube: aí não nasce regen, nem há linha, mesmo que ela já tenha sido regen antes
    let vai = *((d(0xae2d0c) + 79 * d(p) + 0xb) as *const u8) & 0x30 != 0 && d(p + 0x39) == 0;
    let original: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7abac0usize);
    let r = original(this, p);
    let ficha = d(p + 0x61);
    if vai && ficha == d(0xae23c4) + 70 * ficha_id {
        let clube = d(p + 0x39);
        if b.0 == "?" {
            crate::registro::linha(&format!("regen origins: unknown record owner for person {} (record {})", d(p), ficha_id));
        }
        gravar(&format!("{};{};{};regen;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}", agora(), crate::data_do_jogo(), carreira(),
            d(p), nome_da_pessoa(p), *((p + 0x18) as *const u8), nacao(p), posicao(ficha),
            core::ptr::read_unaligned((ficha + 7) as *const i16), core::ptr::read_unaligned((ficha + 5) as *const i16),
            if clube != 0 { texto(clube + 4) } else { "(no club)".into() },
            a_id, a_nome, a_idade, a_era, b.0, b.1, b_nacao));
    }
    r
}

/// O começo da carga (`push -1; mov eax, fs:[0]`), da gravação (`mov eax, fs:[0]`) e da liberação (`push ebx;
/// mov ecx, 0xb74450`) no 3.9.68.
const ENTRADA_CARREGAR: [u8; 8] = [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00];
const ENTRADA_GRAVAR: [u8; 6] = [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00];
const ENTRADA_LIBERAR: [u8; 6] = [0x53, 0xb9, 0x50, 0x44, 0xb7, 0x00];

/// Liga o registro: a criação do regen (0x7a664f), as duas entradas de ficha na reserva (a fila e o técnico), e a carga,
/// a gravação e a liberação do jogo (a carreira e o arquivo dos donos). Só liga se TODOS os pontos estão como no
/// 3.9.68: com um faltando, haveria regens com o dono errado ou desconhecido.
pub unsafe fn ligar(registrar: bool) -> bool {
    let chamadas: [(usize, usize, usize); 4] = [
        (0x7a664f, 0x7abac0, criar_regen as *const () as usize),
        (0x7a6335, 0x7a8350, guardar_ficha as *const () as usize),
        (0x7a64c9, 0x7a87d0, guardar_fila as *const () as usize),
        (0x7a64e2, 0x7a87d0, guardar_fila as *const () as usize),
    ];
    let chama = |de: usize, alvo: usize| *(de as *const u8) == 0xe8 && (de + 5).wrapping_add(core::ptr::read_unaligned((de + 1) as *const i32) as usize) == alvo;
    let comeca = |a: usize, b: &[u8]| core::slice::from_raw_parts(a as *const u8, b.len()) == b;
    let falta = chamadas.iter().find(|&&(de, alvo, _)| !chama(de, alvo)).map(|&(de, alvo, _)| format!("{:#x} (the call to {:#x})", de, alvo))
        .or_else(|| [(0x82a9c0usize, &ENTRADA_CARREGAR[..]), (0x82e430, &ENTRADA_GRAVAR[..]), (0x82ff40, &ENTRADA_LIBERAR[..])].iter()
            .find(|&&(a, b)| !comeca(a, b)).map(|&(a, _)| format!("{:#x} (the start of the routine)", a)));
    if let Some(onde) = falta {
        if registrar {
            crate::registro::linha(&format!("regen origins: this exe differs from 3.9.68 at {}; not recorded", onde));
        }
        return false;
    }
    definir_carreira(format!("new career ({})", agora()).as_bytes());
    recuperar_pendentes();
    for &(de, _, para) in &chamadas {
        crate::chamar(de, para);
    }
    ORIGENS = true;
    ganchos_do_save()
}

/// Os desvios da carga (0x82a9c0), da gravação (0x82e430) e da liberação (0x82ff40) do jogo, uma vez só: servem ao
/// registro das origens e ao público (o arquivo ao lado do save). Falso se alguma entrada não está como no 3.9.68.
pub unsafe fn ganchos_do_save() -> bool {
    if GANCHOS {
        return true;
    }
    let comeca = |a: usize, b: &[u8]| core::slice::from_raw_parts(a as *const u8, b.len()) == b;
    if !comeca(0x82a9c0, &ENTRADA_CARREGAR) || !comeca(0x82e430, &ENTRADA_GRAVAR) || !comeca(0x82ff40, &ENTRADA_LIBERAR) {
        return false;
    }
    CARREGAR_ORIGINAL = trampolim(0x82a9c0, &ENTRADA_CARREGAR, carregar_jogo as *const () as usize).unwrap_or(0x82a9c0);
    GRAVAR_ORIGINAL = trampolim(0x82e430, &ENTRADA_GRAVAR, gravar_jogo as *const () as usize).unwrap_or(0x82e430);
    LIBERAR_ORIGINAL = trampolim(0x82ff40, &ENTRADA_LIBERAR, liberar_jogo as *const () as usize).unwrap_or(0x82ff40);
    GANCHOS = true;
    true
}
