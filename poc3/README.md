I tried this code:
```rust
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

thread 'rustc' (3582037) panicked at kani-compiler/src/kani_middle/points_to/points_to_analysis.rs:441:54:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: core::option::unwrap_failed
   4: <core::option::Option<rustc_public::mir::body::Body>>::unwrap
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1016:21
   5: <kani_compiler::kani_middle::points_to::points_to_analysis::PointsToAnalysis>::apply_regular_call_effect
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/points_to/points_to_analysis.rs:441:54
   6: <kani_compiler::kani_middle::points_to::points_to_analysis::PointsToAnalysis as rustc_mir_dataflow::framework::Analysis>::apply_primary_terminator_effect
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/points_to/points_to_analysis.rs:330:30
   7: <rustc_mir_dataflow::framework::direction::Forward as rustc_mir_dataflow::framework::direction::Direction>::apply_effects_in_block::<kani_compiler::kani_middle::points_to::points_to_analysis::PointsToAnalysis, <kani_compiler::kani_middle::points_to::points_to_analysis::PointsToAnalysis as rustc_mir_dataflow::framework::Analysis>::iterate_to_fixpoint::{closure#1}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_mir_dataflow/src/framework/direction.rs:262:30
   8: <kani_compiler::kani_middle::points_to::points_to_analysis::PointsToAnalysis as rustc_mir_dataflow::framework::Analysis>::iterate_to_fixpoint
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_mir_dataflow/src/framework/mod.rs:288:13
   9: <kani_compiler::kani_middle::points_to::points_to_analysis::PointsToAnalysis>::run
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/points_to/points_to_analysis.rs:99:22
  10: kani_compiler::kani_middle::points_to::points_to_analysis::run_points_to_analysis
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/points_to/points_to_analysis.rs:75:5
  11: <kani_compiler::kani_middle::transform::check_uninit::delayed_ub::DelayedUbPass as kani_compiler::kani_middle::transform::GlobalPass>::transform
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/check_uninit/delayed_ub/mod.rs:108:35
  12: <kani_compiler::kani_middle::transform::GlobalPasses>::run_global_passes
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/kani_middle/transform/mod.rs:248:37
  13: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:116:47
  14: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:408:72
  15: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:60
  16: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:40
  17: <scoped_tls::ScopedKey<core::cell::Cell<*const ()>>>::set::<rustc_public::compiler_interface::run<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_public::error::Error>>
             at /rust/deps/scoped-tls-1.0.1/src/lib.rs:137:9
  18: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:13
  19: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:5
  20: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:330:23
  21: <rustc_interface::queries::Linker>::codegen_and_build_linker
  22: <rustc_interface::passes::create_and_enter_global_ctxt<core::option::Option<rustc_interface::queries::Linker>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2}>::{closure#2} as core::ops::function::FnOnce<(&rustc_session::session::Session, rustc_middle::ty::context::CurrentGcx, alloc::sync::Arc<rustc_data_structures::jobserver::Proxy>, &std::sync::once_lock::OnceLock<rustc_middle::ty::context::GlobalCtxt>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_middle::arena::Arena>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_hir::Arena>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2})>>::call_once::{shim:vtable#0}
  23: rustc_interface::interface::run_compiler::<(), rustc_driver_impl::run_compiler::{closure#0}>::{closure#1}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

Kani unexpectedly panicked during compilation.
Please file an issue here: https://github.com/model-checking/kani/issues/new?labels=bug&template=bug_report.md

[Kani] no current codegen item.
[Kani] no current codegen location.
error: /home/yuchen/verus-rag/targets/kani/target/kani/bin/kani-compiler exited with status exit status: 101
```