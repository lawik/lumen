use std::convert::TryInto;
use std::sync::Arc;

use liblumen_alloc::erts::exception::system::Alloc;
use liblumen_alloc::erts::process::code::stack::frame::{Frame, Placement};
use liblumen_alloc::erts::process::{code, Process};
use liblumen_alloc::erts::term::{atom_unchecked, Boxed, Term, Tuple};
use liblumen_alloc::ModuleFunctionArity;

use super::label_3;

pub fn place_frame_with_arguments(
    process: &Process,
    placement: Placement,
    document: Term,
) -> Result<(), Alloc> {
    process.stack_push(document)?;
    process.place_frame(frame(), placement);

    Ok(())
}

// Private

// ```elixir
// # label 2
// # pushed to stack: (document)
// # returned form call: {:ok, old_child}
// # full stack: ({:ok, old_child}, document)
// # returns: {:ok, parent}
// {:ok, parent} = Lumen.Web.Document.create_element(document, "div")
// :ok = Lumen.Web.Node.append_child(parent, old_child)
// {:error, :hierarchy_request} = Lumen.Web.replace_child(parent, old_child, parent)
// ```
fn code(arc_process: &Arc<Process>) -> code::Result {
    arc_process.reduce();

    let ok_old_child = arc_process.stack_pop().unwrap();
    assert!(
        ok_old_child.is_tuple(),
        "ok_old_child ({:?}) is not a tuple",
        ok_old_child
    );
    let ok_old_child_tuple: Boxed<Tuple> = ok_old_child.try_into().unwrap();
    assert_eq!(ok_old_child_tuple.len(), 2);
    assert_eq!(ok_old_child_tuple[0], atom_unchecked("ok"));
    let old_child = ok_old_child_tuple[1];
    assert!(old_child.is_resource_reference());

    let document = arc_process.stack_pop().unwrap();
    assert!(document.is_resource_reference());

    label_3::place_frame_with_arguments(arc_process, Placement::Replace, old_child)?;

    let parent_tag = arc_process.binary_from_str("div")?;
    lumen_web::document::create_element_2::place_frame_with_arguments(
        arc_process,
        Placement::Push,
        document,
        parent_tag,
    )?;

    Process::call_code(arc_process)
}

fn frame() -> Frame {
    let module_function_arity = Arc::new(ModuleFunctionArity {
        module: super::module(),
        function: super::function(),
        arity: 0,
    });

    Frame::new(module_function_arity, code)
}
