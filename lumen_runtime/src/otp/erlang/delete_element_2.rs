// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::convert::TryInto;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::{Boxed, Term, Tuple};

use lumen_runtime_macros::native_implemented_function;

use crate::tuple::ZeroBasedIndex;

/// `delete_element/2`
#[native_implemented_function(delete_element/2)]
pub fn native(process: &Process, index: Term, tuple: Term) -> exception::Result {
    let initial_inner_tuple: Boxed<Tuple> = tuple.try_into()?;
    let ZeroBasedIndex(index_zero_based): ZeroBasedIndex = index.try_into()?;
    let initial_len = initial_inner_tuple.len();

    if index_zero_based < initial_len {
        let smaller_len = initial_len - 1;
        let smaller_element_iterator =
            initial_inner_tuple
                .iter()
                .enumerate()
                .filter_map(|(old_index, old_term)| {
                    if old_index == index_zero_based {
                        None
                    } else {
                        Some(old_term)
                    }
                });
        let smaller_tuple = process.tuple_from_iter(smaller_element_iterator, smaller_len)?;

        Ok(smaller_tuple)
    } else {
        Err(badarg!().into())
    }
}
