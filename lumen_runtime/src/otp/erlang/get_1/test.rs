use proptest::prop_assert_eq;
use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::erts::term::atom_unchecked;

use crate::otp::erlang::get_1::native;
use crate::scheduler::with_process_arc;
use crate::test::strategy;

#[test]
fn without_key_returns_undefined() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::term(arc_process.clone()), |key| {
                prop_assert_eq!(native(&arc_process, key), atom_unchecked("undefined"));

                Ok(())
            })
            .unwrap();
    });
}

#[test]
fn with_key_returns_value() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term(arc_process.clone()),
                    strategy::term(arc_process.clone()),
                ),
                |(key, value)| {
                    arc_process.put(key, value).unwrap();

                    prop_assert_eq!(arc_process.get_value_from_key(key), value);

                    prop_assert_eq!(native(&arc_process, key), value);

                    Ok(())
                },
            )
            .unwrap();
    });
}
