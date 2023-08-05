pub struct UnionFindTree {
    n: usize,
    data: Vec<i32>,
}

impl UnionFindTree {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![-1; n],
        }
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        assert!(x < self.n && y < self.n);
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return false;
        }
        if self.data[x] > self.data[y] {
            mem::swap(&mut x, &mut y);
        }
        self.data[x] += self.data[y];
        self.data[y] = x as i32;
        true
    }

    pub fn find(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.data[x] < 0 {
            return x;
        }
        self.data[x] = self.find(self.data[x] as usize) as i32;
        self.data[x] as usize
    }
}
