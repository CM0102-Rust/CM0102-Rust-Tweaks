//! Cópia de `rust/src/reducao.rs` do projeto CM0102 (02/11/2026), para `video = window`; mudar lá e copiar para cá.
//!
//! Reducao de imagem por media de area: cada pixel de destino e a media dos pixels de origem que
//! ele cobre, ponderada pela fracao de area coberta. E o filtro certo para reduzir (nao deixa
//! aliasing e preserva o traco fino do texto melhor que o HALFTONE do Windows).
//!
//! Pixels em 0x00RRGGBB. Os pesos de cada eixo sao calculados uma vez por tamanho; a cada quadro
//! so a regiao de destino afetada pela regiao suja da origem e recalculada.

/// Soma dos pesos de cada eixo (inteiros); o peso de um pixel e px * py, com soma UM * UM.
const UM: u32 = 256;

/// Para cada indice de destino num eixo: primeiro indice de origem e os pesos (soma exata `UM`).
struct Eixo {
    inicio: Vec<u32>,
    base: Vec<u32>,
    quantos: Vec<u32>,
    pesos: Vec<u32>,
}

impl Eixo {
    fn novo(origem: usize, destino: usize) -> Eixo {
        let escala = origem as f64 / destino as f64;
        let mut e = Eixo { inicio: vec![], base: vec![], quantos: vec![], pesos: vec![] };
        for d in 0..destino {
            let (a, b) = (d as f64 * escala, (d + 1) as f64 * escala);
            let i0 = a.floor() as usize;
            let i1 = (b.ceil() as usize).min(origem).max(i0 + 1);
            e.inicio.push(i0 as u32);
            e.base.push(e.pesos.len() as u32);
            e.quantos.push((i1 - i0) as u32);
            let ws: Vec<f64> = (i0..i1).map(|i| (b.min(i as f64 + 1.0) - a.max(i as f64)).max(0.0)).collect();
            let soma: f64 = ws.iter().sum();
            // Arredonda cada peso e acerta o maior para a soma dar exatamente UM (cor uniforme nao muda).
            let mut inteiros: Vec<u32> = ws.iter().map(|w| (w / soma * UM as f64).round() as u32).collect();
            let total: u32 = inteiros.iter().sum();
            let maior = (0..inteiros.len()).max_by_key(|&i| inteiros[i]).unwrap();
            inteiros[maior] = (inteiros[maior] + UM).saturating_sub(total);
            e.pesos.extend(inteiros);
        }
        e
    }

    /// Faixa de destino [d0, d1) afetada pela faixa de origem [o0, o1).
    fn afetados(&self, o0: usize, o1: usize) -> (usize, usize) {
        let d0 = self.inicio.partition_point(|&i| (i as usize) < o0).saturating_sub(1);
        let d1 = self.inicio.partition_point(|&i| (i as usize) < o1);
        (d0, d1)
    }
}

pub struct Reducao {
    pub origem: (usize, usize),
    pub destino: (usize, usize),
    ex: Eixo,
    ey: Eixo,
    pub saida: Vec<u32>,
}

impl Reducao {
    pub fn nova(ow: usize, oh: usize, dw: usize, dh: usize) -> Reducao {
        Reducao { origem: (ow, oh), destino: (dw, dh), ex: Eixo::novo(ow, dw), ey: Eixo::novo(oh, dh), saida: vec![0; dw * dh] }
    }

