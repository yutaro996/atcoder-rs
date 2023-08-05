pub trait Monoid {
    type S: Copy;
    fn e() -> Self::S;
    fn op(a: Self::S, b: Self::S) -> Self::S;
}

pub struct DualSegmentTree<M: Monoid> {
    n: usize,
    size: usize,
    log: usize,
    lazy: Vec<M::S>,
}

impl<M: Monoid> DualSegmentTree<M> {
    pub fn new(n: usize) -> Self {
        vec![M::e(); n].into()
    }

    pub fn get(&mut self, mut i: usize) -> M::S {
        assert!(i < self.n);
        i += self.size;
        for j in (1..=self.log).rev() {
            self.push(i >> j);
        }
        self.lazy[i]
    }

    pub fn apply(&mut self, range: impl ops::RangeBounds<usize>, f: M::S) {
        let mut l = match range.start_bound() {
            ops::Bound::Included(&l) => l,
            ops::Bound::Excluded(&l) => l + 1,
            ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            ops::Bound::Included(&r) => r + 1,
            ops::Bound::Excluded(&r) => r,
            ops::Bound::Unbounded => self.n,
        };
        assert!(l <= r && r <= self.n);
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 != 0 {
                self.lazy[l] = M::op(self.lazy[l], f);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.lazy[r] = M::op(self.lazy[r], f);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    fn push(&mut self, i: usize) {
        self.lazy[i << 1] = M::op(self.lazy[i << 1], self.lazy[i]);
        self.lazy[i << 1 | 1] = M::op(self.lazy[i << 1 | 1], self.lazy[i]);
        self.lazy[i] = M::e();
    }
}

impl<M: Monoid> From<Vec<M::S>> for DualSegmentTree<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let log = (size as f64).log2() as usize;
        let mut lazy = vec![M::e(); size << 1];
        lazy[size..][..n].clone_from_slice(&v);
        let mut ret = Self { n, size, log, lazy };
        for i in (1..size).rev() {
            ret.push(i);
        }
        ret
    }
}
