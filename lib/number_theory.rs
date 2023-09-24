pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn sieve(n: usize) -> Vec<bool> {
    let mut res = vec![true; n + 1];
    res[0] = false;
    res[1] = false;
    let mut i = 2;
    while i * i <= n {
        if res[i] {
            for j in (i * i..=n).step_by(i) {
                res[j] = false;
            }
        }
        i += 1
    }
    res
}

pub fn divisors(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i * i != n {
                res.push(n / i);
            }
        }
        i += 1;
    }
    res.sort();
    res
}

pub fn prime_factorization(mut n: usize) -> std::collections::HashMap<usize, usize> {
    let mut res = std::collections::HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            res.entry(i).and_modify(|e| *e += 1).or_insert(1);
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }
    res
}

pub fn mod_pow(mut x: usize, mut n: usize, m: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n & 1 != 0 {
            res = res * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    res
}

pub fn mod_inv(x: usize, m: usize) -> usize {
    mod_pow(x, m - 2, m)
}
