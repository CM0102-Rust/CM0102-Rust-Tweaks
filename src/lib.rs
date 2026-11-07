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
// O retrato do exe contra o 3.9.68 (o arquivo na abertura, a memória depois das DLLs do exe).
mod referencia;
mod retrato;
// A pasta do jogo sem o SetupReg e a marca RUNASADMIN da NickDLL.
mod pasta;
// A tela do projeto CM0102: a camada de vídeo (cópia de video.rs, apresentacao.rs, reducao.rs e win.rs de lá) e as trocas
// do CD e da resolução (geradas por tools/gerar_trocas.py).
// De quem é cada regen (regen_origins.csv).
mod origem;
mod apresentacao;
mod cinzentos;
// A fornada de regens (experimental, desde a 1.2).
mod fornada;
mod nomes;
mod reducao;
mod trocas;
mod video;
mod win;

/// O diagnóstico (as linhas para nós, fora do cfg do release): só com `log = 1`.
pub(crate) fn diagnostico() -> bool {
    unsafe { REGRAS.registrar }
}

/// O registro da camada de vídeo: só com `log = 1`.
pub(crate) fn linha_de_video(texto: &str) {
    if unsafe { REGRAS.registrar } {
        registro::linha(texto);
    }
}

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

// As partes e as assinaturas (também usadas pelo configurador).
mod partes;
use partes::*;
/// Pode ligar a parte? Escreve no log o motivo quando não liga, ou quando sobrescreve uma modificação do exe.
unsafe fn pode_ligar(p: &Parte, sobrescrever: bool) -> bool {
    let b = assinatura(p.bordas);
    if b != p.assinatura_das_bordas {
        registro::linha(&format!("{}: this exe differs from 3.9.68 where this part hooks in (signature {:016x}); nothing was changed", p.nome, b));
        return false;
    }
    let c = assinatura(p.corpo);
    if c == p.assinatura_do_corpo {
        (*core::ptr::addr_of_mut!(LIGADAS)).push((p.nome, p.bordas));
        return true;
    }
    // o Regen Tweak de xeno/Saturn (elenco 45 em 0x7ad1bd) é o caso conhecido; nomeá-lo ajuda quem lê o log
    let conhecido = if p.nome == DISPUTA.nome && *(0x7ad1bf as *const u8) == 0x2d { " (Regen tweak 2 / Regen fixes: squad limit 45)" } else { "" };
    if sobrescrever {
        registro::linha(&format!("{}: this exe already changes this routine{} (signature {:016x}); overriding it (override_exe_changes = 1)", p.nome, conhecido, c));
        (*core::ptr::addr_of_mut!(LIGADAS)).push((p.nome, p.bordas));
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
    (*core::ptr::addr_of_mut!(MARCAS)).push((de, 5));
    let mut antiga = 0u32;
    VirtualProtect(de as *mut c_void, 5, 0x40, &mut antiga);
    *(de as *mut u8) = 0xe9;
    core::ptr::write_unaligned((de + 1) as *mut i32, para.wrapping_sub(de + 5) as i32);
    VirtualProtect(de as *mut c_void, 5, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), de as *const c_void, 5);
}

/// Aplica uma lista de trocas (VA, antes, depois), fazendo o possível: o trecho como no 3.9.68 é trocado; o já trocado
/// fica (o exe já tem essa parte, como o do CLZ, que tem metade das checagens do CD e alguns trechos da resolução); o
/// trecho diferente dos dois é desconhecido. `inteira`: um trecho desconhecido recusa a lista toda (a resolução: metade
/// dela daria telas quebradas); senão só ele fica de fora (cada checagem do CD é independente). Devolve se a lista vale.
unsafe fn aplicar_trocas(nome: &str, lista: &[(usize, &[u8], &[u8])], inteira: bool) -> bool {
    let em = |va: usize, b: &[u8]| core::slice::from_raw_parts(va as *const u8, b.len()) == b;
    let novas: Vec<_> = lista.iter().filter(|t| em(t.0, t.1)).collect();
    let ja = lista.iter().filter(|t| em(t.0, t.2)).count();
    let estranhas = lista.len() - novas.len() - ja;
    if estranhas > 0 && inteira {
        registro::linha(&format!("{}: {} of {} places in this exe are neither 3.9.68 nor this change (another patch there); nothing was changed", nome, estranhas, lista.len()));
        return false;
    }
    for &&(va, _, depois) in &novas {
        (*core::ptr::addr_of_mut!(MARCAS)).push((va, depois.len()));
        let mut antiga = 0u32;
        VirtualProtect(va as *mut c_void, depois.len(), 0x40, &mut antiga);
        core::ptr::copy_nonoverlapping(depois.as_ptr(), va as *mut u8, depois.len());
        VirtualProtect(va as *mut c_void, depois.len(), antiga, &mut antiga);
        FlushInstructionCache(GetCurrentProcess(), va as *const c_void, depois.len());
    }
    registro::linha(&format!("{}: {} places changed now, {} already in the exe{}", nome, novas.len(), ja,
        if estranhas > 0 { format!(", {} left alone (another patch there)", estranhas) } else { String::new() }));
    novas.len() + ja > 0
}

/// A resolução que a NickDLL ao lado do exe aplica sozinha ("resolucao" no NickDLL_config.json, nas versões que têm a
/// "resolucao ampliada", como a do CLZ 6.1, que usa a mesma tabela de trocas): `Some(valor)` quando ela vai mudar a
/// resolução, para a nossa ficar de fora (a dela recusa se achar os bytes já trocados).
fn resolucao_da_nickdll() -> Option<String> {
    let pasta = std::env::current_exe().ok()?;
    let dll = std::fs::read(pasta.with_file_name("NickDLL.dll")).ok()?;
    if !dll.windows(19).any(|w| w == b"layout Nick Patcher") {
        return None;
    }
    let cfg = std::fs::read_to_string(pasta.with_file_name("NickDLL_config.json")).ok()?;
    let depois = cfg.split("\"resolucao\"").nth(1)?;
    let valor = depois.split('"').nth(1)?.trim().to_string();
    (valor != "800x600" && !valor.is_empty()).then_some(valor)
}

