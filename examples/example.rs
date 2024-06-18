use encodable::{decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Foo {
    a: f64,
    b: i64,
    c: bool,
}

fn main() {
    let foo = Foo {
        a: 1.0,
        b: 2,
        c: true,
    };

    let encoding = encode(&foo).unwrap();
    println!("{:?}", encoding);
    // Encoding { f: [1.0], i: [2], b: [true] }

    let foo_decoded: Foo = decode(&encoding).unwrap();
    assert_eq!(foo, foo_decoded);
}
