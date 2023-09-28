use std::mem::size_of;

use tracing::info;

use super::pointers;

pub struct CFunction {
    pub pad: [char; 192],
}

impl CFunction {}

type VoidPtr = *const ();
type ScriptFunction<F, RET> = dyn Fn(F, *mut GlobalScriptingContext, *mut RET);

unsafe fn register_name_string(name: &'static str) -> i32 {
    let pointer = pointers::REGISTER_NAME_FN as VoidPtr;
    let register_name_function: fn(*mut i32, &'static str) -> i32 = std::mem::transmute(pointer);

    let mut name_id = 0;
    register_name_function(&mut name_id, name);

    name_id
}

unsafe fn allocate_cfunction<F>(name_id: i32, function: *mut F) -> *mut CFunction {
    // first param: size
    // second param: maybe alignment
    let pointer_1 = pointers::MEMORY_ALLOCATION_FN as VoidPtr;
    let perform_memory_allocation: fn(usize, usize) -> *mut CFunction =
        std::mem::transmute(pointer_1);

    let pointer_2 = pointers::CFUNCTION_CONSTRUCTOR_FN as VoidPtr;
    let cfunction_constructor: fn(*mut CFunction, &i32, *mut F) -> *mut CFunction =
        std::mem::transmute(pointer_2);

    let memory: *mut CFunction = perform_memory_allocation(size_of::<CFunction>(), 16);

    if memory.is_null() {
        info!("failed to allocate function memory");
        panic!("failed to allocate function memory");
    }

    std::ptr::write_bytes(memory, 0, size_of::<CFunction>());
    cfunction_constructor(memory, &name_id, function);

    memory
}

struct GlobalScriptingContext {
    // pub some_value: i64,
    // pub pad: [char; 0x28],
    // pub some_stack: *mut u8,
}

unsafe fn register_function(cfunction: *mut CFunction) {
    let pointer_1 = pointers::SCRIPTING_CONTEXT_SINGLETON_FN as VoidPtr;
    let scripting_context_singleton: fn() -> *mut GlobalScriptingContext =
        std::mem::transmute(pointer_1);

    let pointer_2 = pointers::REGISTER_SCRIPT_FUNCTION_FN as VoidPtr;
    let register_script_function: fn(*mut GlobalScriptingContext, *mut CFunction) =
        std::mem::transmute(pointer_2);

    let context = scripting_context_singleton();
    // let ctx = &mut *context;
    // ctx.some_stack = ctx.some_stack.wrapping_add(1);
    register_script_function(context, cfunction);
}

pub unsafe fn perform_script_function_registration<F>(name: &'static str, function: *mut F) {
    let id = register_name_string(name);
    let cfunction = allocate_cfunction(id, function);
    register_function(cfunction);
}
