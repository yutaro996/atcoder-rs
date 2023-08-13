#![allow(non_snake_case, unused_variables)]
use rand::prelude::*;
use std::f64::consts::PI;

const L_MIN: usize = 10;
const L_MAX: usize = 50;
const N_MIN: usize = 60;
const N_MAX: usize = 100;
const S_SQRT_MIN: usize = 1;
const S_SQRT_MAX: usize = 30;
const P_MIN: usize = 0;
const P_MAX: usize = 1000;

fn f(rng: &mut ThreadRng, sigma: f64) -> f64 {
    let X: f64 = rng.gen_range(0.0..1.0);
    let Y: f64 = rng.gen_range(0.0..1.0);
    let Z: f64 = (-2.0 * X.ln()).sqrt() * (2.0 * PI * Y).cos();
    sigma * Z
}

fn main() {
    let mut rng: ThreadRng = thread_rng();

    // グリッドのサイズを表す整数
    let L: usize = rng.gen_range(L_MIN..=L_MAX);

    // 出口セルの数
    let N: usize = rng.gen_range(N_MIN..=N_MAX);

    // 計測値の計算に用いられる標準偏差
    let S: usize = rng.gen_range(S_SQRT_MIN..=S_SQRT_MAX).pow(2);
    let S = 0;

    // 出口セルの座標情報
    let (Y, X): (Vec<usize>, Vec<usize>) = {
        let mut indices: Vec<usize> = (0..L.pow(2)).collect();
        indices.shuffle(&mut rng);
        let mut YX: Vec<(usize, usize)> = indices
            .iter()
            .take(N)
            .map(|&index| (index / L, index % L))
            .collect();
        YX.sort();
        YX.into_iter().unzip()
    };

    // ワームホールと出口セルの対応関係
    let A: Vec<usize> = {
        let mut A: Vec<usize> = (0..N).collect();
        A.shuffle(&mut rng);
        A
    };
    
    // 配置
    let P: Vec<Vec<usize>> = (0..L)
        .map(|_| (0..L).map(|_| rng.gen_range(P_MIN..=P_MAX)).collect())
        .collect();

    // 計測
    let m: Vec<usize> = (0..N)
        .map(|i| {
            ((P[Y[A[i]]][X[A[i]]] as f64 + f(&mut rng, S as f64)).round() as usize)
                .min(1000)
                .max(0)
        })
        .collect();

    // 回答
    let E: Vec<usize> = m
        .iter()
        .map(|&mi| {
            let (diff, idx) = Y
                .iter()
                .zip(X.iter())
                .enumerate()
                .map(|(i, (&y, &x))| ((mi as i32 - P[y][x] as i32).abs(), i))
                .min_by_key(|&(diff, _)| diff)
                .unwrap();
            idx
        })
        .collect();

    let cnt: f64 = (0..N).filter(|&i| E[i] == A[i]).sum::<usize>() as f64 / N as f64;
    println!("collect: {cnt:.1}%");
}
