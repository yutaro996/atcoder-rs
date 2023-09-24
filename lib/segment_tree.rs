pub trait Monoid {
    type S: std::fmt::Debug + Copy;
    fn e() -> Self::S;
    fn op(a: Self::S, b: Self::S) -> Self::S;
}

#[derive(Debug)]
pub struct SegmentTree<M: Monoid> {
    n: usize,
    size: usize,
    log: usize,
    data: Vec<M::S>,
}

impl<M: Monoid> From<Vec<M::S>> for SegmentTree<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let log = size.ilog2() as usize;
        let mut data = vec![M::e(); 2 * size];
        data[size..][..n].clone_from_slice(&v);
        let mut ret = Self { n, size, log, data };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> Self {
        vec![M::e(); n].into()
    }

    pub fn set(&mut self, mut i: usize, x: M::S) {
        assert!(i < self.n);
        i += self.size;
        self.data[i] = x;
        for j in 1..=self.log {
            self.update(i >> j)
        }
    }

    pub fn prod(&self, range: impl std::ops::RangeBounds<usize>) -> M::S {
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
        let mut sml = M::e();
        let mut smr = M::e();
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 != 0 {
                sml = M::op(sml, self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::op(self.data[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(sml, smr)
    }

    fn update(&mut self, i: usize) {
        self.data[i] = M::op(self.data[2 * i], self.data[2 * i + 1]);
    }
}
