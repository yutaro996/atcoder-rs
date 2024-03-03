fn depth_first_search(g: &[Vec<usize>], v: usize) {
    let mut stack = Vec::new();
    let mut visit = vec![false; g.len()];
    visit[v] = true;
    stack.push(v);
    while let Some(v) = stack.pop() {
        for &u in &g[v] {
            if !visit[u] {
                visit[u] = true;
                stack.push(u);
            }
        }
    }
}
