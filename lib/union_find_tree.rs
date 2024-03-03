#[derive(Debug)]
struct UnionFindTree {
    n: usize,
    data: Vec<i32>,
}

impl UnionFindTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![-1; n],
        }
    }

    fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        assert!(x < self.n && y < self.n);
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return false;
        }
        if self.data[x] > self.data[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.data[x] += self.data[y];
        self.data[y] = x as i32;
        true
    }

    fn find(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.data[x] < 0 {
            return x;
        }
        self.data[x] = self.find(self.data[x] as usize) as i32;
        self.data[x] as usize
    }
}
