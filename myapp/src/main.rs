use std::fs::File;
use mylib1;
use mylib1::DataWrapper;

/// Force drop by ownership swallow
fn force_drop(_obj: DataWrapper) {
}

use std::io::{Error, Read};
use std::path::Path;


#[derive(Debug)]
struct Foo {
    bar: i32,
}


fn main() {
    println!("Starting...");
    let foo =  Foo{bar: 5};
    println!("Foo ist {} {:?}", 5, &foo);
    let obj: DataWrapper = mylib1::DataWrapper::new("NATIVE_DATA")
        .expect("data object");

    match obj.value() {
        Some(val) => println!("Foreign data has value: {}", val),
        None => println!("Foreign object has no value"),
    }

    force_drop(obj); // demonstrating
    // end of scope obj would be dropped anyway
}
