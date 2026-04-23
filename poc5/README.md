I tried this code:
```rust
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
```

using the following command line invocation:

```
 RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z uninit-checks
```

with Kani version:bc27348476839097611ea4af25e2af06be8dce20

I expected to see this happen: 
```
VERIFICATION:- SUCCESSFUL
```

Instead, this happened: 
```
Kani Rust Verifier 0.67.0 (standalone)
warning: method `print` is never used
 --> /home/yuchen/PoC-Kani/poc5/poc.rs:2:8
  |
1 | trait Printable {
  |       --------- method in this trait
2 |     fn print(&self) -> u32;
  |        ^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `data` is never read
 --> /home/yuchen/PoC-Kani/poc5/poc.rs:5:16
  |
5 | struct MyVal { data: u32 }
  |        -----   ^^^^
  |        |
  |        field in this struct


thread 'rustc' (3597821) panicked at kani-compiler/src/kani_middle/transform/check_uninit/mod.rs:440:17:
internal error: entered unreachable code: Cannot change the initialization state of a trait object directly.
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <kani_compiler::kani_middle::transform::check_uninit::UninitInstrumenter>::build_set
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/check_uninit/mod.rs:440:17
   3: <kani_compiler::kani_middle::transform::check_uninit::UninitInstrumenter>::build_check_for_operation
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/check_uninit/mod.rs:183:22
   4: <kani_compiler::kani_middle::transform::check_uninit::UninitInstrumenter>::build_check_for_instruction
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/check_uninit/mod.rs:129:18
   5: <kani_compiler::kani_middle::transform::check_uninit::UninitInstrumenter>::instrument::<kani_compiler::kani_middle::transform::check_uninit::ptr_uninit::uninit_visitor::CheckUninitVisitor>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/check_uninit/mod.rs:113:18
   6: <kani_compiler::kani_middle::transform::check_uninit::UninitInstrumenter>::run::<kani_compiler::kani_middle::transform::check_uninit::ptr_uninit::uninit_visitor::CheckUninitVisitor>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/check_uninit/mod.rs:92:48
   7: <kani_compiler::kani_middle::transform::check_uninit::ptr_uninit::UninitPass as kani_compiler::kani_middle::transform::TransformPass>::transform
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/check_uninit/ptr_uninit/mod.rs:67:45
   8: <kani_compiler::kani_middle::transform::BodyTransformation>::body_ref::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/mod.rs:125:39
   9: <std::collections::hash::map::Entry<rustc_public::mir::mono::Instance, kani_compiler::kani_middle::transform::TransformationResult>>::or_insert_with::<<kani_compiler::kani_middle::transform::BodyTransformation>::body_ref::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:2374:43
  10: <kani_compiler::kani_middle::transform::BodyTransformation>::body_ref
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/mod.rs:119:14
  11: <kani_compiler::kani_middle::reachability::MonoItemsCollector>::visit_fn
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/reachability.rs:173:37
  12: <kani_compiler::kani_middle::reachability::MonoItemsCollector>::reachable_items
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/reachability.rs:154:52
  13: <kani_compiler::kani_middle::reachability::MonoItemsCollector>::collect
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/reachability.rs:144:14
  14: kani_compiler::kani_middle::reachability::collect_reachable_items
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/reachability.rs:58:19
  15: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:98:16
  16: kani_compiler::codegen_cprover_gotoc::compiler_interface::with_timer::<(alloc::vec::Vec<rustc_public::mir::mono::MonoItem>, kani_compiler::kani_middle::reachability::CallGraph), <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#0}>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:900:15
  17: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:97:39
  18: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:408:72
  19: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:60
  20: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:40
  21: <scoped_tls::ScopedKey<core::cell::Cell<*const ()>>>::set::<rustc_public::compiler_interface::run<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_public::error::Error>>
             at /rust/deps/scoped-tls-1.0.1/src/lib.rs:137:9
  22: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:13
  23: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:5
  24: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:330:23
  25: <rustc_interface::queries::Linker>::codegen_and_build_linker
  26: <rustc_interface::passes::create_and_enter_global_ctxt<core::option::Option<rustc_interface::queries::Linker>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2}>::{closure#2} as core::ops::function::FnOnce<(&rustc_session::session::Session, rustc_middle::ty::context::CurrentGcx, alloc::sync::Arc<rustc_data_structures::jobserver::Proxy>, &std::sync::once_lock::OnceLock<rustc_middle::ty::context::GlobalCtxt>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_middle::arena::Arena>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_hir::Arena>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2})>>::call_once::{shim:vtable#0}
  27: rustc_interface::interface::run_compiler::<(), rustc_driver_impl::run_compiler::{closure#0}>::{closure#1}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

Kani unexpectedly panicked during compilation.
Please file an issue here: https://github.com/model-checking/kani/issues/new?labels=bug&template=bug_report.md

[Kani] no current codegen item.
[Kani] no current codegen location.
warning: 2 warnings emitted

error: /home/yuchen/verus-rag/targets/kani/target/kani/bin/kani-compiler exited with status exit status: 101
```