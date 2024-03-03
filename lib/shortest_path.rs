#[derive(Debug)]
struct Edge {
    to: usize,
    cost: usize,
}

type Edges = Vec<Edge>;
type Graph = Vec<Edges>;

fn dijkstra(g: &Graph, s: usize) -> Vec<usize> {
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
