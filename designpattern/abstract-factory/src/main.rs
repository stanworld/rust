mod lib;

use lib::WhiteTable;

fn main() {
    let num = 10;
    let x = WhiteTable {
        width:10,
        height: 20,
    };

    println!("Hello, world! {} {}",num, x.width);
}