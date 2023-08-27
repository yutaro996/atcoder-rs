pub trait Monoid {
    type S: Copy;
    fn e() -> Self::S;
    fn op(a: Self::S, b: Self::S) -> Self::S;
}

pub trait ActedMonoid {
    type X: Monoid;
    type F: Monoid;
    fn act(f: <Self::F as Monoid>::S, x: <Self::X as Monoid>::S) -> <Self::X as Monoid>::S;
}

pub struct LazySegmentTree<AM: ActedMonoid> {
    n: usize,
    size: usize,
    log: usize,
    data: Vec<<AM::X as Monoid>::S>,
    lazy: Vec<<AM::F as Monoid>::S>,
}

impl<AM: ActedMonoid> LazySegmentTree<AM> {
    pub fn new(n: usize) -> Self {
        vec![AM::X::e(); n].into()
    }

    pub fn set(&mut self, mut i: usize, x: <AM::X as Monoid>::S) {
        assert!(i < self.n);
        i += self.size;
        for j in (1..=self.log).rev() {
            self.push(i >> j);
        }
        self.data[i] = x;
        for j in 1..=self.log {
            self.update(i >> j);
        }
    }

    pub fn get(&mut self, mut i: usize) -> <AM::X as Monoid>::S {
        assert!(i < self.n);
        i += self.size;
        for j in (1..=self.log).rev() {
            self.push(i >> j);
        }
        self.data[i]
    }

    pub fn prod(&mut self, range: impl ops::RangeBounds<usize>) -> <AM::X as Monoid>::S {
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
        if l == r {
            return AM::X::e();
        }
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let mut sml = AM::X::e();
        let mut smr = AM::X::e();
        while l < r {
            if l & 1 != 0 {
                sml = AM::X::op(sml, self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = AM::X::op(self.data[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        AM::X::op(sml, smr)
    }

    pub fn apply(&mut self, range: impl ops::RangeBounds<usize>, f: <AM::F as Monoid>::S) {
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
        if l == r {
            return;
        }
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let l2 = l;
        let r2 = r;
        while l < r {
            if l & 1 != 0 {
                self.all_apply(l, f);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }
        l = l2;
        r = r2;
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    fn update(&mut self, i: usize) {
        self.data[i] = AM::X::op(self.data[i << 1], self.data[i << 1 | 1]);
    }

    fn all_apply(&mut self, i: usize, f: <AM::F as Monoid>::S) {
        self.data[i] = AM::act(f, self.data[i]);
        if i < self.size {
            self.lazy[i] = AM::F::op(f, self.lazy[i]);
        }
    }

    fn push(&mut self, i: usize) {
        self.all_apply(i << 1, self.lazy[i]);
        self.all_apply(i << 1 | 1, self.lazy[i]);
        self.lazy[i] = AM::F::e();
    }
}

impl<AM: ActedMonoid> From<Vec<<AM::X as Monoid>::S>> for LazySegmentTree<AM> {
    fn from(v: Vec<<AM::X as Monoid>::S>) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let mut data = vec![AM::X::e(); size << 1];
        data[size..][..n].clone_from_slice(&v);
        let mut ret = Self {
            n,
            size,
            log: (size as f64).log2() as usize,
            data,
            lazy: vec![AM::F::e(); size],
        };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
