use super::*;

#[test]
fn with_number_or_atom_second_returns_second() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::local_reference(arc_process.clone()),
                    strategy::term::number_or_atom(arc_process.clone()),
                ),
                |(first, second)| {
                    prop_assert_eq!(native(first, second), second);

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_lesser_local_reference_second_returns_second() {
    min(|_, process| process.reference(0).unwrap(), Second);
}

#[test]
fn with_same_local_reference_second_returns_first() {
    min(|first, _| first, First);
}

#[test]
fn with_same_value_local_reference_second_returns_first() {
    min(|_, process| process.reference(1).unwrap(), First);
}

#[test]
fn with_greater_local_reference_second_returns_first() {
    min(|_, process| process.reference(2).unwrap(), First);
}

#[test]
fn with_function_port_pid_tuple_map_list_or_bitstring_second_returns_first() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::local_reference(arc_process.clone()),
                    strategy::term::function_port_pid_tuple_map_list_or_bitstring(
                        arc_process.clone(),
                    ),
                ),
                |(first, second)| {
                    prop_assert_eq!(native(first, second), first);

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn min<R>(second: R, which: FirstSecond)
where
    R: FnOnce(Term, &Process) -> Term,
{
    super::min(|process| process.reference(1).unwrap(), second, which);
}
