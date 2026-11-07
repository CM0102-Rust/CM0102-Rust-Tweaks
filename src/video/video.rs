//! Cópia de `rust/src/video.rs` do projeto CM0102 (02/11/2026), para `video = window`; mudar lá e copiar para cá.
//!
//! Camada de video propria: implementa o pedaco do DirectDraw que o cm0102.exe usa.
//!
//! Levantado com o espiao (ver `espiao.rs`): o jogo cria uma superficie primaria e um quadro de
//! 800x600 em 16 bits (RGB565) na memoria; desenha no quadro com Lock/Unlock, pinta areas com
//! Blt(COLORFILL) e copia o quadro para a primaria com Blt. Aqui o quadro e um buffer nosso, a
//! primaria e so a janela, e SetDisplayMode nao troca o modo do Windows.
//!
//! Metodos que o jogo nao usa existem (com o numero certo de argumentos, por ser stdcall) e
//! devolvem DDERR_UNSUPPORTED, registrando a primeira chamada.

use core::ffi::c_void;
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::apresentacao;
use crate::linha_de_video as linha;
use crate::win::{self, RECT};

const DD_OK: i32 = 0;
const DDERR_UNSUPPORTED: i32 = 0x8000_4001_u32 as i32;
const E_NOINTERFACE: i32 = 0x8000_4002_u32 as i32;
const DDERR_INVALIDPARAMS: i32 = 0x8007_0057_u32 as i32;
const DDERR_NOTFOUND: i32 = 0x8876_00FF_u32 as i32;

const DDSCL_FULLSCREEN: usize = 0x1;
const DDSD_HEIGHT: u32 = 0x2;
const DDSD_WIDTH: u32 = 0x4;
const DDSCAPS_PRIMARYSURFACE: u32 = 0x200;
const DDBLT_COLORFILL: usize = 0x400;

// ---------- objetos ----------

#[repr(C)]
struct DirectDraw {
    vtbl: *const usize,
    refs: u32,
    hwnd: usize,
    tela_cheia: bool,
}

#[repr(C)]
struct Superficie {
    vtbl: *const usize,
    refs: u32,
    primaria: bool,
    largura: i32,
    altura: i32,
    pixels: Vec<u16>,
}

#[repr(C)]
struct Clipper {
    vtbl: *const usize,
    refs: u32,
    hwnd: usize,
}

fn aviso_unico(nome: &'static str) {
    static VISTOS: Mutex<Option<HashSet<&'static str>>> = Mutex::new(None);
    let mut v = VISTOS.lock().unwrap_or_else(|e| e.into_inner());
    if v.get_or_insert_with(HashSet::new).insert(nome) {
        linha(&format!("video: {} nao suportado (primeira chamada)", nome));
    }
}

macro_rules! nao_suportado {
    ($f:ident, $nome:literal $(, $a:ident)*) => {
        unsafe extern "system" fn $f(_this: *mut c_void $(, $a: usize)*) -> i32 {
            $( let _ = $a; )*
            aviso_unico($nome);
            DDERR_UNSUPPORTED
        }
    };
}

macro_rules! aceita {
    ($f:ident $(, $a:ident)*) => {
        unsafe extern "system" fn $f(_this: *mut c_void $(, $a: usize)*) -> i32 {
            $( let _ = $a; )*
            DD_OK
        }
    };
}

pub unsafe fn criar() -> *mut c_void {
    linha("video: camada propria ativa");
    // Objetos e pixels que o jogo guarda.
    (Box::into_raw(Box::new(DirectDraw { vtbl: vtable(0), refs: 1, hwnd: 0, tela_cheia: false }))) as *mut c_void
}

// ---------- IUnknown ----------

unsafe extern "system" fn query_interface(_this: *mut c_void, _iid: usize, saida: *mut *mut c_void) -> i32 {
    aviso_unico("QueryInterface");
    if !saida.is_null() {
        *saida = core::ptr::null_mut();
    }
    E_NOINTERFACE
}

// refs e o segundo campo dos tres objetos (#[repr(C)], logo apos a vtable).
unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
    let refs = (this as *mut u32).add(1);
    *refs += 1;
    *refs
}

unsafe extern "system" fn dd_release(this: *mut DirectDraw) -> u32 {
    (*this).refs -= 1;
    let r = (*this).refs;
    if r == 0 {
        drop(Box::from_raw(this));
    }
    r
}

