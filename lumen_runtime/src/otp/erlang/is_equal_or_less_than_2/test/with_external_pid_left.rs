use super::*;

#[test]
fn with_number_atom_reference_function_port_or_local_pid_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::pid::external(arc_process.clone()),
                    strategy::term::number_atom_reference_function_port_or_local_pid(
                        arc_process.clone(),
                    ),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(left, right), false.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_lesser_external_pid_right_returns_false() {
    is_equal_or_less_than(
        |_, process| process.external_pid_with_node_id(1, 1, 3).unwrap(),
        false,
    );
}

#[test]
fn with_same_value_external_pid_right_returns_true() {
    is_equal_or_less_than(
        |_, process| process.external_pid_with_node_id(1, 2, 3).unwrap(),
        true,
    );
}

#[test]
fn with_greater_external_pid_right_returns_true() {
    is_equal_or_less_than(
        |_, process| process.external_pid_with_node_id(1, 3, 3).unwrap(),
        true,
    );
}

#[test]
fn with_tuple_map_list_or_bitstring_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::pid::external(arc_process.clone()),
                    strategy::term::tuple_map_list_or_bitstring(arc_process),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(left, right), true.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn is_equal_or_less_than<R>(right: R, expected: bool)
where
    R: FnOnce(Term, &Process) -> Term,
{
    super::is_equal_or_less_than(
        |process| process.external_pid_with_node_id(1, 2, 3).unwrap(),
        right,
        expected,
    );
}