/// A trava das 30 linhas do `Area::init` (0x402ac0, thiscall; a 10ª palavra da pilha é o número de linhas, a 14ª o
/// dono): a grade tem no máximo 30 linhas, e em 1920×1080 o relatório de uma partida antiga pede mais e o jogo para na
/// caixa "Area.cpp 507". Como no projeto CM0102: acima de 30, 30, e o resto vai para a barra de rolagem. O trecho:
/// `cmp byte [esp+0x28], 30; jbe +5; mov byte [esp+0x28], 30`, a instrução tirada da entrada
/// (`cmp word [esp+0x38], -1`, cujas flags o `je` de 0x402acc usa) e a volta para 0x402ac6.
const ENTRADA_DA_AREA: [u8; 6] = [0x66, 0x83, 0x7c, 0x24, 0x38, 0xff];

unsafe fn travar_linhas_da_area() -> bool {
    if core::slice::from_raw_parts(0x402ac0 as *const u8, 6) != ENTRADA_DA_AREA {
        return false;
    }
    let t = VirtualAlloc(core::ptr::null_mut(), 32, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return false;
    }
    let mut c: Vec<u8> = vec![0x80, 0x7c, 0x24, 0x28, 0x1e, 0x76, 0x05, 0xc6, 0x44, 0x24, 0x28, 0x1e];
    c.extend_from_slice(&ENTRADA_DA_AREA);
    c.push(0xe9);
    let fim = t as usize + c.len() + 4;
    c.extend_from_slice(&(0x402ac6usize.wrapping_sub(fim) as i32).to_le_bytes());
    core::ptr::copy_nonoverlapping(c.as_ptr(), t, c.len());
    desviar(0x402ac0, t as usize);
    true
}

/// `ChangeDisplaySettingsA` com a camada de vídeo: o modo do Windows nunca muda (o jogo pede 800×600×16 ao abrir e
/// devolve o modo ao sair).
unsafe extern "system" fn mudar_modo_de_video(_modo: *const c_void, _flags: u32) -> i32 {
    0 // DISP_CHANGE_SUCCESSFUL
}

/// `no_cd`, `resolution` e `video`, fazendo o possível: o CD checagem por checagem (fica com a LaMOHatesCD quando ela
/// está ao lado do exe); a resolução de 1080 completa o que o exe ainda não tem, vem com a camada de vídeo e fica de
/// fora quando a NickDLL do CLZ cuida da resolução; a trava das 30 linhas entra em qualquer resolução alta.
unsafe fn ligar_tela(r: &mut Regras) {
    if r.sem_cd {
        let outro = std::env::current_exe().map(|p| p.with_file_name("LaMOHatesCD.dll").exists()).unwrap_or(false);
        if outro {
            registro::linha("no CD: LaMOHatesCD.dll is next to the exe and already takes care of the CD; left to it");
        } else {
            aplicar_trocas("no CD", &trocas::CD, false);
        }
    }
    let da_nick = resolucao_da_nickdll();
    if r.resolucao {
        if let Some(v) = &da_nick {
            registro::linha(&format!("resolution 1080: NickDLL sets the resolution here (\"resolucao\": \"{}\" in NickDLL_config.json); left to it", v));
        } else if aplicar_trocas("resolution 1080", &trocas::RESOLUCAO, true) {
            r.video = true;
            registro::linha("resolution 1080: on (1920x1080; video = window too)");
        }
    }
    // a trava das 30 linhas vale em qualquer resolução alta (a nossa, a da NickDLL, a de uma janela grande) e não muda
    // nada em 800x600
    if r.resolucao || r.video || da_nick.is_some() {
        if travar_linhas_da_area() {
            registro::linha("match report: at most 30 rows per area (no \"Area.cpp 507\" at high resolutions)");
        } else {
            registro::linha("match report: this exe differs from 3.9.68 at Area::init (0x402ac0); the 30-row limit was not added");
        }
    }
    if r.video {
        match pasta::trocar_importacao("USER32.dll", "ChangeDisplaySettingsA", mudar_modo_de_video as *const () as usize) {
            Some(_) => registro::linha("video: window (the project's video layer: an ordinary window, Print Screen works, Windows' video mode never changes)"),
            None => {
                r.video = false;
                registro::linha("video: ChangeDisplaySettingsA is not in this exe's import table; keeping the system's DirectDraw");
            }
        }
    }
}

/// Troca o destino de uma chamada (E8) do exe.
pub(crate) unsafe fn chamar(de: usize, para: usize) {
    (*core::ptr::addr_of_mut!(MARCAS)).push((de, 5));
    let mut antiga = 0u32;
    VirtualProtect(de as *mut c_void, 5, 0x40, &mut antiga);
    *(de as *mut u8) = 0xe8;
    core::ptr::write_unaligned((de + 1) as *mut i32, para.wrapping_sub(de + 5) as i32);
    VirtualProtect(de as *mut c_void, 5, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), de as *const c_void, 5);
}

/// Troca `n` bytes do exe por NOP.
unsafe fn nops(de: usize, n: usize) {
    (*core::ptr::addr_of_mut!(MARCAS)).push((de, n));
    let mut antiga = 0u32;
    VirtualProtect(de as *mut c_void, n, 0x40, &mut antiga);
    core::ptr::write_bytes(de as *mut u8, 0x90, n);
    VirtualProtect(de as *mut c_void, n, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), de as *const c_void, n);
}

// ---------- diagnóstico (só com log = 1) ----------

