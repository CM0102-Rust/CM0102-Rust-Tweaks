//! Vários jogos ao mesmo tempo (`multiple_instances`): o mutex do jogo por pasta.

use super::*;

/// O nome do mutex do jogo único ("Championship Manager 2001/02 Process", 0x9f183c no 3.9.68).
pub(crate) const NOME_DO_MUTEX: &[u8] = b"Championship Manager 2001/02 Process";

/// multiple_instances = 1: o "Process" do nome do mutex vira 7 dígitos tirados da pasta do exe (FNV-1a do caminho em
/// minúsculas). O nome fica do mesmo tamanho; cada pasta tem a sua trava: jogos de pastas diferentes abrem juntos, e
/// dois na mesma pasta continuam barrados (usariam os mesmos temporários e o mesmo save).
pub(crate) unsafe fn trava_por_pasta() {
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
pub(crate) unsafe fn mutex_dos_testes() {
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
