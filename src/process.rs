pub fn find_successor(node, key) {
    let pred = find_predecessor(node, id);
    return get_successor(pred);
}

fn find_predecessor(node, id) {
    let mut n = node;
    while !(id > n && id < n.successor())
        n = find_closest_preceding_finger(n);
    return n;
}

fn get_successor(node) {

}

fn find_closest_preceding_finger(id, m) {
    for i in (0..m-1).rev() {
        if i = m
    }
}