use num_complex::Complex;
use std::f64::consts::TAU;

fn butterfly(n: usize, q: usize, x: &mut [Complex<f64>]) {
    if n == 1 {
        return;
    }
    let m = n / 2;
    let theta0 = TAU / n as f64;
    for p in 0..m {
        let wp = Complex::from_polar(1.0, -theta0 * p as f64);
        let a = x[q + p];
        let b = x[q + p + m];
        x[q + p] = a + b;
        x[q + p + m] = (a - b) * wp;
    }
    butterfly(m, q, x);
    butterfly(m, q + m, x);
}

fn bit_reverse(n: usize, x: &mut [Complex<f64>]) {
    let mut i = 0;
    for j in 1..n - 1 {
        let mut k = n >> 1;
        while k > (i ^ k) {
            i ^= k;
            k >>= 1;
        }
        i ^= k;
        if i < j {
            x.swap(i, j);
        }
    }
}

fn fft(n: usize, x: &mut [Complex<f64>]) {
    butterfly(n, 0, x);
    bit_reverse(n, x);
}

fn ifft(n: usize, x: &mut [Complex<f64>]) {
    x.iter_mut().for_each(|x| *x = x.conj());
    fft(n, x);
    x.iter_mut().for_each(|x| *x = x.conj() / n as f64);
}
