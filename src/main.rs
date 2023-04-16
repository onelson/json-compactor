use serde_json::{value::RawValue, Value};

mod parser;
use crate::parser::compactor;

fn main() {
    let s = std::env::args()
        .nth(1)
        .expect("first arg must be json string");

    let v: Value = serde_json::from_str(&s).expect("parse Value");
    let rv: Box<RawValue> =
        RawValue::from_string(compactor(&s).expect("compactor")).expect("parse RawValue");

    let vc = serde_json::to_string(&v).expect("Value compact");
    let rvc = serde_json::to_string(&rv).expect("RawValue compact");

    debug_assert_eq!(&vc, &rvc);

    println!("Compact from Value:");
    println!("{}", vc);
    println!("Compact from RawValue:");
    println!("{}", rvc);
}
