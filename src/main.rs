#![allow(dead_code, unused_variables, deprecated, unused_imports)]

mod matrix;
mod plot;

use matrix::*;
use plot::*;

use gnuplot::{AxesCommon, Color, Figure, Fix, RGBString};

fn a() {
    let m = 1000;
    let n = 1000;

    let matrix = gaussian_matrix(m, n);
    let norms = column_norms(&matrix);

    let (bins, counts) = histograma(&norms, 20);
    let mut fg = Figure::new();

    let mean_chi = ((m as f64) - 0.5).sqrt();

    fg.set_terminal("pngcairo size 800, 600", "histograma.png");
    let axes = fg.axes2d();

    axes
        .boxes(&bins, &counts, &[])
        .set_x_label("Norma das colunas", &[])
        .set_y_label("Frequência", &[])
        .set_title("Histograma da Norma 2 das Colunas", &[]);

    let max_count = *counts.iter().max().unwrap() as f64;

    axes
        .lines(
            &[mean_chi, mean_chi],
            &[0.0, max_count],
            &[Color(RGBString("red"))],
        );

    fg.show().unwrap();
}

fn b() {
    let m = 100;
    let n = 1000;
    let num_samples = 10000;

    let matrix = gaussian_matrix(m, n);
    let products = inner_products(&matrix, num_samples);
        
    let (bins, counts) = histograma(&products, 20);
    let mut fg = Figure::new();

    fg.set_terminal("pngcairo size 800, 600", "histograma.png");
    let axes = fg.axes2d();

    axes
        .boxes(&bins, &counts, &[])
        .set_x_label("Produtos internos", &[])
        .set_y_label("Frequência", &[])
        .set_title("Histograma dos produtos internos", &[]);

    fg.show().unwrap();
}

fn c() {
    let m = 100;
    let n = 300;
    let k = 1000;

    let mut results = Vec::with_capacity(k);

    for _ in 0..k {
        let matrix = gaussian_matrix(m, n);
        let max_cos = max_product_normalized(&matrix);
        results.push(max_cos);
    }

    let (bins, counts) = histograma(&results, 20);
    let mut fg = Figure::new();

    fg.set_terminal("pngcairo size 800, 600", "histograma.png");
    let axes = fg.axes2d();

    axes
        .boxes(&bins, &counts, &[])
        .set_x_label("Cossenos máximos", &[])
        .set_y_label("Frequência", &[])
        .set_title("Histograma dos produtos internos", &[]);

    fg.show().unwrap();
}

fn e() {
    let m = 1000;
    let n = 3000;
    let c = 1.0;
    let error = 0.1;
    let k = (c / (error * error * (n as f64).ln())) as usize;

    let mut results = Vec::with_capacity(k);

    for _ in 0..k {
        let matrix = gaussian_matrix(m, n);
        let max_cos = max_product_normalized(&matrix);
        results.push(max_cos);
    }

    let (bins, counts) = histograma(&results, 20);
    let mut fg = Figure::new();

    fg.set_terminal("pngcairo size 800, 600", "histograma.png");
    let axes = fg.axes2d();

    axes
        .boxes(&bins, &counts, &[])
        .set_x_label("Cossenos máximos", &[])
        .set_y_label("Frequência", &[])
        .set_title("Histograma dos produtos internos", &[]);

    fg.show().unwrap();
}

fn main() {
    a();
}