static mut FALHAS: u32 = 0;
/// A linha de execução principal do jogo (a que carrega a DLL), para o vigia de travamento.
static mut LINHA_PRINCIPAL: u32 = 0;
/// Para o resumo da batida de relógio: jovens da disputa, regens nascidos (pela tabela de idades), aposentados pelos inativos.
static JOVENS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static NASCIDOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static APOSENTADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
/// Os desvios instalados: (endereço no exe, destino), para conferir depois que as outras DLLs do exe carregaram.
static mut DESVIOS: Vec<(usize, usize)> = Vec::new();
/// Todo trecho do exe que esta DLL escreve (início, tamanho): o retrato da memória não os conta como de outra DLL.
static mut MARCAS: Vec<(usize, usize)> = Vec::new();
/// As partes que ligaram e as bordas que conferiram: o retrato da memória avisa se outra DLL mexeu nelas depois.
static mut LIGADAS: Vec<(&'static str, &'static [(usize, usize)])> = Vec::new();

/// As primeiras exceções graves (até 3: violação de acesso, divisão por zero, estouro de pilha, instrução inválida) vão para o log com o endereço, os registradores e o topo da pilha, antes de o
/// jogo cair: sem isso, uma queda num exe com outra DLL (a NickDLL do CLZ) não deixa rastro.
unsafe extern "system" fn ao_falhar(ponteiros: *mut usize) -> i32 {
    let registro_da_excecao = *ponteiros;
    let contexto = *ponteiros.add(1);
    let codigo = *(registro_da_excecao as *const u32);
    // violação de acesso, divisão inteira por zero, estouro de pilha, instrução inválida ou privilegiada
    let grave = matches!(codigo, 0xc000_0005 | 0xc000_0094 | 0xc000_00fd | 0xc000_001d | 0xc000_0096);
    if grave && FALHAS < 3 {
        FALHAS += 1;
        let r = |o: usize| *((contexto + o) as *const u32);
        let alvo = *((registro_da_excecao + 0x18) as *const u32);
        let modo = *((registro_da_excecao + 0x14) as *const u32);
        let esp = r(0xc4) as usize;
        let pilha: Vec<String> = (0..16).map(|k| format!("{:08x}", core::ptr::read_volatile((esp + 4 * k) as *const u32))).collect();
        registro::linha(&format!("crash\t{}\tcode {:08x}\teip {:08x} {} {:08x}\teax {:08x} ebx {:08x} ecx {:08x} edx {:08x} esi {:08x} edi {:08x} ebp {:08x} esp {:08x}\tdll {:08x}\tstack {}",
            data_do_jogo(), codigo, r(0xb8), if modo == 1 { "writing" } else { "reading" }, alvo, r(0xb0), r(0xa4), r(0xac), r(0xa8), r(0xa0), r(0x9c), r(0xb4), r(0xc4),
            ao_falhar as *const () as usize, pilha.join(" ")));
    }
    0 // EXCEPTION_CONTINUE_SEARCH: o jogo segue como seguiria
}

