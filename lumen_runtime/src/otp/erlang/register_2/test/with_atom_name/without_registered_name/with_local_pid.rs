use super::*;

#[test]
fn without_process() {
    with_process_arc(|process_arc| {
        let pid_or_port = next_pid();

        assert_badarg!(native(process_arc, registered_name(), pid_or_port,));
    });
}

#[test]
fn with_same_process() {
    with_process_arc(|process_arc| {
        let name = registered_name();
        let name_atom: Atom = name.try_into().unwrap();
        let pid_or_port = unsafe { process_arc.pid().as_term() };

        assert_eq!(
            native(process_arc.clone(), name, pid_or_port),
            Ok(true.into())
        );
        assert_eq!(*process_arc.registered_name.read(), Some(name_atom));

        let name_atom: Atom = name.try_into().unwrap();

        assert_eq!(registry::atom_to_process(&name_atom), Some(process_arc));
    });
}

#[test]
fn with_different_process() {
    with_process_arc(|process_arc| {
        let name = registered_name();

        let another_process_arc = process::test(&process_arc);
        let pid_or_port = unsafe { another_process_arc.pid().as_term() };

        assert_eq!(
            erlang::register_2::native(process_arc, name, pid_or_port),
            Ok(true.into())
        );

        let name_atom: Atom = name.try_into().unwrap();

        assert_eq!(*another_process_arc.registered_name.read(), Some(name_atom));

        assert_eq!(
            registry::atom_to_process(&name_atom),
            Some(another_process_arc)
        );
    });
}
