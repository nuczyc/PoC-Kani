I tried this code:
```rust
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
warning: field `tail` is never read
 --> /home/yuchen/PoC-Kani/poc2/poc.rs:9:5
  |
7 | struct ForeignWrapper {
  |        -------------- field in this struct
8 |     data: u32,
9 |     tail: Opaque,
  |     ^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default


thread 'rustc' (3578079) panicked at /home/yuchen/verus-rag/targets/kani/cprover_bindings/src/goto_program/expr.rs:759:9:
Can't apply .member operation to
        Expr { value: Symbol { identifier: "_RNvCs9CWOZ0PQzZG_3poc20check_uninit_foreign::1::var_1::ptr" }, typ: Pointer { typ: StructTag("tag-_223764733396407873491208198432281969775") }, location: None, size_of_annotation: None }
        data
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <cprover_bindings::goto_program::expr::Expr>::member::<&str>
             at /home/yuchen/verus-rag/targets/kani/cprover_bindings/src/goto_program/expr.rs:759:9
   3: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_projection
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/place.rs:480:30
   4: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_place_stable::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/place.rs:726:58
   5: <core::slice::iter::Iter<rustc_public::mir::body::ProjectionElem> as core::iter::traits::iterator::Iterator>::fold::<core::result::Result<kani_compiler::codegen_cprover_gotoc::codegen::place::ProjectedPlace, alloc::boxed::Box<kani_compiler::codegen_cprover_gotoc::codegen::place::UnimplementedData>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_place_stable::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:279:27
   6: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_place_stable
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/place.rs:726:14
   7: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_place_ref_stable
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/place.rs:657:64
   8: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_rvalue_stable
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/rvalue.rs:757:38
   9: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_statement
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/statement.rs:129:34
  10: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_block
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/block.rs:33:34
  11: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/function.rs:81:57
  12: core::iter::traits::iterator::Iterator::for_each::call::<usize, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:825:29
  13: <alloc::vec::into_iter::IntoIter<usize> as core::iter::traits::double_ended::DoubleEndedIterator>::rfold::<(), core::iter::traits::iterator::Iterator::for_each::call<usize, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/double_ended.rs:308:21
  14: <core::iter::adapters::rev::Rev<alloc::vec::into_iter::IntoIter<usize>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<usize, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/rev.rs:83:19
  15: <core::iter::adapters::rev::Rev<alloc::vec::into_iter::IntoIter<usize>> as core::iter::traits::iterator::Iterator>::for_each::<<kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:828:14
  16: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/function.rs:81:38
  17: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:167:43
  18: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info::<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/utils/debug.rs:74:13
  19: <std::thread::local::LocalKey<core::cell::RefCell<(core::option::Option<alloc::boxed::Box<dyn core::ops::function::Fn<(), Output = alloc::string::String>>>, core::option::Option<cprover_bindings::goto_program::location::Location>)>>>::try_with::<<kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>::{closure#0}, ()>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:513:12
  20: <std::thread::local::LocalKey<core::cell::RefCell<(core::option::Option<alloc::boxed::Box<dyn core::ops::function::Fn<(), Output = alloc::string::String>>>, core::option::Option<cprover_bindings::goto_program::location::Location>)>>>::with::<<kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>::{closure#0}, ()>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:477:20
  21: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info::<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/utils/debug.rs:69:30
  22: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:166:33
  23: kani_compiler::codegen_cprover_gotoc::compiler_interface::with_timer::<core::option::Option<kani_metadata::harness::AssignsContract>, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:900:15
  24: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:139:29
  25: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:408:72
  26: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:60
  27: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:40
  28: <scoped_tls::ScopedKey<core::cell::Cell<*const ()>>>::set::<rustc_public::compiler_interface::run<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_public::error::Error>>
             at /rust/deps/scoped-tls-1.0.1/src/lib.rs:137:9
  29: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:13
  30: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:5
  31: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:330:23
  32: <rustc_interface::queries::Linker>::codegen_and_build_linker
  33: <rustc_interface::passes::create_and_enter_global_ctxt<core::option::Option<rustc_interface::queries::Linker>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2}>::{closure#2} as core::ops::function::FnOnce<(&rustc_session::session::Session, rustc_middle::ty::context::CurrentGcx, alloc::sync::Arc<rustc_data_structures::jobserver::Proxy>, &std::sync::once_lock::OnceLock<rustc_middle::ty::context::GlobalCtxt>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_middle::arena::Arena>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_hir::Arena>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2})>>::call_once::{shim:vtable#0}
  34: rustc_interface::interface::run_compiler::<(), rustc_driver_impl::run_compiler::{closure#0}>::{closure#1}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

Kani unexpectedly panicked during compilation.
Please file an issue here: https://github.com/model-checking/kani/issues/new?labels=bug&template=bug_report.md

[Kani] current codegen item: codegen_function: check_uninit_foreign
_RNvCs9CWOZ0PQzZG_3poc20check_uninit_foreign
[Kani] current codegen location: Loc { file: "/home/yuchen/PoC-Kani/poc2/poc.rs", function: None, start_line: 13, start_col: Some(1), end_line: 13, end_col: Some(26), pragmas: [] }
warning: 1 warning emitted

error: /home/yuchen/verus-rag/targets/kani/target/kani/bin/kani-compiler exited with status exit status: 101
```
