pub fn beam_search_action(state: &State, beam_width: i32, beam_depth: i32) -> i32 {
    let mut now_beam = BinaryHeap::new();
    now_beam.push(state.clone());
    let mut best_state = now_beam.peek().unwrap();
    for t in 0..beam_depth {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..beam_width {
            if now_beam.is_empty() {
                break;
            }
            let now_state = now_beam.pop().unwrap();
            let legal_actions = now_state.legal_actions();
            for action in legal_actions {
                let mut next_state = now_state.clone();
                next_state.advance(action);
                next_state.evaluate_score();
                if t == 0 {
                    next_state.first_action = action;
                }
                next_beam.push(next_state);
            }
        }
        now_beam = next_beam;
        best_state = now_beam.peek().unwrap();
        if best_state.is_done() {
            break;
        }
    }
    best_state.first_action
}
