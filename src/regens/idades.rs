//! As idades dos regens (`age_NN`, `contest_max_age`, `ages_out_of_game`).

use super::*;

/// As idades que o `.cfg` aceita (`age_14` a `age_35`).
pub(crate) const IDADE_MIN: usize = 14;
pub(crate) const IDADES: usize = 22;

/// Uma idade da tabela do `.cfg` (`age_NN`), pelo sorteio do jogo.
pub(crate) unsafe fn idade_da_tabela(r: &Regras) -> usize {
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
pub(crate) fn rejuvenescer(r: &Regras, idade: usize) -> usize {
    if r.mais_novo_em > 0 && idade as i32 >= r.mais_novo_a_partir {
        (idade as i32 - r.mais_novo_em).max(IDADE_MIN as i32) as usize
    } else {
        idade
    }
}

pub(crate) unsafe extern "C" fn sortear_idade_fora(pessoa: usize) -> u32 {
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
pub(crate) unsafe extern "C" fn sortear_idade(pessoa: usize) -> u32 {
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
pub(crate) unsafe fn instalar_idade(r: &Regras) -> bool {
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
pub(crate) unsafe fn instalar_idade_fora() -> bool {
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
