#[derive(Debug)]
pub struct Edge {
    to: usize,
    cost: usize,
}

pub type Edges = Vec<Edge>;
pub type Graph = Vec<Edges>;

pub fn dijkstra(g: &Graph, s: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; g.len()];
    let mut q = std::collections::BinaryHeap::new();
    dist[s] = 0;
    q.push((std::cmp::Reverse(dist[s]), s));
    while let Some((std::cmp::Reverse(cost), v)) = q.pop() {
        if dist[v] < cost {
            continue;
        }
        for e in &g[v] {
            if dist[e.to] > cost + e.cost {
                dist[e.to] = cost + e.cost;
                q.push((std::cmp::Reverse(dist[e.to]), e.to));
            }
        }
    }
    dist
}
