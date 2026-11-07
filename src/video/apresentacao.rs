//! Cópia de `rust/src/apresentacao.rs` do projeto CM0102 (02/11/2026), para `video = window`; mudar lá e copiar para cá.
//!
//! Mostra o quadro de 16 bits do jogo na janela, sem trocar o modo de video do Windows.
//!
//! - O quadro e desenhado com escala proporcional na area cliente, com faixas pretas se sobrar
//!   espaco; 1:1 (pixel a pixel) quando a area cliente tem o tamanho do quadro.
//! - Em janela, a janela fica redimensionavel (borda e maximizar); o ultimo quadro e redesenhado
//!   durante o redimensionamento. Em tela cheia, a janela do jogo ja cobre a tela.
//! - As mensagens de mouse sao convertidas da area cliente para coordenadas do quadro.
//! - Escala: vizinho mais proximo quando o fator e inteiro (nitido); reducao por media de area
//!   (`reducao.rs`) quando a janela e menor que o quadro; HALFTONE do Windows para ampliar em fator
//!   fracionario. `CM_REDUCAO=windows` volta a reducao para o HALFTONE (para comparar).
//!
//! RGB565 -> 8 bits por canal com replicacao de bits (31 -> 255, 63 -> 255).

use std::sync::{Mutex, OnceLock};

use crate::reducao::Reducao;
use crate::linha_de_video as linha;
use crate::win::{self, HWND, RECT};

fn reducao_propria() -> bool {
    static PROPRIA: OnceLock<bool> = OnceLock::new();
    *PROPRIA.get_or_init(|| {
        let n: Vec<u16> = "CM_REDUCAO".encode_utf16().chain([0]).collect();
        let mut buf = [0u16; 16];
        let k = unsafe { win::GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
        let propria = String::from_utf16_lossy(&buf[..k.min(16)]).to_lowercase() != "windows";
        linha(&format!("apresentacao: reducao {}", if propria { "por media de area" } else { "HALFTONE do Windows" }));
        propria
    })
}

/// Menor area cliente permitida ao redimensionar.
const CLIENTE_MINIMO: (i32, i32) = (400, 300);

struct Tela {
    hwnd: usize,
    /// Quadro convertido para 32 bits, guardado entre apresentacoes (so a regiao suja e reconvertida).
    convertido: Vec<u32>,
    largura: i32,
    altura: i32,
    /// Onde o quadro foi desenhado na area cliente na ultima apresentacao.
    destino: RECT,
    proc_original: isize,
    /// Quadro reduzido ao tamanho da janela (quando ela e menor que o quadro).
    reduzido: Option<Reducao>,
}

static TELA: Mutex<Tela> = Mutex::new(Tela {
    hwnd: 0,
    convertido: Vec::new(),
    largura: 0,
    altura: 0,
    destino: RECT { left: 0, top: 0, right: 0, bottom: 0 },
    proc_original: 0,
    reduzido: None,
});

fn tela() -> std::sync::MutexGuard<'static, Tela> {
    TELA.lock().unwrap_or_else(|e| e.into_inner())
}

#[inline]
pub fn rgb565_para_32(p: u16) -> u32 {
    let r = ((p >> 11) & 31) as u32;
    let g = ((p >> 5) & 63) as u32;
    let b = (p & 31) as u32;
    ((r << 3 | r >> 2) << 16) | ((g << 2 | g >> 4) << 8) | (b << 3 | b >> 2)
}

/// `CM_OCULTO=1` (simulacao em lote): janela fora da tela e nada e desenhado.
pub fn oculto() -> bool {
    static OCULTO: OnceLock<bool> = OnceLock::new();
    *OCULTO.get_or_init(|| win::ambiente("CM_OCULTO"))
}

/// Registra a janela do jogo (filtra as mensagens dela) e, em janela, a torna redimensionavel.
pub unsafe fn definir_janela(hwnd: HWND, tela_cheia: bool) {
    {
        let mut t = tela();
        if t.hwnd != hwnd as usize {
            t.hwnd = hwnd as usize;
            t.proc_original = win::SetWindowLongA(hwnd, win::GWL_WNDPROC, proc_janela as *const () as isize);
            linha(&format!("apresentacao: janela {:p}, procedimento original {:#x}", hwnd, t.proc_original));
        }
    }
    if oculto() {
        // O jogo recoloca a janela no centro sempre que reinicia o video; ela volta para fora da tela.
        win::SetWindowPos(hwnd, core::ptr::null_mut(), -32000, -32000, 0, 0, win::SWP_NOSIZE | win::SWP_NOZORDER | win::SWP_NOACTIVATE);
        return;
    }
    if !tela_cheia {
        tornar_redimensionavel(hwnd);
    }
}

