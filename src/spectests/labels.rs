// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/labels.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, Instance};
use crate::runtime::types::{Value};

use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (type (;1;) (func (param i32) (result i32)))
      (func (;0;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          br 0 (;@1;)
          i32.const 0
        end)
      (func (;1;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            get_local 0
            i32.const 5
            i32.eq
            if  ;; label = @3
              get_local 0
              br 2 (;@1;)
            end
            br 0 (;@2;)
          end
        end)
      (func (;2;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            get_local 0
            i32.const 5
            i32.eq
            if  ;; label = @3
              br 1 (;@2;)
            end
            get_local 0
            i32.const 8
            i32.eq
            if  ;; label = @3
              get_local 0
              br 2 (;@1;)
            end
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            br 0 (;@2;)
          end
        end)
      (func (;3;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            get_local 0
            i32.const 5
            i32.eq
            if  ;; label = @3
              get_local 0
              br 2 (;@1;)
            end
            get_local 0
          end
        end)
      (func (;4;) (type 1) (param i32) (result i32)
        (local i32)
        i32.const 1
        set_local 1
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 1
            get_local 1
            i32.add
            set_local 1
            get_local 1
            get_local 0
            i32.gt_u
            if  ;; label = @3
              get_local 1
              br 2 (;@1;)
            end
            br 0 (;@2;)
          end
        end)
      (func (;5;) (type 0) (result i32)
        loop (result i32)  ;; label = @1
          i32.const 1
        end
        i32.const 1
        i32.add)
      (func (;6;) (type 0) (result i32)
        loop (result i32)  ;; label = @1
          i32.const 0
          br_if 0 (;@1;)
          i32.const 3
        end)
      (func (;7;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block  ;; label = @1
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
        end
        get_local 0)
      (func (;8;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block  ;; label = @1
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
        end
        get_local 0)
      (func (;9;) (type 1) (param i32) (result i32)
        block (result i32)  ;; label = @1
          i32.const 10
          block (result i32)  ;; label = @2
            block  ;; label = @3
              block  ;; label = @4
                block  ;; label = @5
                  block  ;; label = @6
                    block  ;; label = @7
                      get_local 0
                      br_table 4 (;@3;) 0 (;@7;) 1 (;@6;) 2 (;@5;) 3 (;@4;)
                    end
                  end
                  i32.const 2
                  br 3 (;@2;)
                end
                i32.const 3
                br 3 (;@1;)
              end
            end
            i32.const 5
          end
          i32.mul
        end)
      (func (;10;) (type 1) (param i32) (result i32)
        block  ;; label = @1
          block  ;; label = @2
            block  ;; label = @3
              get_local 0
              br_table 0 (;@3;) 1 (;@2;)
              br 2 (;@1;)
            end
            i32.const 0
            return
          end
        end
        i32.const 2)
      (func (;11;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          block  ;; label = @2
            i32.const 0
            br_if 0 (;@2;)
            get_local 0
            i32.const 1
            i32.or
            set_local 0
            i32.const 1
            br_if 0 (;@2;)
            get_local 0
            i32.const 2
            i32.or
            set_local 0
          end
          block (result i32)  ;; label = @2
            get_local 0
            i32.const 4
            i32.or
            set_local 0
            get_local 0
          end
          i32.const 0
          br_if 0 (;@1;)
          drop
          get_local 0
          i32.const 8
          i32.or
          set_local 0
          block (result i32)  ;; label = @2
            get_local 0
            i32.const 16
            i32.or
            set_local 0
            get_local 0
          end
          i32.const 1
          br_if 0 (;@1;)
          drop
          get_local 0
          i32.const 32
          i32.or
          set_local 0
          get_local 0
        end)
      (func (;12;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          block (result i32)  ;; label = @2
            i32.const 1
            br 0 (;@2;)
          end
          i32.const 1
          br_if 0 (;@1;)
          drop
          i32.const 1
        end)
      (func (;13;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          if  ;; label = @2
            block (result i32)  ;; label = @3
              i32.const 1
              br 0 (;@3;)
            end
            br 1 (;@1;)
          end
          i32.const 1
        end)
      (func (;14;) (type 0) (result i32)
        (local i32)
        block (result i32)  ;; label = @1
          block (result i32)  ;; label = @2
            i32.const 1
            set_local 0
            get_local 0
          end
          block (result i32)  ;; label = @2
            i32.const 2
            set_local 0
            get_local 0
          end
          br_if 0 (;@1;)
          drop
          i32.const 0
        end
        i32.const 0
        i32.add
        drop
        get_local 0)
      (func (;15;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          if  ;; label = @2
            block (result i32)  ;; label = @3
              i32.const 1
              br 0 (;@3;)
            end
            br 1 (;@1;)
          else
            block  ;; label = @3
              block (result i32)  ;; label = @4
                i32.const 1
                br 0 (;@4;)
              end
              drop
            end
          end
          i32.const 1
        end)
      (func (;16;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          br 0 (;@1;)
          i32.const 2
          i32.xor
        end)
      (func (;17;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          block (result i32)  ;; label = @2
            i32.const 2
          end
          block (result i32)  ;; label = @2
            i32.const 3
            br 0 (;@2;)
          end
          i32.add
        end)
      (export \"block\" (func 0))
      (export \"loop1\" (func 1))
      (export \"loop2\" (func 2))
      (export \"loop3\" (func 3))
      (export \"loop4\" (func 4))
      (export \"loop5\" (func 5))
      (export \"loop6\" (func 6))
      (export \"if\" (func 7))
      (export \"if2\" (func 8))
      (export \"switch\" (func 9))
      (export \"return\" (func 10))
      (export \"br_if0\" (func 11))
      (export \"br_if1\" (func 12))
      (export \"br_if2\" (func 13))
      (export \"br_if3\" (func 14))
      (export \"br\" (func 15))
      (export \"shadowing\" (func 16))
      (export \"redefinition\" (func 17)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 284
fn c1_l284_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c1_l284_action_invoke");
    let result = result_object.instance.call("c1_l284_action_invoke", &[]).expect("Missing result in c1_l284_action_invoke");
    assert_eq!(result, Some(Value::I32(1 as i32)));
}

// Line 285
fn c2_l285_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c2_l285_action_invoke");
    let result = result_object.instance.call("c2_l285_action_invoke", &[]).expect("Missing result in c2_l285_action_invoke");
    assert_eq!(result, Some(Value::I32(5 as i32)));
}

// Line 286
fn c3_l286_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c3_l286_action_invoke");
    let result = result_object.instance.call("c3_l286_action_invoke", &[]).expect("Missing result in c3_l286_action_invoke");
    assert_eq!(result, Some(Value::I32(8 as i32)));
}

// Line 287
fn c4_l287_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c4_l287_action_invoke");
    let result = result_object.instance.call("c4_l287_action_invoke", &[]).expect("Missing result in c4_l287_action_invoke");
    assert_eq!(result, Some(Value::I32(1 as i32)));
}

// Line 288
fn c5_l288_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c5_l288_action_invoke");
    let result = result_object.instance.call("c5_l288_action_invoke", &[Value::I32(8 as i32)]).expect("Missing result in c5_l288_action_invoke");
    assert_eq!(result, Some(Value::I32(16 as i32)));
}

// Line 289
fn c6_l289_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c6_l289_action_invoke");
    let result = result_object.instance.call("c6_l289_action_invoke", &[]).expect("Missing result in c6_l289_action_invoke");
    assert_eq!(result, Some(Value::I32(2 as i32)));
}

// Line 290
fn c7_l290_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c7_l290_action_invoke");
    let result = result_object.instance.call("c7_l290_action_invoke", &[]).expect("Missing result in c7_l290_action_invoke");
    assert_eq!(result, Some(Value::I32(3 as i32)));
}

// Line 291
fn c8_l291_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c8_l291_action_invoke");
    let result = result_object.instance.call("c8_l291_action_invoke", &[]).expect("Missing result in c8_l291_action_invoke");
    assert_eq!(result, Some(Value::I32(5 as i32)));
}

// Line 292
fn c9_l292_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c9_l292_action_invoke");
    let result = result_object.instance.call("c9_l292_action_invoke", &[]).expect("Missing result in c9_l292_action_invoke");
    assert_eq!(result, Some(Value::I32(5 as i32)));
}

// Line 293
fn c10_l293_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c10_l293_action_invoke");
    let result = result_object.instance.call("c10_l293_action_invoke", &[Value::I32(0 as i32)]).expect("Missing result in c10_l293_action_invoke");
    assert_eq!(result, Some(Value::I32(50 as i32)));
}

// Line 294
fn c11_l294_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c11_l294_action_invoke");
    let result = result_object.instance.call("c11_l294_action_invoke", &[Value::I32(1 as i32)]).expect("Missing result in c11_l294_action_invoke");
    assert_eq!(result, Some(Value::I32(20 as i32)));
}

// Line 295
fn c12_l295_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c12_l295_action_invoke");
    let result = result_object.instance.call("c12_l295_action_invoke", &[Value::I32(2 as i32)]).expect("Missing result in c12_l295_action_invoke");
    assert_eq!(result, Some(Value::I32(20 as i32)));
}

// Line 296
fn c13_l296_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c13_l296_action_invoke");
    let result = result_object.instance.call("c13_l296_action_invoke", &[Value::I32(3 as i32)]).expect("Missing result in c13_l296_action_invoke");
    assert_eq!(result, Some(Value::I32(3 as i32)));
}

// Line 297
fn c14_l297_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c14_l297_action_invoke");
    let result = result_object.instance.call("c14_l297_action_invoke", &[Value::I32(4 as i32)]).expect("Missing result in c14_l297_action_invoke");
    assert_eq!(result, Some(Value::I32(50 as i32)));
}

// Line 298
fn c15_l298_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c15_l298_action_invoke");
    let result = result_object.instance.call("c15_l298_action_invoke", &[Value::I32(5 as i32)]).expect("Missing result in c15_l298_action_invoke");
    assert_eq!(result, Some(Value::I32(50 as i32)));
}

// Line 299
fn c16_l299_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c16_l299_action_invoke");
    let result = result_object.instance.call("c16_l299_action_invoke", &[Value::I32(0 as i32)]).expect("Missing result in c16_l299_action_invoke");
    assert_eq!(result, Some(Value::I32(0 as i32)));
}

// Line 300
fn c17_l300_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c17_l300_action_invoke");
    let result = result_object.instance.call("c17_l300_action_invoke", &[Value::I32(1 as i32)]).expect("Missing result in c17_l300_action_invoke");
    assert_eq!(result, Some(Value::I32(2 as i32)));
}

// Line 301
fn c18_l301_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c18_l301_action_invoke");
    let result = result_object.instance.call("c18_l301_action_invoke", &[Value::I32(2 as i32)]).expect("Missing result in c18_l301_action_invoke");
    assert_eq!(result, Some(Value::I32(2 as i32)));
}

// Line 302
fn c19_l302_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c19_l302_action_invoke");
    let result = result_object.instance.call("c19_l302_action_invoke", &[]).expect("Missing result in c19_l302_action_invoke");
    assert_eq!(result, Some(Value::I32(29 as i32)));
}

// Line 303
fn c20_l303_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c20_l303_action_invoke");
    let result = result_object.instance.call("c20_l303_action_invoke", &[]).expect("Missing result in c20_l303_action_invoke");
    assert_eq!(result, Some(Value::I32(1 as i32)));
}

// Line 304
fn c21_l304_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c21_l304_action_invoke");
    let result = result_object.instance.call("c21_l304_action_invoke", &[]).expect("Missing result in c21_l304_action_invoke");
    assert_eq!(result, Some(Value::I32(1 as i32)));
}

// Line 305
fn c22_l305_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c22_l305_action_invoke");
    let result = result_object.instance.call("c22_l305_action_invoke", &[]).expect("Missing result in c22_l305_action_invoke");
    assert_eq!(result, Some(Value::I32(2 as i32)));
}

// Line 306
fn c23_l306_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c23_l306_action_invoke");
    let result = result_object.instance.call("c23_l306_action_invoke", &[]).expect("Missing result in c23_l306_action_invoke");
    assert_eq!(result, Some(Value::I32(1 as i32)));
}

// Line 307
fn c24_l307_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c24_l307_action_invoke");
    let result = result_object.instance.call("c24_l307_action_invoke", &[]).expect("Missing result in c24_l307_action_invoke");
    assert_eq!(result, Some(Value::I32(1 as i32)));
}

// Line 308
fn c25_l308_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c25_l308_action_invoke");
    let result = result_object.instance.call("c25_l308_action_invoke", &[]).expect("Missing result in c25_l308_action_invoke");
    assert_eq!(result, Some(Value::I32(5 as i32)));
}

// Line 311
#[test]
fn c26_l311_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 0, 2, 64, 65, 1, 13, 0, 140, 1, 11, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 315
#[test]
fn c27_l315_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 16, 1, 14, 0, 2, 64, 67, 0, 0, 0, 0, 65, 1, 13, 0, 11, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 319
#[test]
fn c28_l319_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 16, 1, 14, 0, 2, 64, 67, 0, 0, 0, 0, 65, 1, 13, 0, 11, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let mut result_object = create_module_1();
    // We group the calls together
    start_module_1(&mut result_object);
    c1_l284_action_invoke(&mut result_object);
    c2_l285_action_invoke(&mut result_object);
    c3_l286_action_invoke(&mut result_object);
    c4_l287_action_invoke(&mut result_object);
    c5_l288_action_invoke(&mut result_object);
    c6_l289_action_invoke(&mut result_object);
    c7_l290_action_invoke(&mut result_object);
    c8_l291_action_invoke(&mut result_object);
    c9_l292_action_invoke(&mut result_object);
    c10_l293_action_invoke(&mut result_object);
    c11_l294_action_invoke(&mut result_object);
    c12_l295_action_invoke(&mut result_object);
    c13_l296_action_invoke(&mut result_object);
    c14_l297_action_invoke(&mut result_object);
    c15_l298_action_invoke(&mut result_object);
    c16_l299_action_invoke(&mut result_object);
    c17_l300_action_invoke(&mut result_object);
    c18_l301_action_invoke(&mut result_object);
    c19_l302_action_invoke(&mut result_object);
    c20_l303_action_invoke(&mut result_object);
    c21_l304_action_invoke(&mut result_object);
    c22_l305_action_invoke(&mut result_object);
    c23_l306_action_invoke(&mut result_object);
    c24_l307_action_invoke(&mut result_object);
    c25_l308_action_invoke(&mut result_object);
}
