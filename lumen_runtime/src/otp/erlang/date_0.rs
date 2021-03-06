#[cfg(test)]
mod test;

use crate::time::datetime;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use lumen_runtime_macros::native_implemented_function;

#[native_implemented_function(date/0)]
pub fn native(process: &Process) -> exception::Result {
    let date: [usize; 3] = datetime::local_date();

    process
        .tuple_from_slice(&[
            process.integer(date[0])?,
            process.integer(date[1])?,
            process.integer(date[2])?,
        ])
        .map_err(|error| error.into())
}
