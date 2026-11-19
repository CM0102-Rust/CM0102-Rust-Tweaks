//! `manage_any_team`: assumir qualquer seleção principal, no começo da carreira e durante ela. Porte da parte das
//! seleções da chave de mesmo nome do `clz.dll` (que também libera os clubes sem divisão do CLZ). Com o `clz.dll` ao
//! lado do exe e `manage_any_team = 1` no `competitions.cfg`, a regra e a escolha ficam com ele (os dois desviam a mesma
//! rotina); os cinzentos da seleção do humano (abaixo) seguem com o Tweaks, que o `clz.dll` não tem.
//!
//! O jogo decide se um time pode ter técnico humano em `club_can_be_managed` (Database.cpp, 0x544820, cdecl(clube)
//! -> al): o botão de candidatura, a contratação de técnicos, as regens e a escolha do clube no começo da carreira. Para
//! uma seleção (id em [total − 2 × nações, total − nações)), o original exige mais de 150 pessoas cadastradas na nação
//! (+0x88, NumberStaff): as seleções pequenas ficam de fora. Com a chave, todas as do primeiro conjunto passam; o
//! segundo conjunto continua com o original.
//!
//! Os cinzentos: a criação dos cinzentos de um time (0x90dae0) sai sem criar nenhum quando o time é uma seleção com
//! técnico humano (`0x600a50(time, 1, 1, 0, 0)` em 0x90dc6d): o humano tinha de convocar só jogadores reais, e uma
//! seleção pequena nunca completava a lista. Com a chave, a seleção principal do humano recebe cinzentos como um clube
//! (e, com `grey_signing`, o item Efetivar).
//!
//! A escolha do clube na criação da carreira (Setup.cpp, 0x820d40) tirava as seleções da lista; com a chave elas entram,
//! num item "Seleções" no fim do seletor de nação (que aparece também com uma nação só).

use crate::{d, registro};

const ROTINA: usize = 0x544820;
/// push esi; mov esi, [esp+8]
const PROLOGO: &[u8] = &[0x56, 0x8b, 0x74, 0x24, 0x08];
static mut ORIGINAL: usize = 0;

/// A chave no `tweaks.cfg`: `None` desligada; `Some(true)` tudo aqui; `Some(false)` a regra fica com o `clz.dll`.
fn ligada() -> Option<bool> {
    let pasta = std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())).unwrap_or_default();
    let chave = |arq: &str| {
        std::fs::read_to_string(pasta.join(arq)).ok().is_some_and(|t| {
            t.lines().any(|l| {
                let l = l.split('#').next().unwrap_or("");
                l.split_once('=').is_some_and(|(k, v)| k.trim() == "manage_any_team" && matches!(v.trim(), "1" | "on"))
            })
        })
    };
    if !chave("tweaks.cfg") {
        return None;
    }
    if pasta.join("clz.dll").exists() && chave("competitions.cfg") {
        registro::linha("manage any team: which teams can be taken is left to clz.dll (manage_any_team = 1 in competitions.cfg)");
        return Some(false);
    }
    Some(true)
}

