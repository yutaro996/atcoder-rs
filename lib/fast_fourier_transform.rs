use num_complex::Complex;

#[derive(Debug)]
struct FastFourierTransform {
    base: u32,
    rts: Vec<Complex<f64>>,
    rev: Vec<usize>,
}

impl Default for FastFourierTransform {
    fn default() -> Self {
        Self::new()
    }
}

impl FastFourierTransform {
    fn new() -> Self {
        Self {
            base: 1,
            rts: vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            rev: vec![0, 1],
        }
    }

    fn fft(&mut self, a: &mut [Complex<f64>], n: usize) {
        assert!(n.is_power_of_two());
        let zeros = n.trailing_zeros();
        self.ensure_base(zeros);
        let shift = self.base - zeros;
        for i in 0..n {
            if i < (self.rev[i] >> shift) {
                a.swap(i, self.rev[i] >> shift);
            }
        }
        let mut k = 1;
        while k < n {
            for i in (0..n).step_by(2 * k) {
                for j in 0..k {
                    let z = a[i + j + k] * self.rts[j + k];
                    a[i + j + k] = a[i + j] - z;
                    a[i + j] += z;
                }
            }
            k <<= 1;
        }
    }

    fn multiply(&mut self, a: &[i32], b: &[i32]) -> Vec<i64> {
        let need = a.len() + b.len() - 1;
        let mut nbase = 1;
        while (1 << nbase) < need {
            nbase += 1;
        }
        self.ensure_base(nbase);
        let sz = 1 << nbase;
        let mut fa = vec![Complex::new(0.0, 0.0); sz];
        for i in 0..sz {
            let x = if i < a.len() { a[i] } else { 0 };
            let y = if i < b.len() { b[i] } else { 0 };
            fa[i] = Complex::new(x as f64, y as f64);
        }
        self.fft(&mut fa, sz);
        let r = Complex::new(0.0, -0.25 / (sz >> 1) as f64);
        let s = Complex::new(0.0, 1.0);
        let t = Complex::new(0.5, 0.0);
        for i in 0..=(sz >> 1) {
            let j = (sz - i) & (sz - 1);
            let z = (fa[j] * fa[j] - (fa[i] * fa[i]).conj()) * r;
            fa[j] = (fa[i] * fa[i] - (fa[j] * fa[j]).conj()) * r;
            fa[i] = z;
        }
        for i in 0..(sz >> 1) {
            let a0 = (fa[i] + fa[i + (sz >> 1)]) * t;
            let a1 = (fa[i] - fa[i + (sz >> 1)]) * t * self.rts[(sz >> 1) + i];
            fa[i] = a0 + a1 * s;
        }
        self.fft(&mut fa, sz >> 1);
        let mut ret = vec![0; need];
        for i in 0..need {
            ret[i] = if i & 1 > 0 {
                fa[i >> 1].im.round() as i64
            } else {
                fa[i >> 1].re.round() as i64
            }
        }
        ret
    }

    fn ensure_base(&mut self, nbase: u32) {
        if nbase <= self.base {
            return;
        }
        self.rev.resize(1 << nbase, 0);
        self.rts.resize(1 << nbase, Complex::new(0.0, 0.0));
        for i in 0..(1 << nbase) {
            self.rev[i] = (self.rev[i >> 1] >> 1) + ((i & 1) << (nbase - 1));
        }
        while self.base < nbase {
            let angle = std::f64::consts::PI * 2.0 / (1 << (self.base + 1)) as f64;
            for i in (1 << (self.base - 1))..(1 << self.base) {
                self.rts[i << 1] = self.rts[i];
                let angle_i = angle * (2 * i + 1 - (1 << self.base)) as f64;
                self.rts[(i << 1) + 1] = Complex::from_polar(1.0, angle_i);
            }
            self.base += 1;
        }
    }
}