/// Acrescenta borda redimensionavel e botao maximizar e aplica o tamanho escolhido pelo jogador.
///
/// O jogo chama isto de novo sempre que reinicia o video ("Iniciar Novo Jogo", salvar o jogo...),
/// e nessa hora ele mesmo devolve a janela a 800x600 no centro da tela; por isso o tamanho e a
/// posicao escolhidos pelo jogador sao reaplicados.
unsafe fn tornar_redimensionavel(hwnd: HWND) {
    let estilo = win::GetWindowLongA(hwnd, win::GWL_STYLE) as u32;
    let novo = estilo | win::WS_THICKFRAME | win::WS_MAXIMIZEBOX;
    let mut cliente = RECT::default();
    win::GetClientRect(hwnd, &mut cliente);
    let preferido = preferencia::ler();
    let (cw, ch) = preferido.map_or((cliente.right, cliente.bottom), |p| (p.largura, p.altura));
    let mut janela = RECT::default();
    win::GetWindowRect(hwnd, &mut janela);
    win::SetWindowLongA(hwnd, win::GWL_STYLE, novo as isize);
    let mut r = RECT { left: 0, top: 0, right: cw, bottom: ch };
    win::AdjustWindowRectEx(&mut r, novo, 0, win::GetWindowLongA(hwnd, win::GWL_EXSTYLE) as u32);
    let (x, y) = preferido
        .and_then(|p| p.posicao)
        .filter(|&(x, y)| posicao_visivel(x, y))
        .unwrap_or((janela.left, janela.top));
    win::SetWindowPos(
        hwnd, core::ptr::null_mut(), x, y, r.right - r.left, r.bottom - r.top,
        win::SWP_NOZORDER | win::SWP_NOACTIVATE | win::SWP_FRAMECHANGED,
    );
    if preferido.is_some_and(|p| p.maximizada) {
        win::ShowWindow(hwnd, win::SW_MAXIMIZE);
    }
    linha(&format!("apresentacao: janela redimensionavel, area cliente {}x{}{}", cw, ch,
        if preferido.is_some_and(|p| p.maximizada) { " (maximizada)" } else { "" }));
}

/// O canto superior esquerdo da janela cai numa area visivel (algum monitor ligado)?
unsafe fn posicao_visivel(x: i32, y: i32) -> bool {
    let (vx, vy) = (win::GetSystemMetrics(76), win::GetSystemMetrics(77)); // SM_X/YVIRTUALSCREEN
    let (vw, vh) = (win::GetSystemMetrics(78), win::GetSystemMetrics(79)); // SM_CX/CYVIRTUALSCREEN
    x >= vx - 50 && y >= vy - 10 && x < vx + vw - 100 && y < vy + vh - 50
}

/// Guarda tamanho e posicao atuais como escolha do jogador (fim de arrastar/mover, maximizar, restaurar).
unsafe fn guardar_tamanho(hwnd: HWND) {
    let mut cliente = RECT::default();
    win::GetClientRect(hwnd, &mut cliente);
    let mut janela = RECT::default();
    win::GetWindowRect(hwnd, &mut janela);
    let maximizada = win::IsZoomed(hwnd) != 0;
    let atual = preferencia::Janela {
        largura: cliente.right,
        altura: cliente.bottom,
        maximizada,
        posicao: Some((janela.left, janela.top)),
    };
    if atual.largura < CLIENTE_MINIMO.0 || atual.altura < CLIENTE_MINIMO.1 {
        return; // minimizada ou no meio de uma troca de modo
    }
    // Maximizada: guarda so o estado; tamanho e posicao de "restaurar" continuam os anteriores.
    let gravar = match (maximizada, preferencia::ler()) {
        (true, Some(p)) => preferencia::Janela { maximizada: true, ..p },
        (true, None) => preferencia::Janela { largura: 800, altura: 600, maximizada: true, posicao: None },
        (false, _) => atual,
    };
    if preferencia::ler() != Some(gravar) {
        preferencia::gravar(gravar);
        linha(&format!("apresentacao: tamanho escolhido {}x{}{}", gravar.largura, gravar.altura,
            if gravar.maximizada { " (maximizada)" } else { "" }));
    }
}

/// Janela escolhida pelo jogador, em `janela.cfg` ao lado do exe: "largura altura maximizada [x y]"
/// (tamanho da area cliente; x y = canto da janela na tela).
mod preferencia {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Janela {
        pub largura: i32,
        pub altura: i32,
        pub maximizada: bool,
        pub posicao: Option<(i32, i32)>,
    }