/// O vigia de travamento: com a data parada, pausa a linha principal 5 vezes, meio segundo entre elas, e registra onde
/// ela está (eip, esp e o topo da pilha). Um laço infinito aparece como o mesmo trecho de código nas 5 amostras.
unsafe fn registrar_travamento(data: String) {
    let linha = OpenThread(0x0002 | 0x0008, 0, LINHA_PRINCIPAL); // THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT
    if linha.is_null() {
        registro::linha("hang\tcould not open the main thread");
        return;
    }
    let mut amostras = Vec::new();
    for _ in 0..5 {
        let mut ctx = [0u8; 0x2cc]; // CONTEXT do x86
        core::ptr::write_unaligned(ctx.as_mut_ptr() as *mut u32, 0x0001_0003); // CONTEXT_CONTROL | CONTEXT_INTEGER
        SuspendThread(linha);
        let ok = GetThreadContext(linha, ctx.as_mut_ptr()) != 0;
        let (eip, esp) = (core::ptr::read_unaligned(ctx.as_ptr().add(0xb8) as *const u32), core::ptr::read_unaligned(ctx.as_ptr().add(0xc4) as *const u32));
        let pilha: Vec<String> = if ok { (0..12).map(|k| format!("{:08x}", core::ptr::read_volatile((esp as usize + 4 * k) as *const u32))).collect() } else { Vec::new() };
        ResumeThread(linha);
        amostras.push(format!("eip {:08x} esp {:08x} stack {}", eip, esp, pilha.join(" ")));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    for a in amostras {
        registro::linha(&format!("hang\t{}\t{}", data, a));
    }
}
/// A batida de relógio (log = 1): uma linha `tick` a cada mês de jogo, com a data, os segundos desde o começo e o que a DLL
/// já registrou. Só lê a data do jogo ([0xae2c90] dia, [0xae2c92] ano), numa linha de execução à parte, sem desviar nada:
/// mostra se a carreira anda (e em que ritmo) mesmo quando nenhum ajuste escreve no log (override 0 num exe modificado).
fn iniciar_batida() {
    use core::sync::atomic::Ordering::Relaxed;
    std::thread::spawn(|| {
        let inicio = std::time::Instant::now();
        let mut ultimo = (0u16, 255u8);
        let (mut data_vista, mut desde, mut avisado) = ((0u16, 0u16), std::time::Instant::now(), false);
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3));
            let (dia, ano) = unsafe { (w(0xae2c90) as u16, w(0xae2c92) as u16) };
            // o vigia: a data parada por 60 s (com o jogo já numa carreira) é um travamento, ou o jogo esperando um clique
            if (dia, ano) != data_vista {
                data_vista = (dia, ano);
                desde = std::time::Instant::now();
                avisado = false;
            } else if ano > 1900 && !avisado && desde.elapsed().as_secs() >= 60 {
                avisado = true;
                unsafe { registrar_travamento(format!("{}-{:03}", ano, dia)) };
            }
            let mes = ((dia as u32 * 12) / 366) as u8;
            if ano > 1900 && (ano, mes) != ultimo {
                ultimo = (ano, mes);
                registro::linha(&format!("tick\t{}-{:03}\t{}s\tcontest {}\tbirths {}\tidle {}{}", ano, dia, inicio.elapsed().as_secs(),
                    JOVENS.load(Relaxed), NASCIDOS.load(Relaxed), APOSENTADOS.load(Relaxed),
                    if unsafe { arse() } { "\tarse (player_setup.cfg)" } else { "" }));
                if mes == 6 {
                    unsafe { registrar_temporada(format!("{}-{:03}", ano, dia)) };
                }
            }
        }
    });
}
/// A linha `season` (log = 1, em julho de cada ano): o retrato da população, só lendo a memória.
/// `season  data  jogadores  sem_clube  elenco min/média/max (clubes de competição no jogo)  regens  regens_sem_clube
///  max_regens_num_clube  PA≥150: total / sem clube / em clube grande (reputação ≥ 7000) / reputação média do clube`.
/// Pessoas: [0xae23bc] + 110 × id, [0xae2364] delas; jogador = +0x61, clube = +0x39, regen = bit 0x40 do +0xb do
/// registro de 79 bytes. ponytail: lê de outra linha de execução sem trava (só leitura de tabelas fixas); se um dia der
/// falha aqui, passar para um desvio na virada do dia.
unsafe fn registrar_temporada(data: String) {
    let (pessoas, n) = (d(0xae23bc), d(0xae2364));
    let (clubes, nc) = (d(0xae23b4), d(0xae235c));
    if pessoas == 0 || clubes == 0 || n > 2_000_000 || nc > 100_000 {
        return;
    }
    let (mut elenco, mut regens_no) = (vec![0u16; nc], vec![0u16; nc]);
    let (mut jogadores, mut livres, mut regens, mut regens_livres) = (0, 0, 0, 0);
    let (mut altos, mut altos_livres, mut altos_grandes, mut rep_altos) = (0, 0, 0, 0i64);
    for s in 0..n {
        let p = pessoas + 110 * s;
        let ficha = d(p + 0x61);
        if ficha == 0 {
            continue;
        }
        jogadores += 1;
        let clube = d(p + 0x39);
        let id = if clube != 0 { d(clube) } else { usize::MAX };
        let regen = *((d(0xae2d0c) + 79 * d(p) + 0xb) as *const u8) & 0x40 != 0;
        if id < nc {
            elenco[id] += 1;
            if regen {
                regens_no[id] += 1;
            }
        } else {
            livres += 1;
        }
        if regen {
            regens += 1;
            if id >= nc {
                regens_livres += 1;
            }
            if w(ficha + 7) >= 150 {
                altos += 1;
                if id >= nc {
                    altos_livres += 1;
                } else {
                    let rep = w(clube + 0x80) as i64;
                    rep_altos += rep;
                    altos_grandes += (rep >= 7000) as i32;
                }
            }
        }
    }
    let em_jogo: Vec<u16> = (0..nc).filter(|&i| em_competicao(clubes + 0x245 * i)).map(|i| elenco[i]).collect();
    let (min, max) = (em_jogo.iter().min().copied().unwrap_or(0), em_jogo.iter().max().copied().unwrap_or(0));
    let media = em_jogo.iter().map(|&x| x as f64).sum::<f64>() / em_jogo.len().max(1) as f64;
    let com_clube = altos - altos_livres;
    let topo = (0..nc).max_by_key(|&i| regens_no[i]).unwrap_or(0);
    // quantos regens do clube líder chegaram pela disputa (o resto veio por contratação ou por outro caminho)
    let (mut dados, mut de_outro) = (0, 0);
    // os clubes de reputação negativa (diagnóstico do clube depósito): os regens deles e por onde chegaram
    let (mut neg_clubes, mut neg_regens, mut neg_cota, mut neg_sobra, mut neg_disputa) = (0, 0, 0, 0, 0);
    let (mut topo_cota, mut topo_sobra) = (0, 0);
    for i in 0..nc {
        neg_clubes += (w(clubes + 0x245 * i + 0x80) < 0) as i32;
    }
    for p in (0..n).map(|s| pessoas + 110 * s).filter(|&p| d(p + 0x61) != 0 && d(p + 0x39) != 0) {
        if *((d(0xae2d0c) + 79 * d(p) + 0xb) as *const u8) & 0x40 == 0 {
            continue;
        }
        let c = d(d(p + 0x39));
        let f = pela_fornada(d(p), c);
        if c == topo {
            match destino(d(p)) {
                Some(x) if x == topo => dados += 1,
                Some(_) => de_outro += 1,
                None => {}
            }
            topo_cota += (f == 1) as i32;
            topo_sobra += (f == 2) as i32;
        }
        if w(d(p + 0x39) + 0x80) < 0 {
            neg_regens += 1;
            neg_cota += (f == 1) as i32;
            neg_sobra += (f == 2) as i32;
            neg_disputa += (destino(d(p)) == Some(c)) as i32;
        }
    }
    registro::linha(&(format!("season\t{}\tplayers {}\tfree {}\tsquad {}/{:.1}/{} ({} clubs)\tregens {}\tregens_free {}\tmax_regens_club {} ({}, {} given by the contest, {} via the contest to another club, {} by the intake quota, {} as intake leftovers, reputation {})\tpa150 {} free {} big {} rep {}\tnegative-reputation clubs {}: regens {} ({} by the intake quota, {} as intake leftovers, {} by the contest, the rest signed later)",
        data, jogadores, livres, min, media, max, em_jogo.len(), regens, regens_livres, regens_no[topo], nome(clubes + 0x245 * topo), dados, de_outro,
        topo_cota, topo_sobra, w(clubes + 0x245 * topo + 0x80),
        altos, altos_livres, altos_grandes, if com_clube > 0 { rep_altos / com_clube as i64 } else { 0 },
        neg_clubes, neg_regens, neg_cota, neg_sobra, neg_disputa)
        + if arse() { "\tarse (player_setup.cfg): every person has the regen bit, so the regen counts mean nothing" } else { "" }));
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
    /// outside_competitions: o peso dos clubes fora das competições que rodam no jogo (1 = como o resto; 0 = nunca).
    fora_competicao: f64,
    temperatura: f64,
    registrar: bool,
    /// unattached = only_if_no_room: todo jovem de nação no jogo vai à disputa (qualquer idade, sem o 1 em 4); fica sem
    /// clube só quando nenhum clube do país é elegível.
    so_sem_vaga: bool,
    /// portable_folder: o Location do registro é a pasta do exe (sem SetupReg).
    pasta: bool,
    /// multiple_instances: a trava de jogo único (o mutex) vale por pasta: jogos de pastas diferentes abrem juntos.
    instancias: bool,
    /// clear_admin_flag: tirar a marca WIN95 RUNASADMIN que a NickDLL grava para o exe.
    limpar_marca: bool,
    /// video = window: a camada de vídeo do projeto CM0102 (o quadro numa janela comum) em vez do DirectDraw do sistema.
    video: bool,
    /// resolution = 1080: a resolução 1920×1080 do projeto CM0102, em memória (liga também a camada de vídeo).
    resolucao: bool,
    /// no_cd = 1: as 33 checagens do CD contam como achadas, como no exe portátil do projeto CM0102.
    sem_cd: bool,
    /// regen_origins = 1: de quem é cada regen, em regen_origins.csv.
    origens: bool,
    /// grey_players = squad: o CA dos jogadores cinzentos (VIRTUAL_STAFF) pelo nível do clube (o maior entre o elenco e a reputação).
    cinzentos: bool,
    /// grey_players_spread: a largura da faixa do CA do cinzento, centrada no nível.
    cinzentos_espalhar: i32,
    /// regen_names = own: a nação usa os próprios nomes a partir de `nomes_minimo` (a original, só acima de 100).
    nomes: bool,
    nomes_minimo: i32,
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
    proteger_jovens: 0,
    surpresa: 15.0,
    fora_competicao: 1.0,
    temperatura: 1.0,
    registrar: false,
    so_sem_vaga: false,
    pasta: false,
    instancias: false,
    limpar_marca: false,
    video: false,
    resolucao: false,
    sem_cd: false,
    origens: false,
    cinzentos: false,
    cinzentos_espalhar: 10,
    nomes: false,
    nomes_minimo: 20,
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
        if k == "unattached" {
            r.so_sem_vaga = v == "only_if_no_room"; // unattached = original | only_if_no_room
            continue;
        }
        if k == "save_over_2gb" {
            r.save = v == "1";
            continue;
        }
        if k == "portable_folder" {
            r.pasta = v == "1";
            continue;
        }
        if k == "multiple_instances" {
            r.instancias = v == "1";
            continue;
        }
        if k == "video" {
            r.video = v == "window"; // video = original | window
            continue;
        }
        if k == "resolution" {
            r.resolucao = v == "1080"; // resolution = original | 1080
            continue;
        }
        if k == "regen_origins" {
            r.origens = v == "1";
            continue;
        }
        if k == "regen_names" {
            r.nomes = v == "own"; // regen_names = original | own
            continue;
        }
        if k == "grey_players" {
            r.cinzentos = v == "squad"; // grey_players = original | squad
            continue;
        }
        if k == "no_cd" {
            r.sem_cd = v == "1";
            continue;
        }
        if k == "clear_admin_flag" {
            r.limpar_marca = v == "1";
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
            "outside_competitions" => r.fora_competicao = x.clamp(0.0, 1.0),
            "temperature" => r.temperatura = x,
            "log" => r.registrar = x != 0.0,
            "contest_max_age" => r.idade_max_disputa = (x as i32).clamp(0, 126),
            "ca_cap_skip" => r.pular_teto = x,
            "younger_from_age" => r.mais_novo_a_partir = x as i32,
            "younger_by" => r.mais_novo_em = (x as i32).max(0),
            "regen_names_min" => r.nomes_minimo = (x as i32).clamp(1, 101),
            "grey_players_spread" => r.cinzentos_espalhar = (x as i32).clamp(0, 100),
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

/// O regen basco só vai para os clubes bascos. A 0x7ad0e0 olha só a segunda nacionalidade (+0x1e) e aceita 5 clubes
/// (Athletic e o B, Real Sociedad e o B, Alavés); aqui vale também a primeira (+0x1a; bancos novos trazem bascos nela) e
/// também os clubes que o próprio jogo marca como "só contrata bascos" (0x53d7f0: Eibar, Barakaldo, Sestao..., que a
/// original deixava sem regen basco).
unsafe fn basco(jogador: usize) -> bool {
    let e = |n: usize| n != 0 && d(n) == d(0x9cf424);
    e(d(jogador + 0x1a)) || e(d(jogador + 0x1e))
}

unsafe fn vetado(jogador: usize, clube: usize) -> bool {
    if !basco(jogador) {
        return false;
    }
    let so_bascos: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d7f0usize);
    let id = d(clube);
    ![0x9d0120usize, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124].iter().any(|&g| d(g) == id) && so_bascos(clube) == 0
}

/// Os filtros da 0x7ad0e0: não estrangeiro (0x53d980), não é o time B (0x540a50), não vetado, pode chegar ao jogador
/// (0x8ce990). Devolve o elenco (0x53c590) do clube elegível.
unsafe fn elegivel(jogador: usize, clube: usize) -> Option<i32> {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    // o basco não é estrangeiro na Espanha (só aqui, na escolha do clube do regen: num banco com bascos na primeira
    // nacionalidade, a 0x53d980 original o barraria em todo clube espanhol; o desvio no jogo inteiro está no ROADMAP)
    let nacao = d(clube + 0x53);
    let espanhol = nacao != 0 && d(nacao) == d(0x9cf488) && basco(jogador);
    if (estrangeiro(jogador, nacao, 1) != 0 && !espanhol) || time_b(clube) || vetado(jogador, clube) {
        return None;
    }
    let mut motivo = 0i16;
    if pode(d(0xde1f54), jogador, clube, &mut motivo, 0) == 0 {
        return None;
    }
    Some(elenco_de(clube, 0, 0) as i8 as i32)
}

/// Diagnóstico da fornada: por que o clube não pode receber o jovem ("ok" se pode). `bid N` = o motivo da 0x8ce990.
pub(crate) unsafe fn motivo_recusa(jogador: usize, clube: usize) -> String {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    if estrangeiro(jogador, d(clube + 0x53), 1) != 0 {
        return "foreign".into();
    }
    if vetado(jogador, clube) {
        return "vetoed".into();
    }
    let mut motivo = 0i16;
    if pode(d(0xde1f54), jogador, clube, &mut motivo, 0) == 0 {
        return format!("bid {}", motivo);
    }
    "ok".into()
}

/// O time B de outro clube (0x540a50: tem par e não é o time A); a disputa não manda jovem para ele.
pub(crate) unsafe fn time_b(clube: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a = 0u32;
    parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0
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

/// O clube (id + 1) para o qual a disputa mandou cada pessoa, pela regra nova ou pela do jogo: a linha `season` separa
/// os regens que a disputa deu ao clube dos que passaram por ela rumo a outro clube e chegaram depois (transferência ou
/// empréstimo). Vetor fixo, sem alocar na linha do jogo (o jogo lê memória não inicializada, e a heap muda a trajetória
/// da carreira; TESTES §4.5). ponytail: um id reaproveitado por outro regen herda o destino; ids acima de 2^18 não contam.
static DESTINO: [core::sync::atomic::AtomicU16; 1 << 18] = [const { core::sync::atomic::AtomicU16::new(0) }; 1 << 18];

/// Por onde o regen chegou pela fornada (diagnóstico da linha `season`): o clube + 1 nos 16 bits de baixo, 1 (cota)
/// ou 2 (sobra) nos de cima. ponytail: o mesmo limite de ids do DESTINO.
static FORNADA: [core::sync::atomic::AtomicU32; 1 << 18] = [const { core::sync::atomic::AtomicU32::new(0) }; 1 << 18];

pub(crate) fn marcar_fornada(pessoa: usize, clube: usize, sobra: bool) {
    if pessoa < 1 << 18 && clube < 0xffff {
        FORNADA[pessoa].store((clube as u32 + 1) | if sobra { 2 << 16 } else { 1 << 16 }, core::sync::atomic::Ordering::Relaxed);
    }
}

/// 0: não chegou ao `clube` pela fornada; 1: pela cota; 2: como sobra.
fn pela_fornada(pessoa: usize, clube: usize) -> u32 {
    let v = if pessoa < 1 << 18 { FORNADA[pessoa].load(core::sync::atomic::Ordering::Relaxed) } else { 0 };
    if v & 0xffff == clube as u32 + 1 { v >> 16 } else { 0 }
}

fn marcar_destino(pessoa: usize, clube: usize) {
    if pessoa < 1 << 18 && clube < 0xffff {
        DESTINO[pessoa].store(clube as u16 + 1, core::sync::atomic::Ordering::Relaxed);
    }
}

/// `Some(clube)` se a pessoa passou pela disputa.
fn destino(pessoa: usize) -> Option<usize> {
    let v = if pessoa < 1 << 18 { DESTINO[pessoa].load(core::sync::atomic::Ordering::Relaxed) } else { 0 };
    (v != 0).then(|| v as usize - 1)
}

/// A carreira foi criada com a linha `ARSE` no `player_setup.cfg`? A linha (tipo 0x20 da 0x7a3a10, laço 0x7a3c0c) liga o
/// bit 0x40 do regen em todas as pessoas no fim da `create_game` (0x5cc850, chamada em 0x829fd9, lê o arquivo com a
/// máscara 0x203b em 0x5ccc9c), e nada desliga o bit. Os técnicos humanos (classificação +0x5f = 0, as últimas vagas da
/// tabela) nunca viram regen (a 0x7abac0 grava +0x5f = 2): um deles com o bit só pode ser obra do ARSE. Vale para a
/// carreira nova e para o save carregado, sem contar quantos têm o bit nem ler o .cfg de agora.
unsafe fn arse() -> bool {
    let (pessoas, n, t) = (d(0xae23bc), d(0xae2364), d(0xae2d0c));
    if pessoas == 0 || t == 0 || n > 2_000_000 {
        return false;
    }
    // ponytail: só as últimas 64 vagas, onde a criação põe os 16 técnicos humanos
    (n.saturating_sub(64)..n).rev().map(|s| pessoas + 110 * s).find(|&p| *((p + 0x5f) as *const u8) == 0)
        .is_some_and(|p| *((t + 79 * d(p) + 0xb) as *const u8) & 0x40 != 0)
}

/// Os jovens que a disputa nova entregou: (id do clube, dia corrido = ano × 365 + dia).
static mut ENTREGUES: Vec<(usize, i32)> = Vec::new();

unsafe fn hoje() -> i32 {
    w(0xae2c92) as i32 * 365 + w(0xae2c90) as i32
}

/// Quantos jovens o clube recebeu nos últimos 365 dias: os que esta DLL entregou (mesmo que o clube já os tenha
/// emprestado, vendido ou mandado para o time B; contar só o elenco deixava o teto vazar, 15 no Real Madrid em 4 anos)
/// ou, se for maior, os regens (bit 0x40 do +0xb do registro de 79 bytes) do elenco (+0xd7) que entraram no clube
/// (+0x3e dia, +0x40 ano) no período (o que vale logo depois de carregar um save, com a lista vazia). Numa carreira
/// criada com ARSE (`arse`), só a lista.
/// ponytail: a lista não vai para o save; depois da carga, só a contagem pelo elenco até a lista encher de novo.
unsafe fn recebidos_no_ano(clube: usize) -> i32 {
    let (id, agora) = (d(clube), hoje());
    let pela_lista = (*core::ptr::addr_of!(ENTREGUES)).iter().filter(|&&(c, t)| c == id && agora - t < 365).count() as i32;
    if arse() {
        return pela_lista; // com ARSE todo jogador tem o bit: o elenco não diz quem é regen
    }
    let pelo_elenco = (0..50usize)
        .map(|k| d(clube + 0xd7 + 4 * k))
        .filter(|&p| p != 0 && *((d(0xae2d0c) + 79 * d(p) + 0xb) as *const u8) & 0x40 != 0)
        .filter(|&p| agora - (w(p + 0x40) as i32 * 365 + w(p + 0x3e) as i32) < 365)
        .count() as i32;
    pela_lista.max(pelo_elenco)
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
    let p = (nivel * inst * comissao * vaga).max(0.0).powf(r.temperatura);
    // os clubes fora das competições que rodam (a divisão não roda no jogo): o peso vezes `outside_competitions`; na
    // surpresa eles já ficaram de fora acima
    if r.fora_competicao < 1.0 && !surpresa && !em_competicao(clube) { p * r.fora_competicao } else { p }
}

/// A fornada: o peso do clube para o jovem pela curva da aba Regens, sem o teto por temporada e sem o espaço no elenco
/// (a cota da fornada já é o limite; elenco cheio, 50, fica de fora). `None`: o clube não pode recebê-lo (os filtros
/// da disputa: estrangeiro, time B, vetado).
pub(crate) unsafe fn peso_na_fornada(jogador: usize, clube: usize, surpresa: bool) -> Option<f64> {
    elegivel(jogador, clube)?;
    // as 50 vagas do próprio clube; a 0x53c590 soma o time B (na Espanha, em Portugal e na Alemanha, A + B passa de 50)
    let elenco = (0..50).filter(|&k| d(clube + 0xd7 + 4 * k) != 0).count() as i32;
    if elenco >= 50 {
        return None;
    }
    let mut r = REGRAS;
    r.elenco_teto = 50.0;
    r.elenco_exp = 0.0;
    r.teto_temporada = 0.0;
    let ficha = d(jogador + 0x61);
    Some(peso(&r, w(ficha + 7) as f64, w(ficha + 5) as f64, clube, elenco, surpresa))
}

/// A surpresa da aba Regens (`surprise`, %) para um jovem.
pub(crate) unsafe fn sorteia_surpresa() -> bool {
    REGRAS.surpresa > 0.0 && (sorteio(10000) as f64) < REGRAS.surpresa * 100.0
}

/// Um número uniforme em [0, 1) com dois sorteios do jogo (30 bits).
unsafe fn uniforme() -> f64 {
    ((sorteio(0x8000) as u32) << 15 | sorteio(0x8000) as u32) as f64 / (1u32 << 30) as f64
}

pub(crate) unsafe fn nome(clube: usize) -> String {
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
    let lista = &mut *core::ptr::addr_of_mut!(ENTREGUES);
    let agora = hoje();
    lista.retain(|&(_, t)| agora - t < 365);
    lista.push((d(escolhido.0), agora));
    escolhido.0
}

/// A data do jogo como AAAA-ddd (o dia do ano em [0xae2c90], o ano em [0xae2c92]).
pub(crate) unsafe fn data_do_jogo() -> String {
    format!("{}-{:03}", w(0xae2c92) as u16, w(0xae2c90) as u16)
}

/// Uma linha do log por jovem da disputa, separada por tabulação (para contar depois):
/// `regen  regra  data  idade  PA  CA  reputação  Training  elenco  clube  [chance  disputaram  surpresa]`.
unsafe fn registrar_escolha(regra: &str, jogador: usize, clube: usize, extra: &str) {
    JOVENS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    if clube != 0 {
        marcar_destino(d(jogador), d(clube));
    }
    let ficha = d(jogador + 0x61);
    let (rep, tr, nome_clube) = if clube != 0 { (w(clube + 0x80) as i32, b(clube + 0x7f) as i32, nome(clube)) } else { (-1, -1, "(no club)".into()) };
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let elenco = if clube != 0 { elenco_de(clube, 0, 0) as i32 } else { -1 };
    registro::linha(&format!("regen\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", regra, data_do_jogo(), b(jogador + 0x18), w(ficha + 7), w(ficha + 5), rep, tr, elenco, nome_clube, extra));
}

/// A entrada original da 0x7ad0e0, para o modo de observação (`rule = original` com `log = 1`): os 7 bytes do prólogo
/// (`sub esp, 0x14; mov [esp+0x10], ecx`, sem salto relativo) copiados para um trecho que volta em 0x7ad0e7.
static mut DISPUTA_ORIGINAL: usize = 0;
/// "original" (rule = original) ou "exe" (a disputa ficou com o exe modificado, override 0).
static mut DISPUTA_ROTULO: &str = "original";

/// { 0x7ad0e0 } no modo de observação: a disputa da original escolhe, e a escolha vai para o log.
unsafe extern "thiscall" fn observar_clube(this: usize, jogador: usize) -> usize {
    let original: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(DISPUTA_ORIGINAL);
    let clube = original(this, jogador);
    registrar_escolha(DISPUTA_ROTULO, jogador, clube, "-\t-\t-");
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
        NASCIDOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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
        NASCIDOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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
    (*core::ptr::addr_of_mut!(MARCAS)).push((0x7ace69, 1));
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
                APOSENTADOS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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
/// O nome do mutex do jogo único ("Championship Manager 2001/02 Process", 0x9f183c no 3.9.68).
const NOME_DO_MUTEX: &[u8] = b"Championship Manager 2001/02 Process";

/// multiple_instances = 1: o "Process" do nome do mutex vira 7 dígitos tirados da pasta do exe (FNV-1a do caminho em
/// minúsculas). O nome fica do mesmo tamanho; cada pasta tem a sua trava: jogos de pastas diferentes abrem juntos, e
/// dois na mesma pasta continuam barrados (usariam os mesmos temporários e o mesmo save).
unsafe fn trava_por_pasta() {
    let a = 0x9f183cusize;
    if core::slice::from_raw_parts(a as *const u8, NOME_DO_MUTEX.len()) != NOME_DO_MUTEX {
        registro::linha("multiple instances: the game's mutex name isn't where 3.9.68 has it (0x9f183c); left alone, one game at a time");
        return;
    }
    let pasta = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_string_lossy().to_lowercase())).unwrap_or_default();
    let h = pasta.bytes().fold(0x811c9dc5u32, |h, b| (h ^ b as u32).wrapping_mul(0x01000193));
    let sufixo = format!("{:07x}", h & 0x0fff_ffff);
    let ini = a + NOME_DO_MUTEX.len() - 7;
    (*core::ptr::addr_of_mut!(MARCAS)).push((ini, 7));
    let mut antiga = 0u32;
    VirtualProtect(ini as *mut c_void, 7, 0x40, &mut antiga);
    core::ptr::copy_nonoverlapping(sufixo.as_ptr(), ini as *mut u8, 7);
    VirtualProtect(ini as *mut c_void, 7, antiga, &mut antiga);
    registro::linha(&format!("multiple instances: on (this folder's own lock: \"Championship Manager 2001/02 {}\"; games in other folders can run at the same time)", sufixo));
}

/// Só para as provas (variável de ambiente `TWEAKS_MUTEX` = um caractere): troca a última letra do nome do mutex do jogo
/// ("Championship Manager 2001/02 Process", 0x9f183c no 3.9.68), para duas cópias rodarem ao mesmo tempo em pastas
/// diferentes. Sem a variável, nada muda.
unsafe fn mutex_dos_testes() {
    const NOME: &[u8] = NOME_DO_MUTEX;
    let Some(c) = std::env::var("TWEAKS_MUTEX").ok().and_then(|v| v.bytes().next()) else { return };
    let a = 0x9f183cusize;
    if core::slice::from_raw_parts(a as *const u8, NOME.len()) != NOME {
        registro::linha("TWEAKS_MUTEX: the mutex name isn't at 0x9f183c in this exe; left alone");
        return;
    }
    let fim = a + NOME.len() - 1;
    (*core::ptr::addr_of_mut!(MARCAS)).push((fim, 1));
    let mut antiga = 0u32;
    VirtualProtect(fim as *mut c_void, 1, 0x40, &mut antiga);
    *(fim as *mut u8) = c;
    VirtualProtect(fim as *mut c_void, 1, antiga, &mut antiga);
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
    }
    1
}