pub(crate) unsafe fn instalar() {
    let Some(tudo) = ligada() else { return };
    // os cinzentos da seleção do humano: a chamada de 0x90dc6d passa por `humano_na_selecao`
    let cinzentos = [(0x90dc6dusize, call(0x90dc6d, 0x600a50), call(0x90dc6d, humano_na_selecao as *const () as usize))];
    let lista: Vec<(usize, &[u8], &[u8])> = cinzentos.iter().map(|(a, b, c)| (*a, &b[..], &c[..])).collect();
    if crate::aplicar_trocas("manage any team (grey players for the human's national team)", &lista, true) {
        registro::linha("manage any team: the human manager's national team gets grey players to fill its squad, like a club");
    }
    if !tudo {
        return;
    }
    let Some(t) = crate::origem::trampolim(ROTINA, PROLOGO, assumivel as *const () as usize) else {
        return registro::linha("manage any team: club_can_be_managed at 0x544820 differs from 3.9.68 (another DLL or patch); off");
    };
    ORIGINAL = t;
    let mut cav = Caverna::nova();
    let mut trocas: Vec<(usize, Vec<u8>, Vec<u8>)> = Vec::new();
    // a carreira pelo auto_setup.cfg (0x827110) segue a regra original
    trocas.push((0x827d0b, call(0x827d0b, ROTINA), call(0x827d0b, t)));
    // as seleções entram na lista da escolha: test eax, eax; jne +0x16 -> test eax, eax; nop; nop
    trocas.push((0x8226b9, vec![0x85, 0xc0, 0x75, 0x16], vec![0x85, 0xc0, 0x90, 0x90]));
    // a linha da seleção (0x8227b1) escreve [nação+0x71]+0x1f; sem o +0x71 (nação não carregada), o texto vazio
    {
        let vazio = VAZIO.as_ptr() as usize - 0x1f;
        let c = cav.aqui();
        let mut k: Vec<u8> = vec![0x8b, 0x42, 0x71, 0x85, 0xc0, 0x75, 0x05, 0xb8];
        k.extend((vazio as u32).to_le_bytes());
        k.extend([0x8d, 0x57, 0x01, 0x83, 0xc0, 0x1f]);
        k.extend(jmp(c + k.len(), 0x8227ba));
        cav.por(&k);
        let mut depois = jmp(0x8227b1, c);
        depois.extend([0x90; 4]);
        trocas.push((0x8227b1, vec![0x8b, 0x42, 0x71, 0x8d, 0x57, 0x01, 0x83, 0xc0, 0x1f], depois));
    }
    // o filtro da lista (0x822698): com a nação falsa SELECOES escolhida, só as seleções; senão o do original
    {
        const ENTRA: usize = 0x8226a3;
        const PULA: usize = 0x8226d3;
        let fake = core::ptr::addr_of_mut!(SELECOES) as usize;
        let c = cav.aqui();
        let mut k: Vec<u8> = vec![0x8b, 0x4c, 0x24, 0x30, 0x81, 0xf9]; // mov ecx, [esp+0x30]; cmp ecx, fake
        k.extend((fake as u32).to_le_bytes());
        k.extend([0x75, 0x21]); // jne normal
        k.extend([0x8b, 0x06]); // mov eax, [esi] (o id)
        k.extend([0x8b, 0x15, 0x5c, 0x23, 0xae, 0x00]); // mov edx, [0xae235c]
        k.extend([0x2b, 0x15, 0x50, 0x23, 0xae, 0x00]); // sub edx, [0xae2350]
        k.extend([0x2b, 0x15, 0x50, 0x23, 0xae, 0x00]); // sub edx, [0xae2350]
        k.extend([0x3b, 0xc2, 0x0f, 0x8c]); // cmp eax, edx; jl PULA
        k.extend(rel(c + k.len() + 4, PULA));
        k.extend(jmp(c + k.len(), ENTRA));
        k.extend([0x8b, 0x46, 0x53, 0x3b, 0xc1, 0x0f, 0x85]); // normal: mov eax, [esi+0x53]; cmp eax, ecx; jne PULA
        k.extend(rel(c + k.len() + 4, PULA));
        k.extend(jmp(c + k.len(), ENTRA));
        cav.por(&k);
        let mut depois = jmp(0x822698, c);
        depois.extend([0x90; 6]);
        trocas.push((0x822698, vec![0x8b, 0x4c, 0x24, 0x30, 0x8b, 0x46, 0x53, 0x3b, 0xc1, 0x75, 0x30], depois));
    }
    // o item Seleções no fim do menu de nações (0x82257a): pushad; push ebx; call item_selecoes; add esp, 4; popad;
    // mov edi, 0xaa; jmp 0x82257f
    {
        let c = cav.aqui();
        let mut k: Vec<u8> = vec![0x60, 0x53, 0xe8];
        k.extend(rel(c + k.len() + 4, item_selecoes as *const () as usize));
        k.extend([0x83, 0xc4, 0x04, 0x61, 0xbf, 0xaa, 0x00, 0x00, 0x00]);
        k.extend(jmp(c + k.len(), 0x82257f));
        cav.por(&k);
        trocas.push((0x82257a, vec![0xbf, 0xaa, 0x00, 0x00, 0x00], jmp(0x82257a, c)));
    }
    // o seletor também com uma nação só: o menu (0x822306) e o filtro (0x822693) comparavam [0xae2d04] com 1, agora com 0
    trocas.push((0x822306, vec![0x83, 0xf8, 0x01, 0x0f, 0x8e], vec![0x83, 0xf8, 0x00, 0x0f, 0x8e]));
    // o NickPatcher atual já apaga o `jle` do filtro (0x822696, `90 90`): o mesmo efeito; aí o ponto fica como está
    if *(0x822696 as *const [u8; 2]) != [0x90, 0x90] {
        trocas.push((0x822693, vec![0x83, 0xf8, 0x01, 0x7e, 0x0b], vec![0x83, 0xf8, 0x00, 0x7e, 0x0b]));
    }
    let lista: Vec<(usize, &[u8], &[u8])> = trocas.iter().map(|(a, b, c)| (*a, &b[..], &c[..])).collect();
    if !crate::aplicar_trocas("manage any team (team choice screen)", &lista, true) {
        registro::linha("manage any team: national teams can be taken during the career, but not picked when starting one");
    }
    registro::linha("manage any team: on (every main national team can have a human manager)");
}