    fn caminho() -> std::path::PathBuf {
        std::env::current_exe().map(|p| p.with_file_name("janela.cfg")).unwrap_or_else(|_| "janela.cfg".into())
    }

    pub fn interpretar(texto: &str) -> Option<Janela> {
        let mut v = texto.split_whitespace().map(|s| s.parse::<i32>().ok());
        let (largura, altura, maximizada) = (v.next()??, v.next()??, v.next().flatten().unwrap_or(0));
        let posicao = match (v.next().flatten(), v.next().flatten()) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        };
        (largura >= super::CLIENTE_MINIMO.0 && altura >= super::CLIENTE_MINIMO.1 && largura <= 16384 && altura <= 16384)
            .then_some(Janela { largura, altura, maximizada: maximizada != 0, posicao })
    }

    /// `CM_JANELA=LxA` (testes automatizados): tamanho fixo, sem maximizar e sem gravar nada,
    /// para nao mexer na janela do jogador nem cobrir a tela dele.
    fn de_teste() -> Option<Janela> {
        let n: Vec<u16> = "CM_JANELA".encode_utf16().chain([0]).collect();
        let mut buf = [0u16; 32];
        let k = unsafe { crate::win::GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
        let s = String::from_utf16_lossy(&buf[..k.min(32)]);
        let (l, a) = s.split_once(['x', 'X'])?;
        interpretar(&format!("{} {} 0 40 40", l.trim(), a.trim()))
    }

    pub fn ler() -> Option<Janela> {
        de_teste().or_else(|| interpretar(&std::fs::read_to_string(caminho()).ok()?))
    }

    pub fn gravar(j: Janela) {
        if de_teste().is_some() {
            return;
        }
        let mut s = format!("{} {} {}", j.largura, j.altura, j.maximizada as i32);
        if let Some((x, y)) = j.posicao {
            s += &format!(" {} {}", x, y);
        }
        let _ = std::fs::write(caminho(), s + "\n");
    }
}

/// Converte a regiao `suja` do quadro e mostra o quadro inteiro na janela.
pub unsafe fn apresentar(pixels: &[u16], largura: i32, altura: i32, suja: RECT) {
    if oculto() {
        return;
    }
    let mut t = tela();
    if t.hwnd == 0 {
        return;
    }
    let n = (largura * altura) as usize;
    if t.largura != largura || t.altura != altura || t.convertido.len() != n {
        t.largura = largura;
        t.altura = altura;
        t.convertido = vec![0; n];
    }
    let x0 = suja.left.clamp(0, largura);
    let x1 = suja.right.clamp(0, largura);
    let y0 = suja.top.clamp(0, altura);
    let y1 = suja.bottom.clamp(0, altura);
    for y in y0..y1 {
        let ini = (y * largura) as usize;
        let (orig, dest) = (&pixels[ini..ini + largura as usize], &mut t.convertido[ini..ini + largura as usize]);
        for x in x0 as usize..x1 as usize {
            dest[x] = rgb565_para_32(orig[x]);
        }
    }
    desenhar(&mut t, Some(RECT { left: x0, top: y0, right: x1, bottom: y1 }));
}

/// Onde o quadro cabe na area cliente: escala proporcional, centralizado.
fn encaixe(cw: i32, ch: i32, largura: i32, altura: i32) -> RECT {
    let (dw, dh) = if cw * altura <= ch * largura { (cw, (cw * altura) / largura) } else { ((ch * largura) / altura, ch) };
    let (ox, oy) = ((cw - dw) / 2, (ch - dh) / 2);
    RECT { left: ox, top: oy, right: ox + dw, bottom: oy + dh }
}