unsafe extern "system" fn s_release(this: *mut Superficie) -> u32 {
    (*this).refs -= 1;
    let r = (*this).refs;
    if r == 0 {
        drop(Box::from_raw(this));
    }
    r
}

unsafe extern "system" fn c_release(this: *mut Clipper) -> u32 {
    (*this).refs -= 1;
    let r = (*this).refs;
    if r == 0 {
        drop(Box::from_raw(this));
    }
    r
}

// ---------- IDirectDraw ----------

unsafe fn ler(p: usize, off: usize) -> u32 {
    *((p + off) as *const u32)
}

unsafe fn escrever(p: usize, off: usize, v: u32) {
    *((p + off) as *mut u32) = v;
}

unsafe extern "system" fn dd_create_clipper(_this: *mut DirectDraw, _flags: usize, saida: *mut *mut c_void, _ext: usize) -> i32 {
    if saida.is_null() {
        return DDERR_INVALIDPARAMS;
    }
    *saida = (Box::into_raw(Box::new(Clipper { vtbl: vtable(2), refs: 1, hwnd: 0 }))) as *mut c_void;
    DD_OK
}

unsafe extern "system" fn dd_create_surface(_this: *mut DirectDraw, desc: usize, saida: *mut *mut c_void, _ext: usize) -> i32 {
    if desc == 0 || saida.is_null() {
        return DDERR_INVALIDPARAMS;
    }
    let flags = ler(desc, 4);
    let caps = ler(desc, 0x68);
    let primaria = caps & DDSCAPS_PRIMARYSURFACE != 0;
    let (largura, altura) = if primaria {
        (win::GetSystemMetrics(0), win::GetSystemMetrics(1))
    } else if flags & (DDSD_WIDTH | DDSD_HEIGHT) == (DDSD_WIDTH | DDSD_HEIGHT) {
        (ler(desc, 0xc) as i32, ler(desc, 8) as i32)
    } else {
        return DDERR_INVALIDPARAMS;
    };
    let pixels = if primaria { Vec::new() } else { vec![0u16; (largura * altura) as usize] };
    linha(&format!("video: superficie {} {}x{}", if primaria { "primaria" } else { "quadro" }, largura, altura));
    *saida = (Box::into_raw(Box::new(Superficie { vtbl: vtable(1), refs: 1, primaria, largura, altura, pixels }))) as *mut c_void;
    DD_OK
}

unsafe extern "system" fn dd_set_cooperative_level(this: *mut DirectDraw, hwnd: usize, flags: usize) -> i32 {
    (*this).hwnd = hwnd;
    (*this).tela_cheia = flags & DDSCL_FULLSCREEN != 0;
    linha(&format!("video: SetCooperativeLevel(hwnd={:#x}, flags={:#x}) tela cheia={}", hwnd, flags, (*this).tela_cheia));
    apresentacao::definir_janela(hwnd as _, (*this).tela_cheia);
    DD_OK
}

unsafe extern "system" fn dd_set_display_mode(_this: *mut DirectDraw, largura: usize, altura: usize, bpp: usize) -> i32 {
    // O modo do Windows nunca muda: e isso que acaba com a "piscada".
    linha(&format!("video: SetDisplayMode({}x{}x{}) ignorado", largura, altura, bpp));
    DD_OK
}

nao_suportado!(dd_compact, "DD.Compact");
nao_suportado!(dd_create_palette, "DD.CreatePalette", a, b, c, d);
nao_suportado!(dd_duplicate_surface, "DD.DuplicateSurface", a, b);
nao_suportado!(dd_enum_display_modes, "DD.EnumDisplayModes", a, b, c, d);
nao_suportado!(dd_enum_surfaces, "DD.EnumSurfaces", a, b, c, d);
aceita!(dd_flip_to_gdi_surface);
nao_suportado!(dd_get_caps, "DD.GetCaps", a, b);
nao_suportado!(dd_get_display_mode, "DD.GetDisplayMode", a);
nao_suportado!(dd_get_fourcc_codes, "DD.GetFourCCCodes", a, b);
nao_suportado!(dd_get_gdi_surface, "DD.GetGDISurface", a);
nao_suportado!(dd_get_monitor_frequency, "DD.GetMonitorFrequency", a);
nao_suportado!(dd_get_scan_line, "DD.GetScanLine", a);
nao_suportado!(dd_get_vertical_blank_status, "DD.GetVerticalBlankStatus", a);
aceita!(dd_initialize, a);
aceita!(dd_restore_display_mode);
aceita!(dd_wait_for_vertical_blank, a, b);

