mod lib;

use lib::*;

fn main() {

    let builder1 = Box::new(WhiteHouseBuilder::new());
    let builder2 = Box::new(BlueHouseBuilder::new());
    director(builder1);
    director(builder2);
    
}