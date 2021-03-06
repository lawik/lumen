use super::*;

use proptest::strategy::Strategy;

#[test]
fn with_number_atom_reference_function_port_pid_tuple_map_or_list_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::binary::heap(arc_process.clone()),
                    strategy::term(arc_process.clone()).prop_filter("Right must be number, atom, reference, function, port, pid, tuple, map, or list", |right| right.is_number() || right.is_atom() || right.is_reference() || right.is_closure() || right.is_port() || right.is_pid() || right.is_tuple() || right.is_list()),
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
fn with_prefix_heap_binary_right_returns_false() {
    is_equal_or_less_than(|_, process| process.binary_from_bytes(&[1]).unwrap(), false);
}

#[test]
fn with_same_length_heap_binary_with_lesser_byte_right_returns_false() {
    is_equal_or_less_than(|_, process| process.binary_from_bytes(&[0]).unwrap(), false);
}

#[test]
fn with_longer_heap_binary_with_lesser_byte_right_returns_false() {
    is_equal_or_less_than(
        |_, process| process.binary_from_bytes(&[0, 1, 2]).unwrap(),
        false,
    );
}

#[test]
fn with_same_value_heap_binary_right_returns_true() {
    super::is_equal_or_less_than(
        |process| {
            let mut heap = process.acquire_heap();

            let original = heap.binary_from_bytes(&[1]).unwrap();

            heap.subbinary_from_original(original, 0, 0, 1, 0).unwrap()
        },
        |_, process| process.binary_from_bytes(&[1]).unwrap(),
        true,
    )
}

#[test]
fn with_shorter_heap_binary_with_greater_byte_right_returns_true() {
    is_equal_or_less_than(|_, process| process.binary_from_bytes(&[2]).unwrap(), true);
}

#[test]
fn with_heap_binary_with_greater_byte_right_returns_true() {
    is_equal_or_less_than(
        |_, process| process.binary_from_bytes(&[2, 1]).unwrap(),
        true,
    );
}

#[test]
fn with_heap_binary_with_greater_byte_than_bits_right_returns_true() {
    is_equal_or_less_than(
        |_, process| process.binary_from_bytes(&[1, 0b1000_0000]).unwrap(),
        true,
    );
}

#[test]
fn with_prefix_subbinary_right_returns_false() {
    is_equal_or_less_than(
        |_, process| {
            let mut heap = process.acquire_heap();

            let original = heap.binary_from_bytes(&[1]).unwrap();

            heap.subbinary_from_original(original, 0, 0, 1, 0).unwrap()
        },
        false,
    );
}

#[test]
fn with_same_length_subbinary_with_lesser_byte_right_returns_false() {
    is_equal_or_less_than(
        |_, process| {
            let mut heap = process.acquire_heap();
            let original = heap.binary_from_bytes(&[0, 1]).unwrap();

            heap.subbinary_from_original(original, 0, 0, 2, 0).unwrap()
        },
        false,
    );
}

#[test]
fn with_longer_subbinary_with_lesser_byte_right_returns_false() {
    is_equal_or_less_than(|_, process| bitstring!(0, 1, 0b10 :: 2, &process), false);
}

#[test]
fn with_same_value_subbinary_right_returns_true() {
    is_equal_or_less_than(|_, process| bitstring!(1, 1 :: 2, &process), true);
}

#[test]
fn with_shorter_subbinary_with_greater_byte_right_returns_true() {
    is_equal_or_less_than(
        |_, process| {
            let mut heap = process.acquire_heap();
            let original = heap.binary_from_bytes(&[2]).unwrap();

            heap.subbinary_from_original(original, 0, 0, 1, 0).unwrap()
        },
        true,
    );
}

#[test]
fn with_subbinary_with_greater_byte_right_returns_true() {
    is_equal_or_less_than(
        |_, process| {
            let mut heap = process.acquire_heap();
            let original = heap.binary_from_bytes(&[2, 1]).unwrap();

            heap.subbinary_from_original(original, 0, 0, 2, 0).unwrap()
        },
        true,
    );
}

#[test]
fn with_subbinary_with_different_greater_byte_right_returns_true() {
    is_equal_or_less_than(
        |_, process| {
            let mut heap = process.acquire_heap();
            let original = heap.binary_from_bytes(&[1, 2]).unwrap();

            heap.subbinary_from_original(original, 0, 0, 2, 0).unwrap()
        },
        true,
    );
}

#[test]
fn with_subbinary_with_value_with_shorter_length_returns_true() {
    is_equal_or_less_than(|_, process| bitstring!(1, 1 :: 1, &process), true)
}

fn is_equal_or_less_than<R>(right: R, expected: bool)
where
    R: FnOnce(Term, &Process) -> Term,
{
    super::is_equal_or_less_than(|process| bitstring!(1, 1 :: 2, &process), right, expected);
}
