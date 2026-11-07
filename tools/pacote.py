"""Monta os dois zips de uma release: o pacote para o jogador e o código-fonte.

Uso: python tools/pacote.py [--saida PASTA]   (padrão: target/pacote)
Antes: `cargo build --release` na raiz e em configurador/. A versão vem do Cargo.toml. O código-fonte é o do commit
atual (git archive), então faça o commit antes.
- CM0102-Rust-Tweaks-X.Y.zip: ddraw.dll, tweaks-config.exe, tweaks.cfg, intake_news.txt, os guias, o NICKPATCHER, a
  LICENSE e os textos de pacote/ (README.txt e LEIAME.txt, com a versão no título);
- CM0102-Rust-Tweaks-X.Y-source.zip: o repositório no commit atual."""
import argparse, os, re, subprocess, zipfile

RAIZ = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))

def main():
    a = argparse.ArgumentParser()
    a.add_argument('--saida', default=os.path.join(RAIZ, 'target', 'pacote'))
    o = a.parse_args()
    versao = re.search(r'^version = "(\d+\.\d+)', open(os.path.join(RAIZ, 'Cargo.toml'), encoding='utf-8').read(), re.M).group(1)
    nome = f'CM0102-Rust-Tweaks-{versao}'
    os.makedirs(o.saida, exist_ok=True)
    arquivos = {
        'ddraw.dll': 'target/i686-pc-windows-msvc/release/ddraw.dll',
        'tweaks-config.exe': 'configurador/target/i686-pc-windows-msvc/release/tweaks-config.exe',
        'tweaks.cfg': 'tweaks.cfg',
        'intake_news.txt': 'intake_news.txt',
        'GUIA_DAS_CHAVES.md': 'docs/GUIA_DAS_CHAVES.md',
        'GUIA_DAS_CHAVES.en.md': 'docs/GUIA_DAS_CHAVES.en.md',
        'NICKPATCHER.md': 'docs/NICKPATCHER.md',
        'NICKPATCHER.en.md': 'docs/NICKPATCHER.en.md',
        'LICENSE': 'LICENSE',
    }
    zip_jogo = os.path.join(o.saida, nome + '.zip')
    with zipfile.ZipFile(zip_jogo, 'w', zipfile.ZIP_DEFLATED) as z:
        for destino, origem in arquivos.items():
            caminho = os.path.join(RAIZ, origem)
            if not os.path.exists(caminho):
                raise SystemExit(f'falta {origem}: compile antes (cargo build --release na raiz e em configurador/)')
            z.write(caminho, f'{nome}/{destino}')
        for texto in ('README.txt', 'LEIAME.txt'):
            corpo = open(os.path.join(RAIZ, 'pacote', texto), encoding='utf-8').read().replace('{versao}', versao)
            z.writestr(f'{nome}/{texto}', corpo.replace('\n', '\r\n'))
    zip_fonte = os.path.join(o.saida, nome + '-source.zip')
    subprocess.run(['git', 'archive', '--format=zip', f'--prefix={nome}-source/', '-o', zip_fonte, 'HEAD'], cwd=RAIZ, check=True)
    for z in (zip_jogo, zip_fonte):
        print(z, os.path.getsize(z), 'bytes')

if __name__ == '__main__':
    main()
