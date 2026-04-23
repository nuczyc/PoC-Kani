I tried this code:
```rust
fn pred(i: usize) -> bool {
    i < 4
}

#[kani::proof]
fn check_quantifier_fn_item() {
    let _ = kani::internal::kani_forall(0usize, 8usize, pred as fn(usize) -> bool);
}
```

using the following command line invocation:

```
 RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z quantifiers
```

with Kani version:bc27348476839097611ea4af25e2af06be8dce20

I expected to see this happen: 
```
VERIFICATION:- SUCCESSFUL
```

Instead, this happened: 
```
Kani Rust Verifier 0.67.0 (standalone)

thread 'rustc' (3604476) panicked at kani-compiler/src/codegen_cprover_gotoc/overrides/hooks.rs:959:28:
internal error: entered unreachable code: Failed to find closure call expression
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: kani_compiler::codegen_cprover_gotoc::overrides::hooks::handle_quantifier::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/overrides/hooks.rs:959:28
   3: <core::option::Option<cprover_bindings::goto_program::expr::Expr>>::unwrap_or_else::<kani_compiler::codegen_cprover_gotoc::overrides::hooks::handle_quantifier::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1067:21
   4: kani_compiler::codegen_cprover_gotoc::overrides::hooks::handle_quantifier
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/overrides/hooks.rs:959:10
   5: <kani_compiler::codegen_cprover_gotoc::overrides::hooks::Forall as kani_compiler::codegen_cprover_gotoc::overrides::hooks::GotocHook>::handle
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/overrides/hooks.rs:917:9
   6: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_funcall
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/statement.rs:727:31
   7: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_terminator
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/statement.rs:311:22
   8: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_block
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/block.rs:41:34
   9: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/function.rs:81:57
  10: core::iter::traits::iterator::Iterator::for_each::call::<usize, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:825:29
  11: <alloc::vec::into_iter::IntoIter<usize> as core::iter::traits::double_ended::DoubleEndedIterator>::rfold::<(), core::iter::traits::iterator::Iterator::for_each::call<usize, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/double_ended.rs:308:21
  12: <core::iter::adapters::rev::Rev<alloc::vec::into_iter::IntoIter<usize>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<usize, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/rev.rs:83:19
  13: <core::iter::adapters::rev::Rev<alloc::vec::into_iter::IntoIter<usize>> as core::iter::traits::iterator::Iterator>::for_each::<<kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function::{closure#0}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:828:14
  14: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/function.rs:81:38
  15: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:167:43
  16: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info::<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/utils/debug.rs:74:13
  17: <std::thread::local::LocalKey<core::cell::RefCell<(core::option::Option<alloc::boxed::Box<dyn core::ops::function::Fn<(), Output = alloc::string::String>>>, core::option::Option<cprover_bindings::goto_program::location::Location>)>>>::try_with::<<kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>::{closure#0}, ()>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:513:12
  18: <std::thread::local::LocalKey<core::cell::RefCell<(core::option::Option<alloc::boxed::Box<dyn core::ops::function::Fn<(), Output = alloc::string::String>>>, core::option::Option<cprover_bindings::goto_program::location::Location>)>>>::with::<<kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>::{closure#0}, ()>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:477:20
  19: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::call_with_panic_debug_info::<rustc_public::mir::mono::InstanceDef, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#4}, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#5}>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/utils/debug.rs:69:30
  20: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:166:33
  21: kani_compiler::codegen_cprover_gotoc::compiler_interface::with_timer::<core::option::Option<kani_metadata::harness::AssignsContract>, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:900:15
  22: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:139:29
  23: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:408:72
  24: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:60
  25: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:40
  26: <scoped_tls::ScopedKey<core::cell::Cell<*const ()>>>::set::<rustc_public::compiler_interface::run<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_public::error::Error>>
             at /rust/deps/scoped-tls-1.0.1/src/lib.rs:137:9
  27: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:13
  28: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:5
  29: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:330:23
  30: <rustc_interface::queries::Linker>::codegen_and_build_linker
  31: <rustc_interface::passes::create_and_enter_global_ctxt<core::option::Option<rustc_interface::queries::Linker>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2}>::{closure#2} as core::ops::function::FnOnce<(&rustc_session::session::Session, rustc_middle::ty::context::CurrentGcx, alloc::sync::Arc<rustc_data_structures::jobserver::Proxy>, &std::sync::once_lock::OnceLock<rustc_middle::ty::context::GlobalCtxt>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_middle::arena::Arena>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_hir::Arena>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2})>>::call_once::{shim:vtable#0}
  32: rustc_interface::interface::run_compiler::<(), rustc_driver_impl::run_compiler::{closure#0}>::{closure#1}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

Kani unexpectedly panicked during compilation.
Please file an issue here: https://github.com/model-checking/kani/issues/new?labels=bug&template=bug_report.md

[Kani] current codegen item: codegen_function: check_quantifier_fn_item
_RNvCs9CWOZ0PQzZG_3poc24check_quantifier_fn_item
[Kani] current codegen location: Loc { file: "/home/yuchen/PoC-Kani/poc6/poc.rs", function: None, start_line: 6, start_col: Some(1), end_line: 6, end_col: Some(30), pragmas: [] }
error: /home/yuchen/verus-rag/targets/kani/target/kani/bin/kani-compiler exited with status exit status: 101
```