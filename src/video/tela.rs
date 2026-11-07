//! A tela: a resolução da NickDLL, as linhas da área e ligar a camada de vídeo (`video`, `resolution`, `no_cd`).

use super::*;

/// A resolução que a NickDLL ao lado do exe aplica sozinha ("resolucao" no NickDLL_config.json, nas versões que têm a
/// "resolucao ampliada", como a do CLZ 6.1, que usa a mesma tabela de trocas): `Some(valor)` quando ela vai mudar a
/// resolução, para a nossa ficar de fora (a dela recusa se achar os bytes já trocados).
pub(crate) fn resolucao_da_nickdll() -> Option<String> {
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
pub(crate) const ENTRADA_DA_AREA: [u8; 6] = [0x66, 0x83, 0x7c, 0x24, 0x38, 0xff];

pub(crate) unsafe fn travar_linhas_da_area() -> bool {
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
pub(crate) unsafe extern "system" fn mudar_modo_de_video(_modo: *const c_void, _flags: u32) -> i32 {
    0 // DISP_CHANGE_SUCCESSFUL
}

/// `no_cd`, `resolution` e `video`, fazendo o possível: o CD checagem por checagem (fica com a LaMOHatesCD quando ela
/// está ao lado do exe); a resolução de 1080 completa o que o exe ainda não tem, vem com a camada de vídeo e fica de
/// fora quando a NickDLL do CLZ cuida da resolução; a trava das 30 linhas entra em qualquer resolução alta.
pub(crate) unsafe fn ligar_tela(r: &mut Regras) {
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
