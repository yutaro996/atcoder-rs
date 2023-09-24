pub trait Monoid {
    type S: std::fmt::Debug + Copy;
    fn e() -> Self::S;
    fn op(a: Self::S, b: Self::S) -> Self::S;
}

#[derive(Debug)]
pub struct DualSegmentTree<M: Monoid> {
    n: usize,
    size: usize,
    log: usize,
    lazy: Vec<M::S>,
}

impl<M: Monoid> From<Vec<M::S>> for DualSegmentTree<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let log = size.ilog2() as usize;
        let mut lazy = vec![M::e(); 2 * size];
        lazy[size..][..n].clone_from_slice(&v);
        let mut ret = Self { n, size, log, lazy };
        for i in (1..size).rev() {
            ret.push(i);
        }
        ret
    }
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

    pub fn apply(&mut self, range: impl std::ops::RangeBounds<usize>, f: M::S) {
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.n,
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
        self.lazy[2 * i] = M::op(self.lazy[2 * i], self.lazy[i]);
        self.lazy[2 * i + 1] = M::op(self.lazy[2 * i + 1], self.lazy[i]);
        self.lazy[i] = M::e();
    }
}
