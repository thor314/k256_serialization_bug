use serde::{Serialize, Deserialize};
mod k256_serde;
use k256_serde::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyStruct {
    a: usize,
}
impl MyStruct {
    fn new() -> Self {
        Self { a: 0 }
    }
}
/// I make some data, serialize it, and then deserialize it to sanity check serialization!
fn i_do_a_serialize() {
    let data0: MyStruct = MyStruct::new();
    println!("data: {data0:?}");
    let data1: String = serde_json::to_string(&data0).unwrap();
    println!("data1: {data1:?}");
    let data2: MyStruct = serde_json::from_str(&data1).unwrap();
    println!("data2: {data2:?}");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HairyStruct {
    a: usize,
    b: ProjectivePoint,
}

impl HairyStruct {
    fn new() -> Self {
        Self {
            a: 0,
            b: ProjectivePoint::generator(),
        }
    }
}
fn i_dont_serialize_good() {
    let data0: HairyStruct = HairyStruct::new();
    println!("data: {data0:?}");
    let data1: String = serde_json::to_string(&data0).unwrap();
    println!("data1: {data1:?}");
    let data2: HairyStruct = serde_json::from_str(&data1).unwrap();
    println!("data2: {data2:?}");
}

fn main() {
    i_do_a_serialize();
    println!("\n beep boop breakin things \n");
    i_dont_serialize_good();
}