// ---------- as caixas de mensagem do jogo no log (log = 1) ----------

/// A caixa de mensagem original (ou a da DLL que a desviou antes), pelo trampolim.
static mut CAIXA_ORIGINAL: usize = 0;

/// `gui_show_message_box` (0x5e8290, cdecl: título, texto, permitir sair): toda caixa do jogo, inclusive os erros com o
/// arquivo de código e a linha (`Database.cpp 7784`, `comp_stats.cpp 6000`), vai para o log antes de aparecer. A caixa é
/// desenhada pelo jogo: sem isto, um teste com o jogo oculto não a vê.
unsafe extern "C" fn caixa_de_mensagem(titulo: *const u8, texto: *const u8, sair: u32) {
    let ler = |p: *const u8| -> String {
        if p.is_null() {
            return String::new();
        }
        let n = (0..1024).position(|k| *p.add(k) == 0).unwrap_or(1024);
        core::slice::from_raw_parts(p, n).iter().map(|&c| if c == b'\n' || c == b'\r' || c == b'\t' { ' ' } else { c as char }).collect()
    };
    registro::linha(&format!("msgbox\t{}\t{}\t{}", data_do_jogo(), ler(titulo), ler(texto)));
    // o detalhe que o jogo monta antes de algumas caixas (0xde1f64: "Only %d home players ** A v B..." em match_eng.cpp
    // 612, por exemplo); só se for texto legível
    let detalhe = ler(0xde1f64 as *const u8);
    if !detalhe.is_empty() && detalhe.len() < 1000 && detalhe.chars().all(|c| (' '..='~').contains(&c) || c as u32 >= 0xa0) {
        registro::linha(&format!("msgbox-detail\t{}\t{}", data_do_jogo(), detalhe));
    }
    let original: unsafe extern "C" fn(*const u8, *const u8, u32) = core::mem::transmute(CAIXA_ORIGINAL);
    original(titulo, texto, sair)
}

