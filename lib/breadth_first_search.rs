fn breadth_first_search(g: &[Vec<usize>], v: usize) {
    let mut queue = VecDeque::new();
    let mut visit = vec![false; g.len()];
    visit[v] = true;
    queue.push_back(v);
    while let Some(v) = queue.pop_front() {
        for &u in &g[v] {
            if !visit[u] {
                visit[u] = true;
                queue.push_back(u);
            }
        }
    }
}
