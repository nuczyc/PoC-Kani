#[kani::proof]
fn check_uninit_transmute_ptr_ref() {
    let x: u32 = 42;
    let ptr: *const u32 = &x;
    let _ref: &u8 = unsafe { std::mem::transmute(ptr) };
}