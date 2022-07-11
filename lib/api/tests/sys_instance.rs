#[cfg(feature = "sys")]
mod sys {
    use anyhow::Result;
    use wasmer::Context as WasmerContext;
    use wasmer::*;

    #[test]
    fn exports_work_after_multiple_instances_have_been_freed() -> Result<()> {
        let store = Store::default();
        let mut ctx = WasmerContext::new(&store, (), ());
        let module = Module::new(
            &store,
            "
    (module
      (type $sum_t (func (param i32 i32) (result i32)))
      (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
        local.get $x
        local.get $y
        i32.add)
      (export \"sum\" (func $sum_f)))
",
        )?;

        let imports = Imports::new();
        let instance = Instance::new(&mut ctx, &module, &imports)?;
        let instance2 = instance.clone();
        let instance3 = instance.clone();

        // The function is cloned to “break” the connection with `instance`.
        let sum = instance.exports.get_function("sum")?.clone();

        drop(instance);
        drop(instance2);
        drop(instance3);

        // All instances have been dropped, but `sum` continues to work!
        assert_eq!(
            sum.call(&mut ctx, &[Value::I32(1), Value::I32(2)])?
                .into_vec(),
            vec![Value::I32(3)],
        );

        Ok(())
    }

    #[test]
    fn unit_native_function_env() -> Result<()> {
        let store = Store::default();
        #[derive(Clone)]
        struct Env {
            multiplier: u32,
        }

        fn imported_fn(
            ctx: ContextMut<(), Env>,
            args: &[Value],
        ) -> Result<Vec<Value>, RuntimeError> {
            let value = ctx.data().multiplier * args[0].unwrap_i32() as u32;
            Ok(vec![Value::I32(value as _)])
        }

        let env = Env { multiplier: 3 };
        let mut ctx = WasmerContext::new(&store, (), env);
        let imported_signature = FunctionType::new(vec![Type::I32], vec![Type::I32]);
        let imported = Function::new(&mut ctx, imported_signature, imported_fn);

        let expected = vec![Value::I32(12)].into_boxed_slice();
        let result = imported.call(&mut ctx, &[Value::I32(4)])?;
        assert_eq!(result, expected);

        Ok(())
    }
}
