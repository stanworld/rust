mod lib;
use lib::*;
use std::rc::Rc;

fn main() {
    let a = ConcreteComponent;
    client(&a);
    let b = ConcreteDecorator::new(Rc::new(a));
    println!("this is the decorated result {}",b.do_work());
    client(&b);
}