//! Cópia de `rust/src/win.rs` do projeto CM0102 (02/11/2026), para `video = window`; mudar lá e copiar para cá.
//!
//! Declaracoes minimas da API do Windows usadas pelo nucleo (sem dependencias externas).

#![allow(non_snake_case, clippy::upper_case_acronyms, dead_code)]

use core::ffi::c_void;

pub type HWND = *mut c_void;
pub type HDC = *mut c_void;

#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}

pub const SRCCOPY: u32 = 0x00CC_0020;
pub const BLACKNESS: u32 = 0x0000_0042;
pub const DIB_RGB_COLORS: u32 = 0;
pub const COLORONCOLOR: i32 = 3;
pub const HALFTONE: i32 = 4;
pub const GWL_WNDPROC: i32 = -4;
pub const GWL_STYLE: i32 = -16;
pub const GWL_EXSTYLE: i32 = -20;
pub const WS_THICKFRAME: u32 = 0x0004_0000;
pub const WS_MAXIMIZEBOX: u32 = 0x0001_0000;
pub const SW_MAXIMIZE: i32 = 3;
pub const SWP_NOSIZE: u32 = 0x0001;
pub const SWP_NOZORDER: u32 = 0x0004;
pub const SWP_NOACTIVATE: u32 = 0x0010;
pub const SWP_FRAMECHANGED: u32 = 0x0020;
pub const PAGE_READWRITE: u32 = 0x04;
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const MEM_COMMIT: u32 = 0x1000;
pub const MEM_RESERVE: u32 = 0x2000;

#[link(name = "user32")]
extern "system" {
    pub fn GetDC(hwnd: HWND) -> HDC;
    pub fn ReleaseDC(hwnd: HWND, hdc: HDC) -> i32;
    pub fn GetClientRect(hwnd: HWND, r: *mut RECT) -> i32;
    pub fn GetSystemMetrics(i: i32) -> i32;
    pub fn SetWindowLongA(hwnd: HWND, i: i32, v: isize) -> isize;
    pub fn CallWindowProcA(anterior: isize, hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize;
    pub fn DefWindowProcA(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize;
    pub fn GetWindowLongA(hwnd: HWND, i: i32) -> isize;
    pub fn GetWindowRect(hwnd: HWND, r: *mut RECT) -> i32;
    pub fn AdjustWindowRectEx(r: *mut RECT, estilo: u32, menu: i32, estilo_ex: u32) -> i32;
    pub fn IsZoomed(hwnd: HWND) -> i32;
    pub fn ShowWindow(hwnd: HWND, modo: i32) -> i32;
    pub fn PeekMessageA(msg: *mut [u32; 7], hwnd: HWND, min: u32, max: u32, remover: u32) -> i32;
    pub fn TranslateMessage(msg: *const [u32; 7]) -> i32;
    pub fn DispatchMessageA(msg: *const [u32; 7]) -> isize;
    pub fn SetWindowPos(hwnd: HWND, depois: HWND, x: i32, y: i32, w: i32, h: i32, flags: u32) -> i32;
}

#[link(name = "gdi32")]
extern "system" {
    pub fn StretchDIBits(
        hdc: HDC, xd: i32, yd: i32, wd: i32, hd: i32, xs: i32, ys: i32, ws: i32, hs: i32,
        bits: *const c_void, info: *const BITMAPINFOHEADER, uso: u32, rop: u32,
    ) -> i32;
    pub fn SetStretchBltMode(hdc: HDC, modo: i32) -> i32;
    pub fn SetBrushOrgEx(hdc: HDC, x: i32, y: i32, anterior: *mut c_void) -> i32;
    pub fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: u32) -> i32;
}

#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleFileNameA(modulo: *mut c_void, saida: *mut u8, tam: u32) -> u32;
    pub fn GetModuleHandleW(nome: *const u16) -> *mut c_void;
    pub fn LoadLibraryA(nome: *const u8) -> *mut c_void;
    pub fn GetProcAddress(modulo: *mut c_void, nome: *const u8) -> *mut c_void;
    pub fn VirtualProtect(end: *mut c_void, tam: usize, nova: u32, antiga: *mut u32) -> i32;
    pub fn VirtualAlloc(end: *mut c_void, tam: usize, tipo: u32, protecao: u32) -> *mut c_void;
    pub fn FlushInstructionCache(processo: *mut c_void, end: *const c_void, tam: usize) -> i32;
    pub fn GetCurrentProcess() -> *mut c_void;
    pub fn TerminateProcess(processo: *mut c_void, codigo: u32) -> i32;
    pub fn GetCurrentThreadId() -> u32;
    pub fn GetEnvironmentVariableW(nome: *const u16, buf: *mut u16, tam: u32) -> u32;
    pub fn SetEnvironmentVariableW(nome: *const u16, valor: *const u16) -> i32;
}

/// Variavel de ambiente ligada: definida, nao vazia e diferente de "0". (A simulacao passa todas
/// as variaveis CM_* sempre, com 0 ou 1: o ambiente vai para a heap do jogo e tem de ter o mesmo
/// tamanho em todos os modos.)
/// Valor de uma variavel de ambiente (ate 63 caracteres); vazio se nao existe.
pub fn texto_ambiente(nome: &str) -> String {
    let n: Vec<u16> = nome.encode_utf16().chain([0]).collect();
    let mut buf = [0u16; 64];
    let k = unsafe { GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
    String::from_utf16_lossy(&buf[..k.min(buf.len())])
}

pub fn ambiente(nome: &str) -> bool {
    let n: Vec<u16> = nome.encode_utf16().chain([0]).collect();
    let mut buf = [0u16; 8];
    let k = unsafe { GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
    k > 0 && !(k == 1 && buf[0] == '0' as u16)
}
