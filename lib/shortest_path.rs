pub fn dijkstra(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX; graph.len()];
    let mut que = BinaryHeap::new();
    dist[s] = 0;
    que.push((Reverse(dist[s]), s));
    while let Some((Reverse(cost), v)) = que.pop() {
        if cost > dist[v] {
            continue;
        }
        for e in &graph[v] {
            if dist[e.0] < cost + e.1 {
                dist[e.0] = cost + e.1;
                que.push((Reverse(dist[e.0]), e.0));
            }
        }
    }
    dist
}
