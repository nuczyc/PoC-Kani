fn pred(i: usize) -> bool {
    i < 4
}

#[kani::proof]
fn check_quantifier_fn_item() {
    let _ = kani::internal::kani_forall(0usize, 8usize, pred as fn(usize) -> bool);
}