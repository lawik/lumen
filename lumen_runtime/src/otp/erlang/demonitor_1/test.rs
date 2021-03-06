mod with_reference;

use proptest::prop_assert_eq;
use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::badarg;
use liblumen_alloc::erts::term::Term;

use crate::otp::erlang::demonitor_1::native;
use crate::scheduler::with_process_arc;
use crate::test::strategy;

#[test]
fn without_reference_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::is_not_reference(arc_process.clone()),
                |reference| {
                    prop_assert_eq!(native(&arc_process, reference), Err(badarg!().into()));

                    Ok(())
                },
            )
            .unwrap();
    });
}
