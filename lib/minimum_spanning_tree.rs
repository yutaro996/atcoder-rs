#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

type Edges = Vec<Edge>;

fn kruskal(edges: &mut Edges, v: usize) -> Edges {
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
