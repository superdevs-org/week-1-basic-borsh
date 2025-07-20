use std::fmt::Error;
use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeserializeNumberStruct,Debug,PartialEq)]
struct Swap {
    qty_1: i8 ,
    qty_2: u32 
}

fn main() {
    //println!("Hello, world!");
    let s = Swap {
        qty_1: -7,
        qty_2: 7
    };
    let mut data = vec![];
     s.serialize(&mut data).unwrap();
    let decoded = Swap::deserialize(&mut &data[..]).unwrap();
    print!("Original = {:?}" , s);
    print!("decoded = {:?}" , decoded);
    assert_eq!(s,decoded);
}