    /// Recalcula os pixels de destino afetados pela regiao [x0,x1) x [y0,y1) da origem.
    pub fn atualizar(&mut self, origem: &[u32], x0: usize, y0: usize, x1: usize, y1: usize) {
        let (ow, dw) = (self.origem.0, self.destino.0);
        let (dx0, dx1) = self.ex.afetados(x0, x1);
        let (dy0, dy1) = self.ey.afetados(y0, y1);
        for dy in dy0..dy1 {
            let (yi, yb, yn) = (self.ey.inicio[dy] as usize, self.ey.base[dy] as usize, self.ey.quantos[dy] as usize);
            for dx in dx0..dx1 {
                let (xi, xb, xn) = (self.ex.inicio[dx] as usize, self.ex.base[dx] as usize, self.ex.quantos[dx] as usize);
                let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
                let px = &self.ex.pesos[xb..xb + xn];
                for j in 0..yn {
                    let wy = self.ey.pesos[yb + j];
                    let linha = &origem[(yi + j) * ow + xi..(yi + j) * ow + xi + xn];
                    for (&p, &wx) in linha.iter().zip(px) {
                        let w = wy * wx;
                        r += ((p >> 16) & 0xff) * w;
                        g += ((p >> 8) & 0xff) * w;
                        b += (p & 0xff) * w;
                    }
                }
                const METADE: u32 = UM * UM / 2;
                const BITS: u32 = (UM * UM).trailing_zeros();
                self.saida[dy * dw + dx] = (((r + METADE) >> BITS) << 16) | (((g + METADE) >> BITS) << 8) | ((b + METADE) >> BITS);
            }
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn mesma_escala_e_identidade() {
        let orig: Vec<u32> = (0..12).map(|i| i * 0x010203).collect();
        let mut r = Reducao::nova(4, 3, 4, 3);
        r.atualizar(&orig, 0, 0, 4, 3);
        assert_eq!(r.saida, orig);
    }

    #[test]
    fn metade_tira_a_media_de_2x2() {
        let orig = vec![0x000000, 0x0000ff, 0x00ff00, 0xff0000];
        let mut r = Reducao::nova(2, 2, 1, 1);
        r.atualizar(&orig, 0, 0, 2, 2);
        assert_eq!(r.saida, vec![0x404040]); // (0+0+0+255)/4 = 63.75 -> 64 em cada canal
    }

    #[test]
    fn escala_fracionaria_preserva_cor_uniforme_e_pesos() {
        let e = Eixo::novo(1920, 1583);
        for d in 0..1583 {
            let (b, n) = (e.base[d] as usize, e.quantos[d] as usize);
            assert_eq!(e.pesos[b..b + n].iter().sum::<u32>(), UM);
        }
        let orig = vec![0x336699; 1920 * 1080];
        let mut r = Reducao::nova(1920, 1080, 1583, 890);
        r.atualizar(&orig, 0, 0, 1920, 1080);
        assert!(r.saida.iter().all(|&p| p == 0x336699));
    }

    /// `cargo test --release -- --ignored --nocapture tempo`
    #[test]
    #[ignore]
    fn tempo_quadro_inteiro() {
        let orig: Vec<u32> = (0..1920 * 1080).map(|i| (i as u32).wrapping_mul(2654435761)).collect();
        let mut r = Reducao::nova(1920, 1080, 1583, 890);
        let t = std::time::Instant::now();
        for _ in 0..20 {
            r.atualizar(&orig, 0, 0, 1920, 1080);
        }
        println!("1920x1080 -> 1583x890: {:.1} ms por quadro inteiro", t.elapsed().as_secs_f64() * 1000.0 / 20.0);
    }

    #[test]
    fn regiao_suja_so_recalcula_o_afetado() {
        let mut orig = vec![0u32; 100 * 100];
        let mut r = Reducao::nova(100, 100, 60, 60);
        r.atualizar(&orig, 0, 0, 100, 100);
        orig[50 * 100 + 50] = 0xffffff;
        r.atualizar(&orig, 50, 50, 51, 51);
        let acesos: Vec<usize> = (0..3600).filter(|&i| r.saida[i] != 0).collect();
        assert!(!acesos.is_empty());
        assert!(acesos.iter().all(|&i| (29..=31).contains(&(i / 60)) && (29..=31).contains(&(i % 60))));
        // O resultado parcial e igual a recalcular tudo.
        let mut cheio = Reducao::nova(100, 100, 60, 60);
        cheio.atualizar(&orig, 0, 0, 100, 100);
        assert_eq!(r.saida, cheio.saida);
    }
}
