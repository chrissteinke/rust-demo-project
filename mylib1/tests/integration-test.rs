// using the public paths of library items
use mylib1::*;

/// test public api
///
///
#[test]
fn foreign() {
    let obj: DataWrapper = DataWrapper::new("FOREIGN_A")
        .expect("data");

    assert_eq!(obj.value(), Some(1009));
}

/// test public api
///
///
#[test]
fn native() {
    let obj: DataWrapper = DataWrapper::new("NATIVE")
        .expect("data");

    assert_eq!(obj.value(), Some(6));
}