// ---------- IDirectDrawSurface ----------

/// Retangulo pedido (ou a superficie inteira), recortado aos limites.
unsafe fn ret_ou_inteira(s: &Superficie, p: usize) -> RECT {
    let r = if p == 0 { RECT { left: 0, top: 0, right: s.largura, bottom: s.altura } } else { *(p as *const RECT) };
    RECT {
        left: r.left.clamp(0, s.largura),
        top: r.top.clamp(0, s.altura),
        right: r.right.clamp(0, s.largura),
        bottom: r.bottom.clamp(0, s.altura),
    }
}

/// Copia `ro` de `origem` para `rd` de `dest` (vizinho mais proximo se os tamanhos diferirem).
fn copiar(dest: &mut Superficie, rd: RECT, origem: &Superficie, ro: RECT) {
    let (dw, dh, ow, oh) = (rd.right - rd.left, rd.bottom - rd.top, ro.right - ro.left, ro.bottom - ro.top);
    if dw <= 0 || dh <= 0 || ow <= 0 || oh <= 0 {
        return;
    }
    for y in 0..dh {
        let oy = ro.top + y * oh / dh;
        for x in 0..dw {
            let ox = ro.left + x * ow / dw;
            dest.pixels[((rd.top + y) * dest.largura + rd.left + x) as usize] = origem.pixels[(oy * origem.largura + ox) as usize];
        }
    }
}

unsafe extern "system" fn s_blt(this: *mut Superficie, ret_dest: usize, origem: *mut Superficie, ret_origem: usize, flags: usize, fx: usize) -> i32 {
    let dest = &mut *this;
    if flags & DDBLT_COLORFILL != 0 {
        if fx == 0 {
            return DDERR_INVALIDPARAMS;
        }
        if !dest.primaria {
            let cor = ler(fx, 0x50) as u16; // DDBLTFX.dwFillColor
            let r = ret_ou_inteira(dest, ret_dest);
            for y in r.top..r.bottom {
                let ini = (y * dest.largura) as usize;
                dest.pixels[ini + r.left as usize..ini + r.right as usize].fill(cor);
            }
        }
        return DD_OK;
    }
    if origem.is_null() {
        aviso_unico("S.Blt sem origem nem COLORFILL");
        return DDERR_UNSUPPORTED;
    }
    let origem = &*origem;
    if origem.primaria {
        aviso_unico("S.Blt a partir da primaria");
        return DDERR_UNSUPPORTED;
    }
    let ro = ret_ou_inteira(origem, ret_origem);
    if dest.primaria {
        // O destino vem em coordenadas de tela; o que importa e a regiao do quadro que mudou.
        apresentacao::apresentar(&origem.pixels, origem.largura, origem.altura, ro);
    } else if !core::ptr::eq(dest as *const Superficie, origem as *const Superficie) {
        let rd = ret_ou_inteira(dest, ret_dest);
        copiar(dest, rd, origem, ro);
    }
    DD_OK
}

unsafe extern "system" fn s_blt_fast(this: *mut Superficie, x: usize, y: usize, origem: *mut Superficie, ret_origem: usize, _trans: usize) -> i32 {
    if origem.is_null() || (*origem).primaria {
        return DDERR_INVALIDPARAMS;
    }
    let origem = &*origem;
    let ro = ret_ou_inteira(origem, ret_origem);
    let dest = &mut *this;
    if dest.primaria {
        apresentacao::apresentar(&origem.pixels, origem.largura, origem.altura, ro);
    } else if !core::ptr::eq(dest as *const Superficie, origem as *const Superficie) {
        let (x, y) = (x as i32, y as i32);
        let rd = RECT { left: x, top: y, right: x + ro.right - ro.left, bottom: y + ro.bottom - ro.top };
        copiar(dest, rd, origem, ro);
    }
    DD_OK
}

