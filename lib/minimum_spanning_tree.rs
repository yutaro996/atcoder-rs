#[derive(Debug, Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

pub type Edges = Vec<Edge>;

pub fn kruskal(edges: &mut Edges, v: usize) -> Edges {
    edges.sort_by(|a, b| a.cost.cmp(&b.cost));
    let mut uf = UnionFindTree::new(v);
    let mut es = vec![];
    for e in edges.iter() {
        if uf.union(e.from, e.to) {
            es.push(e.clone());
        }
    }
    es
}
