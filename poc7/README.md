I tried this code:
```rust
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
```

using the following command line invocation:

```
 RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z function-contracts
```

with Kani version:bc27348476839097611ea4af25e2af06be8dce20

I expected to see this happen: 
```
VERIFICATION:- SUCCESSFUL
```

Instead, this happened: 
```
Kani Rust Verifier 0.67.0 (standalone)
warning: method `bump` is never used
 --> /home/yuchen/PoC-Kani/poc7/poc.rs:2:8
  |
1 | trait Demo {
  |       ---- method in this trait
2 |     fn bump(&mut self);
  |        ^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default


thread 'rustc' (3616332) panicked at kani-compiler/src/codegen_cprover_gotoc/codegen/contract.rs:187:33:
internal error: entered unreachable code: Generating a slice fat pointer to RigidTy(Dynamic([Binder { value: Trait(ExistentialTraitRef { def_id: TraitDef(DefId { id: 373, name: "Demo" }), generic_args: GenericArgs([]) }), bound_vars: [] }], Region { kind: ReErased }))
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/contract.rs:187:33
   3: <&mut <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5} as core::ops::function::FnOnce<((usize, rustc_public::ty::Ty),)>>::call_once
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
   4: <core::option::Option<(usize, rustc_public::ty::Ty)>>::map::<cprover_bindings::goto_program::symbol::Lambda, &mut <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:29
   5: <core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}> as core::iter::traits::iterator::Iterator>::next
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
   6: <<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}> as core::iter::traits::iterator::Iterator>::next as core::ops::function::FnOnce<(&mut core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>,)>>::call_once
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   7: core::iter::adapters::chain::and_then_or_clear::<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>, cprover_bindings::goto_program::symbol::Lambda, <core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}> as core::iter::traits::iterator::Iterator>::next>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:332:13
   8: <core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>, alloc::vec::into_iter::IntoIter<cprover_bindings::goto_program::symbol::Lambda>> as core::iter::traits::iterator::Iterator>::next
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:82:9
   9: <alloc::vec::Vec<cprover_bindings::goto_program::symbol::Lambda> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<cprover_bindings::goto_program::symbol::Lambda, core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>, alloc::vec::into_iter::IntoIter<cprover_bindings::goto_program::symbol::Lambda>>>>::from_iter
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:24:41
  10: <alloc::vec::Vec<cprover_bindings::goto_program::symbol::Lambda> as alloc::vec::spec_from_iter::SpecFromIter<cprover_bindings::goto_program::symbol::Lambda, core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>, alloc::vec::into_iter::IntoIter<cprover_bindings::goto_program::symbol::Lambda>>>>::from_iter
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  11: <alloc::vec::Vec<cprover_bindings::goto_program::symbol::Lambda> as core::iter::traits::collect::FromIterator<cprover_bindings::goto_program::symbol::Lambda>>::from_iter::<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>, alloc::vec::into_iter::IntoIter<cprover_bindings::goto_program::symbol::Lambda>>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3719:9
  12: <core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_public::ty::Ty>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#4}>>, <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract::{closure#5}>, alloc::vec::into_iter::IntoIter<cprover_bindings::goto_program::symbol::Lambda>> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<cprover_bindings::goto_program::symbol::Lambda>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2028:9
  13: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_modifies_contract
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/contract.rs:219:14
  14: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::attach_modifies_contract
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/contract.rs:235:34
  15: <kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::handle_check_contract
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/codegen/contract.rs:47:14
  16: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#8}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:189:51
  17: <core::option::Option<rustc_span::def_id::DefId>>::map::<kani_metadata::harness::AssignsContract, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}::{closure#8}>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1165:29
  18: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:189:32
  19: kani_compiler::codegen_cprover_gotoc::compiler_interface::with_timer::<core::option::Option<kani_metadata::harness::AssignsContract>, <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items::{closure#4}>
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:900:15
  20: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend>::codegen_items
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:139:29
  21: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:408:72
  22: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:60
  23: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:40
  24: <scoped_tls::ScopedKey<core::cell::Cell<*const ()>>>::set::<rustc_public::compiler_interface::run<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_public::error::Error>>
             at /rust/deps/scoped-tls-1.0.1/src/lib.rs:137:9
  25: rustc_public::compiler_interface::run::<rustc_public::rustc_internal::run<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/compiler_interface.rs:856:13
  26: rustc_public::rustc_internal::run::<<kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::{closure#0}, alloc::boxed::Box<dyn core::any::Any>>
             at /home/yuchen/.rustup/toolchains/nightly-2025-12-03-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_public/src/rustc_internal/mod.rs:79:5
  27: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at /home/yuchen/verus-rag/targets/kani/kani-compiler/src/codegen_cprover_gotoc/compiler_interface.rs:330:23
  28: <rustc_interface::queries::Linker>::codegen_and_build_linker
  29: <rustc_interface::passes::create_and_enter_global_ctxt<core::option::Option<rustc_interface::queries::Linker>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2}>::{closure#2} as core::ops::function::FnOnce<(&rustc_session::session::Session, rustc_middle::ty::context::CurrentGcx, alloc::sync::Arc<rustc_data_structures::jobserver::Proxy>, &std::sync::once_lock::OnceLock<rustc_middle::ty::context::GlobalCtxt>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_middle::arena::Arena>, &rustc_data_structures::sync::worker_local::WorkerLocal<rustc_hir::Arena>, rustc_driver_impl::run_compiler::{closure#0}::{closure#2})>>::call_once::{shim:vtable#0}
  30: rustc_interface::interface::run_compiler::<(), rustc_driver_impl::run_compiler::{closure#0}>::{closure#1}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

Kani unexpectedly panicked during compilation.
Please file an issue here: https://github.com/model-checking/kani/issues/new?labels=bug&template=bug_report.md

[Kani] no current codegen item.
[Kani] no current codegen location.
warning: 1 warning emitted

error: /home/yuchen/verus-rag/targets/kani/target/kani/bin/kani-compiler exited with status exit status: 101
```