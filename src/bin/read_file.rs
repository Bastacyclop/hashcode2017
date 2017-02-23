extern crate hashcode2017 as code;

use std::env;
use code::*;

fn main() {
    let path = env::args().skip(1).next().unwrap();
    println!("{:#?}", Input::from_file(&path));
}
