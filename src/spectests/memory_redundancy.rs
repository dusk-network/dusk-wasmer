// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/memory_redundancy.wast
#![allow(
    warnings,
    dead_code
)]
use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::spectest_importobject;
use wabt::wat2wasm;


// Line 5
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (result i32)))
      (type (;2;) (func (result f32)))
      (type (;3;) (func (param i32) (result i32)))
      (func (;0;) (type 0)
        i32.const 0
        i32.const 0
        i32.store
        i32.const 4
        i32.const 0
        i32.store
        i32.const 8
        i32.const 0
        i32.store
        i32.const 12
        i32.const 0
        i32.store)
      (func (;1;) (type 1) (result i32)
        i32.const 8
        i32.const 0
        i32.store
        i32.const 5
        f32.const -0x0p+0 (;=-0;)
        f32.store
        i32.const 8
        i32.load)
      (func (;2;) (type 1) (result i32)
        (local i32 i32)
        i32.const 8
        i32.load
        set_local 0
        i32.const 5
        i32.const -2147483648
        i32.store
        i32.const 8
        i32.load
        set_local 1
        get_local 0
        get_local 1
        i32.add)
      (func (;3;) (type 2) (result f32)
        (local f32)
        i32.const 8
        i32.const 589505315
        i32.store
        i32.const 11
        f32.load
        set_local 0
        i32.const 8
        i32.const 0
        i32.store
        get_local 0)
      (func (;4;) (type 3) (param i32) (result i32)
        i32.const 16)
      (func (;5;) (type 1) (result i32)
        (local i32 i32)
        i32.const 4
        call 4
        set_local 0
        i32.const 4
        call 4
        set_local 1
        get_local 0
        i32.const 42
        i32.store
        get_local 1
        i32.const 43
        i32.store
        get_local 0
        i32.load)
      (memory (;0;) 1 1)
      (export \"zero_everything\" (func 0))
      (export \"test_store_to_load\" (func 1))
      (export \"test_redundant_load\" (func 2))
      (export \"test_dead_store\" (func 3))
      (export \"malloc\" (func 4))
      (export \"malloc_aliasing\" (func 5)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_1(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 59
fn c1_l59_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c1_l59_action_invoke");
    let func_index = match result_object.module.info.exports.get("test_store_to_load") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 128 as i32);
}

// Line 60
fn c2_l60_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c2_l60_action_invoke");
    let func_index = match result_object.module.info.exports.get("zero_everything") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    
}

// Line 61
fn c3_l61_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c3_l61_action_invoke");
    let func_index = match result_object.module.info.exports.get("test_redundant_load") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 128 as i32);
}

// Line 62
fn c4_l62_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c4_l62_action_invoke");
    let func_index = match result_object.module.info.exports.get("zero_everything") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    
}

// Line 63
fn c5_l63_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c5_l63_action_invoke");
    let func_index = match result_object.module.info.exports.get("test_dead_store") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 0.000000000000000000000000000000000000000000049 as f32);
}

// Line 64
fn c6_l64_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c6_l64_action_invoke");
    let func_index = match result_object.module.info.exports.get("zero_everything") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    
}

// Line 65
fn c7_l65_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c7_l65_action_invoke");
    let func_index = match result_object.module.info.exports.get("malloc_aliasing") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 43 as i32);
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_1(&result_object, &vm_context);
    c1_l59_action_invoke(&result_object, &vm_context);
    c2_l60_action_invoke(&result_object, &vm_context);
    c3_l61_action_invoke(&result_object, &vm_context);
    c4_l62_action_invoke(&result_object, &vm_context);
    c5_l63_action_invoke(&result_object, &vm_context);
    c6_l64_action_invoke(&result_object, &vm_context);
    c7_l65_action_invoke(&result_object, &vm_context);
}
