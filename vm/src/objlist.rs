use super::objsequence::PySliceableSequence;
use super::objtype;
use super::pyobject::{
    AttributeProtocol, PyContext, PyFuncArgs, PyObjectKind, PyObjectRef, PyResult, TypeProtocol,
};
use super::vm::VirtualMachine;

// set_item:
pub fn set_item(
    vm: &mut VirtualMachine,
    l: &mut Vec<PyObjectRef>,
    idx: PyObjectRef,
    obj: PyObjectRef,
) -> PyResult {
    match &(idx.borrow()).kind {
        PyObjectKind::Integer { value } => {
            let pos_index = l.get_pos(*value);
            l[pos_index] = obj;
            Ok(vm.get_none())
        }
        _ => panic!(
            "TypeError: indexing type {:?} with index {:?} is not supported (yet?)",
            l, idx
        ),
    }
}

fn append(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("list.append called with: {:?}", args);
    arg_check!(vm, args, (list, Some(vm.ctx.list_type.clone())), (x, None));
    let mut list_obj = list.borrow_mut();
    if let PyObjectKind::List { ref mut elements } = list_obj.kind {
        elements.push(x.clone());
        Ok(vm.get_none())
    } else {
        Err(vm.new_type_error("list.append is called with no list".to_string()))
    }
}

fn clear(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("list.clear called with: {:?}", args);
    arg_check!(vm, args, (list, Some(vm.ctx.list_type.clone())));
    let mut list_obj = list.borrow_mut();
    if let PyObjectKind::List { ref mut elements } = list_obj.kind {
        elements.clear();
        Ok(vm.get_none())
    } else {
        Err(vm.new_type_error("list.clear is called with no list".to_string()))
    }
}

fn len(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("list.len called with: {:?}", args);
    arg_check!(vm, args, (list, Some(vm.ctx.list_type.clone())));
    let list_obj = list.borrow();
    if let PyObjectKind::List { ref elements } = list_obj.kind {
        Ok(vm.context().new_int(elements.len() as i32))
    } else {
        Err(vm.new_type_error("list.len is called with no list".to_string()))
    }
}

fn reverse(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    trace!("list.reverse called with: {:?}", args);
    arg_check!(vm, args, (list, Some(vm.ctx.list_type.clone())));
    let mut list_obj = list.borrow_mut();
    if let PyObjectKind::List { ref mut elements } = list_obj.kind {
        elements.reverse();
        Ok(vm.get_none())
    } else {
        Err(vm.new_type_error("list.reverse is called with no list".to_string()))
    }
}

pub fn init(context: &PyContext) {
    let ref list_type = context.list_type;
    list_type.set_attr("__len__", context.new_rustfunc(len));
    list_type.set_attr("append", context.new_rustfunc(append));
    list_type.set_attr("clear", context.new_rustfunc(clear));
    list_type.set_attr("reverse", context.new_rustfunc(reverse));
}
