// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use num_bigint::BigInt;

use liblumen_alloc::badarith;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::{Term, TypedTerm};

use lumen_runtime_macros::native_implemented_function;

/// `bnot/1` prefix operator.
#[native_implemented_function(bnot/1)]
pub fn native(process: &Process, integer: Term) -> exception::Result {
    match integer.to_typed_term().unwrap() {
        TypedTerm::SmallInteger(small_integer) => {
            let integer_isize: isize = small_integer.into();
            let output = !integer_isize;
            let output_term = process.integer(output)?;

            Ok(output_term)
        }
        TypedTerm::Boxed(boxed) => match boxed.to_typed_term().unwrap() {
            TypedTerm::BigInteger(big_integer) => {
                let big_int: &BigInt = big_integer.as_ref().into();
                let output_big_int = !big_int;
                let output_term = process.integer(output_big_int)?;

                Ok(output_term)
            }
            _ => Err(badarith!().into()),
        },
        _ => Err(badarith!().into()),
    }
}