/// Desenha o ultimo quadro convertido na area cliente atual. `suja`: regiao do quadro que mudou
/// desde o ultimo desenho (None = tudo).
unsafe fn desenhar(t: &mut Tela, suja: Option<RECT>) {
    let (largura, altura) = (t.largura, t.altura);
    if t.hwnd == 0 || t.convertido.is_empty() {
        return;
    }
    let hwnd = t.hwnd as HWND;
    let mut cliente = RECT::default();
    win::GetClientRect(hwnd, &mut cliente);
    let (cw, ch) = (cliente.right, cliente.bottom);
    if cw <= 0 || ch <= 0 {
        return;
    }
    let d = encaixe(cw, ch, largura, altura);
    t.destino = d;
    let (ox, oy, dw, dh) = (d.left, d.top, d.right - d.left, d.bottom - d.top);

    let info = win::BITMAPINFOHEADER {
        biSize: core::mem::size_of::<win::BITMAPINFOHEADER>() as u32,
        biWidth: largura,
        biHeight: -altura, // de cima para baixo
        biPlanes: 1,
        biBitCount: 32,
        biCompression: 0,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };
    let hdc = win::GetDC(hwnd);
    if hdc.is_null() {
        return;
    }
    if (dw, dh) != (cw, ch) {
        win::PatBlt(hdc, 0, 0, cw, oy, win::BLACKNESS);
        win::PatBlt(hdc, 0, oy + dh, cw, ch - oy - dh, win::BLACKNESS);
        win::PatBlt(hdc, 0, oy, ox, dh, win::BLACKNESS);
        win::PatBlt(hdc, ox + dw, oy, cw - ox - dw, dh, win::BLACKNESS);
    }
    if dw < largura && dh < altura && reducao_propria() {
        let (lw, lh, rw, rh) = (largura as usize, altura as usize, dw as usize, dh as usize);
        let mut regiao = suja.unwrap_or(RECT { left: 0, top: 0, right: largura, bottom: altura });
        if t.reduzido.as_ref().map_or(true, |r| r.origem != (lw, lh) || r.destino != (rw, rh)) {
            t.reduzido = Some(Reducao::nova(lw, lh, rw, rh));
            regiao = RECT { left: 0, top: 0, right: largura, bottom: altura };
        }
        let Tela { reduzido, convertido, .. } = t;
        let r = reduzido.as_mut().unwrap();
        r.atualizar(convertido, regiao.left as usize, regiao.top as usize, regiao.right as usize, regiao.bottom as usize);
        let info_r = win::BITMAPINFOHEADER { biWidth: dw, biHeight: -dh, ..info };
        win::StretchDIBits(
            hdc, ox, oy, dw, dh, 0, 0, dw, dh,
            r.saida.as_ptr() as *const _, &info_r, win::DIB_RGB_COLORS, win::SRCCOPY,
        );
    } else {
        let inteiro = dw % largura == 0 && dh % altura == 0;
        win::SetStretchBltMode(hdc, if inteiro { win::COLORONCOLOR } else { win::HALFTONE });
        win::SetBrushOrgEx(hdc, 0, 0, core::ptr::null_mut());
        win::StretchDIBits(
            hdc, ox, oy, dw, dh, 0, 0, largura, altura,
            t.convertido.as_ptr() as *const _, &info, win::DIB_RGB_COLORS, win::SRCCOPY,
        );
    }
    win::ReleaseDC(hwnd, hdc);
}

/// Area cliente -> coordenadas do quadro, para o jogo continuar recebendo o mouse em 800x600.
fn converter_mouse(t: &Tela, lp: isize) -> isize {
    let d = t.destino;
    let (dw, dh) = (d.right - d.left, d.bottom - d.top);
    if dw <= 0 || dh <= 0 || (d.left, d.top, dw, dh) == (0, 0, t.largura, t.altura) {
        return lp;
    }
    let x = (lp & 0xffff) as i16 as i32;
    let y = ((lp >> 16) & 0xffff) as i16 as i32;
    let qx = ((x - d.left) * t.largura / dw).clamp(0, t.largura - 1);
    let qy = ((y - d.top) * t.altura / dh).clamp(0, t.altura - 1);
    ((qy as isize) << 16) | (qx as isize & 0xffff)
}

const WM_SIZE: u32 = 0x05;
const WM_PAINT: u32 = 0x0f;
const WM_ERASEBKGND: u32 = 0x14;
const WM_SETCURSOR: u32 = 0x20;
const WM_GETMINMAXINFO: u32 = 0x24;
const WM_NCHITTEST: u32 = 0x84;
const WM_SYSCOMMAND: u32 = 0x112;
const WM_EXITSIZEMOVE: u32 = 0x232;
const HTCLIENT: isize = 1;
const SC_MAXIMIZE: usize = 0xf030;
const SC_RESTORE: usize = 0xf120;

