//! `match_text_guard` (nosso padrão: ligado com tweaks.cfg; `match_text_guard = 0` desliga): a narração da partida
//! (`Match_Events::get_next_event` 0x6d78b0) mede o texto do lance com `repne scasb` em 0x6d79ab sem testar o ponteiro
//! (`mov ebx, [edi + eax + 0x108]` em 0x6d79a0, o texto da variação do lance, carregado do events_<idioma>.cfg). Visto
//! uma vez no CLZ v6 (16/11/2026, 0x6d79ab lendo 0): o jogo fechava no meio da partida. Com o ponteiro vazio, o lance
//! passa por texto vazio (o caminho que o próprio jogo já tem para a linha vazia, 0x6d7a0c) e o tweaks.log registra
//! o id do lance. ponytail: só contorna; a causa do ponteiro vazio não está achada.

use crate::registro;

static VAZIO: [u8; 1] = [0];
static mut AVISOS: u32 = 0;

/// O lance sem texto: o id (word em +2 do registro do evento, em esi).
unsafe extern "C" fn aviso(evento: usize) {
    AVISOS += 1;
    if AVISOS <= 20 {
        let id = if evento > 0x10000 { core::ptr::read_unaligned((evento + 2) as *const i16) as i32 } else { -1 };
        registro::linha(&format!("match text guard\t{}\tmatch event {} has no text; shown as an empty line instead of closing the game", crate::data_do_jogo(), id));
    }
}

#[unsafe(naked)]
unsafe extern "C" fn trecho() {
    // mov ebx, [edi + eax + 0x108]; test ebx, ebx; jnz 1f; pushad; push esi; call aviso; add esp, 4; popad;
    // lea ebx, VAZIO; 1: push 0x6d79a7; ret
    core::arch::naked_asm!("mov ebx, dword ptr [edi + eax + 0x108]", "test ebx, ebx", "jnz 2f", "pushad", "push esi", "call {a}",
        "add esp, 4", "popad", "lea ebx, [{v}]", "2:", "push 0x6d79a7", "ret", a = sym aviso, v = sym VAZIO);
}

fn ler() -> bool {
    let caminho = std::env::current_exe().map(|x| x.with_file_name("tweaks.cfg")).unwrap_or_else(|_| "tweaks.cfg".into());
    let Ok(texto) = std::fs::read_to_string(caminho) else { return false };
    let mut ligado = true; // o nosso padrão, como o squad_guard
    for linha in texto.lines() {
        let linha = linha.split('#').next().unwrap_or("");
        if let Some((k, v)) = linha.split_once('=') {
            if k.trim() == "match_text_guard" {
                ligado = v.trim() != "off" && v.trim() != "0";
            }
        }
    }
    ligado
}

pub(crate) unsafe fn instalar() {
    if !ler() {
        return;
    }
    if core::slice::from_raw_parts(0x6d79a0 as *const u8, 9) != [0x8b, 0x9c, 0x07, 0x08, 0x01, 0x00, 0x00, 0x33, 0xc0] {
        return registro::linha("match text guard: 0x6d79a0 differs from 3.9.68; off");
    }
    crate::desviar(0x6d79a0, trecho as *const () as usize);
    registro::linha("match text guard: on (a match event with no text is shown as an empty line instead of closing the game)");
}