unsafe fn preencher_desc(s: &Superficie, desc: usize, mem: usize) {
    escrever(desc, 4, 0x100f); // CAPS | HEIGHT | WIDTH | PITCH | PIXELFORMAT
    escrever(desc, 8, s.altura as u32);
    escrever(desc, 0xc, s.largura as u32);
    escrever(desc, 0x10, (s.largura * 2) as u32);
    escrever(desc, 0x24, mem as u32);
    escrever(desc, 0x48, 32); // DDPIXELFORMAT
    escrever(desc, 0x4c, 0x40); // DDPF_RGB
    escrever(desc, 0x50, 0);
    escrever(desc, 0x54, 16);
    escrever(desc, 0x58, 0xf800);
    escrever(desc, 0x5c, 0x07e0);
    escrever(desc, 0x60, 0x001f);
    escrever(desc, 0x64, 0);
    escrever(desc, 0x68, if s.primaria { 0x1000_c200 } else { 0x840 }); // os mesmos caps que o DirectDraw devolvia
}

unsafe extern "system" fn s_get_surface_desc(this: *mut Superficie, desc: usize) -> i32 {
    if desc == 0 {
        return DDERR_INVALIDPARAMS;
    }
    preencher_desc(&*this, desc, 0);
    DD_OK
}

unsafe extern "system" fn s_lock(this: *mut Superficie, ret: usize, desc: usize, _flags: usize, _evento: usize) -> i32 {
    let s = &mut *this;
    if s.primaria || desc == 0 {
        aviso_unico("S.Lock na primaria");
        return DDERR_UNSUPPORTED;
    }
    // Com retangulo, o ponteiro devolvido aponta para o canto dele (como no DirectDraw).
    let r = ret_ou_inteira(s, ret);
    let mem = s.pixels.as_mut_ptr().add((r.top * s.largura + r.left) as usize) as usize;
    preencher_desc(s, desc, mem);
    DD_OK
}

unsafe extern "system" fn s_get_attached_surface(_this: *mut c_void, _caps: usize, saida: *mut *mut c_void) -> i32 {
    if !saida.is_null() {
        *saida = core::ptr::null_mut();
    }
    DDERR_NOTFOUND
}

unsafe extern "system" fn s_set_clipper(_this: *mut c_void, _clipper: usize) -> i32 {
    DD_OK
}

aceita!(s_unlock, a);
aceita!(s_is_lost);
aceita!(s_restore);
aceita!(s_flip, a, b);
aceita!(s_get_blt_status, a);
aceita!(s_get_flip_status, a);
nao_suportado!(s_add_attached_surface, "S.AddAttachedSurface", a);
nao_suportado!(s_add_overlay_dirty_rect, "S.AddOverlayDirtyRect", a);
nao_suportado!(s_blt_batch, "S.BltBatch", a, b, c);
nao_suportado!(s_delete_attached_surface, "S.DeleteAttachedSurface", a, b);
nao_suportado!(s_enum_attached_surfaces, "S.EnumAttachedSurfaces", a, b);
nao_suportado!(s_enum_overlay_z_orders, "S.EnumOverlayZOrders", a, b, c);
nao_suportado!(s_get_caps, "S.GetCaps", a);
nao_suportado!(s_get_clipper, "S.GetClipper", a);
nao_suportado!(s_get_color_key, "S.GetColorKey", a, b);
nao_suportado!(s_get_dc, "S.GetDC", a);
nao_suportado!(s_get_overlay_position, "S.GetOverlayPosition", a, b);
nao_suportado!(s_get_palette, "S.GetPalette", a);
nao_suportado!(s_get_pixel_format, "S.GetPixelFormat", a);
nao_suportado!(s_initialize, "S.Initialize", a, b);
nao_suportado!(s_release_dc, "S.ReleaseDC", a);
nao_suportado!(s_set_color_key, "S.SetColorKey", a, b);
nao_suportado!(s_set_overlay_position, "S.SetOverlayPosition", a, b);
nao_suportado!(s_set_palette, "S.SetPalette", a);
nao_suportado!(s_update_overlay, "S.UpdateOverlay", a, b, c, d, e);
nao_suportado!(s_update_overlay_display, "S.UpdateOverlayDisplay", a);
nao_suportado!(s_update_overlay_z_order, "S.UpdateOverlayZOrder", a, b);

// ---------- IDirectDrawClipper ----------

unsafe extern "system" fn c_set_hwnd(this: *mut Clipper, _flags: usize, hwnd: usize) -> i32 {
    (*this).hwnd = hwnd;
    DD_OK
}

unsafe extern "system" fn c_get_hwnd(this: *mut Clipper, saida: *mut usize) -> i32 {
    if saida.is_null() {
        return DDERR_INVALIDPARAMS;
    }
    *saida = (*this).hwnd;
    DD_OK
}

