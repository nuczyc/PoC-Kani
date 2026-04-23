trait Printable {
    fn print(&self) -> u32;
}

struct MyVal { data: u32 }

impl Printable for MyVal {
    fn print(&self) -> u32 { self.data }
}

#[kani::proof]
fn check_uninit_box_dyn() {
    let b: Box<dyn Printable> = Box::new(MyVal { data: kani::any() });
    let ptr: *const dyn Printable = &*b as *const dyn Printable;
    let _ = unsafe { &*ptr };
}