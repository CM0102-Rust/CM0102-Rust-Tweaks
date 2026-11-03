// O ícone do tweaks-config.exe no Explorer e na barra de tarefas (assets/icon, o .ico com 16 a 256 px).
fn main() {
    let mut r = winresource::WindowsResource::new();
    r.set_icon("../assets/icon/cm0102-rust-tweaks-windows.ico");
    r.compile().expect("o recurso do ícone");
    println!("cargo:rerun-if-changed=../assets/icon/cm0102-rust-tweaks-windows.ico");
}
