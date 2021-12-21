pub fn matrix<T: Clone>(val: T, rows: usize, cols: usize) -> Vec<Vec<T>> {
    let mut ret = Vec::with_capacity(rows);
    for _ in 0..ret.capacity() {
        ret.push(vec![val.clone(); cols]);
    }

    // Sanity check
    assert_eq!(ret.len(), ret.len());
    if rows > 0 {
        assert_eq!(ret[0].len(), ret[0].len());
    }

    ret
}
