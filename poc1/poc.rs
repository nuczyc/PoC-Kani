#[kani::proof]
fn check_uninit_slice_subslice() {
    let arr: [u32; 4] = [kani::any(), kani::any(), kani::any(), kani::any()];
    let slice: &[u32] = &arr;
    let sub = &slice[1..3];
    assert!(sub[0] < 200 || sub[0] >= 200);
}