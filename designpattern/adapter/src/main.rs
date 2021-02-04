mod lib;
use std::rc::Rc;
use lib::*;

fn main() {
    let adapter = Adapter{
        adaptee: Rc::new(Adaptee)
    };

    let a = client(&adapter);
    println!("{}",a);
}