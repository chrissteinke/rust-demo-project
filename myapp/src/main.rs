use mylib1;
use mylib1::DataWrapper;

/// Force drop by ownership swallow
fn force_drop(_obj: DataWrapper) {
}

fn main() {
    println!("Starting...");
    let obj: DataWrapper = mylib1::DataWrapper::new("NATIVE_DATA")
        .expect("data object");

    match obj.value() {
        Some(val) => println!("Foreign data has value: {}", val),
        None => println!("Foreign object has no value"),
    }

    force_drop(obj); // demonstrating
    // end of scope obj would be dropped anyway
}