nao_suportado!(c_get_clip_list, "C.GetClipList", a, b, c);
aceita!(c_initialize, a, b);
nao_suportado!(c_is_clip_list_changed, "C.IsClipListChanged", a);
nao_suportado!(c_set_clip_list, "C.SetClipList", a, b);

macro_rules! tabela {
    ($($f:ident),* $(,)?) => { vec![$($f as *const () as usize),*] };
}

fn vtable(i: usize) -> *const usize {
    static TABELAS: OnceLock<[Vec<usize>; 3]> = OnceLock::new();
    let t = TABELAS.get_or_init(|| {
        let dd = tabela![
            query_interface, add_ref, dd_release, dd_compact, dd_create_clipper, dd_create_palette, dd_create_surface,
            dd_duplicate_surface, dd_enum_display_modes, dd_enum_surfaces, dd_flip_to_gdi_surface, dd_get_caps,
            dd_get_display_mode, dd_get_fourcc_codes, dd_get_gdi_surface, dd_get_monitor_frequency, dd_get_scan_line,
            dd_get_vertical_blank_status, dd_initialize, dd_restore_display_mode, dd_set_cooperative_level,
            dd_set_display_mode, dd_wait_for_vertical_blank,
        ];
        let s = tabela![
            query_interface, add_ref, s_release, s_add_attached_surface, s_add_overlay_dirty_rect, s_blt, s_blt_batch,
            s_blt_fast, s_delete_attached_surface, s_enum_attached_surfaces, s_enum_overlay_z_orders, s_flip,
            s_get_attached_surface, s_get_blt_status, s_get_caps, s_get_clipper, s_get_color_key, s_get_dc,
            s_get_flip_status, s_get_overlay_position, s_get_palette, s_get_pixel_format, s_get_surface_desc,
            s_initialize, s_is_lost, s_lock, s_release_dc, s_restore, s_set_clipper, s_set_color_key,
            s_set_overlay_position, s_set_palette, s_unlock, s_update_overlay, s_update_overlay_display,
            s_update_overlay_z_order,
        ];
        let c = tabela![
            query_interface, add_ref, c_release, c_get_clip_list, c_get_hwnd, c_initialize, c_is_clip_list_changed,
            c_set_clip_list, c_set_hwnd,
        ];
        assert_eq!((dd.len(), s.len(), c.len()), (23, 36, 9));
        [dd, s, c]
    });
    t[i].as_ptr()
}

#[cfg(test)]
mod testes {
    use super::*;

    fn quadro(l: i32, a: i32) -> Superficie {
        Superficie { vtbl: core::ptr::null(), refs: 1, primaria: false, largura: l, altura: a, pixels: vec![0; (l * a) as usize] }
    }

    #[test]
    fn lock_com_retangulo_aponta_para_o_canto() {
        let mut s = quadro(800, 600);
        let mut desc = [0u32; 27];
        let r = RECT { left: 140, top: 40, right: 660, bottom: 560 };
        unsafe {
            assert_eq!(s_lock(&mut s, &r as *const RECT as usize, desc.as_mut_ptr() as usize, 0, 0), DD_OK);
        }
        let base = s.pixels.as_ptr() as usize;
        assert_eq!(desc[0x24 / 4] as usize - base, 40 * 1600 + 140 * 2); // como o DirectDraw devolvia
        assert_eq!(desc[0x10 / 4], 1600);
        assert_eq!(desc[0x54 / 4], 16);
    }

    #[test]
    fn colorfill_e_copia() {
        let mut a = quadro(4, 3);
        let mut fx = [0u32; 25];
        fx[0x50 / 4] = 0xabcd;
        let r = RECT { left: 1, top: 1, right: 3, bottom: 2 };
        unsafe {
            s_blt(&mut a, &r as *const RECT as usize, core::ptr::null_mut(), 0, DDBLT_COLORFILL, fx.as_ptr() as usize);
        }
        assert_eq!(a.pixels, vec![0, 0, 0, 0, 0, 0xabcd, 0xabcd, 0, 0, 0, 0, 0]);
        let mut b = quadro(4, 3);
        unsafe {
            s_blt_fast(&mut b, 0, 0, &mut a, &r as *const RECT as usize, 0);
        }
        assert_eq!(&b.pixels[..4], &[0xabcd, 0xabcd, 0, 0]);
    }
}
