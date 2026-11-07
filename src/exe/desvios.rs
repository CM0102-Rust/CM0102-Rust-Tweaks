//! Ligar uma parte (a conferência das assinaturas) e mexer no exe em memória: desvios, chamadas, trocas de bytes.

use super::*;

/// Pode ligar a parte? Escreve no log o motivo quando não liga, ou quando sobrescreve uma modificação do exe.
pub(crate) unsafe fn pode_ligar(p: &Parte, sobrescrever: bool) -> bool {
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

pub(crate) unsafe fn assinatura(trechos: &[(usize, usize)]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &(va, n) in trechos {
        for i in 0..n {
            h = (h ^ *((va + i) as *const u8) as u64).wrapping_mul(0x100000001b3);
        }
    }
    h
}

/// Salto (E9) da entrada da rotina original para a versão em Rust.
pub(crate) unsafe fn desviar(de: usize, para: usize) {
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
pub(crate) unsafe fn aplicar_trocas(nome: &str, lista: &[(usize, &[u8], &[u8])], inteira: bool) -> bool {
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
pub(crate) unsafe fn nops(de: usize, n: usize) {
    (*core::ptr::addr_of_mut!(MARCAS)).push((de, n));
    let mut antiga = 0u32;
    VirtualProtect(de as *mut c_void, n, 0x40, &mut antiga);
    core::ptr::write_bytes(de as *mut u8, 0x90, n);
    VirtualProtect(de as *mut c_void, n, antiga, &mut antiga);
    FlushInstructionCache(GetCurrentProcess(), de as *const c_void, n);
}
