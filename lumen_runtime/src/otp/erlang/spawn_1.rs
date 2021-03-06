// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::Term;

use lumen_runtime_macros::native_implemented_function;

use crate::otp::erlang::apply_2;
use crate::scheduler::Scheduler;

#[native_implemented_function(spawn/1)]
pub fn native(process: &Process, function: Term) -> exception::Result {
    if function.is_function() {
        let arguments = &[function, Term::NIL];

        // The :badarity error is raised in the child process and not in the parent process, so the
        // child process must be running the equivalent of `apply(functon, [])`.
        let child_arc_process = Scheduler::spawn_code(
            process,
            Default::default(),
            apply_2::module(),
            apply_2::function(),
            arguments,
            apply_2::code,
        )?;

        Ok(child_arc_process.pid_term())
    } else {
        Err(badarg!().into())
    }
}
