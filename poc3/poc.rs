trait Calc {
    fn calc(&self, x: u32) -> u32;
}

struct Adder;

impl Calc for Adder {
    fn calc(&self, x: u32) -> u32 { x + 1 }
}

#[kani::proof]
fn check_uninit_fn_call_dyn() {
    let adder = Adder;
    let dyn_ref: &dyn Calc = &adder;
    let result = dyn_ref.calc(kani::any());
    assert!(result < 200 || result >= 200);
}