static VAZIO: [u8; 0x20] = [0; 0x20];
/// A "nação" falsa do item Seleções: o seletor guarda o ponteiro da nação escolhida e a lista o compara com +0x53 de
/// cada clube; um bloco zerado com o nome em +4.
static mut SELECOES: [u8; 0x200] = [0; 0x200];

/// O item no menu do seletor (`menu`: o ebx de 0x822387), com os argumentos dos itens de nação (0x55f990, thiscall).
unsafe extern "C" fn item_selecoes(menu: usize) {
    let fake = core::ptr::addr_of_mut!(SELECOES) as usize;
    let texto: &[u8] = if matches!(d(0x9eb2dc) as i32, 9 | 17) { b"Sele\xe7\xf5es\0" } else { b"National teams\0" };
    core::ptr::write_bytes((fake + 4) as *mut u8, 0, 0x40);
    core::ptr::copy_nonoverlapping(texto.as_ptr(), (fake + 4) as *mut u8, texto.len());
    let w = |a: usize| core::ptr::read_unaligned(a as *const u16) as u32;
    let tela = core::ptr::read_unaligned(0xb77c7a as *const i16) as i32 as usize;
    let obj = d(0xb74c4c + tela.wrapping_mul(0x300));
    #[allow(clippy::type_complexity)]
    let item: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, u32, usize, usize) -> usize =
        core::mem::transmute(0x55f990usize);
    item(obj, 2, 0, 0, 0, 0, 0, 0, 0x10, w(0xae31aa), w(0xaebdec), 1, 1, w(0xaebdea), fake + 4, 0, 0x1f, fake, menu);
}

/// { 0x90dc6d } `0x600a50(time, 1, 1, 0, 0)` (cdecl): o time tem técnico humano? Para uma seleção principal, 0: a
/// criação dos cinzentos segue como para uma seleção da IA.
unsafe extern "C" fn humano_na_selecao(time: usize, a: u32, b: u32, c: u32, e: u32) -> u32 {
    let (nacoes, total) = (d(0xae2350) as i32, d(0xae235c) as i32);
    let id = if time == 0 { -1 } else { d(time) as i32 };
    if id >= total - 2 * nacoes && id < total - nacoes {
        return 0;
    }
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(time, a, b, c, e)
}

unsafe extern "C" fn assumivel(clube: usize) -> u32 {
    let original: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(ORIGINAL);
    let r = original(clube);
    if r as u8 != 0 || clube == 0 || d(clube + 0x53) == 0 {
        return r;
    }
    let (nacoes, total, id) = (d(0xae2350) as i32, d(0xae235c) as i32, d(clube) as i32);
    if id >= total - 2 * nacoes && id < total - nacoes {
        return 1;
    }
    r
}

fn rel(de: usize, para: usize) -> [u8; 4] {
    (para.wrapping_sub(de) as u32).to_le_bytes()
}
fn jmp(de: usize, para: usize) -> Vec<u8> {
    let mut v = vec![0xe9u8];
    v.extend(rel(de + 5, para));
    v
}
fn call(de: usize, para: usize) -> Vec<u8> {
    let mut v = vec![0xe8u8];
    v.extend(rel(de + 5, para));
    v
}

/// Uma página executável para os trechos novos.
struct Caverna(usize, usize);
impl Caverna {
    unsafe fn nova() -> Self {
        Caverna(crate::VirtualAlloc(core::ptr::null_mut(), 4096, 0x3000, 0x40) as usize, 0)
    }
    fn aqui(&self) -> usize {
        self.0 + self.1
    }
    unsafe fn por(&mut self, k: &[u8]) {
        core::ptr::copy_nonoverlapping(k.as_ptr(), self.aqui() as *mut u8, k.len());
        self.1 += (k.len() + 15) & !15;
    }
}
