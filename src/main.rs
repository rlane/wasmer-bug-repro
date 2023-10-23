use testresult::TestResult;
use wasmer::*;

fn main() {
    web_sys::console::log_1(&"Starting test".into());
    calling_function_exports().unwrap();
    web_sys::console::log_1(&"Finished test".into());
}

fn calling_function_exports() -> TestResult {
    let mut store = Store::default();
    let wat = r#"(module
    (func (export "add") (param $lhs i32) (param $rhs i32) (result i32)
        local.get $lhs
        local.get $rhs
        i32.add)
)"#;
    let module = Module::new(&store, wat)?;
    let imports = imports! {};
    let instance = Instance::new(&mut store, &module, &imports)?;

    let add: TypedFunction<(i32, i32), i32> =
        instance.exports.get_typed_function(&mut store, "add")?;

    let result = add.call(&mut store, 10, 20)?;
    assert_eq!(result, 30);

    Ok(())
}
