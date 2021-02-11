mod lib;

use crate::lib::MyWindow;
use crate::lib::client;

fn main() {
    let result = client(Box::new(MyWindow::new()));
    println!("Yes, {}", result);
}