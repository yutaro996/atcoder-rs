#[derive(Debug)]
pub struct Edge {
    to: usize,
    cost: usize,
}

pub type Edges = Vec<Edge>;
pub type Graph = Vec<Edges>;

pub fn dijkstra(g: &Graph, s: usize) -> Vec<usize> {
    let mut d = vec![usize::MAX; g.len()];
    let mut q = std::collections::BinaryHeap::new();
    d[s] = 0;
    q.push((std::cmp::Reverse(d[s]), s));
    while let Some((std::cmp::Reverse(c), v)) = q.pop() {
        if d[v] < c {
            continue;
        }
        for e in &g[v] {
            if d[e.to] > c + e.cost {
                d[e.to] = c + e.cost;
                q.push((std::cmp::Reverse(d[e.to]), e.to));
            }
        }
    }
    d
}
