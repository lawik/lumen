use super::*;

use liblumen_alloc::erts::term::atom_unchecked;

#[test]
fn without_key_returns_default() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term(arc_process.clone()),
                    strategy::term(arc_process.clone()),
                )
                    .prop_filter("Key and non-key must be different", |(key, non_key)| {
                        key != non_key
                    })
                    .prop_map(|(key, non_key)| {
                        let value = atom_unchecked("value");

                        (
                            non_key,
                            arc_process.map_from_slice(&[(key, value)]).unwrap(),
                        )
                    }),
                |(key, map)| {
                    let default = atom_unchecked("default");
                    prop_assert_eq!(native(&arc_process, key, map, default), Ok(default.into()));

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_key_returns_value() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term(arc_process.clone()).prop_map(|key| {
                    let value = atom_unchecked("value");

                    (key, arc_process.map_from_slice(&[(key, value)]).unwrap())
                }),
                |(key, map)| {
                    let default = atom_unchecked("default");
                    let value = atom_unchecked("value");
                    prop_assert_eq!(native(&arc_process, key, map, default), Ok(value.into()));

                    Ok(())
                },
            )
            .unwrap();
    });
}
