mod lib;
use lib::get_singleton_item;

fn main() {
    let a = get_singleton_item().lock().unwrap();
    println!("{}",a.p1);
}
