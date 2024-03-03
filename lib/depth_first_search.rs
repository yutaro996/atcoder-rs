fn depth_first_search(graph: &[Vec<usize>], v: usize) {
    let mut stack = Vec::new();
    let mut visit = vec![false; graph.len()];
    visit[v] = true;
    stack.push(v);
    while let Some(v) = stack.pop() {
        for &u in &graph[v] {
            if !visit[u] {
                visit[u] = true;
                stack.push(u);
            }
        }
    }
}
