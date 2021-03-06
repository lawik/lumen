use liblumen_alloc::erts::term::{atom_unchecked, Atom};

use crate::module::NativeModule;

pub fn make_lumen_intrinsics() -> NativeModule {
    let mut native = NativeModule::new(Atom::try_from_str("lumen_intrinsics").unwrap());

    native.add_simple(Atom::try_from_str("println").unwrap(), 1, |_proc, args| {
        lumen_runtime::system::io::puts(&format!("{}", args[0]));
        Ok(atom_unchecked("ok"))
    });

    native.add_simple(Atom::try_from_str("format").unwrap(), 1, |proc, args| {
        let string = format!("{}", args[0]);
        let term = proc.binary_from_str(&string).unwrap();
        Ok(term)
    });

    native.add_simple(
        Atom::try_from_str("dump_process_heap").unwrap(),
        0,
        |proc, _args| {
            lumen_runtime::system::io::puts(&format!("{:?}", proc.acquire_heap()));
            Ok(atom_unchecked("ok"))
        },
    );

    native
}