/// Instala o desvio da caixa depois das DLLs do exe (a LaMOHatesCD do CLZ troca os mesmos 9 bytes ao carregar). A entrada
/// original (`sub esp, 0x6c; mov ecx, [0xb67a30]`) vai para o trampolim; um salto de outra DLL ali é encadeado; outra coisa,
/// não mexe.
unsafe fn instalar_caixa() {
    const ENTRADA: [u8; 9] = [0x83, 0xec, 0x6c, 0x8b, 0x0d, 0x30, 0x7a, 0xb6, 0x00];
    let e = 0x5e8290usize;
    let atual = core::slice::from_raw_parts(e as *const u8, 9);
    let t = VirtualAlloc(core::ptr::null_mut(), 32, 0x3000, 0x40) as *mut u8;
    if t.is_null() {
        return;
    }
    let como = if atual == ENTRADA {
        core::ptr::copy_nonoverlapping(ENTRADA.as_ptr(), t, 9);
        *t.add(9) = 0xe9;
        core::ptr::write_unaligned(t.add(10) as *mut i32, (e + 9).wrapping_sub(t as usize + 14) as i32); // o E9 em t+9 termina em t+14
        "the exe's own"
    } else if atual[0] == 0xe9 {
        let alvo = (e + 5).wrapping_add(core::ptr::read_unaligned((e + 1) as *const i32) as usize);
        *t = 0xe9;
        core::ptr::write_unaligned(t.add(1) as *mut i32, alvo.wrapping_sub(t as usize + 5) as i32);
        "chained to another DLL's hook"
    } else {
        registro::linha("msgbox: not logged (another DLL changed 0x5e8290 in a way this DLL doesn't know)");
        return;
    };
    CAIXA_ORIGINAL = t as usize;
    let mut antiga = 0u32;
    VirtualProtect(e as *mut c_void, 5, 0x40, &mut antiga);
    *(e as *mut u8) = 0xe9;
    core::ptr::write_unaligned((e + 1) as *mut i32, (caixa_de_mensagem as *const () as usize).wrapping_sub(e + 5) as i32);
    VirtualProtect(e as *mut c_void, 5, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), e as *const c_void, 5);
    registro::linha(&format!("msgbox: every game message box goes to the log ({})", como));
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
