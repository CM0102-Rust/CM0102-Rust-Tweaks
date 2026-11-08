//! O diagnóstico (só com `log = 1`): falhas, travamentos, o resumo da temporada, a conferência dos desvios e as caixas de mensagem do jogo.

use super::*;


pub(crate) static mut FALHAS: u32 = 0;
/// A linha de execução principal do jogo (a que carrega a DLL), para o vigia de travamento.
pub(crate) static mut LINHA_PRINCIPAL: u32 = 0;
/// Para o resumo da batida de relógio: jovens da disputa, regens nascidos (pela tabela de idades), aposentados pelos inativos.
pub(crate) static JOVENS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static NASCIDOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static APOSENTADOS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
/// Os desvios instalados: (endereço no exe, destino), para conferir depois que as outras DLLs do exe carregaram.
pub(crate) static mut DESVIOS: Vec<(usize, usize)> = Vec::new();
/// Todo trecho do exe que esta DLL escreve (início, tamanho): o retrato da memória não os conta como de outra DLL.
pub(crate) static mut MARCAS: Vec<(usize, usize)> = Vec::new();
/// As partes que ligaram e as bordas que conferiram: o retrato da memória avisa se outra DLL mexeu nelas depois.
pub(crate) static mut LIGADAS: Vec<(&'static str, &'static [(usize, usize)])> = Vec::new();

/// As primeiras exceções graves (até 3: violação de acesso, divisão por zero, estouro de pilha, instrução inválida) vão para o log com o endereço, os registradores e o topo da pilha, antes de o
/// jogo cair: sem isso, uma queda num exe com outra DLL (a NickDLL do CLZ) não deixa rastro.
pub(crate) unsafe extern "system" fn ao_falhar(ponteiros: *mut usize) -> i32 {
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
pub(crate) unsafe fn registrar_travamento(data: String) {
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
pub(crate) fn iniciar_batida() {
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
                if crate::estudo::ligado() {
                    unsafe { crate::estudo::medir(&format!("{}-{:03}", ano, dia), mes == 6) };
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
pub(crate) unsafe fn registrar_temporada(data: String) {
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
pub(crate) unsafe fn conferir_desvios() {
    for &(de, para) in (*core::ptr::addr_of!(DESVIOS)).iter() {
        let ok = *(de as *const u8) == 0xe9 && (core::ptr::read_unaligned((de + 1) as *const i32) as usize).wrapping_add(de + 5) == para;
        if !ok {
            let b: Vec<String> = (0..8).map(|k| format!("{:02x}", *((de + k) as *const u8))).collect();
            registro::linha(&format!("check: the hook at {:08x} was overwritten after loading ({}); another DLL changed it", de, b.join(" ")));
        }
    }
    registro::linha(&format!("check: {} hooks checked after the exe's DLLs loaded", (*core::ptr::addr_of!(DESVIOS)).len()));
}


/// A caixa de mensagem original (ou a da DLL que a desviou antes), pelo trampolim.
pub(crate) static mut CAIXA_ORIGINAL: usize = 0;

/// `gui_show_message_box` (0x5e8290, cdecl: título, texto, permitir sair): toda caixa do jogo, inclusive os erros com o
/// arquivo de código e a linha (`Database.cpp 7784`, `comp_stats.cpp 6000`), vai para o log antes de aparecer. A caixa é
/// desenhada pelo jogo: sem isto, um teste com o jogo oculto não a vê.
pub(crate) unsafe extern "C" fn caixa_de_mensagem(titulo: *const u8, texto: *const u8, sair: u32) {
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
pub(crate) unsafe fn instalar_caixa() {
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
