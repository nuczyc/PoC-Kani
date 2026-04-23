trait Demo {
    fn bump(&mut self);
}

struct Counter;

impl Demo for Counter {
    fn bump(&mut self) {}
}

#[kani::modifies(ptr)]
fn touch_dyn(ptr: *mut dyn Demo) {
    let _ = ptr;
}

#[kani::proof_for_contract(touch_dyn)]
fn check_touch_dyn() {
    let mut value = Counter;
    let ptr: *mut dyn Demo = &mut value;
    touch_dyn(ptr);
}