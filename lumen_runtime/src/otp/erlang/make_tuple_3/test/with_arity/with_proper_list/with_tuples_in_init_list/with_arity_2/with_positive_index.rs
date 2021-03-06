use super::*;

#[test]
fn with_positive_index_greater_than_length_errors_badarg() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &strategy::process().prop_flat_map(|arc_process| {
                (
                    Just(arc_process.clone()),
                    (1_usize..3_usize),
                    strategy::term(arc_process.clone()),
                    (1_usize..3_usize),
                    strategy::term(arc_process),
                )
                    .prop_flat_map(
                        |(arc_process, len, default_value, index_offset, index_element)| {
                            (
                                Just(arc_process.clone()),
                                Just(len),
                                Just(default_value),
                                Just(
                                    arc_process
                                        .list_from_slice(&[arc_process
                                            .tuple_from_slice(&[
                                                arc_process.integer(len + index_offset).unwrap(),
                                                index_element,
                                            ])
                                            .unwrap()])
                                        .unwrap(),
                                ),
                            )
                        },
                    )
            }),
            |(arc_process, arity_usize, default_value, init_list)| {
                let arity = arc_process.integer(arity_usize).unwrap();

                prop_assert_eq!(
                    native(&arc_process, arity, default_value, init_list),
                    Err(badarg!().into())
                );

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn with_positive_index_less_than_or_equal_to_length_replaces_default_value_at_index_with_init_list_element(
) {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &strategy::process()
                .prop_flat_map(|arc_process| {
                    (
                        Just(arc_process.clone()),
                        (1_usize..3_usize),
                        strategy::term(arc_process),
                    )
                })
                .prop_flat_map(|(arc_process, len, default_value)| {
                    (
                        Just(arc_process.clone()),
                        Just(len),
                        Just(default_value),
                        0..len,
                        strategy::term(arc_process),
                    )
                }),
            |(arc_process, arity_usize, default_value, zero_based_index, init_list_element)| {
                let arity = arc_process.integer(arity_usize).unwrap();
                let one_based_index = arc_process.integer(zero_based_index + 1).unwrap();
                let init_list = arc_process
                    .list_from_slice(&[arc_process
                        .tuple_from_slice(&[one_based_index, init_list_element])
                        .unwrap()])
                    .unwrap();

                let result = native(&arc_process, arity, default_value, init_list);

                prop_assert!(result.is_ok());

                let tuple_term = result.unwrap();

                prop_assert!(tuple_term.is_tuple());

                let boxed_tuple: Boxed<Tuple> = tuple_term.try_into().unwrap();

                prop_assert_eq!(boxed_tuple.len(), arity_usize);

                for (index, element) in boxed_tuple.iter().enumerate() {
                    if index == zero_based_index {
                        prop_assert_eq!(element, init_list_element);
                    } else {
                        prop_assert_eq!(element, default_value);
                    }
                }

                Ok(())
            },
        )
        .unwrap();
}

// > If a position occurs more than once in the list, the term corresponding to the last occurrence
// > is used.
// - http://erlang.org/doc/man/erlang.html#make_tuple-3
#[test]
fn with_multiple_values_at_same_index_then_last_value_is_used() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &strategy::process()
                .prop_flat_map(|arc_process| {
                    (
                        Just(arc_process.clone()),
                        (1_usize..3_usize),
                        strategy::term(arc_process),
                    )
                })
                .prop_flat_map(|(arc_process, len, default_value)| {
                    (
                        Just(arc_process.clone()),
                        Just(len),
                        Just(default_value),
                        0..len,
                        strategy::term(arc_process.clone()),
                        strategy::term(arc_process),
                    )
                }),
            |(
                arc_process,
                arity_usize,
                default_value,
                init_list_zero_based_index,
                init_list_ignored_element,
                init_list_used_element,
            )| {
                let arity = arc_process.integer(arity_usize).unwrap();
                let init_list_one_base_index =
                    arc_process.integer(init_list_zero_based_index + 1).unwrap();
                let init_list = arc_process
                    .list_from_slice(&[
                        arc_process
                            .tuple_from_slice(&[
                                init_list_one_base_index,
                                init_list_ignored_element,
                            ])
                            .unwrap(),
                        arc_process
                            .tuple_from_slice(&[init_list_one_base_index, init_list_used_element])
                            .unwrap(),
                    ])
                    .unwrap();

                let result = native(&arc_process, arity, default_value, init_list);

                prop_assert!(result.is_ok());

                let tuple_term = result.unwrap();

                prop_assert!(tuple_term.is_tuple());

                let boxed_tuple: Boxed<Tuple> = tuple_term.try_into().unwrap();

                prop_assert_eq!(boxed_tuple.len(), arity_usize);

                for (index, element) in boxed_tuple.iter().enumerate() {
                    if index == init_list_zero_based_index {
                        prop_assert_eq!(element, init_list_used_element);
                    } else {
                        prop_assert_eq!(element, default_value);
                    }
                }

                Ok(())
            },
        )
        .unwrap();
}
