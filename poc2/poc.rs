#![feature(extern_types)]

extern "C" {
    type Opaque;
}

struct ForeignWrapper {
    data: u32,
    tail: Opaque,
}

#[kani::proof]
fn check_uninit_foreign() {
    let ptr: *const ForeignWrapper = kani::any::<usize>() as *const ForeignWrapper;
    let r: &ForeignWrapper = unsafe { &*ptr };
    let _ = r.data;
}