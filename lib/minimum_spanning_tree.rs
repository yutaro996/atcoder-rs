pub fn kruskal(edges: &mut [(usize, usize, i64)], v: usize) -> Vec<(usize, usize, i64)> {
    edges.sort_by(|a, b| a.2.cmp(&b.2));
    let mut uf = UnionFindTree::new(v);
    let mut es = vec![];
    for &e in edges.iter() {
        if uf.union(e.0, e.1) {
            es.push(e);
        }
    }
    es
}