unsafe extern "system" fn proc_janela(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize {
    match msg {
        // Bordas e cursores de redimensionar ficam com o Windows (o jogo nao conhece borda).
        WM_NCHITTEST => return win::DefWindowProcA(hwnd, msg, wp, lp),
        WM_SETCURSOR if (lp & 0xffff) != HTCLIENT => return win::DefWindowProcA(hwnd, msg, wp, lp),
        WM_ERASEBKGND => return 1, // o quadro cobre tudo; apagar antes so faria piscar
        WM_GETMINMAXINFO => {
            let r = win::DefWindowProcA(hwnd, msg, wp, lp);
            let mut m = RECT { left: 0, top: 0, right: CLIENTE_MINIMO.0, bottom: CLIENTE_MINIMO.1 };
            win::AdjustWindowRectEx(
                &mut m,
                win::GetWindowLongA(hwnd, win::GWL_STYLE) as u32,
                0,
                win::GetWindowLongA(hwnd, win::GWL_EXSTYLE) as u32,
            );
            let minimo = (lp as *mut i32).add(6); // MINMAXINFO.ptMinTrackSize
            *minimo = m.right - m.left;
            *minimo.add(1) = m.bottom - m.top;
            return r;
        }
        _ => {}
    }
    let (original, lp) = {
        let t = tela();
        // WM_MOUSEMOVE..WM_MBUTTONDBLCLK e WM_XBUTTON*: coordenadas da area cliente.
        // WM_MOUSEWHEEL (0x20a) traz coordenadas de tela e nao e convertida.
        let mouse = (0x200..=0x209).contains(&msg) || (0x20b..=0x20d).contains(&msg);
        (t.proc_original, if mouse { converter_mouse(&t, lp) } else { lp })
    };
    // Escolha do jogador: fim de arrastar a borda (WM_EXITSIZEMOVE) ou maximizar/restaurar pelo menu de
    // sistema, botao ou duplo clique (WM_SYSCOMMAND). Os WM_SIZE vindos do proprio jogo (ele impoe
    // 800x600 ao reiniciar o video) nao contam.
    let escolha = msg == WM_EXITSIZEMOVE || (msg == WM_SYSCOMMAND && matches!(wp & 0xfff0, SC_MAXIMIZE | SC_RESTORE));
    let r = win::CallWindowProcA(original, hwnd, msg, wp, lp);
    if msg == WM_SIZE || msg == WM_PAINT {
        desenhar(&mut tela(), None);
    }
    if escolha {
        guardar_tamanho(hwnd);
    }
    r
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn conversao_565() {
        assert_eq!(rgb565_para_32(0x0000), 0x000000);
        assert_eq!(rgb565_para_32(0xffff), 0xffffff);
        assert_eq!(rgb565_para_32(0xf800), 0xff0000);
        assert_eq!(rgb565_para_32(0x07e0), 0x00ff00);
        assert_eq!(rgb565_para_32(0x001f), 0x0000ff);
        assert_eq!(rgb565_para_32(0x8410), 0x848284); // meio-cinza: 16/31, 32/63, 16/31
    }

    #[test]
    fn preferencia_da_janela() {
        use preferencia::{interpretar, Janela};
        assert_eq!(interpretar("1200 900 0\n"), Some(Janela { largura: 1200, altura: 900, maximizada: false, posicao: None }));
        assert_eq!(interpretar("1200 900 1"), Some(Janela { largura: 1200, altura: 900, maximizada: true, posicao: None }));
        assert_eq!(interpretar("1200 900"), Some(Janela { largura: 1200, altura: 900, maximizada: false, posicao: None }));
        assert_eq!(interpretar("1199 699 0 -8 120"), Some(Janela { largura: 1199, altura: 699, maximizada: false, posicao: Some((-8, 120)) }));
        assert_eq!(interpretar("100 900 0"), None); // menor que o minimo
        assert_eq!(interpretar("lixo"), None);
        assert_eq!(interpretar(""), None);
    }

    #[test]
    fn encaixe_proporcional() {
        assert_eq!(encaixe(800, 600, 800, 600), RECT { left: 0, top: 0, right: 800, bottom: 600 });
        assert_eq!(encaixe(1920, 1080, 800, 600), RECT { left: 240, top: 0, right: 1680, bottom: 1080 });
        assert_eq!(encaixe(1600, 1400, 800, 600), RECT { left: 0, top: 100, right: 1600, bottom: 1300 });
    }

    #[test]
    fn mouse_escalado() {
        let t = Tela {
            hwnd: 1,
            convertido: vec![],
            largura: 800,
            altura: 600,
            destino: RECT { left: 240, top: 0, right: 1680, bottom: 1080 },
            proc_original: 0,
            reduzido: None,
        };
        assert_eq!(converter_mouse(&t, (540 << 16) | 960), (300 << 16) | 400); // centro
        assert_eq!(converter_mouse(&t, 100), 0); // faixa preta: fica na borda do quadro
        let um_para_um = Tela { destino: RECT { left: 0, top: 0, right: 800, bottom: 600 }, ..t };
        assert_eq!(converter_mouse(&um_para_um, (10 << 16) | 20), (10 << 16) | 20);
    }
}
