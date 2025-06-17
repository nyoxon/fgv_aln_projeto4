use gnuplot::{Figure, AxesCommon};

pub fn histograma(data: &[f64], bins: usize) -> (Vec<f64>, Vec<u32>) {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let bin_width = (max - min) / bins as f64;

    let mut counts = vec![0u32; bins];
    let mut bin_centers = Vec::with_capacity(bins);

    for i in 0..bins {
        bin_centers.push(min + bin_width * (i as f64 + 0.5));
    }

    for &value in data {
        let bin = ((value - min) / bin_width).floor() as usize;
        let bin = if bin >= bins { bins - 1 } else { bin };
        counts[bin] += 1;
    }

    (bin_centers, counts)
}
