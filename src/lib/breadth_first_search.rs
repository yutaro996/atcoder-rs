pub fn breadth_first_search(graph: &[Vec<usize>], v: usize) {
    let mut queue = VecDeque::new();
    let mut visit = vec![false; graph.len()];
    visit[v] = true;
    queue.push_back(v);
    while let Some(v) = queue.pop_front() {
        for &u in &graph[v] {
            if !visit[u] {
                visit[u] = true;
                queue.push_back(u);
            }
        }
    }
}
