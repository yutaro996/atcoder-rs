#[derive(Debug)]
struct BinaryIndexedTree {
    n: usize,
    data: Vec<i64>,
}

impl From<Vec<i64>> for BinaryIndexedTree {
    fn from(v: Vec<i64>) -> Self {
        let n = v.len();
        let mut data = vec![0; n + 1];
        data[1..][..n].copy_from_slice(&v);
        for i in 1..=n {
            let j = i + (i & i.wrapping_neg());
            if j <= n {
                data[j] += data[i];
            }
        }
        Self { n, data }
    }
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        vec![0; n].into()
    }

    fn add(&mut self, mut i: usize, x: i64) {
        assert!(i < self.n);
        i += 1;
        while i <= self.n {
            self.data[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, range: impl std::ops::RangeBounds<usize>) -> i64 {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.n,
        };
        assert!(l <= r && r <= self.n);
        self.prefix_sum(r) - self.prefix_sum(l)
    }

    fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.data[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}
