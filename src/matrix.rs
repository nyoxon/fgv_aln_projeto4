use nalgebra::{DMatrix, DVector};
use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use rand::Rng;

pub fn gaussian_matrix(m: usize, n: usize) -> DMatrix<f64> {
    // normal padrão
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = thread_rng();

    // vetor com m * n entradas
    // aleatorias vindas da
    // normal padrao
    let data: Vec<f64> = (0..(m * n))
        .map(|_| normal.sample(&mut rng))
        .collect();

    DMatrix::from_row_slice(
        m, n, &data)    
}

pub fn column_norms(matrix: &DMatrix<f64>) -> Vec<f64> {
    (0..matrix.ncols())
        .map(|j| matrix.column(j).norm())
        .collect()
}

pub fn inner_products(matrix: &DMatrix<f64>, num_samples: usize) -> Vec<f64> {
    let m = matrix.ncols();
    let mut rng = thread_rng();
    let mut results = Vec::with_capacity(num_samples);

    for _ in 0..num_samples {
        let i = rng.gen_range(0..m);
        let mut j = rng.gen_range(0..m);
        
        while j == i {
            j = rng.gen_range(0..m);
        }

        let dot = matrix.column(i).dot(&matrix.column(j));
        results.push(dot);
    }

    results
}

pub fn max_product_normalized(matrix: &DMatrix<f64>) -> f64 {
    let m = matrix.ncols();
    let mut max_val = 0.0;

    let columns: Vec<DVector<f64>> = (0..m)
        .map(|j| {
            let col = matrix.column(j).into_owned();
            col.clone() / col.norm()
        })
        .collect();

    for i in 0..m {
        for j in (i+1)..m {
            let dot = columns[i].dot(&columns[j]).abs();

            if dot > max_val {
                max_val = dot;
            }
        }
    }

    max_val
}

