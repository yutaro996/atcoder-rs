pub fn lower_bound<T: Ord>(arr: &[T], key: T) -> usize {
    let mut ng = -1;
    let mut ok = arr.len() as i32;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if arr[mid as usize] >= key {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok as usize
}
