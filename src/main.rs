extern crate console_error_panic_hook;

use std::panic;
use wasmer::*;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    web_sys::console::log_1(&"Starting test".into());
    calling_function_exports();
    web_sys::console::log_1(&"Finished test".into());
}

fn calling_function_exports() {
    let mut store = Store::default();
    let wat = r#"(module
    (func (export "add") (param $lhs i32) (param $rhs i32) (result i32)
        local.get $lhs
        local.get $rhs
        i32.add)
)"#;
    let module = Module::new(&store, wat).unwrap();
    let imports = imports! {};
    let instance = Instance::new(&mut store, &module, &imports).unwrap();

    let add: TypedFunction<(i32, i32), i32> = instance
        .exports
        .get_typed_function(&mut store, "add")
        .expect("get_typed_function failed");

    let result = add.call(&mut store, 10, 20).unwrap();
    assert_eq!(result, 30);
}
