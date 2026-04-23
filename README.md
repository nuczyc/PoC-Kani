| PoC | Error Location | Reproduction Command |
| --- | --- | --- |
| poc1 | `kani-compiler/src/kani_middle/transform/check_uninit/delayed_ub/initial_target_visitor.rs:105:75` | `RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z uninit-checks` |
| poc2 | `/home/yuchen/verus-rag/targets/kani/cprover_bindings/src/goto_program/expr.rs:759:9` | `RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z uninit-checks` |
| poc3 | `kani-compiler/src/kani_middle/points_to/points_to_analysis.rs:441:54` | `RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z uninit-checks` |
| poc4 | `kani-compiler/src/kani_middle/transform/check_uninit/ptr_uninit/uninit_visitor.rs:548:25` | `RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z uninit-checks` |
| poc5 | `kani-compiler/src/kani_middle/transform/check_uninit/mod.rs:440:17` | `RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z uninit-checks` |
| poc6 | `kani-compiler/src/codegen_cprover_gotoc/overrides/hooks.rs:959:28` | `RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z quantifiers` |
| poc7 | `kani-compiler/src/codegen_cprover_gotoc/codegen/contract.rs:187:33` | `RUST_BACKTRACE=1 ./scripts/kani poc.rs -Z function-contracts` |
