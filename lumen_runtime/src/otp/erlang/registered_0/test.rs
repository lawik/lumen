// because registry is global and tests are concurrent, there is no way to test for completely
// empty registry

use liblumen_alloc::erts::term::{atom_unchecked, TypedTerm};

use crate::otp::erlang;
use crate::otp::erlang::registered_0::native;
use crate::scheduler::with_process_arc;

#[test]
fn includes_registered_process_name() {
    with_process_arc(|process_arc| {
        let name = atom_unchecked("registered_process_name");

        let before = native(&process_arc).unwrap();

        match before.to_typed_term().unwrap() {
            TypedTerm::Nil => (),
            TypedTerm::List(before_cons) => {
                assert!(!before_cons.contains(name));
            }
            typed_term => panic!("Wrong TypedTerm ({:?})", typed_term),
        }

        assert_eq!(
            erlang::register_2::native(process_arc.clone(), name, process_arc.pid_term()),
            Ok(true.into())
        );

        let after = native(&process_arc).unwrap();

        match after.to_typed_term().unwrap() {
            TypedTerm::List(after_cons) => assert!(after_cons.contains(name)),
            typed_term => panic!("Wrong TypedTerm ({:?})", typed_term),
        }
    });
}
